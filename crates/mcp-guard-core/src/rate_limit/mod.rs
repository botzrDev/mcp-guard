//! Rate limiting service for mcp-guard
//!
//! Implements per-identity and per-tool rate limiting with support for:
//! - Global default rate limits
//! - Per-identity custom rate limits
//! - Per-tool rate limits with glob pattern matching
//! - Token bucket algorithm via Governor crate
//! - TTL-based eviction to prevent memory growth
//! - Background cleanup task to avoid inline latency spikes
//!
//! See PRD FR-RATE-01 through FR-RATE-07 for requirements.

use dashmap::DashMap;
use glob::Pattern;
use governor::{
    clock::{Clock, DefaultClock},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

/// Rate limiter type alias for a direct (non-keyed) token bucket limiter
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// Default TTL for idle rate limiter entries.
/// 1 hour balances memory cleanup with user experience (users reconnecting within
/// an hour keep their rate limit state). Typical sessions are shorter.
const DEFAULT_ENTRY_TTL: Duration = Duration::from_secs(3600);

/// Default cleanup interval for background task.
/// 5 minutes balances timely cleanup with low overhead.
const DEFAULT_CLEANUP_INTERVAL_SECS: u64 = 300;

/// Default requests per second
/// SAFETY: 100 is non-zero, so new_unchecked is safe
const DEFAULT_RPS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

/// Default burst size
/// SAFETY: 50 is non-zero, so new_unchecked is safe
const DEFAULT_BURST: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(50) };

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
    /// The configured rate limit (requests per second)
    pub limit: u32,
    /// Approximate remaining requests in the current window
    pub remaining: u32,
    /// Unix timestamp when the rate limit resets
    pub reset_at: u64,
}

impl RateLimitResult {
    fn allowed(limit: u32, remaining: u32, reset_at: u64) -> Self {
        Self {
            allowed: true,
            retry_after_secs: None,
            limit,
            remaining,
            reset_at,
        }
    }

    fn denied(retry_after_secs: u64, limit: u32, reset_at: u64) -> Self {
        Self {
            allowed: false,
            retry_after_secs: Some(retry_after_secs),
            limit,
            remaining: 0,
            reset_at,
        }
    }
}

/// Compiled tool rate limit pattern
struct ToolPattern {
    pattern: Pattern,
    rps: u32,
    burst: u32,
}

/// Rate limiting service with per-identity and per-tool tracking
pub struct RateLimitService {
    enabled: bool,
    /// Default rate limit (requests per second)
    default_rps: u32,
    /// Default burst size
    default_burst: u32,
    /// Per-identity rate limiters (created lazily) with last access time
    identity_limiters: DashMap<String, RateLimitEntry>,
    /// Per-tool rate limiters (key = "identity:tool")
    tool_limiters: DashMap<String, RateLimitEntry>,
    /// Compiled tool patterns with their rate limits
    tool_patterns: Vec<ToolPattern>,
    /// TTL for idle entries
    entry_ttl: Duration,
}

impl RateLimitService {
    /// Create a new rate limiting service
    pub fn new(config: &crate::config::RateLimitConfig) -> Self {
        // Compile tool patterns from config
        let tool_patterns: Vec<ToolPattern> = config
            .tool_limits
            .iter()
            .filter_map(
                |tool_config| match Pattern::new(&tool_config.tool_pattern) {
                    Ok(pattern) => Some(ToolPattern {
                        pattern,
                        rps: tool_config.requests_per_second,
                        burst: tool_config.burst_size,
                    }),
                    Err(e) => {
                        tracing::warn!(
                            pattern = %tool_config.tool_pattern,
                            error = %e,
                            "Failed to compile tool rate limit pattern, skipping"
                        );
                        None
                    }
                },
            )
            .collect();

        if !tool_patterns.is_empty() {
            tracing::info!(
                count = tool_patterns.len(),
                "Compiled tool rate limit patterns"
            );
        }

        Self {
            enabled: config.enabled,
            default_rps: config.requests_per_second,
            default_burst: config.burst_size,
            identity_limiters: DashMap::new(),
            tool_limiters: DashMap::new(),
            tool_patterns,
            entry_ttl: DEFAULT_ENTRY_TTL,
        }
    }

    /// Create a rate limiter with the specified configuration
    fn create_limiter(requests_per_second: u32, burst_size: u32) -> Limiter {
        let rps = NonZeroU32::new(requests_per_second).unwrap_or(DEFAULT_RPS);
        let burst = NonZeroU32::new(burst_size).unwrap_or(DEFAULT_BURST);

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

        // Note: Cleanup is now handled by a background task to avoid latency spikes
        // See start_cleanup_task() for the background cleanup implementation

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
        self.identity_limiters
            .insert(identity_id.to_string(), entry);
        limiter
    }

