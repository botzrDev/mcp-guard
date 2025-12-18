# Jaeger Integration Guide

Set up distributed tracing for MCP Guard using Jaeger.

## Prerequisites

- Docker installed
- MCP Guard installed and running
- Basic familiarity with distributed tracing concepts

---

## Step 1: Start Jaeger

The easiest way to run Jaeger is using the all-in-one Docker image:

```bash
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  -e COLLECTOR_OTLP_ENABLED=true \
  jaegertracing/all-in-one:1.52
```

**Exposed ports:**

| Port | Protocol | Description |
|------|----------|-------------|
| 16686 | HTTP | Jaeger UI |
| 4317 | gRPC | OTLP receiver |

Verify Jaeger is running:

```bash
curl http://localhost:16686/api/services
```

---

## Step 2: Configure MCP Guard

Add the tracing configuration to your `mcp-guard.toml`:

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"
sample_rate = 1.0        # 100% for development
propagate_context = true
```

For production, use a lower sample rate:

```toml
[tracing]
enabled = true
service_name = "mcp-guard-prod"
otlp_endpoint = "http://jaeger.monitoring.svc:4317"
sample_rate = 0.1  # 10% sampling
propagate_context = true
```

---

## Step 3: Generate Traces

1. Start MCP Guard:
   ```bash
   mcp-guard run
   ```

2. Make some requests to generate traces:
   ```bash
   curl -X POST http://localhost:3000/mcp \
     -H "Authorization: Bearer YOUR_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
   ```

---

## Step 4: View Traces in Jaeger UI

1. Open Jaeger UI: http://localhost:16686

2. Select service: **mcp-guard** from the dropdown

3. Click **Find Traces**

4. Click on a trace to see the span breakdown

---

## Understanding Trace Data

### Span Attributes

MCP Guard traces include these attributes:

| Attribute | Description |
|-----------|-------------|
| `http.method` | HTTP method (GET, POST) |
| `http.url` | Request URL |
| `http.status_code` | Response status code |
| `identity.id` | Authenticated user ID |
| `mcp.method` | JSON-RPC method name |

### Trace Structure

A typical MCP Guard trace shows:

```
mcp-guard: POST /mcp
├── auth: authenticate
├── rate_limit: check
├── authz: authorize_tool
└── transport: forward_request
```

---

## Docker Compose Setup

For a complete local observability stack:

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
      - ./mcp-guard.toml:/etc/mcp-guard/config.toml:ro
    depends_on:
      - jaeger
    environment:
      - RUST_LOG=info
```

**mcp-guard.toml:**

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[auth.api_keys]]
id = "dev"
key_hash = "YOUR_KEY_HASH"

[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://jaeger:4317"
sample_rate = 1.0
propagate_context = true
```

Start the stack:

```bash
docker-compose up -d
```

---

## Trace Context Propagation

MCP Guard supports W3C trace context propagation. When `propagate_context = true`:

1. **Incoming requests:** Extracts `traceparent` and `tracestate` headers
2. **Outgoing requests:** Injects trace context into upstream requests
3. **Audit logs:** Includes `trace_id` for correlation

### Testing Propagation

Send a request with trace context:

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -H "traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

The trace will continue in Jaeger under the provided trace ID.

---

## Sampling Strategies

| sample_rate | Behavior | Use Case |
|-------------|----------|----------|
| `1.0` | Trace every request | Development, debugging |
| `0.1` | 10% of requests | Production default |
| `0.01` | 1% of requests | High-volume production |
| `0.0` | No sampling | Effectively disabled |

---

## Finding Specific Traces

### By Trace ID

If you have a trace ID from logs or errors:

1. Go to Jaeger UI
2. Use the **Trace ID** search box
3. Enter the trace ID (e.g., `0af7651916cd43dd8448eb211c80319c`)

### By Duration

Find slow requests:

1. Go to **Search** tab
2. Set **Min Duration** (e.g., `100ms`)
3. Click **Find Traces**

### By Tags

Filter by specific attributes:

1. Use the **Tags** field
2. Enter: `http.status_code=500` or `identity.id=user123`

---

## Production Deployment

### Jaeger with Persistent Storage

For production, use Jaeger with Elasticsearch or Cassandra:

```yaml
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:7.17.9
    environment:
      - discovery.type=single-node
      - ES_JAVA_OPTS=-Xms512m -Xmx512m
    volumes:
      - esdata:/usr/share/elasticsearch/data

  jaeger:
    image: jaegertracing/all-in-one:1.52
    ports:
      - "16686:16686"
      - "4317:4317"
    environment:
      - SPAN_STORAGE_TYPE=elasticsearch
      - ES_SERVER_URLS=http://elasticsearch:9200
      - COLLECTOR_OTLP_ENABLED=true
    depends_on:
      - elasticsearch

volumes:
  esdata:
```

### Kubernetes Deployment

Use the Jaeger Operator for Kubernetes:

```yaml
apiVersion: jaegertracing.io/v1
kind: Jaeger
metadata:
  name: mcp-jaeger
spec:
  strategy: production
  storage:
    type: elasticsearch
    options:
      es:
        server-urls: http://elasticsearch:9200
```

---

## Troubleshooting

### "No traces appearing"

1. Verify Jaeger is running:
   ```bash
   curl http://localhost:16686/api/services
   ```

2. Check OTLP endpoint is reachable:
   ```bash
   curl -v http://localhost:4317
   ```

3. Verify tracing is enabled in config:
   ```toml
   [tracing]
   enabled = true
   ```

4. Check `sample_rate` is not `0.0`

5. Enable verbose logging:
   ```bash
   RUST_LOG=debug mcp-guard run
   ```

### "Connection refused to OTLP endpoint"

1. Verify the port (4317 for gRPC)
2. Check Jaeger container is healthy
3. Verify network connectivity between MCP Guard and Jaeger

### "Service not appearing in dropdown"

1. Generate at least one trace by making a request
2. Refresh the Jaeger UI
3. Check the service name matches what you configured

### "Traces are incomplete"

1. Verify `propagate_context = true`
2. Check upstream services also support tracing
3. Increase `sample_rate` during debugging

---

## See Also

- [Observability Guide](../observability.md) - Complete tracing documentation
- [Configuration Reference](../configuration.md) - All configuration options
- [Troubleshooting Guide](../troubleshooting.md) - Common issues and solutions
