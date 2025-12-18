# Splunk Integration Guide

Ship MCP Guard audit logs to Splunk using HTTP Event Collector (HEC).

## Prerequisites

- Splunk instance (Cloud or Enterprise) with HEC enabled
- Admin access to create HEC tokens
- MCP Guard installed and running

---

## Step 1: Enable HTTP Event Collector

If HEC is not already enabled:

1. Log in to Splunk Web
2. Go to **Settings** → **Data Inputs** → **HTTP Event Collector**
3. Click **Global Settings**
4. Set **All Tokens** to **Enabled**
5. Configure the HTTP port (default: 8088)
6. Click **Save**

---

## Step 2: Create an HEC Token

1. Go to **Settings** → **Data Inputs** → **HTTP Event Collector**
2. Click **New Token**
3. Configure the token:

| Setting | Value |
|---------|-------|
| **Name** | `mcp-guard` |
| **Source name override** | `mcp-guard` (optional) |
| **Description** | MCP Guard audit logs |

4. Click **Next**
5. Select an index (or create one):
   - Click **Create a new index** or select existing
   - Recommended index name: `mcp_guard_audit`
6. Click **Review** → **Submit**
7. Copy the generated **Token Value**

---

## Step 3: Configure MCP Guard

Add the audit export configuration to your `mcp-guard.toml`:

```toml
[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 30
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
```

Replace:
- `splunk.example.com:8088` with your Splunk HEC endpoint
- `YOUR_HEC_TOKEN` with the token from Step 2

### For Splunk Cloud

```toml
[audit]
enabled = true
export_url = "https://http-inputs-YOUR_STACK.splunkcloud.com/services/collector/event"
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
```

---

## Step 4: Test the Integration

1. Start MCP Guard:
   ```bash
   mcp-guard run
   ```

2. Make some authenticated requests to generate audit events

3. Search in Splunk:
   ```spl
   index=mcp_guard_audit
   ```

---

## Splunk Index Configuration

### Recommended Index Settings

Create a dedicated index for MCP Guard audit logs:

1. Go to **Settings** → **Indexes** → **New Index**
2. Configure:

| Setting | Value |
|---------|-------|
| **Index Name** | `mcp_guard_audit` |
| **Max Size** | Based on retention needs |
| **Retention** | 90 days (or per compliance requirements) |
| **App** | `search` |

3. Click **Save**

### Field Extractions

MCP Guard sends JSON events. Splunk automatically extracts fields, but you can create explicit extractions:

1. Go to **Settings** → **Fields** → **Field Extractions**
2. Click **New Field Extraction**
3. Use automatic JSON extraction or define custom extractions

---

## Example SPL Searches

### Recent Authentication Failures

```spl
index=mcp_guard_audit event_type="AuthFailure"
| stats count by reason, identity_id
| sort -count
```

### Tool Usage by User

```spl
index=mcp_guard_audit event_type="ToolCall"
| stats count by identity_id, tool
| sort -count
```

### Rate Limited Requests

```spl
index=mcp_guard_audit event_type="RateLimited"
| timechart span=1h count by identity_id
```

### Authorization Denials

```spl
index=mcp_guard_audit event_type="AuthzDenied"
| stats count by identity_id, tool, reason
```

### Request Latency Analysis

```spl
index=mcp_guard_audit event_type="ToolCall" duration_ms=*
| stats avg(duration_ms) as avg_latency, max(duration_ms) as max_latency, p95(duration_ms) as p95_latency by tool
| sort -avg_latency
```

### Failed Requests Over Time

```spl
index=mcp_guard_audit (event_type="AuthFailure" OR event_type="AuthzDenied" OR event_type="RateLimited")
| timechart span=1h count by event_type
```

---

## Simple Dashboard

Create a dashboard to monitor MCP Guard:

1. Go to **Dashboards** → **Create New Dashboard**
2. Add panels for key metrics:

### Panel 1: Request Volume

```spl
index=mcp_guard_audit
| timechart span=1h count
```

### Panel 2: Auth Success Rate

```spl
index=mcp_guard_audit event_type IN ("AuthSuccess", "AuthFailure")
| stats count(eval(event_type="AuthSuccess")) as success, count(eval(event_type="AuthFailure")) as failure
| eval success_rate = round(success / (success + failure) * 100, 2)
```

### Panel 3: Top Users

```spl
index=mcp_guard_audit event_type="ToolCall"
| top limit=10 identity_id
```

### Panel 4: Tool Usage

```spl
index=mcp_guard_audit event_type="ToolCall"
| top limit=10 tool
```

---

## Complete Configuration Example

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "http"
url = "http://mcp-server.internal:8080/mcp"

[auth.jwt]
mode = "jwks"
jwks_url = "https://auth.example.com/.well-known/jwks.json"
issuer = "https://auth.example.com/"
audience = "mcp-guard"

[rate_limit]
enabled = true
requests_per_second = 500
burst_size = 100

[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"  # Local backup
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 15
export_headers = { "Authorization" = "Splunk abc123-your-token-here" }
```

---

## Troubleshooting

### "No events in Splunk"

1. Verify HEC is enabled and the token is active
2. Check the endpoint URL includes `/services/collector/event`
3. Test HEC directly:
   ```bash
   curl -k https://splunk.example.com:8088/services/collector/event \
     -H "Authorization: Splunk YOUR_TOKEN" \
     -d '{"event": "test"}'
   ```
4. Check MCP Guard logs for export errors

### "Invalid token"

1. Verify the token is correct (no extra spaces)
2. Check token is not disabled in Splunk
3. Ensure the Authorization header format: `Splunk YOUR_TOKEN`

### "Connection refused"

1. Verify Splunk HEC port (default 8088)
2. Check firewall rules allow outbound connections
3. For Splunk Cloud, use the correct HTTP inputs URL

### "SSL certificate error"

1. For self-signed certificates, you may need to configure trust
2. Consider using a proper SSL certificate
3. Check the endpoint uses the correct protocol (`https://`)

### "Events delayed"

1. Events are batched for efficiency
2. Adjust `export_batch_size` and `export_interval_secs`:
   ```toml
   export_batch_size = 50      # Smaller batches
   export_interval_secs = 10   # More frequent flushes
   ```

### "Missing fields in Splunk"

1. Verify Splunk is parsing JSON correctly
2. Check if field names are indexed
3. Create explicit field extractions if needed

---

## Security Best Practices

1. **Use HTTPS:** Always use HTTPS for HEC endpoints
2. **Restrict token:** Limit token to specific indexes
3. **Rotate tokens:** Periodically generate new HEC tokens
4. **Network security:** Use private networking where possible
5. **Monitor token usage:** Check for unusual patterns

---

## See Also

- [Observability Guide](../observability.md) - Complete audit logging documentation
- [Configuration Reference](../configuration.md) - All configuration options
- [Deployment Guide](../deployment.md) - Production deployment patterns
