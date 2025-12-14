//! Rate limiting service for mcp-guard
//!
//! Implements per-identity rate limiting with support for:
//! - Global default rate limits
//! - Per-identity custom rate limits
//! - Token bucket algorithm via Governor crate
//! - TTL-based eviction to prevent memory growth
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
use std::time::{Duration, Instant};

/// Rate limiter type alias
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Default TTL for idle rate limiter entries (1 hour)
const DEFAULT_ENTRY_TTL: Duration = Duration::from_secs(3600);

/// Cleanup threshold - run cleanup when this many identities are tracked
const CLEANUP_THRESHOLD: usize = 1000;

/// Entry in the rate limiter cache with last access time
struct RateLimitEntry {
    limiter: Arc<Limiter>,
    last_access: Instant,
}

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
    /// Per-identity rate limiters (created lazily) with last access time
    identity_limiters: DashMap<String, RateLimitEntry>,
    /// TTL for idle entries
    entry_ttl: Duration,
}

impl RateLimitService {
    /// Create a new rate limiting service
    pub fn new(config: &crate::config::RateLimitConfig) -> Self {
        Self {
            enabled: config.enabled,
            default_rps: config.requests_per_second,
            default_burst: config.burst_size,
            identity_limiters: DashMap::new(),
            entry_ttl: DEFAULT_ENTRY_TTL,
        }
    }

    /// Create a rate limiter with the specified configuration
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(NonZeroU32::new(100).unwrap());
        let burst = NonZeroU32::new(burst_size).unwrap_or(NonZeroU32::new(50).unwrap());

        let quota = Quota::per_second(rps).allow_burst(burst);
        RateLimiter::direct(quota)
    }

    /// Get or create a rate limiter for the given identity, updating last access time
    fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this identity
        if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Maybe run cleanup if we have too many entries
        if self.identity_limiters.len() >= CLEANUP_THRESHOLD {
            self.cleanup_expired();
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
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.identity_limiters.insert(identity_id.to_string(), entry);
        limiter
    }

    /// Remove expired entries that haven't been accessed within the TTL
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        let ttl = self.entry_ttl;

        self.identity_limiters.retain(|_, entry| {
            now.duration_since(entry.last_access) < ttl
        });

        tracing::debug!(
            remaining = self.identity_limiters.len(),
            "Rate limiter cleanup completed"
        );
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

    /// Set a custom TTL for entry expiration (for testing)
    #[cfg(test)]
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.entry_ttl = ttl;
        self
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

    #[test]
    fn test_ttl_cleanup() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 10,
            burst_size: 10,
        };
        // Set TTL to 0 so entries are immediately expired
        let service = RateLimitService::new(&config).with_ttl(Duration::ZERO);

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);
        service.check("user_c", None);

        assert_eq!(service.tracked_identities(), 3);

        // Cleanup should remove all expired entries
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 0);
    }

    #[test]
    fn test_ttl_preserves_active_entries() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 10,
            burst_size: 10,
        };
        // Set a longer TTL
        let service = RateLimitService::new(&config).with_ttl(Duration::from_secs(3600));

        // Create entries for multiple users
        service.check("user_a", None);
        service.check("user_b", None);

        assert_eq!(service.tracked_identities(), 2);

        // Cleanup should preserve entries that haven't expired
        service.cleanup_expired();

        assert_eq!(service.tracked_identities(), 2);
    }
}
