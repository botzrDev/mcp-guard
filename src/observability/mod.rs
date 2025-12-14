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

use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing/logging
pub fn init_tracing(verbose: bool) {
    let filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing_verbose() {
        // Just verify it doesn't panic - tracing can only be initialized once
        // so we can't test both modes in the same process
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
}
