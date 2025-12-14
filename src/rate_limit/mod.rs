//! Rate limiting service for mcp-guard
//!
//! Implements per-identity rate limiting with support for:
//! - Global default rate limits
//! - Per-identity custom rate limits
//! - Token bucket algorithm via Governor crate
//!
//! See PRD FR-RATE-01 through FR-RATE-07 for requirements.

use dashmap::DashMap;
use governor::{
    clock::{Clock, DefaultClock},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

/// Rate limiter type alias
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Result of a rate limit check
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Seconds until the client can retry (for 429 Retry-After header)
    pub retry_after_secs: Option<u64>,
}

impl RateLimitResult {
    fn allowed() -> Self {
        Self {
            allowed: true,
            retry_after_secs: None,
        }
    }

    fn denied(retry_after_secs: u64) -> Self {
        Self {
            allowed: false,
            retry_after_secs: Some(retry_after_secs),
        }
    }
}

/// Rate limiting service with per-identity tracking
pub struct RateLimitService {
    enabled: bool,
    /// Default rate limit (requests per second)
    default_rps: u32,
    /// Default burst size
    default_burst: u32,
    /// Per-identity rate limiters (created lazily)
    identity_limiters: DashMap<String, Arc<Limiter>>,
}

impl RateLimitService {
    /// Create a new rate limiting service
    pub fn new(config: &crate::config::RateLimitConfig) -> Self {
        Self {
            enabled: config.enabled,
            default_rps: config.requests_per_second,
            default_burst: config.burst_size,
            identity_limiters: DashMap::new(),
        }
    }

    /// Create a rate limiter with the specified configuration
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(NonZeroU32::new(100).unwrap());
        let burst = NonZeroU32::new(burst_size).unwrap_or(NonZeroU32::new(50).unwrap());

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }

    /// Get or create a rate limiter for the given identity
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        // Check if we already have a limiter for this identity
        if let Some(limiter) = self.identity_limiters.get(identity_id) {
            return limiter.clone();
        }

        // Create a new limiter for this identity
        let (rps, burst) = if let Some(custom_rps) = custom_limit {
            // Use custom rate limit with proportional burst
            let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
            (custom_rps, custom_burst)
        } else {
            // Use defaults
            (self.default_rps, self.default_burst)
        };

        let limiter = Arc::new(Self::create_limiter(rps, burst));
        self.identity_limiters
            .insert(identity_id.to_string(), limiter.clone());
        limiter
    }

    /// Check if a request should be allowed for the given identity
    ///
    /// # Arguments
    /// * `identity_id` - Unique identifier for the user/service
    /// * `custom_limit` - Optional per-identity rate limit override (requests per second)
    ///
    /// # Returns
    /// A `RateLimitResult` indicating whether the request is allowed and retry-after time if denied
    pub fn check(&self, identity_id: &str, custom_limit: Option<u32>) -> RateLimitResult {
        if !self.enabled {
            return RateLimitResult::allowed();
        }

        let limiter = self.get_identity_limiter(identity_id, custom_limit);

        match limiter.check() {
            Ok(_) => RateLimitResult::allowed(),
            Err(not_until) => {
                // Calculate retry-after in seconds
                let wait_duration = not_until.wait_time_from(DefaultClock::default().now());
                let retry_secs = wait_duration.as_secs().max(1);
                RateLimitResult::denied(retry_secs)
            }
        }
    }

    /// Check rate limit, returning a simple bool (for backwards compatibility)
    pub fn check_allowed(&self, identity_id: &str, custom_limit: Option<u32>) -> bool {
        self.check(identity_id, custom_limit).allowed
    }

    /// Check and wait if rate limited (for async contexts)
    pub async fn check_or_wait(&self, identity_id: &str, custom_limit: Option<u32>) {
        if !self.enabled {
            return;
        }

        let limiter = self.get_identity_limiter(identity_id, custom_limit);
        limiter.until_ready().await;
    }

    /// Get the number of tracked identities (for monitoring)
    pub fn tracked_identities(&self) -> usize {
        self.identity_limiters.len()
    }

    /// Clear rate limit state for a specific identity (e.g., on identity deletion)
    pub fn clear_identity(&self, identity_id: &str) {
        self.identity_limiters.remove(identity_id);
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
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
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
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }

    #[test]
    fn test_per_identity_isolation() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Exhaust rate limit for user A
        assert!(service.check("user_a", None).allowed);
        assert!(!service.check("user_a", None).allowed);

        // User B should still have their own bucket
        assert!(service.check("user_b", None).allowed);
        assert!(!service.check("user_b", None).allowed);

        // Verify both are tracked
        assert_eq!(service.tracked_identities(), 2);
    }

    #[test]
    fn test_custom_rate_limit() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Default user with burst=1 gets exactly 1 request
        assert!(service.check("default_user", None).allowed);
        assert!(!service.check("default_user", None).allowed);

        // VIP user with custom limit of 10 rps
        // burst is 50% of rps = 5, so should handle 5 instant requests
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);
        assert!(service.check("vip_user", Some(10)).allowed);

        // 6th request should be limited
        assert!(!service.check("vip_user", Some(10)).allowed);
    }

    #[test]
    fn test_clear_identity() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        assert!(service.check("user", None).allowed);
        assert!(!service.check("user", None).allowed);

        // Clear the identity
        service.clear_identity("user");
        assert_eq!(service.tracked_identities(), 0);

        // User should get a fresh bucket
        assert!(service.check("user", None).allowed);
    }

    #[test]
    fn test_check_allowed_backwards_compat() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }

    #[test]
    fn test_retry_after_populated() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 1,
            burst_size: 1,
        };
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }
}
