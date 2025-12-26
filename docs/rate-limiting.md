# Rate Limiting Guide

Configure and tune MCP Guard's per-identity rate limiting to protect upstream servers.

## Overview

MCP Guard implements per-identity rate limiting using the token bucket algorithm. Each authenticated identity gets its own rate limiter, ensuring fair resource allocation and protection against abuse.

### Key Features

- **Per-identity limits** - Each user/service has independent limits
- **Token bucket algorithm** - Allows controlled bursts
- **Custom overrides** - Different limits for different users
- **Automatic cleanup** - Idle rate limiters expire after 1 hour

---

## Configuration

### Global Settings

```toml
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable rate limiting |
| `requests_per_second` | integer | `100` | Requests per second limit |
| `burst_size` | integer | `50` | Maximum burst allowance |

### Per-Identity Overrides

Override the global limit for specific identities:

```toml
[[auth.api_keys]]
id = "high-volume-service"
key_hash = "..."
rate_limit = 1000  # 1000 RPS instead of default 100

[[auth.api_keys]]
id = "low-priority-client"
key_hash = "..."
rate_limit = 10    # Restricted to 10 RPS
```

---

## How It Works

### Token Bucket Algorithm

The token bucket algorithm provides smooth rate limiting with burst support:

1. **Bucket fills** at `requests_per_second` rate
2. **Bucket capacity** is `burst_size`
3. **Each request** removes one token
4. **If empty**, request is rejected with 429

```
Bucket Capacity: 50 tokens (burst_size)
Fill Rate: 100 tokens/second (requests_per_second)

Time 0:    [████████████████████████████████████████████████████] 50/50
Request:   [███████████████████████████████████████████████████ ] 49/50
Request:   [██████████████████████████████████████████████████  ] 48/50
...
Time 1s:   [████████████████████████████████████████████████████] 50/50 (refilled)
```

### Burst Handling

Bursts are allowed up to `burst_size`:

- Client can send up to 50 requests instantly
- After burst, limited to 100/second
- Burst allowance regenerates over time

**Example:**

With `requests_per_second = 100` and `burst_size = 50`:

| Time | Action | Tokens |
|------|--------|--------|
| 0.0s | Start | 50 |
| 0.0s | 30 requests | 20 |
| 0.1s | +10 tokens | 30 |
| 0.1s | 25 requests | 5 |
| 0.2s | +10 tokens | 15 |
| 0.2s | 20 requests | Rejected (only 15 available) |

### Identity Tracking

Rate limiters are created per unique identity:

- **API keys**: By `id` field
- **JWT**: By `sub` claim (or configured `user_id_claim`)
- **OAuth**: By user ID from token
- **mTLS**: By certificate identity (CN, SAN)

### Memory Management

To prevent unbounded memory growth:

- Rate limiters have a 1-hour TTL
- Inactive limiters are automatically cleaned up
- `mcp_guard_active_identities` metric tracks current count

---

## HTTP Headers

### Successful Requests (2xx)

Every successful response includes rate limit headers:

```
x-ratelimit-limit: 100
x-ratelimit-remaining: 47
x-ratelimit-reset: 1702656789
```

| Header | Description |
|--------|-------------|
| `x-ratelimit-limit` | Maximum requests per second |
| `x-ratelimit-remaining` | Tokens remaining in bucket |
| `x-ratelimit-reset` | Unix timestamp when bucket refills |

### Rate Limited Requests (429)

When rate limited, MCP Guard returns:

**Status:** `429 Too Many Requests`

**Headers:**

```
Retry-After: 1
```

**Body:**

```json
{
  "error": "Rate limit exceeded",
  "error_id": "550e8400-e29b-41d4-a716-446655440000",
  "retry_after": 1
}
```

### Client Handling

Clients should implement backoff based on `Retry-After`:

```python
import requests
import time

def call_mcp(endpoint, data, headers):
    response = requests.post(endpoint, json=data, headers=headers)

    if response.status_code == 429:
        retry_after = int(response.headers.get('Retry-After', 1))
        time.sleep(retry_after)
        return call_mcp(endpoint, data, headers)  # Retry

    return response
```

---

## Per-Identity Limits

### Use Cases

**Premium vs Free Tiers:**

```toml
# Free tier: 10 RPS
[[auth.api_keys]]
id = "free-user-123"
key_hash = "..."
rate_limit = 10

# Premium tier: 500 RPS
[[auth.api_keys]]
id = "premium-user-456"
key_hash = "..."
rate_limit = 500
```

**Service Accounts:**

```toml
# CI/CD pipelines need higher limits
[[auth.api_keys]]
id = "github-actions"
key_hash = "..."
rate_limit = 1000

