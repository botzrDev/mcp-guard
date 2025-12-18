# Rate Limiting Internals

This document explains the internal implementation of rate limiting in mcp-guard.

## Overview

mcp-guard uses the Governor crate to implement token bucket rate limiting. Each identity gets their own rate limiter, stored in a concurrent hash map with TTL-based eviction.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                      RateLimitService                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Configuration                             │   │
│  │  - enabled: bool                                            │   │
│  │  - default_rps: u32 (requests per second)                   │   │
│  │  - default_burst: u32                                       │   │
│  │  - entry_ttl: Duration (1 hour default)                     │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │              identity_limiters: DashMap                      │   │
│  │                                                             │   │
│  │   "user-123" ──▶ RateLimitEntry {                          │   │
│  │                     limiter: Arc<Limiter>,                  │   │
│  │                     last_access: Instant,                   │   │
│  │                  }                                          │   │
│  │                                                             │   │
│  │   "api-key-456" ──▶ RateLimitEntry { ... }                 │   │
│  │                                                             │   │
│  │   "oauth-789" ──▶ RateLimitEntry { ... }                   │   │
│  │                                                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Token Bucket Algorithm

The Governor crate implements a Generic Cell Rate Algorithm (GCRA), which is mathematically equivalent to a token bucket:

- **Capacity**: The burst size (how many requests can be made instantly)
- **Refill Rate**: Requests per second (how fast the bucket refills)
- **Cost**: Each request costs one token

```
                    Burst = 10 tokens
                    ┌────────────────────┐
                    │ ████████████████   │ ◀── Current: 8 tokens
                    └────────────────────┘
                              │
                              │  1 token consumed per request
                              ▼
                    ┌────────────────────┐
                    │ ██████████████     │ ◀── Current: 7 tokens
                    └────────────────────┘

                    Refills at 5 tokens/second (rps = 5)
```

## Core Types

### RateLimitService

```rust
// src/rate_limit/mod.rs:86-97

pub struct RateLimitService {
    enabled: bool,
    default_rps: u32,
    default_burst: u32,
    identity_limiters: DashMap<String, RateLimitEntry>,
    entry_ttl: Duration,
}
```

### RateLimitEntry

```rust
// src/rate_limit/mod.rs:43-46

struct RateLimitEntry {
    limiter: Arc<Limiter>,
    last_access: Instant,  // For TTL eviction
}
```

### RateLimitResult

```rust
// src/rate_limit/mod.rs:49-61

pub struct RateLimitResult {
    pub allowed: bool,           // Whether request is allowed
    pub retry_after_secs: Option<u64>,  // Seconds until retry (for 429)
    pub limit: u32,              // Configured rate limit
    pub remaining: u32,          // Approximate remaining requests
    pub reset_at: u64,           // Unix timestamp of reset
}
```

## Constants

```rust
// src/rate_limit/mod.rs:27-40

/// TTL for idle rate limiter entries (1 hour)
const DEFAULT_ENTRY_TTL: Duration = Duration::from_secs(3600);

/// Cleanup threshold (check for expired entries when map exceeds this)
const CLEANUP_THRESHOLD: usize = 1000;

/// Default requests per second
const DEFAULT_RPS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

/// Default burst size
const DEFAULT_BURST: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(50) };
```

## Core Operations

### check()

The main rate limiting check:

```rust
// src/rate_limit/mod.rs:176-210

pub fn check(&self, identity_id: &str, custom_limit: Option<u32>) -> RateLimitResult {
    // Calculate effective limit
    let limit = custom_limit.unwrap_or(self.default_rps);
    let burst = custom_limit
        .map(|rps| (rps as f32 * 0.5).max(1.0) as u32)
        .unwrap_or(self.default_burst);

    // Calculate reset timestamp (1 second from now)
    let reset_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() + 1)
        .unwrap_or(0);

    // Fast path: disabled
    if !self.enabled {
        return RateLimitResult::allowed(limit, burst, reset_at);
    }

    // Get or create limiter for this identity
    let limiter = self.get_identity_limiter(identity_id, custom_limit);

    // Check the token bucket
    match limiter.check() {
        Ok(_) => {
            let remaining = burst.saturating_sub(1);
            RateLimitResult::allowed(limit, remaining, reset_at)
        }
        Err(not_until) => {
            let wait = not_until.wait_time_from(DefaultClock::default().now());
            let retry_secs = wait.as_secs().max(1);
            RateLimitResult::denied(retry_secs, limit, reset_at)
        }
    }
}
```

### Lazy Limiter Creation

Limiters are created on first access for each identity:

```rust
// src/rate_limit/mod.rs:120-151

fn get_identity_limiter(&self, identity_id: &str, custom_limit: Option<u32>) -> Arc<Limiter> {
    let now = Instant::now();

    // Check if limiter already exists
    if let Some(mut entry) = self.identity_limiters.get_mut(identity_id) {
        entry.last_access = now;  // Update access time
        return entry.limiter.clone();
    }

    // Maybe run cleanup if we have too many entries
    if self.identity_limiters.len() >= CLEANUP_THRESHOLD {
        self.cleanup_expired();
    }

    // Calculate rate and burst
    let (rps, burst) = if let Some(custom_rps) = custom_limit {
        // Custom rate limit with proportional burst (50%)
        let custom_burst = (custom_rps as f32 * 0.5).max(1.0) as u32;
        (custom_rps, custom_burst)
    } else {
        (self.default_rps, self.default_burst)
    };

    // Create new limiter
    let limiter = Arc::new(Self::create_limiter(rps, burst));
    let entry = RateLimitEntry {
        limiter: limiter.clone(),
        last_access: now,
    };
    self.identity_limiters.insert(identity_id.to_string(), entry);
    limiter
}
```

