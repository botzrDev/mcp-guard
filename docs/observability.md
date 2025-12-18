# Observability Guide

Monitor MCP Guard with Prometheus metrics, OpenTelemetry tracing, and audit logging.

## Overview

MCP Guard implements the three pillars of observability:

| Pillar | Feature | Use Case |
|--------|---------|----------|
| **Metrics** | Prometheus endpoint | Dashboards, alerting |
| **Traces** | OpenTelemetry OTLP | Distributed tracing |
| **Logs** | Audit logging | Compliance, debugging |

---

## Prometheus Metrics

### Endpoint

```bash
curl http://localhost:3000/metrics
```

**Response:** Prometheus text format

```
# HELP mcp_guard_requests_total Total HTTP requests
# TYPE mcp_guard_requests_total counter
mcp_guard_requests_total{method="POST",status="200"} 1234
mcp_guard_requests_total{method="POST",status="401"} 56
mcp_guard_requests_total{method="POST",status="429"} 12
...
```

### Available Metrics

#### mcp_guard_requests_total

Total HTTP requests by method and status code.

| Label | Values | Description |
|-------|--------|-------------|
| `method` | GET, POST | HTTP method |
| `status` | 200, 401, 403, 429, 500 | HTTP status code |

**Use cases:**

- Request volume monitoring
- Error rate calculation
- Traffic patterns

#### mcp_guard_request_duration_seconds

Request latency histogram.

| Label | Values | Description |
|-------|--------|-------------|
| `method` | GET, POST | HTTP method |

**Buckets:** 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0

**Use cases:**

- P50, P95, P99 latency
- SLA monitoring
- Performance degradation detection

#### mcp_guard_auth_total

Authentication attempts by provider and result.

| Label | Values | Description |
|-------|--------|-------------|
| `provider` | api_key, jwt, oauth, mtls | Auth provider used |
| `result` | success, failure | Authentication result |

**Use cases:**

- Auth success rate
- Provider usage comparison
- Attack detection (high failure rate)

#### mcp_guard_rate_limit_total

Rate limiting decisions.

| Label | Values | Description |
|-------|--------|-------------|
| `allowed` | true, false | Whether request was allowed |

**Use cases:**

- Rate limit hit rate
- Capacity planning
- Abuse detection

#### mcp_guard_active_identities

Current number of tracked identities (gauge).

**Use cases:**

- Memory usage estimation
- Unique client count
- Identity cleanup monitoring

### Prometheus Configuration

**prometheus.yml:**

```yaml
scrape_configs:
  - job_name: 'mcp-guard'
    static_configs:
      - targets: ['mcp-guard:3000']
    scrape_interval: 15s
    metrics_path: /metrics
```

**With service discovery (Kubernetes):**

```yaml
scrape_configs:
  - job_name: 'mcp-guard'
    kubernetes_sd_configs:
      - role: pod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_label_app]
        regex: mcp-guard
        action: keep
```

### Example Queries

**Request rate (per second):**

```promql
rate(mcp_guard_requests_total[5m])
```

**Error rate (%):**

```promql
sum(rate(mcp_guard_requests_total{status=~"4..|5.."}[5m]))
/
sum(rate(mcp_guard_requests_total[5m]))
* 100
```

**P99 latency:**

```promql
histogram_quantile(0.99, rate(mcp_guard_request_duration_seconds_bucket[5m]))
```

**Auth failure rate:**

```promql
sum(rate(mcp_guard_auth_total{result="failure"}[5m]))
/
sum(rate(mcp_guard_auth_total[5m]))
```

**Rate limit rejection rate:**

```promql
sum(rate(mcp_guard_rate_limit_total{allowed="false"}[5m]))
/
sum(rate(mcp_guard_rate_limit_total[5m]))
```

### Grafana Dashboard

Example dashboard panels:

**Request Volume:**

```json
{
  "title": "Request Rate",
  "type": "timeseries",
  "targets": [{
    "expr": "sum(rate(mcp_guard_requests_total[5m]))",
    "legendFormat": "requests/s"
  }]
}
```

**Latency Heatmap:**

```json
{
  "title": "Request Latency",
  "type": "heatmap",
  "targets": [{
    "expr": "sum(rate(mcp_guard_request_duration_seconds_bucket[5m])) by (le)",
    "format": "heatmap"
  }]
}
```

---

## OpenTelemetry Tracing

### Configuration

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"
sample_rate = 0.1
propagate_context = true
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `false` | Enable tracing |
| `service_name` | string | `"mcp-guard"` | Service name in traces |
| `otlp_endpoint` | string | Required | OTLP gRPC endpoint |
| `sample_rate` | float | `0.1` | Sampling rate (0.0-1.0) |
| `propagate_context` | boolean | `true` | W3C trace context propagation |

### Sampling Strategies

| sample_rate | Strategy | Use Case |
|-------------|----------|----------|
| `1.0` | Always sample | Development, debugging |
| `0.1` | 10% sampling | Production (default) |
| `0.01` | 1% sampling | High-volume production |
| `0.0` | Never sample | Disabled |