# Interactive users need less
[[auth.api_keys]]
id = "developer"
key_hash = "..."
rate_limit = 50
```

**Internal vs External:**

```toml
# Internal services: high limit
[auth.mtls]
enabled = true
identity_source = "cn"
rate_limit = 5000

# External API keys: default limit
[[auth.api_keys]]
id = "partner-api"
key_hash = "..."
# Uses global rate_limit (100)
```

### Priority Order

Rate limits are determined in this order:

1. Identity-specific `rate_limit` if set
2. Global `rate_limit.requests_per_second`

---

## Monitoring

Monitor rate limiting performance using the built-in Prometheus metrics and audit logs.

### Prometheus Metrics

The following metrics are specifically relevant for rate limiting:

- `mcp_guard_rate_limit_total`: Tracks the number of allowed and blocked requests.
- `mcp_guard_active_identities`: Tracks the number of unique identities currently being rate-limited.

For a complete list of all metrics and example queries, see the **[Observability Guide](observability.md#metrics)**.

### Audit Logs

```
mcp_guard_active_identities 42
```

### Example Queries

**Rate limit rejection rate:**

```promql
rate(mcp_guard_rate_limit_total{allowed="false"}[5m])
/
rate(mcp_guard_rate_limit_total[5m])
```

**Requests approaching limit:**

Monitor `x-ratelimit-remaining` in your client and alert when low.

### Audit Events

Rate limit events are logged:

```json
{
  "event_type": "RateLimited",
  "identity_id": "user123",
  "timestamp": "2024-12-15T10:30:00Z",
  "retry_after_secs": 1
}
```

---

## Tuning Guide

### Determining Appropriate Limits

Consider these factors:

1. **Upstream capacity** - What can your MCP server handle?
2. **Expected usage** - Normal request patterns
3. **Peak handling** - Burst requirements
4. **Fair sharing** - Multiple clients per server

**Calculation example:**

- Upstream can handle: 1000 RPS
- Expected clients: 10
- Safety margin: 20%

Per-client limit: `1000 * 0.8 / 10 = 80 RPS`

### Burst Size Considerations

| Burst Size | Behavior |
|------------|----------|
| Equal to RPS | Allows 1 second of burst |
| Half of RPS | Moderate burst tolerance |
| Low (10-20) | Strict, smooth traffic |

**Recommendations:**

- **Interactive use**: `burst_size = requests_per_second / 2`
- **Batch operations**: `burst_size = requests_per_second`
- **Strict limiting**: `burst_size = 10`

### Memory Usage

Each tracked identity uses approximately:

- ~200 bytes for the rate limiter
- Additional overhead for the DashMap entry

**Estimation:**

- 1,000 active identities ≈ 200KB
- 10,000 active identities ≈ 2MB
- 100,000 active identities ≈ 20MB

Inactive entries are cleaned up after 1 hour.

---

## Configuration Examples

### High-Throughput API

```toml
[rate_limit]
enabled = true
requests_per_second = 1000
burst_size = 200
```

### Strict Rate Limiting

```toml
[rate_limit]
enabled = true
requests_per_second = 10
burst_size = 5
```

### Disabled (Not Recommended)

```toml
[rate_limit]
enabled = false
```

**Warning:** Disabling rate limiting exposes your upstream to abuse.

---

## Troubleshooting

### Unexpected 429 Responses

**Check current limits:**

```bash
# Make a request and check headers
curl -v -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer mcp_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# Look for x-ratelimit-* headers
```

**Possible causes:**

1. Global limit too low for traffic volume
2. Per-identity limit not set (using restrictive global default)
3. Burst size too small for request pattern

### Limits Not Applying

**Verify configuration:**

```bash
mcp-guard validate
```

**Check identity matching:**

- API key `id` must match exactly
- JWT claims must map correctly
- Enable verbose logging: `mcp-guard -v run`

### Memory Growth

If `mcp_guard_active_identities` grows indefinitely:

1. Check for identity leakage (new ID per request)
2. Verify JWT `sub` claim is stable
3. Wait for 1-hour TTL cleanup

### Rate Limits Not Per-User

If all users share the same limit:

1. Verify authentication is working (401 vs 429)
2. Check identity extraction (logs show identity ID)
3. Ensure different users have different identity IDs

---

## See Also

- [Configuration Reference](configuration.md) - Complete configuration options
- [Authentication Guide](authentication.md) - Identity and auth setup
- [Observability Guide](observability.md) - Monitoring rate limits
- [Troubleshooting Guide](troubleshooting.md) - Common issues
