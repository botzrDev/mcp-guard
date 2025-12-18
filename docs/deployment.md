# Deployment Guide

Production deployment patterns for MCP Guard: binary, Docker, Kubernetes, and high availability.

## Overview

MCP Guard is designed for simple deployment with zero external dependencies. Choose the deployment method that fits your infrastructure.

| Method | Best For | Complexity |
|--------|----------|------------|
| **Binary** | VMs, bare metal, simple setups | Low |
| **Docker** | Container environments | Low |
| **Kubernetes** | Cloud-native, auto-scaling | Medium |

### Production Checklist

Before deploying to production:

- [ ] Configuration validated: `mcp-guard validate`
- [ ] Upstream connectivity tested: `mcp-guard check-upstream`
- [ ] TLS configured (HTTPS)
- [ ] Authentication provider configured
- [ ] Rate limiting enabled and tuned
- [ ] Audit logging configured (file or SIEM)
- [ ] Health checks configured for your platform
- [ ] Monitoring/alerting set up

---

## Binary Deployment

### System Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| CPU | 1 core | 2+ cores |
| Memory | 50 MB | 128 MB |
| Disk | 10 MB | 100 MB (for logs) |

Supported platforms:

- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

### Installation

**From crates.io:**

```bash
cargo install mcp-guard
```

**From GitHub releases:**

```bash
# Linux (x86_64)
curl -L https://github.com/botzrdev/mcp-guard/releases/latest/download/mcp-guard-linux-x86_64.tar.gz | tar xz
sudo mv mcp-guard /usr/local/bin/

# Verify
mcp-guard version
```

**From source:**

```bash
git clone https://github.com/botzrdev/mcp-guard
cd mcp-guard
cargo build --release
sudo cp target/release/mcp-guard /usr/local/bin/
```

### Systemd Service

Create `/etc/systemd/system/mcp-guard.service`:

```ini
[Unit]
Description=MCP Guard Security Gateway
After=network.target

[Service]
Type=simple
User=mcp-guard
Group=mcp-guard
ExecStart=/usr/local/bin/mcp-guard --config /etc/mcp-guard/config.toml run
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=5

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadWritePaths=/var/log/mcp-guard

# Resource limits
LimitNOFILE=65536
MemoryMax=512M

[Install]
WantedBy=multi-user.target
```

**Setup:**

```bash
# Create user
sudo useradd -r -s /bin/false mcp-guard

# Create directories
sudo mkdir -p /etc/mcp-guard /var/log/mcp-guard
sudo chown mcp-guard:mcp-guard /var/log/mcp-guard

# Copy configuration
sudo cp mcp-guard.toml /etc/mcp-guard/config.toml

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable mcp-guard
sudo systemctl start mcp-guard

# Check status
sudo systemctl status mcp-guard
sudo journalctl -u mcp-guard -f
```

### Log Rotation

Create `/etc/logrotate.d/mcp-guard`:

```
/var/log/mcp-guard/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 0640 mcp-guard mcp-guard
    postrotate
        systemctl reload mcp-guard 2>/dev/null || true
    endscript
}
```

### Upgrade Process

```bash
# Download new version
curl -L https://github.com/botzrdev/mcp-guard/releases/latest/download/mcp-guard-linux-x86_64.tar.gz | tar xz

# Verify new version works
./mcp-guard version
./mcp-guard --config /etc/mcp-guard/config.toml validate

# Replace binary
sudo systemctl stop mcp-guard
sudo mv mcp-guard /usr/local/bin/
sudo systemctl start mcp-guard
```

---

## Docker Deployment

### Official Image

```bash
docker pull ghcr.io/botzrdev/mcp-guard:latest
```

### Dockerfile Example

Build your own image:

```dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mcp-guard /usr/local/bin/

# Non-root user
RUN useradd -r -s /bin/false mcp-guard
USER mcp-guard

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/mcp-guard"]
CMD ["--config", "/etc/mcp-guard/config.toml", "run"]
```