    /// Get or create a rate limiter for a specific tool, updating last access time
    fn get_tool_limiter(&self, key: &str, rps: u32, burst: u32) -> Arc<Limiter> {
        let now = Instant::now();

        // Check if we already have a limiter for this tool
        if let Some(mut entry) = self.tool_limiters.get_mut(key) {
            entry.last_access = now;
            return entry.limiter.clone();
        }

        // Create a new limiter for this tool
        let limiter = Arc::new(Self::create_limiter(rps, burst));
        let entry = RateLimitEntry {
            limiter: limiter.clone(),
            last_access: now,
        };
        self.tool_limiters.insert(key.to_string(), entry);
        limiter
    }

    /// Check rate limit for a specific tool call
    ///
    /// Returns `Some(RateLimitResult)` if a matching tool limit exists,
    /// `None` if no tool-specific limit applies (use identity limit instead).
    ///
    /// # Arguments
    /// * `identity_id` - Unique identifier for the user/service
    /// * `tool_name` - Name of the MCP tool being called
    ///
    /// # Returns
    /// `Some(RateLimitResult)` if tool has a rate limit configured, `None` otherwise
    pub fn check_tool(&self, identity_id: &str, tool_name: &str) -> Option<RateLimitResult> {
        if !self.enabled || self.tool_patterns.is_empty() {
            return None;
        }

        // Find matching tool pattern
        let tool_config = self
            .tool_patterns
            .iter()
            .find(|tp| tp.pattern.matches(tool_name))?;

        let rps = tool_config.rps;
        let burst = tool_config.burst;

        // Create composite key: "identity:tool"
        let key = format!("{}:{}", identity_id, tool_name);

        // Calculate reset timestamp
        let reset_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() + 1)
            .unwrap_or(0);

        // Get or create limiter for this identity:tool combination
        let limiter = self.get_tool_limiter(&key, rps, burst);

        match limiter.check() {
            Ok(_) => {
                let remaining = burst.saturating_sub(1);
                Some(RateLimitResult::allowed(rps, remaining, reset_at))
            }
            Err(not_until) => {
                let wait_duration = not_until.wait_time_from(DefaultClock::default().now());
                let retry_secs = wait_duration.as_secs().max(1);
                Some(RateLimitResult::denied(retry_secs, rps, reset_at))
            }
        }
    }

    /// Remove expired entries that haven't been accessed within the TTL
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        let ttl = self.entry_ttl;

        self.identity_limiters
            .retain(|_, entry| now.duration_since(entry.last_access) < ttl);

        self.tool_limiters
            .retain(|_, entry| now.duration_since(entry.last_access) < ttl);

        tracing::debug!(
            identity_remaining = self.identity_limiters.len(),
            tool_remaining = self.tool_limiters.len(),
            "Rate limiter cleanup completed"
        );
    }

    /// Get the number of tracked tool limiters (for monitoring)
    pub fn tracked_tools(&self) -> usize {
        self.tool_limiters.len()
    }

    /// Check if any tool rate limits are configured
    pub fn has_tool_limits(&self) -> bool {
        !self.tool_patterns.is_empty()
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
        // Calculate the effective limit for this identity
        let limit = custom_limit.unwrap_or(self.default_rps);
        let burst = custom_limit
            .map(|rps| (rps as f32 * 0.5).max(1.0) as u32)
            .unwrap_or(self.default_burst);

        // Calculate reset timestamp (1 second from now, since we use per-second limits)
        let reset_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() + 1)
            .unwrap_or(0);

        if !self.enabled {
            // When disabled, report max capacity
            return RateLimitResult::allowed(limit, burst, reset_at);
        }

        let limiter = self.get_identity_limiter(identity_id, custom_limit);

        match limiter.check() {
            Ok(_) => {
                // Estimate remaining tokens (burst - 1 since we just consumed one)
                // This is approximate since Governor doesn't expose exact token count
                let remaining = burst.saturating_sub(1);
                RateLimitResult::allowed(limit, remaining, reset_at)
            }
            Err(not_until) => {
                // Calculate retry-after in seconds
                let wait_duration = not_until.wait_time_from(DefaultClock::default().now());
                let retry_secs = wait_duration.as_secs().max(1);
                RateLimitResult::denied(retry_secs, limit, reset_at)
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

    /// Start a background cleanup task that periodically removes expired entries
    ///
    /// This runs cleanup in the background to avoid latency spikes during request handling.
    /// The task will stop when the shutdown token is cancelled.
    ///
    /// # Arguments
    /// * `shutdown_token` - Token to signal when the task should stop
    /// * `cleanup_interval_secs` - How often to run cleanup (default: 5 minutes)
    ///
    /// # Returns
    /// A JoinHandle for the background task
    pub fn start_cleanup_task(
        self: &Arc<Self>,
        shutdown_token: CancellationToken,
        cleanup_interval_secs: Option<u64>,
    ) -> tokio::task::JoinHandle<()> {
        let service = Arc::clone(self);
        let interval =
            Duration::from_secs(cleanup_interval_secs.unwrap_or(DEFAULT_CLEANUP_INTERVAL_SECS));

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            // Skip the first immediate tick
            interval_timer.tick().await;

            loop {
                tokio::select! {
                    _ = shutdown_token.cancelled() => {
                        tracing::debug!("Rate limiter cleanup task received shutdown signal");
                        break;
                    }
                    _ = interval_timer.tick() => {
                        let identity_before = service.tracked_identities();
                        let tool_before = service.tracked_tools();
                        service.cleanup_expired();
                        let identity_after = service.tracked_identities();
                        let tool_after = service.tracked_tools();
                        if identity_before != identity_after || tool_before != tool_after {
                            tracing::debug!(
                                identity_removed = identity_before - identity_after,
                                identity_remaining = identity_after,
                                tool_removed = tool_before - tool_after,
                                tool_remaining = tool_after,
                                "Rate limiter cleanup task removed expired entries"
                            );
                        }
                    }
                }
            }
            tracing::debug!("Rate limiter cleanup task exiting");
        })
    }
}