### W3C Trace Context

When `propagate_context = true`, MCP Guard:

1. **Extracts** `traceparent` and `tracestate` headers from incoming requests
2. **Creates** child spans within the same trace
3. **Injects** trace context into upstream requests
4. **Includes** `trace_id` in audit logs

**Header format:**

```
traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01
tracestate: rojo=00f067aa0ba902b7
```

### Span Attributes

MCP Guard spans include:

| Attribute | Description |
|-----------|-------------|
| `http.method` | HTTP method (GET, POST) |
| `http.url` | Request URL |
| `http.status_code` | Response status |
| `identity.id` | Authenticated identity ID |
| `mcp.method` | JSON-RPC method (tools/list, etc.) |

### Backend Setup

#### Jaeger

**Docker Compose:**

```yaml
version: '3.8'
services:
  jaeger:
    image: jaegertracing/all-in-one:1.52
    ports:
      - "16686:16686"  # UI
      - "4317:4317"    # OTLP gRPC
    environment:
      - COLLECTOR_OTLP_ENABLED=true

  mcp-guard:
    image: mcp-guard:latest
    ports:
      - "3000:3000"
    volumes:
      - ./mcp-guard.toml:/etc/mcp-guard/config.toml
    depends_on:
      - jaeger
```

**MCP Guard config:**

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://jaeger:4317"
sample_rate = 1.0
```

**Access UI:** http://localhost:16686

#### Grafana Tempo

**Docker Compose:**

```yaml
version: '3.8'
services:
  tempo:
    image: grafana/tempo:2.3.1
    command: ["-config.file=/etc/tempo.yaml"]
    volumes:
      - ./tempo.yaml:/etc/tempo.yaml
    ports:
      - "4317:4317"    # OTLP gRPC
      - "3200:3200"    # Tempo API

  grafana:
    image: grafana/grafana:10.2.2
    ports:
      - "3001:3000"
    environment:
      - GF_AUTH_ANONYMOUS_ENABLED=true
      - GF_AUTH_ANONYMOUS_ORG_ROLE=Admin
```

**tempo.yaml:**

```yaml
server:
  http_listen_port: 3200

distributor:
  receivers:
    otlp:
      protocols:
        grpc:
          endpoint: "0.0.0.0:4317"

storage:
  trace:
    backend: local
    local:
      path: /var/tempo/traces
```

**MCP Guard config:**

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://tempo:4317"
sample_rate = 0.1
```

#### Honeycomb

**MCP Guard config:**

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "https://api.honeycomb.io:443"
sample_rate = 0.1
```

Set environment variable for API key:

```bash
export OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=YOUR_API_KEY"
```

### Correlation IDs

Trace IDs appear in audit logs for correlation:

```json
{
  "event_type": "ToolCall",
  "trace_id": "0af7651916cd43dd8448eb211c80319c",
  "identity_id": "user123",
  "tool": "read_file"
}
```

---

## Audit Logging

### Configuration

```toml
[audit]
enabled = true
stdout = false
file = "/var/log/mcp-guard/audit.log"
export_url = "https://siem.example.com/api/logs"
export_batch_size = 100
export_interval_secs = 30
export_headers = { "Authorization" = "Bearer token" }
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable audit logging |
| `stdout` | boolean | `false` | Log to stdout |
| `file` | string | None | Log file path |
| `export_url` | string | None | HTTP export endpoint |
| `export_batch_size` | integer | `100` | Events per batch |
| `export_interval_secs` | integer | `30` | Max seconds between flushes |
| `export_headers` | table | `{}` | Custom headers for HTTP export |

### Event Types

| Event Type | Description | Fields |
|------------|-------------|--------|
| `AuthSuccess` | Successful authentication | identity_id, provider |
| `AuthFailure` | Failed authentication | reason, provider |
| `ToolCall` | Tool invocation | identity_id, tool, duration_ms |
| `ToolCallResult` | Tool response | identity_id, tool, success |
| `RateLimited` | Rate limit exceeded | identity_id, retry_after_secs |
| `AuthzDenied` | Authorization denied | identity_id, tool, reason |

### Event Schema

```json
{
  "timestamp": "2024-12-15T10:30:00.123Z",
  "event_type": "ToolCall",
  "identity_id": "user123",
  "method": "POST",
  "tool": "read_file",
  "success": true,
  "duration_ms": 45,
  "request_id": "req-abc123",
  "trace_id": "0af7651916cd43dd8448eb211c80319c"
}
```

### SIEM Integration

#### Splunk HEC

**MCP Guard config:**

```toml
[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 15
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
```

**Splunk setup:**

1. Enable HTTP Event Collector
2. Create new token
3. Configure index and sourcetype

#### Datadog

**MCP Guard config:**

```toml
[audit]
enabled = true
export_url = "https://http-intake.logs.datadoghq.com/api/v2/logs"
export_batch_size = 50
export_interval_secs = 10
export_headers = { "DD-API-KEY" = "your-datadog-api-key" }
```

