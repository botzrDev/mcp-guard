//! Observability: metrics, tracing, and logging for mcp-guard

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

/// Metrics for mcp-guard
pub mod metrics {
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Simple counter for tracking metrics
    pub struct Counter {
        value: AtomicU64,
    }

    impl Counter {
        pub const fn new() -> Self {
            Self {
                value: AtomicU64::new(0),
            }
        }

        pub fn inc(&self) {
            self.value.fetch_add(1, Ordering::Relaxed);
        }

        pub fn get(&self) -> u64 {
            self.value.load(Ordering::Relaxed)
        }
    }

    impl Default for Counter {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Application metrics
    pub struct AppMetrics {
        pub requests_total: Counter,
        pub requests_authenticated: Counter,
        pub requests_rejected: Counter,
        pub rate_limited: Counter,
        pub authz_denied: Counter,
        pub upstream_errors: Counter,
    }

    impl AppMetrics {
        pub const fn new() -> Self {
            Self {
                requests_total: Counter::new(),
                requests_authenticated: Counter::new(),
                requests_rejected: Counter::new(),
                rate_limited: Counter::new(),
                authz_denied: Counter::new(),
                upstream_errors: Counter::new(),
            }
        }
    }

    impl Default for AppMetrics {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Global metrics instance
    pub static METRICS: AppMetrics = AppMetrics::new();

    /// Get Prometheus-format metrics
    pub fn prometheus_metrics() -> String {
        format!(
            r#"# HELP mcp_guard_requests_total Total number of requests
# TYPE mcp_guard_requests_total counter
mcp_guard_requests_total {}

# HELP mcp_guard_requests_authenticated Number of authenticated requests
# TYPE mcp_guard_requests_authenticated counter
mcp_guard_requests_authenticated {}

# HELP mcp_guard_requests_rejected Number of rejected requests
# TYPE mcp_guard_requests_rejected counter
mcp_guard_requests_rejected {}

# HELP mcp_guard_rate_limited Number of rate limited requests
# TYPE mcp_guard_rate_limited counter
mcp_guard_rate_limited {}

# HELP mcp_guard_authz_denied Number of authorization denied requests
# TYPE mcp_guard_authz_denied counter
mcp_guard_authz_denied {}

# HELP mcp_guard_upstream_errors Number of upstream errors
# TYPE mcp_guard_upstream_errors counter
mcp_guard_upstream_errors {}
"#,
            METRICS.requests_total.get(),
            METRICS.requests_authenticated.get(),
            METRICS.requests_rejected.get(),
            METRICS.rate_limited.get(),
            METRICS.authz_denied.get(),
            METRICS.upstream_errors.get(),
        )
    }
}