### Docker Compose

**docker-compose.yml:**

```yaml
version: '3.8'

services:
  mcp-guard:
    image: ghcr.io/botzrdev/mcp-guard:latest
    ports:
      - "3000:3000"
    volumes:
      - ./mcp-guard.toml:/etc/mcp-guard/config.toml:ro
      - ./audit-logs:/var/log/mcp-guard
    environment:
      - RUST_LOG=info
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/live"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
    restart: unless-stopped
```

**With stdio transport (requires host processes):**

```yaml
version: '3.8'

services:
  mcp-guard:
    image: ghcr.io/botzrdev/mcp-guard:latest
    ports:
      - "3000:3000"
    volumes:
      - ./mcp-guard.toml:/etc/mcp-guard/config.toml:ro
      - /usr/bin/npx:/usr/bin/npx:ro  # If using npx
      - /home/user/.npm:/home/mcp-guard/.npm:ro  # npm cache
    restart: unless-stopped
```

### Volume Mounts

| Path | Type | Description |
|------|------|-------------|
| `/etc/mcp-guard/config.toml` | Read-only | Configuration file |
| `/etc/ssl/certs` | Read-only | TLS certificates |
| `/var/log/mcp-guard` | Read-write | Audit logs |

### Environment Variables

MCP Guard reads configuration from files, not environment variables. Use your orchestration to substitute values:

```yaml
services:
  mcp-guard:
    image: ghcr.io/botzrdev/mcp-guard:latest
    environment:
      - RUST_LOG=info  # Log level
    command: >
      sh -c "envsubst < /etc/mcp-guard/config.template.toml > /tmp/config.toml &&
             mcp-guard --config /tmp/config.toml run"
```

---

## Kubernetes Deployment

### Deployment Manifest

**mcp-guard-deployment.yaml:**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mcp-guard
  labels:
    app: mcp-guard
spec:
  replicas: 3
  selector:
    matchLabels:
      app: mcp-guard
  template:
    metadata:
      labels:
        app: mcp-guard
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "3000"
        prometheus.io/path: "/metrics"
    spec:
      serviceAccountName: mcp-guard
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000
      containers:
        - name: mcp-guard
          image: ghcr.io/botzrdev/mcp-guard:latest
          ports:
            - containerPort: 3000
              name: http
          args:
            - "--config"
            - "/etc/mcp-guard/config.toml"
            - "run"
          env:
            - name: RUST_LOG
              value: "info"
          resources:
            requests:
              memory: "64Mi"
              cpu: "100m"
            limits:
              memory: "256Mi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /live
              port: http
            initialDelaySeconds: 5
            periodSeconds: 10
          readinessProbe:
            httpGet:
              path: /ready
              port: http
            initialDelaySeconds: 5
            periodSeconds: 5
          volumeMounts:
            - name: config
              mountPath: /etc/mcp-guard
              readOnly: true
            - name: tls
              mountPath: /etc/ssl/mcp-guard
              readOnly: true
      volumes:
        - name: config
          configMap:
            name: mcp-guard-config
        - name: tls
          secret:
            secretName: mcp-guard-tls
```

### Service Manifest

**mcp-guard-service.yaml:**

```yaml
apiVersion: v1
kind: Service
metadata:
  name: mcp-guard
  labels:
    app: mcp-guard
spec:
  type: ClusterIP
  ports:
    - port: 3000
      targetPort: http
      protocol: TCP
      name: http
  selector:
    app: mcp-guard
```

### ConfigMap

**mcp-guard-configmap.yaml:**

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: mcp-guard-config
data:
  config.toml: |
    [server]
    host = "0.0.0.0"
    port = 3000

    [upstream]
    transport = "http"
    url = "http://mcp-server.default.svc:8080/mcp"

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
    stdout = true
```

### Secret

**mcp-guard-secret.yaml:**

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: mcp-guard-secrets
type: Opaque
stringData:
  jwt-secret: "your-jwt-secret-here"
  oauth-client-secret: "your-oauth-secret"
