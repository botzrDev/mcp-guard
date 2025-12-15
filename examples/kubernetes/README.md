# Kubernetes Deployment

This example shows how to deploy mcp-guard on Kubernetes with proper health checks, resource limits, and security settings.

## Features

- **High Availability**: 2 replicas for redundancy
- **Health Checks**: Liveness and readiness probes
- **Resource Limits**: Memory and CPU constraints
- **Security**: Non-root user, read-only filesystem
- **Observability**: Prometheus annotations for metrics scraping
- **Configuration**: ConfigMap for easy updates

## Deploy

```bash
kubectl apply -f deployment.yaml
```

## Verify

```bash
# Check pods are running
kubectl get pods -l app=mcp-guard

# Check readiness
kubectl get endpoints mcp-guard

# View logs
kubectl logs -l app=mcp-guard -f
```

## Access

```bash
# Port forward for local testing
kubectl port-forward svc/mcp-guard 3000:80

# Test health
curl http://localhost:3000/health
```

## Configuration

### Using Secrets for API Keys

For production, store sensitive data in Secrets:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: mcp-guard-secrets
type: Opaque
stringData:
  api-key-hash: "YOUR_ACTUAL_HASH"
```

Then reference in the ConfigMap or use environment variables.

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: mcp-guard
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: mcp-guard
  minReplicas: 2
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
```

## Monitoring

### Prometheus ServiceMonitor

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: mcp-guard
spec:
  selector:
    matchLabels:
      app: mcp-guard
  endpoints:
    - port: http
      path: /metrics
      interval: 30s
```

### Grafana Dashboard

Import the Prometheus metrics:
- `mcp_guard_requests_total`
- `mcp_guard_request_duration_seconds`
- `mcp_guard_auth_total`
- `mcp_guard_rate_limit_total`

## Troubleshooting

### Pod not ready
```bash
kubectl describe pod -l app=mcp-guard
kubectl logs -l app=mcp-guard --previous
```

### Upstream connection issues
Ensure the upstream MCP server is accessible from the pod network:
```bash
kubectl exec -it deploy/mcp-guard -- curl http://mcp-server:8080/health
```