#### Elasticsearch

**MCP Guard config:**

```toml
[audit]
enabled = true
export_url = "https://elasticsearch.example.com:9200/mcp-guard-audit/_bulk"
export_batch_size = 100
export_interval_secs = 30
export_headers = { "Authorization" = "Basic base64-credentials" }
```

### Retry Behavior

HTTP export implements exponential backoff:

- **Attempts:** 3
- **Initial delay:** 1 second
- **Max delay:** 10 seconds
- **On failure:** Events logged locally (if file configured)

### Multiple Outputs

Enable multiple outputs simultaneously:

```toml
[audit]
enabled = true
stdout = true                    # Console output
file = "/var/log/audit.log"      # File output
export_url = "https://siem..."   # SIEM output
```

---

## Health Endpoints

### GET /health

Detailed health status with version and uptime.

**Request:**

```bash
curl http://localhost:3000/health
```

**Response (200 OK):**

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_secs": 3600
}
```

**Use case:** Detailed health information for monitoring systems.

### GET /live

Simple liveness check.

**Request:**

```bash
curl http://localhost:3000/live
```

**Response (200 OK):**

```json
{
  "status": "ok"
}
```

**Use case:** Kubernetes liveness probe.

### GET /ready

Readiness check (returns 503 if not ready).

**Request:**

```bash
curl http://localhost:3000/ready
```

**Response (200 OK):**

```json
{
  "status": "ready"
}
```

**Response (503 Service Unavailable):**

```json
{
  "status": "not_ready"
}
```

**Use case:** Kubernetes readiness probe, load balancer health check.

### Kubernetes Probes

```yaml
apiVersion: v1
kind: Pod
spec:
  containers:
    - name: mcp-guard
      livenessProbe:
        httpGet:
          path: /live
          port: 3000
        initialDelaySeconds: 5
        periodSeconds: 10
      readinessProbe:
        httpGet:
          path: /ready
          port: 3000
        initialDelaySeconds: 5
        periodSeconds: 5
```

---

## Alerting Recommendations

### Critical Alerts

**High error rate:**

```promql
sum(rate(mcp_guard_requests_total{status=~"5.."}[5m]))
/
sum(rate(mcp_guard_requests_total[5m]))
> 0.05
```

**Auth failure spike:**

```promql
sum(rate(mcp_guard_auth_total{result="failure"}[5m])) > 10
```

**Service down:**

```promql
up{job="mcp-guard"} == 0
```

### Warning Alerts

**High latency:**

```promql
histogram_quantile(0.99, rate(mcp_guard_request_duration_seconds_bucket[5m])) > 1
```

**Rate limit saturation:**

```promql
sum(rate(mcp_guard_rate_limit_total{allowed="false"}[5m]))
/
sum(rate(mcp_guard_rate_limit_total[5m]))
> 0.1
```

**Memory growth (identity count):**

```promql
mcp_guard_active_identities > 10000
```

---

## Debugging with Observability

### Tracing a Slow Request

1. Find slow requests in metrics:
   ```promql
   histogram_quantile(0.99, rate(mcp_guard_request_duration_seconds_bucket[5m]))
   ```

2. Search traces by duration in Jaeger/Tempo

3. Check span breakdown:
   - Auth time
   - Upstream time
   - Response processing

### Correlating Logs with Traces

1. Get `trace_id` from trace UI
2. Search audit logs:
   ```bash
   grep "trace_id\":\"0af7651916cd43dd" /var/log/audit.log
   ```

### Finding Authentication Issues

1. Check auth metrics:
   ```promql
   rate(mcp_guard_auth_total{result="failure"}[5m])
   ```

2. Filter audit logs by `AuthFailure` events
3. Check `reason` field for specific errors

---

## Troubleshooting

### Metrics Not Appearing

1. Verify `/metrics` endpoint responds:
   ```bash
   curl http://localhost:3000/metrics
   ```

2. Check Prometheus scrape config
3. Verify network connectivity between Prometheus and MCP Guard

### Traces Not Exporting

1. Verify OTLP endpoint is reachable:
   ```bash
   curl http://jaeger:4317/api/traces
   ```

2. Check `sample_rate` is not 0.0
3. Enable verbose logging: `mcp-guard -v run`

### Audit Logs Missing

1. Verify `audit.enabled = true`
2. Check file permissions if using file output
3. For HTTP export, check endpoint and headers
4. Look for retry/failure messages in console

### High Memory from Tracing

1. Reduce sample rate in production
2. Ensure traces are being exported (check backend)
3. Consider batch size configuration

---

## See Also

- [Configuration Reference](configuration.md) - Complete configuration options
- [Deployment Guide](deployment.md) - Production monitoring setup
- [Troubleshooting Guide](troubleshooting.md) - Common issues
- [Rate Limiting Guide](rate-limiting.md) - Rate limit monitoring