---
apiVersion: v1
kind: Secret
metadata:
  name: mcp-guard-tls
type: kubernetes.io/tls
data:
  tls.crt: <base64-encoded-cert>
  tls.key: <base64-encoded-key>
```

### Horizontal Pod Autoscaler

**mcp-guard-hpa.yaml:**

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
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
```

### Ingress

**mcp-guard-ingress.yaml:**

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: mcp-guard
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
    - hosts:
        - mcp.example.com
      secretName: mcp-guard-tls
  rules:
    - host: mcp.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: mcp-guard
                port:
                  number: 3000
```

---

## TLS Configuration

### Direct TLS Termination

MCP Guard can terminate TLS directly:

```toml
[server]
host = "0.0.0.0"
port = 443

[server.tls]
cert_path = "/etc/ssl/server.crt"
key_path = "/etc/ssl/server.key"
```

### Behind Reverse Proxy (Recommended)

Let a reverse proxy (nginx, HAProxy, cloud load balancer) handle TLS:

```
Client ──── HTTPS ────> nginx ──── HTTP ────> MCP Guard
                        (TLS termination)
```

**nginx configuration:**

```nginx
server {
    listen 443 ssl http2;
    server_name mcp.example.com;

    ssl_certificate /etc/letsencrypt/live/mcp.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/mcp.example.com/privkey.pem;

    # Modern TLS configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256;
    ssl_prefer_server_ciphers off;

    location / {
        proxy_pass http://mcp-guard:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Let's Encrypt Automation

With certbot:

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate
sudo certbot --nginx -d mcp.example.com

# Auto-renewal is configured automatically
```

### Certificate Rotation

For direct TLS, restart MCP Guard after certificate renewal:

```bash
# In certbot post-hook
systemctl reload mcp-guard
```

---

## mTLS Setup

### Architecture

```
Client ──── mTLS ────> nginx ──── Headers ────> MCP Guard
            (cert validation)      (X-Client-Cert-*)
```

### Certificate Authority Setup

Generate a CA for client certificates:

```bash
# Create CA key
openssl genrsa -out ca.key 4096

# Create CA certificate
openssl req -new -x509 -days 3650 -key ca.key -out ca.crt \
  -subj "/CN=MCP Guard Client CA"

# Create client key
openssl genrsa -out client.key 2048

# Create client CSR
openssl req -new -key client.key -out client.csr \
  -subj "/CN=my-service"

# Sign client certificate
openssl x509 -req -in client.csr -CA ca.crt -CAkey ca.key \
  -CAcreateserial -out client.crt -days 365
```

### nginx mTLS Configuration

```nginx
server {
    listen 443 ssl;
    server_name mcp.example.com;

    ssl_certificate /etc/ssl/server.crt;
    ssl_certificate_key /etc/ssl/server.key;

    # Client certificate validation
    ssl_client_certificate /etc/ssl/client-ca.crt;
    ssl_verify_client on;
    ssl_verify_depth 2;

    location / {
        # Forward certificate info to MCP Guard
        proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
        proxy_set_header X-Client-Cert-Verified $ssl_client_verify;

        proxy_pass http://mcp-guard:3000;
    }
}
```

### MCP Guard mTLS Configuration

```toml
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.1"]  # nginx server IP
```

### Testing mTLS

```bash
curl --cert client.crt --key client.key \
  https://mcp.example.com/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

---

## High Availability

### Stateless Design

MCP Guard is stateless by design:

- No session state
- No in-memory data dependencies
- Rate limiters are per-instance (acceptable for most use cases)

### Multiple Instances

Run multiple instances behind a load balancer:

```
                    ┌─────────────────┐
                    │  Load Balancer  │
                    └────────┬────────┘
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
       ┌────────────┐ ┌────────────┐ ┌────────────┐
       │ MCP Guard  │ │ MCP Guard  │ │ MCP Guard  │
       │ Instance 1 │ │ Instance 2 │ │ Instance 3 │
       └────────────┘ └────────────┘ └────────────┘
```

### Load Balancer Configuration

**nginx:**

```nginx
upstream mcp_guard {
    server mcp-guard-1:3000;
    server mcp-guard-2:3000;
    server mcp-guard-3:3000;
}

server {
    listen 443 ssl;

    location / {
        proxy_pass http://mcp_guard;
    }
}
```

### Session Affinity

Session affinity is **not required** because:

- Authentication is per-request
- Rate limiting is per-instance (slight variability acceptable)

For strict rate limit consistency, consider a shared rate limiter (Redis) in future versions.

---

## Security Hardening

### Network Policies (Kubernetes)

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: mcp-guard
spec:
  podSelector:
    matchLabels:
      app: mcp-guard
  policyTypes:
    - Ingress
    - Egress
  ingress:
    - from:
        - namespaceSelector:
            matchLabels:
              name: ingress-nginx
      ports:
        - port: 3000
  egress:
    - to:
        - namespaceSelector:
            matchLabels:
              name: mcp-servers
      ports:
        - port: 8080
    - to:  # Allow JWKS fetch
        - ipBlock:
            cidr: 0.0.0.0/0
      ports:
        - port: 443
```

### Run as Non-Root

Always run MCP Guard as a non-root user:

**Docker:**

```dockerfile
RUN useradd -r -s /bin/false mcp-guard
USER mcp-guard
```

**Kubernetes:**

```yaml
securityContext:
  runAsNonRoot: true
  runAsUser: 1000
```

### Read-Only Filesystem

```yaml
securityContext:
  readOnlyRootFilesystem: true
volumeMounts:
  - name: tmp
    mountPath: /tmp
volumes:
  - name: tmp
    emptyDir: {}
```

### Secrets Management

Never store secrets in:

- Configuration files committed to git
- Environment variables visible in process lists
- Container images

Use:

- Kubernetes Secrets
- HashiCorp Vault
- AWS Secrets Manager
- Cloud-native secret stores

---

## Monitoring in Production

### Prometheus Integration

See [Observability Guide](observability.md) for complete setup.

**Essential alerts:**

```yaml
groups:
  - name: mcp-guard
    rules:
      - alert: MCPGuardDown
        expr: up{job="mcp-guard"} == 0
        for: 1m
        labels:
          severity: critical

      - alert: HighErrorRate
        expr: |
          sum(rate(mcp_guard_requests_total{status=~"5.."}[5m])) /
          sum(rate(mcp_guard_requests_total[5m])) > 0.05
        for: 5m
        labels:
          severity: warning
```

### Log Aggregation

Send audit logs to your centralized logging:

```toml
[audit]
enabled = true
export_url = "https://logs.example.com/api/v1/push"
export_headers = { "Authorization" = "Bearer token" }
```

---

## Backup and Recovery

### Configuration Backup

Store configuration in version control:

```bash
# Backup
cp /etc/mcp-guard/config.toml ./backups/config-$(date +%Y%m%d).toml

# Or use git
cd /etc/mcp-guard
git init
git add config.toml
git commit -m "Configuration backup"
```

### Audit Log Archival

Archive audit logs for compliance:

```bash
# Compress and archive
tar -czf audit-$(date +%Y%m).tar.gz /var/log/mcp-guard/

# Upload to S3
aws s3 cp audit-$(date +%Y%m).tar.gz s3://backup-bucket/mcp-guard/
```

### Disaster Recovery

Recovery procedure:

1. Deploy new MCP Guard instance
2. Apply configuration from backup
3. Verify with `mcp-guard validate`
4. Test with `mcp-guard check-upstream`
5. Update DNS/load balancer

---

## See Also

- [Quick Start Guide](quickstart.md) - Initial setup
- [Configuration Reference](configuration.md) - All configuration options
- [Observability Guide](observability.md) - Monitoring setup
- [Troubleshooting Guide](troubleshooting.md) - Common issues
