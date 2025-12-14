//! Rate limiting service for mcp-guard

use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

/// Rate limiter type alias
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Rate limiting service
pub struct RateLimitService {
    enabled: bool,
    default_limiter: Arc<Limiter>,
}

impl RateLimitService {
    /// Create a new rate limiting service
    pub fn new(config: &crate::config::RateLimitConfig) -> Self {
        let quota = Quota::per_second(
            NonZeroU32::new(config.requests_per_second).unwrap_or(NonZeroU32::new(100).unwrap()),
        )
        .allow_burst(
            NonZeroU32::new(config.burst_size).unwrap_or(NonZeroU32::new(50).unwrap()),
        );

        Self {
            enabled: config.enabled,
            default_limiter: Arc::new(RateLimiter::direct(quota)),
        }
    }

    /// Check if a request should be allowed
    pub fn check(&self, _identity_id: &str, custom_limit: Option<u32>) -> bool {
        if !self.enabled {
            return true;
        }

        // For now, use the default limiter
        // TODO: Implement per-identity rate limiting with custom limits
        let _ = custom_limit;

        self.default_limiter.check().is_ok()
    }

    /// Check and wait if rate limited (for async contexts)
    pub async fn check_or_wait(&self, _identity_id: &str) {
        if !self.enabled {
            return;
        }

        self.default_limiter.until_ready().await;
    }
}

impl Default for RateLimitService {
    fn default() -> Self {
        Self::new(&crate::config::RateLimitConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RateLimitConfig;

    #[test]
    fn test_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            assert!(service.check("test", None));
        }
    }

    #[test]
    fn test_rate_limit_enabled() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 2,
        };
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None));
        assert!(service.check("test", None));

        // Next request should be rate limited
        assert!(!service.check("test", None));
    }
}