impl Default for RateLimitService {
    fn default() -> Self {
        Self::new(&crate::config::RateLimitConfig::default())
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for rate limiting service.
    //!
    //! Tests cover:
    //! - Disabled vs enabled rate limiting
    //! - Per-identity isolation (separate buckets)
    //! - Custom rate limits per identity
    //! - TTL-based cleanup of idle entries
    //! - Per-tool rate limiting with glob patterns

    use super::*;
    use crate::config::{RateLimitConfig, ToolRateLimitConfig};

    /// Helper to create a basic rate limit config for tests
    fn test_config(enabled: bool, rps: u32, burst: u32) -> RateLimitConfig {
        RateLimitConfig {
            enabled,
            requests_per_second: rps,
            burst_size: burst,
            tool_limits: Vec::new(),
        }
    }

    /// Verify disabled rate limiter always allows requests
    #[test]
    fn test_rate_limit_disabled() {
        let config = test_config(false, 1, 1);
        let service = RateLimitService::new(&config);

        // Should always allow when disabled
        for _ in 0..100 {
            let result = service.check("test", None);
            assert!(result.allowed);
            assert!(result.retry_after_secs.is_none());
        }
    }

    /// Verify enabled rate limiter respects burst then denies
    #[test]
    fn test_rate_limit_enabled() {
        let config = test_config(true, 1, 2);
        let service = RateLimitService::new(&config);

        // First requests within burst should succeed
        assert!(service.check("test", None).allowed);
        assert!(service.check("test", None).allowed);

        // Next request should be rate limited
        let result = service.check("test", None);
        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
    }

    /// Verify each identity gets its own rate limit bucket
    #[test]
    fn test_per_identity_isolation() {
        let config = test_config(true, 1, 1);
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

    /// Verify custom rate limits override defaults
    #[test]
    fn test_custom_rate_limit() {
        let config = test_config(true, 1, 1);
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

    /// Verify clearing an identity resets their rate limit bucket
    #[test]
    fn test_clear_identity() {
        let config = test_config(true, 1, 1);
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

    /// Verify backwards-compatible check_allowed returns simple bool
    #[test]
    fn test_check_allowed_backwards_compat() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // check_allowed should return simple bool
        assert!(service.check_allowed("user", None));
        assert!(!service.check_allowed("user", None));
    }

    /// Verify retry_after_secs is populated when rate limited
    #[test]
    fn test_retry_after_populated() {
        let config = test_config(true, 1, 1);
        let service = RateLimitService::new(&config);

        // Exhaust rate limit
        service.check("user", None);
        let result = service.check("user", None);

        assert!(!result.allowed);
        assert!(result.retry_after_secs.is_some());
        // Should be at least 1 second
        assert!(result.retry_after_secs.unwrap() >= 1);
    }

    /// Verify TTL cleanup removes expired entries
    #[test]
    fn test_ttl_cleanup() {
        let config = test_config(true, 10, 10);
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

    /// Verify TTL cleanup preserves recently-accessed entries
    #[test]
    fn test_ttl_preserves_active_entries() {
        let config = test_config(true, 10, 10);
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

    // =========================================================================
    // Per-tool rate limiting tests
    // =========================================================================

    /// Verify tool rate limit returns None when no tool limits configured
    #[test]
    fn test_tool_rate_limit_no_config() {
        let config = test_config(true, 100, 50);
        let service = RateLimitService::new(&config);

        // Should return None when no tool limits are configured
        assert!(service.check_tool("user", "execute_code").is_none());
        assert!(!service.has_tool_limits());
    }

    /// Verify tool rate limit returns None when rate limiting is disabled
    #[test]
    fn test_tool_rate_limit_disabled() {
        let config = RateLimitConfig {
            enabled: false,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 5,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        // Should return None when disabled
        assert!(service.check_tool("user", "execute_code").is_none());
    }

    /// Verify per-tool rate limit with exact match
    #[test]
    fn test_per_tool_rate_limit_basic() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_code".to_string(),
                requests_per_second: 2,
                burst_size: 2,
            }],
        };
        let service = RateLimitService::new(&config);

        assert!(service.has_tool_limits());
        assert_eq!(service.tracked_tools(), 0);

        // First 2 requests within burst should succeed
        let result1 = service.check_tool("user", "execute_code").unwrap();
        assert!(result1.allowed);
        assert_eq!(result1.limit, 2);

        let result2 = service.check_tool("user", "execute_code").unwrap();
        assert!(result2.allowed);

        // 3rd request should be denied
        let result3 = service.check_tool("user", "execute_code").unwrap();
        assert!(!result3.allowed);
        assert!(result3.retry_after_secs.is_some());

        // Verify tool limiter is tracked
        assert_eq!(service.tracked_tools(), 1);
    }

    /// Verify tool rate limit with glob pattern matching
    #[test]
    fn test_per_tool_rate_limit_pattern_matching() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![
                ToolRateLimitConfig {
                    tool_pattern: "execute_*".to_string(),
                    requests_per_second: 2,
                    burst_size: 2,
                },
                ToolRateLimitConfig {
                    tool_pattern: "write_*".to_string(),
                    requests_per_second: 5,
                    burst_size: 3,
                },
            ],
        };
        let service = RateLimitService::new(&config);

        // execute_* pattern should match execute_code
        let result = service.check_tool("user", "execute_code").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // execute_* pattern should match execute_shell
        let result = service.check_tool("user", "execute_shell").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 2);