### TTL-Based Eviction

Expired entries are cleaned up when the map grows large:

```rust
// src/rate_limit/mod.rs:154-166

pub fn cleanup_expired(&self) {
    let now = Instant::now();
    let ttl = self.entry_ttl;

    // Remove entries that haven't been accessed within TTL
    self.identity_limiters.retain(|_, entry| {
        now.duration_since(entry.last_access) < ttl
    });

    tracing::debug!(
        remaining = self.identity_limiters.len(),
        "Rate limiter cleanup completed"
    );
}
```

## Per-Identity Rate Limits

Identities can have custom rate limits:

```rust
// From Identity struct
pub rate_limit: Option<u32>,

// Used in check():
let result = state.rate_limiter.check(&identity.id, identity.rate_limit);
```

This allows VIP users to have higher limits or specific services to have custom quotas.

## HTTP Response Headers

The server adds rate limit headers to responses:

| Header | Description | Example |
|--------|-------------|---------|
| `X-RateLimit-Limit` | Configured rate limit | `100` |
| `X-RateLimit-Remaining` | Approximate remaining requests | `42` |
| `X-RateLimit-Reset` | Unix timestamp when limit resets | `1702900000` |
| `Retry-After` | Seconds until retry (on 429) | `5` |

```rust
// src/server/mod.rs - response handling

if !result.allowed {
    return (
        StatusCode::TOO_MANY_REQUESTS,
        [("Retry-After", result.retry_after_secs.unwrap_or(1).to_string())],
        Json(json!({"error": "Rate limit exceeded"})),
    ).into_response();
}

// Add headers to successful response
response.headers_mut().insert("X-RateLimit-Limit", result.limit.into());
response.headers_mut().insert("X-RateLimit-Remaining", result.remaining.into());
response.headers_mut().insert("X-RateLimit-Reset", result.reset_at.into());
```

## Metrics Integration

Rate limit events are recorded in Prometheus:

```rust
// src/observability/mod.rs

pub fn record_rate_limit(allowed: bool) {
    counter!(
        "mcp_guard_rate_limit_total",
        "allowed" => allowed.to_string(),
    ).increment(1);
}

// Active identities gauge
pub fn set_active_identities(count: usize) {
    gauge!("mcp_guard_active_identities").set(count as f64);
}
```

## Concurrency Design

### DashMap for Thread-Safe Access

`DashMap` provides concurrent access without global locks:

```rust
use dashmap::DashMap;

// Read operations don't block writers
identity_limiters.get(identity_id)

// Write operations use fine-grained locks
identity_limiters.insert(id, entry)
identity_limiters.retain(|_, e| ...)
```

### Arc for Shared Limiters

`Arc<Limiter>` allows the same limiter to be used across async tasks:

```rust
// Multiple concurrent requests for same identity share one limiter
let limiter = Arc::new(RateLimiter::direct(quota));
```

## Testing

### Unit Test Examples

```rust
// Test basic rate limiting
#[test]
fn test_rate_limit_enabled() {
    let config = RateLimitConfig {
        enabled: true,
        requests_per_second: 1,
        burst_size: 2,
    };
    let service = RateLimitService::new(&config);

    // First 2 requests within burst
    assert!(service.check("test", None).allowed);
    assert!(service.check("test", None).allowed);

    // 3rd request should be limited
    let result = service.check("test", None);
    assert!(!result.allowed);
    assert!(result.retry_after_secs.is_some());
}
```

```rust
// Test per-identity isolation
#[test]
fn test_per_identity_isolation() {
    let service = RateLimitService::new(&config);

    // Exhaust rate limit for user A
    service.check("user_a", None);
    assert!(!service.check("user_a", None).allowed);

    // User B should still have their own bucket
    assert!(service.check("user_b", None).allowed);
}
```

```rust
// Test TTL cleanup
#[test]
fn test_ttl_cleanup() {
    let service = RateLimitService::new(&config)
        .with_ttl(Duration::ZERO);  // Immediate expiration

    service.check("user_a", None);
    service.check("user_b", None);
    assert_eq!(service.tracked_identities(), 2);

    service.cleanup_expired();
    assert_eq!(service.tracked_identities(), 0);
}
```

## Configuration

```toml
# mcp-guard.toml
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50
```

```rust
// src/config/mod.rs

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_rps")]
    pub requests_per_second: u32,

    #[serde(default = "default_burst")]
    pub burst_size: u32,
}
```

## Best Practices

1. **Set burst appropriately**: Burst should be ~50% of RPS for smooth traffic
2. **Monitor active identities**: High counts may indicate memory growth
3. **Use custom limits sparingly**: Most clients should use defaults
4. **Don't disable in production**: Even high limits provide protection
5. **Correlate with metrics**: Watch `mcp_guard_rate_limit_total{allowed="false"}`

## Future Improvements

Potential enhancements for horizontal scaling:

1. **Redis-backed rate limiting**: Share state across instances
2. **Sliding window**: More accurate than fixed window token bucket
3. **Dynamic limits**: Adjust limits based on server load
4. **Rate limit by tool**: Different limits for different MCP tools
