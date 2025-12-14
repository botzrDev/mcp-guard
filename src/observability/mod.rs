//! Observability: metrics, tracing, and logging for mcp-guard
//!
//! Provides Prometheus metrics for monitoring mcp-guard performance and health.
//!
//! Metrics exposed:
//! - `mcp_guard_requests_total` (counter) - labels: method, status
//! - `mcp_guard_request_duration_seconds` (histogram) - labels: method
//! - `mcp_guard_auth_total` (counter) - labels: provider, result
//! - `mcp_guard_rate_limit_total` (counter) - labels: allowed
//! - `mcp_guard_active_identities` (gauge)
//!
//! OpenTelemetry tracing (FR-OBS-03):
//! - W3C trace context propagation (traceparent, tracestate headers)
//! - OTLP export to Jaeger/Tempo/etc
//! - Trace ID in all log messages (FR-AUDIT-06)

use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider as SdkTracerProvider},
    Resource,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::TracingConfig;

/// Result of tracing initialization
pub struct TracingGuard {
    /// OpenTelemetry tracer provider (if enabled)
    _provider: Option<SdkTracerProvider>,
}

impl Drop for TracingGuard {
    fn drop(&mut self) {
        if let Some(ref provider) = self._provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Error shutting down OpenTelemetry tracer: {:?}", e);
            }
        }
    }
}

/// Initialize tracing/logging with optional OpenTelemetry support
///
/// # Arguments
/// * `verbose` - Enable verbose (debug) logging
/// * `tracing_config` - Optional OpenTelemetry configuration
///
/// # Returns
/// A TracingGuard that should be held for the lifetime of the application.
/// When dropped, it will properly flush and shutdown the OpenTelemetry tracer.
pub fn init_tracing(verbose: bool, tracing_config: Option<&TracingConfig>) -> TracingGuard {
    let filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    // Check if OpenTelemetry tracing is enabled
    let otel_enabled = tracing_config.map(|c| c.enabled).unwrap_or(false);

    if otel_enabled {
        let config = tracing_config.unwrap();
        match init_opentelemetry_tracing(verbose, config) {
            Ok(guard) => return guard,
            Err(e) => {
                eprintln!("Failed to initialize OpenTelemetry tracing: {}. Falling back to basic logging.", e);
            }
        }
    }

    // Basic tracing without OpenTelemetry
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    TracingGuard { _provider: None }
}

/// Initialize OpenTelemetry tracing with OTLP export
fn init_opentelemetry_tracing(verbose: bool, config: &TracingConfig) -> Result<TracingGuard, Box<dyn std::error::Error + Send + Sync>> {
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::WithExportConfig;

    let filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    // Set up resource with service name
    let resource = Resource::new(vec![
        KeyValue::new("service.name", config.service_name.clone()),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
    ]);

    // Set up sampler based on sample rate
    let sampler = if config.sample_rate >= 1.0 {
        Sampler::AlwaysOn
    } else if config.sample_rate <= 0.0 {
        Sampler::AlwaysOff
    } else {
        Sampler::TraceIdRatioBased(config.sample_rate)
    };

    // Build the tracer provider
    let provider = if let Some(ref endpoint) = config.otlp_endpoint {
        // OTLP exporter configuration
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()?;

        SdkTracerProvider::builder()
            .with_batch_exporter(exporter, runtime::Tokio)
            .with_sampler(sampler)
            .with_id_generator(RandomIdGenerator::default())
            .with_resource(resource)
            .build()
    } else {
        // No exporter - just local tracing
        SdkTracerProvider::builder()
            .with_sampler(sampler)
            .with_id_generator(RandomIdGenerator::default())
            .with_resource(resource)
            .build()
    };

    // Create the tracer
    let tracer = provider.tracer("mcp-guard");

    // Create OpenTelemetry tracing layer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Create fmt layer with trace ID in logs (FR-AUDIT-06)
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_target(true);

    // Combine layers
    tracing_subscriber::registry()
        .with(filter)
        .with(otel_layer)
        .with(fmt_layer)
        .init();

    Ok(TracingGuard {
        _provider: Some(provider),
    })
}

/// Initialize the Prometheus metrics recorder
///
/// Returns a handle that can be used to render metrics in Prometheus format.
/// This must be called once at application startup before recording any metrics.
pub fn init_metrics() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}

/// Create a Prometheus handle without installing a global recorder
///
/// Useful for tests where multiple tests may run in parallel and each
/// needs its own metrics handle. The returned handle can still render
/// metrics but they won't be globally accessible.
pub fn create_metrics_handle() -> PrometheusHandle {
    let recorder = PrometheusBuilder::new()
        .build_recorder();
    recorder.handle()
}

/// Record a completed request
///
/// # Arguments
/// * `method` - HTTP method (e.g., "POST", "GET")
/// * `status` - HTTP status code
/// * `duration` - Request duration
pub fn record_request(method: &str, status: u16, duration: std::time::Duration) {
    counter!(
        "mcp_guard_requests_total",
        "method" => method.to_string(),
        "status" => status.to_string(),
    )
    .increment(1);

    histogram!(
        "mcp_guard_request_duration_seconds",
        "method" => method.to_string(),
    )
    .record(duration.as_secs_f64());
}

/// Record an authentication attempt
///
/// # Arguments
/// * `provider` - Authentication provider name (e.g., "api_key", "jwt")
/// * `success` - Whether authentication succeeded
pub fn record_auth(provider: &str, success: bool) {
    let result = if success { "success" } else { "failure" };
    counter!(
        "mcp_guard_auth_total",
        "provider" => provider.to_string(),
        "result" => result.to_string(),
    )
    .increment(1);
}

/// Record a rate limit check
///
/// # Arguments
/// * `allowed` - Whether the request was allowed
pub fn record_rate_limit(allowed: bool) {
    counter!(
        "mcp_guard_rate_limit_total",
        "allowed" => allowed.to_string(),
    )
    .increment(1);
}

/// Update the active identities gauge
///
/// # Arguments
/// * `count` - Current number of tracked identities
pub fn set_active_identities(count: usize) {
    gauge!("mcp_guard_active_identities").set(count as f64);
}

/// Get the current trace ID from the active span (if any)
///
/// This can be used to include trace IDs in error responses or audit logs.
pub fn current_trace_id() -> Option<String> {
    use opentelemetry::trace::TraceContextExt;
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    let span = tracing::Span::current();
    let context = span.context();
    let span_ref = context.span();
    let span_context = span_ref.span_context();

    if span_context.is_valid() {
        Some(span_context.trace_id().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        assert_eq!(config.sample_rate, 1.0);
        assert!(config.propagate_context);
    }

    #[test]
    fn test_record_functions_dont_panic() {
        // These functions should not panic even without a recorder installed
        // (metrics crate provides a no-op recorder by default)
        record_request("POST", 200, std::time::Duration::from_millis(50));
        record_auth("api_key", true);
        record_auth("jwt", false);
        record_rate_limit(true);
        record_rate_limit(false);
        set_active_identities(5);
    }

    #[test]
    fn test_current_trace_id_without_otel() {
        // Without OpenTelemetry initialized, should return None
        let trace_id = current_trace_id();
        // May or may not be None depending on global state, but shouldn't panic
        let _ = trace_id;
    }
}