        // write_* pattern should match write_file
        let result = service.check_tool("user", "write_file").unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 5);

        // read_file should not match any pattern
        assert!(service.check_tool("user", "read_file").is_none());
    }

    /// Verify different identities have independent tool limiters
    #[test]
    fn test_tool_rate_limit_per_identity() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 1,
                burst_size: 1,
            }],
        };
        let service = RateLimitService::new(&config);

        // User A exhausts their tool limit
        assert!(
            service
                .check_tool("user_a", "execute_code")
                .unwrap()
                .allowed
        );
        assert!(
            !service
                .check_tool("user_a", "execute_code")
                .unwrap()
                .allowed
        );

        // User B should have their own independent limiter
        assert!(
            service
                .check_tool("user_b", "execute_code")
                .unwrap()
                .allowed
        );
        assert!(
            !service
                .check_tool("user_b", "execute_code")
                .unwrap()
                .allowed
        );

        // Two tool limiters should be tracked (user_a:execute_code, user_b:execute_code)
        assert_eq!(service.tracked_tools(), 2);
    }

    /// Verify tool limiters are cleaned up by TTL
    #[test]
    fn test_tool_limiter_cleanup() {
        let config = RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 50,
            tool_limits: vec![ToolRateLimitConfig {
                tool_pattern: "execute_*".to_string(),
                requests_per_second: 10,
                burst_size: 5,
            }],
        };
        let service = RateLimitService::new(&config).with_ttl(Duration::ZERO);

        // Create tool limiter entries
        service.check_tool("user_a", "execute_code");
        service.check_tool("user_b", "execute_shell");
        assert_eq!(service.tracked_tools(), 2);

        // Cleanup should remove expired tool entries
        service.cleanup_expired();
        assert_eq!(service.tracked_tools(), 0);
    }
}
