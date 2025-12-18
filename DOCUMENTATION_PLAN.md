# MCP Guard Documentation Plan

> **Created:** 2025-12-15
> **Purpose:** Comprehensive outline for internal developer and public user documentation
> **Target Audience:** Open source contributors, enterprise users, MCP server operators

---

## Table of Contents

1. [User Documentation (Public)](#user-documentation-public)
2. [Internal Developer Documentation](#internal-developer-documentation)
3. [API Reference](#api-reference)
4. [Configuration Deep-Dives](#configuration-deep-dives)
5. [Documentation Standards](#documentation-standards)

---

## User Documentation (Public)

### 1. Quick Start Guide

**File:** `docs/quickstart.md`
**Goal:** Get users from zero to secured MCP server in 5 minutes

#### Outline

```
1. Introduction
   1.1. What is MCP Guard?
   1.2. What problem does it solve?
   1.3. Prerequisites (Rust toolchain or prebuilt binary)

2. Installation
   2.1. From crates.io (cargo install mcp-guard)
   2.2. From GitHub releases (prebuilt binaries)
   2.3. From source (cargo build --release)
   2.4. Verify installation (mcp-guard version)

3. Generate Configuration
   3.1. Run `mcp-guard init`
   3.2. Understand the generated file structure
   3.3. Choose format: TOML vs YAML

4. Create Your First API Key
   4.1. Run `mcp-guard keygen --user-id my-app`
   4.2. Save the generated key securely
   4.3. Note: key_hash goes in config, key goes to client

5. Configure Your Upstream MCP Server
   5.1. Stdio example (npx filesystem server)
   5.2. Validate config: `mcp-guard validate`

6. Start the Gateway
   6.1. Run `mcp-guard run`
   6.2. Verify with health check: curl localhost:3000/health

7. Test Authentication
   7.1. Unauthenticated request (expect 401)
   7.2. Authenticated request with API key
   7.3. Call tools/list to see available tools

8. Next Steps
   8.1. Link to Configuration Reference
   8.2. Link to Authentication Guide
   8.3. Link to Deployment Guide
```

---

### 2. Configuration Reference

**File:** `docs/configuration.md`
**Goal:** Complete reference for all configuration options

#### Outline

```
1. Overview
   1.1. Supported formats (TOML, YAML)
   1.2. File locations and precedence
   1.3. Environment variable overrides (if supported)
   1.4. Validation: `mcp-guard validate`

2. Server Configuration
   2.1. [server] section
        - host (string, default: "127.0.0.1")
        - port (u16, default: 3000, valid: 1-65535)
   2.2. [server.tls] section
        - enabled (bool)
        - cert_path (path to PEM certificate)
        - key_path (path to PEM private key)
        - client_ca_path (path to CA for mTLS)
   2.3. Example: Basic HTTP server
   2.4. Example: HTTPS with Let's Encrypt
   2.5. Example: mTLS with client certificates

3. Authentication Configuration
   3.1. Overview of auth providers
   3.2. Provider priority and fallback behavior
   3.3. [[auth.api_keys]] array
        - id (string, unique identifier)
        - key_hash (string, base64 SHA-256)
        - allowed_tools (array of strings, optional)
        - rate_limit (u32, requests per second, optional)
   3.4. [auth.jwt] section
        - mode ("simple" | "jwks")
        - Simple mode options:
          * secret (string, HS256 key)
        - JWKS mode options:
          * jwks_url (URL, must be HTTPS in production)
          * algorithms (array, default: ["RS256", "ES256"])
          * cache_duration_secs (u64, default: 3600)
        - Common options:
          * issuer (string, expected iss claim)
          * audience (string, expected aud claim)
          * user_id_claim (string, default: "sub")
          * scopes_claim (string, default: "scope")
          * leeway_secs (u64, default: 60)
        - [auth.jwt.scope_tool_mapping] (map of scope -> tools)
   3.5. [auth.oauth] section
        - provider ("github" | "google" | "okta" | "custom")
        - client_id (string)
        - client_secret (string)
        - redirect_uri (URL)
        - scopes (array of strings)
        - Custom provider options:
          * authorization_url
          * token_url
          * introspection_url
          * userinfo_url
        - user_id_claim (string, default: "sub")
        - [auth.oauth.scope_tool_mapping]
   3.6. [auth.mtls] section
        - enabled (bool)
        - identity_source ("cn" | "san_dns" | "san_email")
        - allowed_tools (array)
        - rate_limit (u32)
   3.7. Example: API keys only
   3.8. Example: JWT with Auth0
   3.9. Example: OAuth with GitHub
   3.10. Example: mTLS with nginx

4. Rate Limiting Configuration
   4.1. [rate_limit] section
        - enabled (bool, default: true)
        - requests_per_second (u32, default: 100)
        - burst_size (u32, default: 50)
   4.2. Per-identity rate limits
   4.3. Rate limit headers explained
   4.4. Example: High-throughput configuration
   4.5. Example: Strict rate limiting

5. Audit Logging Configuration
   5.1. [audit] section
        - enabled (bool)
        - stdout (bool, log to console)
        - file (path, log to file)
        - export_url (URL, HTTP export for SIEM)
        - export_batch_size (u32, 1-10000, default: 100)
        - export_interval_secs (u64, default: 30)
        - [audit.export_headers] (map for auth headers)
   5.2. Audit event types
   5.3. Example: File logging only
   5.4. Example: Splunk HEC integration
   5.5. Example: Datadog logs integration

6. Observability Configuration
   6.1. [tracing] section
        - enabled (bool)
        - service_name (string)
        - otlp_endpoint (URL, gRPC endpoint)
        - sample_rate (f64, 0.0-1.0)
        - propagate_context (bool, W3C trace context)
   6.2. Example: Jaeger setup
   6.3. Example: Grafana Tempo setup
   6.4. Example: Honeycomb setup

7. Upstream Configuration
   7.1. Single-server mode [upstream]
        - transport ("stdio" | "http" | "sse")
        - Stdio options:
          * command (string, executable)
          * args (array of strings)
          * env (map of env vars, optional)
        - HTTP/SSE options:
          * url (URL, upstream endpoint)
   7.2. Multi-server mode [[upstream.servers]]
        - name (string, unique identifier)
        - path_prefix (string, must start with /)
        - transport, command, args, url (same as above)
        - strip_prefix (bool, default: false)
   7.3. Example: Local npx server (stdio)
   7.4. Example: Remote HTTP server
   7.5. Example: Multi-server routing

8. Complete Configuration Examples
   8.1. Minimal development config
   8.2. Production config with JWT
   8.3. Enterprise config with OAuth + mTLS
   8.4. Multi-tenant config with routing
```

---

### 3. CLI Reference

**File:** `docs/cli.md`
**Goal:** Complete reference for all CLI commands and options

#### Outline

```
1. Overview
   1.1. Installation verification
   1.2. Global options
        - --config <path> (default: mcp-guard.toml)
        - --verbose (enable debug logging)
   1.3. Exit codes

2. init Command
   2.1. Purpose: Generate template configuration file
   2.2. Usage: mcp-guard init [OPTIONS]
   2.3. Options:
        - --format <toml|yaml> (default: toml)
        - --force (overwrite existing file)
   2.4. Output: Creates mcp-guard.toml or mcp-guard.yaml
   2.5. Examples:
        - Generate TOML config
        - Generate YAML config
        - Force overwrite

3. validate Command
   3.1. Purpose: Parse and validate configuration file
   3.2. Usage: mcp-guard validate [--config <path>]
   3.3. Validation checks performed:
        - File exists and is readable
        - Valid TOML/YAML syntax
        - Required fields present
        - Field values in valid ranges
        - URL formats valid
        - Path references exist (optional)
   3.4. Exit codes: 0 = valid, 1 = invalid
   3.5. Examples:
        - Validate default config
        - Validate custom path

4. keygen Command
   4.1. Purpose: Generate new API key with hash
   4.2. Usage: mcp-guard keygen --user-id <ID> [OPTIONS]
   4.3. Options:
        - --user-id <string> (required, unique identifier)
        - --rate-limit <u32> (optional, custom RPS)
        - --tools <list> (optional, comma-separated tool names)
   4.4. Output:
        - API key (give to client, store securely)
        - Key hash (put in config file)
        - Config snippet (copy-paste ready)
   4.5. Key format: mcp_<32-bytes-base64url>
   4.6. Security notes:
        - Key shown once, cannot be recovered
        - Only hash stored in config
   4.7. Examples:
        - Basic key generation
        - Key with rate limit
        - Key with tool restrictions

5. hash-key Command
   5.1. Purpose: Hash an existing API key
   5.2. Usage: mcp-guard hash-key <KEY>
   5.3. Use case: Migrating existing keys
   5.4. Output: Base64-encoded SHA-256 hash
   5.5. Examples:
        - Hash a key for config

6. run Command
   6.1. Purpose: Start the MCP Guard server
   6.2. Usage: mcp-guard run [OPTIONS]
   6.3. Options:
        - --host <address> (override config)
        - --port <port> (override config)
   6.4. Startup sequence:
        - Load and validate config
        - Initialize auth providers
        - Initialize rate limiter
        - Initialize transports
        - Start audit logger
        - Initialize metrics/tracing
        - Bind to address
        - Ready for requests
   6.5. Graceful shutdown: Ctrl+C or SIGTERM
   6.6. Examples:
        - Start with defaults
        - Start on custom port
        - Start with verbose logging

7. version Command
   7.1. Purpose: Show build information
   7.2. Usage: mcp-guard version
   7.3. Output:
        - Version number
        - Git commit hash
        - Build timestamp
        - Rust version
        - Enabled features
   7.4. Use case: Debugging, support requests

8. check-upstream Command
   8.1. Purpose: Test upstream MCP server connectivity
   8.2. Usage: mcp-guard check-upstream [OPTIONS]
   8.3. Options:
        - --timeout <seconds> (default: 10)
   8.4. Tests performed:
        - Stdio: Spawn process, send initialize
        - HTTP: POST to URL, check response
        - SSE: Connect, verify event stream
   8.5. Output: Success/failure with details
   8.6. Examples:
        - Check stdio server
        - Check HTTP server with custom timeout
```

---

### 4. Authentication Guide

**File:** `docs/authentication.md`
**Goal:** Deep dive into each authentication provider with setup instructions

#### Outline

```
1. Authentication Overview
   1.1. How authentication works in MCP Guard
   1.2. The Identity model
        - id: unique identifier
        - name: display name (optional)
        - allowed_tools: per-identity authorization
        - rate_limit: per-identity rate limit
        - claims: additional metadata
   1.3. Multi-provider support and fallback
   1.4. Choosing the right provider

2. API Key Authentication
   2.1. Overview and use cases
        - Service-to-service communication
        - CLI tools and scripts
        - Simple deployments
   2.2. Security model
        - Keys are SHA-256 hashed before storage
        - Constant-time comparison
        - No key recovery (hash only)
   2.3. Key format: mcp_<base64url>
   2.4. Configuration reference
   2.5. Step-by-step setup:
        a. Generate key with keygen
        b. Give key to client
        c. Add hash to config
        d. Restart server
   2.6. Client usage:
        - Authorization: Bearer mcp_xxx header
   2.7. Best practices:
        - Rotate keys periodically
        - Use separate keys per client
        - Set appropriate rate limits
   2.8. Troubleshooting:
        - "Invalid API key" errors
        - Key format issues

3. JWT Authentication
   3.1. Overview and use cases
        - Integration with existing IdPs
        - Stateless authentication
        - Rich claims support
   3.2. Supported modes:
        a. Simple (HS256): Shared secret
        b. JWKS (RS256/ES256): Public key verification
   3.3. Simple mode setup:
        - Generate or use existing secret
        - Configure secret in config
        - Example with jsonwebtoken library
   3.4. JWKS mode setup:
        - Point to IdP's JWKS endpoint
        - Configure allowed algorithms
        - Cache behavior and refresh
   3.5. Claims mapping:
        - user_id_claim: where to find user ID
        - scopes_claim: where to find scopes
        - Custom claims in Identity.claims
   3.6. Scope-to-tool mapping:
        - Map OAuth scopes to MCP tools
        - Wildcard support
        - Examples
   3.7. Provider-specific guides:
        a. Auth0 setup
           - Create API in Auth0
           - Configure JWKS URL
           - Set issuer and audience
        b. Keycloak setup
           - Create realm and client
           - Configure JWKS URL
           - Map roles to scopes
        c. Okta setup
           - Create authorization server
           - Configure JWKS URL
           - Custom claims
        d. AWS Cognito setup
           - User pool configuration
           - JWKS URL format
   3.8. Token requirements:
        - Required claims: iss, aud, exp, sub
        - Recommended claims: iat, nbf
   3.9. Troubleshooting:
        - "Token expired" errors
        - "Invalid signature" errors
        - JWKS fetch failures
        - Clock skew issues (leeway)

4. OAuth 2.1 Authentication
   4.1. Overview and use cases
        - User-facing applications
        - Third-party integrations
        - Social login
   4.2. OAuth flow in MCP Guard:
        a. Client redirects to /oauth/authorize
        b. MCP Guard redirects to provider
        c. User authenticates with provider
        d. Provider redirects to /oauth/callback
        e. MCP Guard exchanges code for token
        f. Token returned to client
   4.3. PKCE support (RFC 7636):
        - Why PKCE matters
        - S256 code challenge
        - Automatic handling
   4.4. Token validation:
        - Introspection endpoint (preferred)
        - UserInfo endpoint (fallback)
        - Token caching (5-min TTL)
   4.5. Built-in providers:
        a. GitHub setup
           - Create OAuth App
           - Configure client ID/secret
           - Scopes: read:user, repo, etc.
        b. Google setup
           - Create OAuth credentials
           - Configure consent screen
           - Scopes: openid, profile, email
        c. Okta setup
           - Create OIDC application
           - Configure redirect URIs
   4.6. Custom provider setup:
        - Required endpoints
        - Optional endpoints
        - Example configuration
   4.7. Scope-to-tool mapping:
        - Same as JWT
        - Provider-specific scopes
   4.8. Troubleshooting:
        - Redirect URI mismatch
        - Invalid client credentials
        - Token introspection failures

5. mTLS Authentication
   5.1. Overview and use cases
        - Zero-trust architectures
        - Service mesh integration
        - High-security environments
   5.2. Architecture:
        - Reverse proxy terminates TLS
        - Proxy extracts cert info to headers
        - MCP Guard reads headers
   5.3. Required headers:
        - X-Client-Cert-CN
        - X-Client-Cert-SAN-DNS
        - X-Client-Cert-SAN-Email
        - X-Client-Cert-Verified
   5.4. Identity source options:
        - cn: Common Name
        - san_dns: SAN DNS entry
        - san_email: SAN email entry
   5.5. Reverse proxy configuration:
        a. nginx setup
           - ssl_client_certificate
           - ssl_verify_client
           - proxy_set_header examples
        b. HAProxy setup
           - ca-file configuration
           - Header injection
        c. Envoy setup
           - Client certificate validation
           - Header configuration
   5.6. Configuration reference
   5.7. Troubleshooting:
        - Missing headers
        - Verification failures
        - Certificate chain issues

6. Combining Multiple Providers
   6.1. MultiProvider behavior
   6.2. Provider priority order
   6.3. Error handling and fallback
   6.4. Use cases:
        - API keys for services + OAuth for users
        - JWT primary + API key fallback
        - mTLS for internal + OAuth for external
   6.5. Configuration examples
```

---

### 5. Transport Guide

**File:** `docs/transports.md`
**Goal:** Explain each transport type with use cases and configuration

#### Outline

```
1. Transport Overview
   1.1. What is a transport?
   1.2. Role in MCP Guard architecture
   1.3. Choosing the right transport
   1.4. Transport trait interface

2. Stdio Transport
   2.1. Overview
        - Communicates via stdin/stdout
        - For local MCP server processes
        - Newline-delimited JSON
   2.2. Use cases:
        - npx-based MCP servers
        - Python MCP servers
        - Local development
   2.3. How it works:
        - Spawns child process
        - Writer task: channel -> stdin
        - Reader task: stdout -> channel
        - Process lifecycle management
   2.4. Configuration:
        - command: executable path
        - args: command arguments
        - env: environment variables (optional)
   2.5. Examples:
        a. Filesystem server (npx)
        b. Python server
        c. Custom binary
   2.6. Health monitoring:
        - is_healthy() check
        - Process exit detection
   2.7. Troubleshooting:
        - Process won't start
        - JSON parsing errors
        - Process crashes

3. HTTP Transport
   3.1. Overview
        - POST JSON-RPC to HTTP endpoint
        - Request-response pattern
        - For remote MCP servers
   3.2. Use cases:
        - Cloud-hosted MCP servers
        - Microservice architecture
        - Load-balanced deployments
   3.3. How it works:
        - HTTP POST with JSON body
        - Content-Type: application/json
        - Response body is JSON-RPC response
   3.4. Configuration:
        - url: upstream HTTP endpoint
   3.5. Connection pooling:
        - Keep-alive connections
        - Connection reuse
   3.6. Timeout handling:
        - 30-second default timeout
        - Error on timeout
   3.7. SSRF protection:
        - DNS resolution check
        - Private IP blocking
        - Cloud metadata blocking
   3.8. Examples:
        a. Remote MCP server
        b. Internal service
   3.9. Troubleshooting:
        - Connection refused
        - Timeout errors
        - SSRF validation failures

4. SSE Transport
   4.1. Overview
        - Server-Sent Events for streaming
        - Long-running operations
        - Progressive responses
   4.2. Use cases:
        - Streaming tool responses
        - Real-time updates
        - Large data transfers
   4.3. How it works:
        - POST request initiates
        - Response is SSE stream
        - Events parsed as JSON-RPC
   4.4. Configuration:
        - url: upstream SSE endpoint
   4.5. Event format:
        - data: JSON-RPC message
        - Supports multiple events
   4.6. Examples:
        a. Streaming LLM responses
        b. Progress updates
   4.7. Troubleshooting:
        - Stream disconnections
        - Event parsing errors

5. Multi-Server Routing
   5.1. Overview
        - Multiple upstreams behind one gateway
        - Path-based routing
        - Mixed transport types
   5.2. Use cases:
        - Multiple MCP servers per org
        - Different tools on different servers
        - Gradual migration
   5.3. Configuration:
        - [[upstream.servers]] array
        - name, path_prefix, transport
        - strip_prefix option
   5.4. Routing algorithm:
        - Longest prefix match
        - Case-sensitive matching
   5.5. /routes endpoint:
        - List available routes
        - Useful for discovery
   5.6. Examples:
        a. Two stdio servers
        b. Mix of stdio and HTTP
        c. Different tools per server
   5.7. Troubleshooting:
        - Route not found
        - Prefix conflicts

6. Transport Comparison Table
   6.1. Feature comparison matrix
   6.2. Performance characteristics
   6.3. When to use each
```

---

### 6. Multi-Server Routing Guide

**File:** `docs/multi-server.md`
**Goal:** Detailed guide for organizations running multiple MCP servers

#### Outline

```
1. Introduction
   1.1. What is multi-server routing?
   1.2. Benefits:
        - Single gateway for multiple servers
        - Unified authentication
        - Centralized rate limiting
        - Consolidated audit logs
   1.3. Architecture diagram

2. Configuration
   2.1. Basic structure
   2.2. Server entry fields:
        - name: unique identifier
        - path_prefix: URL path (must start with /)
        - transport: stdio/http/sse
        - command/args or url
        - strip_prefix: remove prefix before forwarding
   2.3. Validation rules

3. Routing Behavior
   3.1. Path matching algorithm
   3.2. Longest prefix wins
   3.3. Request path transformation
   3.4. strip_prefix explained

4. Endpoint: POST /mcp/:server_name
   4.1. Request format
   4.2. Server name extraction
   4.3. Response format
   4.4. Error responses (404, 500)

5. Endpoint: GET /routes
   5.1. Response format
   5.2. Use for service discovery
   5.3. Example response

6. Examples
   6.1. Two local servers (stdio)
        - Filesystem server on /fs
        - GitHub server on /github
   6.2. Mixed transports
        - Local stdio + remote HTTP
   6.3. Microservices architecture
        - Multiple HTTP upstreams
   6.4. Tool segregation
        - Different tools per server
        - Authorization by server

7. Authorization with Multi-Server
   7.1. Per-server tool authorization
   7.2. Identity.allowed_tools applies globally
   7.3. Scope mapping per server

8. Monitoring Multi-Server
   8.1. Metrics per server
   8.2. Audit logs with server context
   8.3. Health checks per upstream

9. Troubleshooting
   9.1. "Server not found" (404)
   9.2. Route conflicts
   9.3. Per-server connectivity issues
```

---

### 7. Rate Limiting Guide

**File:** `docs/rate-limiting.md`
**Goal:** Explain rate limiting behavior and configuration

#### Outline

```
1. Overview
   1.1. Purpose of rate limiting
   1.2. Token bucket algorithm
   1.3. Per-identity rate limiting

2. Configuration
   2.1. Global settings:
        - requests_per_second
        - burst_size
   2.2. Per-identity overrides:
        - In API key config
        - From OAuth/JWT claims

3. How It Works
   3.1. Token bucket explained
   3.2. Burst handling
   3.3. Identity tracking
   3.4. TTL and memory management

4. Rate Limit Headers
   4.1. Response headers:
        - X-RateLimit-Limit
        - X-RateLimit-Remaining
        - X-RateLimit-Reset
   4.2. 429 response:
        - Retry-After header
        - Response body

5. Per-Identity Limits
   5.1. How custom limits work
   5.2. Configuration examples
   5.3. Use cases:
        - Premium vs free tiers
        - Service accounts
        - Burst allowances

6. Monitoring
   6.1. Prometheus metrics:
        - mcp_guard_rate_limit_total
        - mcp_guard_active_identities
   6.2. Audit events

7. Tuning Guide
   7.1. Determining appropriate limits
   7.2. Burst size considerations
   7.3. Memory usage

8. Troubleshooting
   8.1. Unexpected 429 responses
   8.2. Limits not applying
   8.3. Memory growth
```

---

### 8. Observability Guide

**File:** `docs/observability.md`
**Goal:** Complete guide to monitoring, metrics, tracing, and audit logging

#### Outline

```
1. Overview
   1.1. Three pillars: metrics, traces, logs
   1.2. MCP Guard observability features
   1.3. Integration options

2. Prometheus Metrics
   2.1. Endpoint: GET /metrics
   2.2. Available metrics:
        a. mcp_guard_requests_total
           - Labels: method, status
           - Use: Request volume, error rates
        b. mcp_guard_request_duration_seconds
           - Labels: method
           - Buckets: histogram
           - Use: Latency percentiles
        c. mcp_guard_auth_total
           - Labels: provider, result
           - Use: Auth success/failure rates
        d. mcp_guard_rate_limit_total
           - Labels: allowed
           - Use: Rate limit hit rate
        e. mcp_guard_active_identities
           - Gauge
           - Use: Unique clients
   2.3. Prometheus scrape config
   2.4. Example queries:
        - Request rate
        - Error rate
        - P99 latency
        - Auth failure rate
   2.5. Grafana dashboard example

3. OpenTelemetry Tracing
   3.1. Configuration:
        - service_name
        - otlp_endpoint
        - sample_rate
        - propagate_context
   3.2. Span attributes
   3.3. W3C trace context:
        - traceparent header
        - tracestate header
   3.4. Backend setup:
        a. Jaeger
           - Docker compose example
           - Configuration
        b. Grafana Tempo
           - Setup guide
           - Integration with Grafana
        c. Honeycomb
           - API key setup
           - Team configuration
   3.5. Correlation IDs in logs
   3.6. Sampling strategies:
        - Always on (development)
        - Ratio (production)
        - Always off (disabled)

4. Audit Logging
   4.1. Event types:
        - AuthSuccess
        - AuthFailure
        - ToolCall
        - ToolCallResult
        - RateLimited
        - AuthzDenied
   4.2. Event schema:
        - timestamp
        - event_type
        - identity_id
        - method
        - tool
        - success
        - message
        - duration_ms
        - request_id
   4.3. Output destinations:
        a. Stdout (console)
        b. File (append mode)
        c. HTTP export (SIEM)
   4.4. HTTP export configuration:
        - export_url
        - export_batch_size
        - export_interval_secs
        - export_headers (auth)
   4.5. SIEM integration:
        a. Splunk HEC
           - Endpoint format
           - Header configuration
           - Index setup
        b. Datadog
           - API endpoint
           - DD-API-KEY header
        c. Elasticsearch
           - Bulk API
           - Index templates
   4.6. Retry behavior:
        - 3 attempts
        - Exponential backoff

5. Health Endpoints
   5.1. GET /health
        - Response: version, uptime_secs
        - Use: Detailed health info
   5.2. GET /live
        - Response: {"status": "ok"}
        - Use: Kubernetes liveness probe
   5.3. GET /ready
        - Response: 200 or 503
        - Use: Kubernetes readiness probe

6. Alerting Recommendations
   6.1. Critical alerts:
        - High error rate
        - Auth failure spike
        - Upstream unavailable
   6.2. Warning alerts:
        - High latency
        - Rate limit saturation
        - Memory growth

7. Troubleshooting with Observability
   7.1. Using traces to debug requests
   7.2. Correlating logs with traces
   7.3. Finding slow requests
```

---

### 9. Deployment Guide

**File:** `docs/deployment.md`
**Goal:** Production deployment patterns and best practices

#### Outline

```
1. Overview
   1.1. Deployment options
   1.2. Choosing the right approach
   1.3. Production checklist

2. Binary Deployment
   2.1. System requirements
   2.2. Installation steps
   2.3. Systemd service file
   2.4. Log rotation
   2.5. Upgrade process

3. Docker Deployment
   3.1. Official Docker image
   3.2. Dockerfile example
   3.3. Docker Compose example
   3.4. Volume mounts:
        - Config file
        - TLS certificates
        - Audit logs
   3.5. Environment variables
   3.6. Health checks

4. Kubernetes Deployment
   4.1. Deployment manifest
   4.2. Service manifest
   4.3. ConfigMap for configuration
   4.4. Secret for sensitive data:
        - API key hashes
        - JWT secrets
        - OAuth credentials
   4.5. Health probes:
        - livenessProbe: /live
        - readinessProbe: /ready
   4.6. Resource limits
   4.7. Horizontal Pod Autoscaler
   4.8. Ingress configuration

5. TLS Configuration
   5.1. Direct TLS termination
   5.2. Behind reverse proxy
   5.3. Let's Encrypt automation
   5.4. Certificate rotation

6. mTLS Setup
   6.1. Architecture overview
   6.2. Certificate authority setup
   6.3. Client certificate generation
   6.4. Reverse proxy configuration:
        a. nginx
        b. HAProxy
        c. Envoy
   6.5. Testing mTLS

7. High Availability
   7.1. Stateless design
   7.2. Multiple instances
   7.3. Load balancer configuration
   7.4. Session affinity (not required)

8. Security Hardening
   8.1. Network policies
   8.2. Run as non-root
   8.3. Read-only filesystem
   8.4. Secrets management
   8.5. Audit log security

9. Monitoring in Production
   9.1. Prometheus integration
   9.2. Log aggregation
   9.3. Alerting setup
   9.4. Dashboards

10. Backup and Recovery
    10.1. Configuration backup
    10.2. Audit log archival
    10.3. Disaster recovery
```

---

### 10. Troubleshooting Guide

**File:** `docs/troubleshooting.md`
**Goal:** Common issues and their solutions

#### Outline

```
1. Diagnostic Tools
   1.1. mcp-guard validate
   1.2. mcp-guard check-upstream
   1.3. mcp-guard version
   1.4. Verbose logging (--verbose)
   1.5. Health endpoints

2. Startup Issues
   2.1. "Config file not found"
   2.2. "Invalid configuration"
   2.3. "Address already in use"
   2.4. "Permission denied"
   2.5. "TLS certificate error"

3. Authentication Errors
   3.1. HTTP 401 Unauthorized
        - Missing Authorization header
        - Invalid API key format
        - Expired JWT
        - Invalid JWT signature
   3.2. Debug authentication:
        - Check provider logs
        - Verify key/secret
        - Check issuer/audience
   3.3. JWT-specific:
        - Clock skew (adjust leeway)
        - JWKS fetch failures
        - Algorithm mismatch
   3.4. OAuth-specific:
        - Redirect URI mismatch
        - Invalid client credentials
        - Token introspection failure

4. Authorization Errors
   4.1. HTTP 403 Forbidden
   4.2. "Tool not authorized"
   4.3. Scope mapping issues
   4.4. Debug allowed_tools

5. Rate Limiting Issues
   5.1. HTTP 429 Too Many Requests
   5.2. Check Retry-After header
   5.3. Per-identity limits not applying
   5.4. Burst configuration

6. Transport Issues
   6.1. Stdio:
        - "Process failed to start"
        - "Process exited unexpectedly"
        - JSON parsing errors
   6.2. HTTP:
        - "Connection refused"
        - "Request timeout"
        - SSRF validation errors
   6.3. SSE:
        - Stream disconnections
        - Event parsing errors

7. Routing Issues (Multi-Server)
   7.1. HTTP 404 "Server not found"
   7.2. Wrong server receiving requests
   7.3. Path prefix conflicts

8. Performance Issues
   8.1. High latency
   8.2. Memory growth
   8.3. Connection exhaustion

9. Observability Issues
   9.1. Metrics not appearing
   9.2. Traces not exporting
   9.3. Audit logs missing

10. Getting Help
    10.1. Debug logging
    10.2. Collecting diagnostics
    10.3. GitHub issues
    10.4. Community support
```

---

## Internal Developer Documentation

### 1. Architecture Overview

**File:** `docs/dev/architecture.md`
**Goal:** Help developers understand the codebase structure

#### Outline

```
1. High-Level Architecture
   1.1. System context diagram
   1.2. Component diagram
   1.3. Request flow diagram

2. Module Structure
   2.1. src/main.rs: Entry point
        - CLI parsing
        - Provider initialization
        - Server startup
   2.2. src/lib.rs: Library root
        - Module declarations
        - Error type
        - Re-exports
   2.3. src/cli/: CLI commands
   2.4. src/config/: Configuration
   2.5. src/auth/: Authentication
   2.6. src/authz/: Authorization
   2.7. src/rate_limit/: Rate limiting
   2.8. src/transport/: Transports
   2.9. src/router/: Multi-server routing
   2.10. src/server/: HTTP server
   2.11. src/audit/: Audit logging
   2.12. src/observability/: Metrics/tracing

3. Core Abstractions
   3.1. AuthProvider trait
   3.2. Transport trait
   3.3. AppState struct
   3.4. Identity struct
   3.5. Message struct

4. Request Lifecycle
   4.1. Connection accepted
   4.2. Middleware chain
   4.3. Authentication
   4.4. Rate limiting
   4.5. Authorization
   4.6. Transport forwarding
   4.7. Response filtering
   4.8. Audit logging

5. Concurrency Model
   5.1. Tokio runtime
   5.2. Async handlers
   5.3. Background tasks
   5.4. Channel-based communication
   5.5. Shared state (Arc<RwLock>)

6. Error Handling
   6.1. Error enum
   6.2. Error propagation
   6.3. HTTP error responses
   6.4. Error correlation IDs

7. Design Decisions
   7.1. Why Axum?
   7.2. Why Governor for rate limiting?
   7.3. Why channel-based audit logging?
   7.4. Trade-offs made
```

---

### 2. AuthProvider Trait Guide

**File:** `docs/dev/auth-provider.md`
**Goal:** Guide for implementing new authentication providers

#### Outline

```
1. AuthProvider Trait
   1.1. Trait definition
   1.2. Required methods:
        - authenticate(&self, token: &str) -> Result<Identity>
        - name(&self) -> &str
   1.3. Return types

2. Identity Struct
   2.1. Fields:
        - id: String
        - name: Option<String>
        - allowed_tools: Option<Vec<String>>
        - rate_limit: Option<u32>
        - claims: HashMap<String, Value>
   2.2. Usage in authorization
   2.3. Usage in rate limiting

3. Implementing a New Provider
   3.1. Create module file
   3.2. Implement struct
   3.3. Implement AuthProvider trait
   3.4. Add to MultiProvider
   3.5. Add configuration support
   3.6. Write tests

4. Example: LDAP Provider
   4.1. Dependencies
   4.2. Configuration struct
   4.3. Provider implementation
   4.4. Integration

5. MultiProvider
   5.1. How it works
   5.2. Provider ordering
   5.3. Error aggregation

6. Testing Providers
   6.1. Unit tests
   6.2. Integration tests
   6.3. Mock providers

7. Best Practices
   7.1. Error handling
   7.2. Caching considerations
   7.3. Async considerations
   7.4. Security considerations
```

---

### 3. Transport Trait Guide

**File:** `docs/dev/transport.md`
**Goal:** Guide for implementing new transport types

#### Outline

```
1. Transport Trait
   1.1. Trait definition
   1.2. Required methods:
        - send(&self, msg: Message) -> Result<()>
        - receive(&self) -> Result<Message>
        - close(&self) -> Result<()>
   1.3. Message struct

2. Message Format
   2.1. JSON-RPC 2.0 structure
   2.2. Request vs Response vs Notification
   2.3. Serialization

3. Implementing a New Transport
   3.1. Create module file
   3.2. Implement struct
   3.3. Implement Transport trait
   3.4. Add configuration support
   3.5. Write tests

4. Example: WebSocket Transport
   4.1. Dependencies
   4.2. Configuration struct
   4.3. Transport implementation
   4.4. Connection management
   4.5. Integration

5. Task Supervision
   5.1. Background tasks pattern
   5.2. Health checking
   5.3. Graceful shutdown
   5.4. Error recovery

6. Security Considerations
   6.1. SSRF protection
   6.2. Input validation
   6.3. Connection limits

7. Testing Transports
   7.1. Unit tests
   7.2. Integration tests
   7.3. Mock servers
```

---

### 4. Middleware Chain

**File:** `docs/dev/middleware.md`
**Goal:** Explain the middleware architecture

#### Outline

```
1. Middleware Overview
   1.1. Axum middleware model
   1.2. Middleware ordering
   1.3. Request extensions

2. Middleware Chain
   2.1. security_headers_middleware
        - Purpose
        - Headers added
        - Implementation
   2.2. trace_context_middleware
        - W3C trace context
        - Span creation
        - Implementation
   2.3. metrics_middleware
        - Timer pattern
        - Labels
        - Implementation
   2.4. auth_middleware
        - Token extraction
        - Provider invocation
        - Identity insertion
        - Implementation

3. Adding New Middleware
   3.1. Where to add
   3.2. Implementation pattern
   3.3. Testing

4. Request Extensions
   4.1. Identity extension
   4.2. Accessing in handlers
   4.3. Custom extensions

5. Error Handling in Middleware
   5.1. Early returns
   5.2. Error responses
   5.3. Logging
```

---

### 5. Rate Limiting Internals

**File:** `docs/dev/rate-limiting-internals.md`
**Goal:** Deep dive into rate limiting implementation

#### Outline

```
1. Overview
   1.1. Governor crate
   1.2. Token bucket algorithm
   1.3. Per-identity design

2. RateLimitService
   2.1. Struct definition
   2.2. DashMap for per-identity limiters
   2.3. Configuration

3. Limiter Creation
   3.1. Lazy creation pattern
   3.2. Custom vs default limits
   3.3. Quota configuration

4. Check Flow
   4.1. Identity extraction
   4.2. Limiter lookup/create
   4.3. Check and update
   4.4. Result construction

5. TTL and Eviction
   5.1. Why TTL is needed
   5.2. Implementation (1-hour TTL)
   5.3. Cleanup strategy
   5.4. Memory management

6. RateLimitResult
   6.1. Fields
   6.2. HTTP header mapping
   6.3. 429 response generation

7. Testing
   7.1. Unit tests
   7.2. Concurrent access tests
   7.3. TTL tests
```

---

### 6. Testing Guide

**File:** `docs/dev/testing.md`
**Goal:** How to run and write tests

#### Outline

```
1. Test Structure
   1.1. Unit tests (in-module)
   1.2. Integration tests (tests/)
   1.3. E2E tests
   1.4. Benchmarks (benches/)

2. Running Tests
   2.1. cargo test (all tests)
   2.2. cargo test <name> (specific)
   2.3. cargo test --lib (unit only)
   2.4. cargo test --test integration_tests

3. Test Fixtures
   3.1. Test configuration files
   3.2. Mock servers
   3.3. Test identities

4. Writing Unit Tests
   4.1. Module structure
   4.2. #[cfg(test)] mod tests
   4.3. Async tests (#[tokio::test])
   4.4. Mocking dependencies

5. Writing Integration Tests
   5.1. Test server setup
   5.2. HTTP client usage
   5.3. Assertions
   5.4. Cleanup

6. Test Coverage
   6.1. Running with tarpaulin
   6.2. Coverage targets
   6.3. CI integration

7. Benchmarks
   7.1. Criterion setup
   7.2. Running benchmarks
   7.3. Interpreting results
   7.4. CI benchmarks
```

---

### 7. Contributing Guide

**File:** `docs/dev/contributing.md`
**Goal:** How to contribute to MCP Guard

#### Outline

```
1. Getting Started
   1.1. Fork and clone
   1.2. Development setup
   1.3. Building locally
   1.4. Running tests

2. Development Workflow
   2.1. Branch naming
   2.2. Commit messages
   2.3. Pull request process
   2.4. Code review

3. Code Style
   3.1. rustfmt configuration
   3.2. Clippy lints
   3.3. Documentation comments
   3.4. Error handling patterns

4. Adding Features
   4.1. RFC process (major features)
   4.2. Implementation checklist:
        - Code
        - Tests
        - Documentation
        - Config template update
   4.3. Examples:
        a. Adding an auth provider
        b. Adding a transport
        c. Adding a CLI command

5. Testing Requirements
   5.1. Unit test coverage
   5.2. Integration tests
   5.3. No regressions

6. Documentation
   6.1. Code documentation
   6.2. User documentation
   6.3. Changelog entries

7. Release Process
   7.1. Versioning (semver)
   7.2. Changelog
   7.3. Release checklist
```

---

## API Reference

### HTTP API Reference

**File:** `docs/api/http.md`
**Goal:** Complete HTTP endpoint documentation

#### Outline

```
1. Overview
   1.1. Base URL
   1.2. Authentication
   1.3. Content types
   1.4. Error format

2. Health Endpoints

   2.1. GET /health
        Request: None
        Response: 200 OK
        {
          "status": "healthy",
          "version": "1.0.0",
          "uptime_secs": 3600
        }

   2.2. GET /live
        Request: None
        Response: 200 OK
        {"status": "ok"}

   2.3. GET /ready
        Request: None
        Response: 200 OK or 503 Service Unavailable
        {"status": "ready"} or {"status": "not_ready"}

3. Metrics Endpoint

   3.1. GET /metrics
        Request: None
        Response: 200 OK (text/plain)
        # Prometheus format metrics

4. OAuth Endpoints

   4.1. GET /oauth/authorize
        Query Parameters:
        - redirect_uri (optional)
        Response: 302 Redirect to OAuth provider

   4.2. GET /oauth/callback
        Query Parameters:
        - code (authorization code)
        - state (PKCE state)
        Response: 200 OK with access token or error

5. Routes Endpoint (Multi-Server)

   5.1. GET /routes
        Request: None
        Response: 200 OK
        {
          "routes": [
            {"name": "github", "path_prefix": "/github"},
            {"name": "filesystem", "path_prefix": "/fs"}
          ]
        }

6. MCP Endpoints

   6.1. POST /mcp (Single-Server)
        Headers:
        - Authorization: Bearer <token>
        - Content-Type: application/json
        Request Body: JSON-RPC 2.0
        Response: JSON-RPC 2.0

   6.2. POST /mcp/:server_name (Multi-Server)
        Path Parameters:
        - server_name: Target server
        Headers: Same as /mcp
        Request/Response: Same as /mcp

7. Error Responses
   7.1. 400 Bad Request
   7.2. 401 Unauthorized
   7.3. 403 Forbidden
   7.4. 404 Not Found
   7.5. 429 Too Many Requests
   7.6. 500 Internal Server Error

8. Rate Limit Headers
   8.1. X-RateLimit-Limit
   8.2. X-RateLimit-Remaining
   8.3. X-RateLimit-Reset
   8.4. Retry-After (429 only)
```

---

## Configuration Deep-Dives

### Auth0 Integration

**File:** `docs/integrations/auth0.md`

#### Outline

```
1. Prerequisites
   1.1. Auth0 account
   1.2. API created in Auth0

2. Auth0 Setup
   2.1. Create API
   2.2. Configure permissions (scopes)
   2.3. Get JWKS URL
   2.4. Note issuer and audience

3. MCP Guard Configuration
   3.1. JWT config with JWKS mode
   3.2. Scope-to-tool mapping
   3.3. Complete example

4. Testing
   4.1. Get test token from Auth0
   4.2. Call MCP Guard
   4.3. Verify authorization

5. Troubleshooting
```

### GitHub OAuth Integration

**File:** `docs/integrations/github-oauth.md`

#### Outline

```
1. Prerequisites
2. GitHub OAuth App Setup
3. MCP Guard Configuration
4. Testing the Flow
5. Scope Recommendations
6. Troubleshooting
```

### Splunk Integration

**File:** `docs/integrations/splunk.md`

#### Outline

```
1. Prerequisites
2. Splunk HEC Setup
3. MCP Guard Audit Configuration
4. Index Configuration
5. Search Examples
6. Dashboard Setup
7. Troubleshooting
```

### Jaeger Integration

**File:** `docs/integrations/jaeger.md`

#### Outline

```
1. Prerequisites
2. Jaeger Setup (Docker)
3. MCP Guard Tracing Configuration
4. Viewing Traces
5. Span Analysis
6. Troubleshooting
```

---

## Documentation Standards

### Style Guide

**File:** `docs/STYLE_GUIDE.md`

#### Outline

```
1. Writing Style
   1.1. Use active voice
   1.2. Be concise
   1.3. Use present tense
   1.4. Address reader as "you"

2. Formatting
   2.1. Headers: Title Case
   2.2. Code blocks: Always specify language
   2.3. Lists: Use for 3+ items
   2.4. Tables: For comparisons

3. Code Examples
   3.1. Must be complete and runnable
   3.2. Include comments for complex parts
   3.3. Show expected output
   3.4. Use realistic values

4. Configuration Examples
   4.1. TOML as primary format
   4.2. Include comments
   4.3. Show minimal and complete versions

5. Terminology
   5.1. Consistent term usage
   5.2. Glossary reference
   5.3. Avoid jargon

6. Diagrams
   6.1. Use Mermaid for diagrams
   6.2. Keep diagrams simple
   6.3. Include alt text
```

---

## Documentation Inventory

| Priority | Document | Target Audience | Est. Pages |
|----------|----------|-----------------|------------|
| P0 | Quick Start Guide | Users | 3-4 |
| P0 | Configuration Reference | Users | 10-15 |
| P0 | CLI Reference | Users | 4-5 |
| P0 | Authentication Guide | Users | 12-15 |
| P1 | Transport Guide | Users | 6-8 |
| P1 | Multi-Server Routing | Users | 4-5 |
| P1 | Rate Limiting Guide | Users | 3-4 |
| P1 | Observability Guide | Users | 8-10 |
| P1 | Deployment Guide | Users/DevOps | 10-12 |
| P1 | Troubleshooting | Users | 6-8 |
| P2 | Architecture Overview | Developers | 8-10 |
| P2 | AuthProvider Trait | Developers | 4-5 |
| P2 | Transport Trait | Developers | 4-5 |
| P2 | Middleware Chain | Developers | 3-4 |
| P2 | Rate Limiting Internals | Developers | 3-4 |
| P2 | Testing Guide | Developers | 4-5 |
| P2 | Contributing Guide | Developers | 4-5 |
| P2 | HTTP API Reference | Users/Devs | 6-8 |
| P3 | Auth0 Integration | Users | 2-3 |
| P3 | GitHub OAuth Integration | Users | 2-3 |
| P3 | Splunk Integration | Users | 2-3 |
| P3 | Jaeger Integration | Users | 2-3 |
| P3 | Style Guide | Writers | 2-3 |

**Total estimated pages: 110-140**

---

## Progress Tracking

### Phase 1 (P0) - COMPLETE ✅

Completed: 2024-12-18

| Document | File | Lines | Status |
|----------|------|-------|--------|
| Quick Start Guide | `docs/quickstart.md` | 304 | ✅ Complete |
| CLI Reference | `docs/cli.md` | 527 | ✅ Complete |
| Configuration Reference | `docs/configuration.md` | 791 | ✅ Complete |
| Authentication Guide | `docs/authentication.md` | 936 | ✅ Complete |

**Total: 2,558 lines, ~59KB**

**Decisions Made:**
- IdP Priority: Auth0 + Keycloak for step-by-step examples
- Quick Start Auth: API Keys + JWT simple mode
- Config Format: TOML only (no YAML examples)

### Phase 2 (P1) - COMPLETE ✅

Completed: 2024-12-18

| Document | File | Lines | Status |
|----------|------|-------|--------|
| Transport Guide | `docs/transports.md` | 462 | ✅ Complete |
| Multi-Server Routing | `docs/multi-server.md` | 490 | ✅ Complete |
| Rate Limiting Guide | `docs/rate-limiting.md` | 424 | ✅ Complete |
| Observability Guide | `docs/observability.md` | 736 | ✅ Complete |
| Deployment Guide | `docs/deployment.md` | 876 | ✅ Complete |
| Troubleshooting Guide | `docs/troubleshooting.md` | 732 | ✅ Complete |

**Total: 3,720 lines, ~85KB**

### Phase 3 (P2) - COMPLETE ✅

Completed: 2024-12-18

| Document | File | Lines | Status |
|----------|------|-------|--------|
| Architecture Overview | `docs/dev/architecture.md` | 295 | ✅ Complete |
| AuthProvider Trait | `docs/dev/auth-provider.md` | 390 | ✅ Complete |
| Transport Trait | `docs/dev/transport.md` | 489 | ✅ Complete |
| Middleware Chain | `docs/dev/middleware.md` | 527 | ✅ Complete |
| Rate Limiting Internals | `docs/dev/rate-limiting-internals.md` | 418 | ✅ Complete |
| Testing Guide | `docs/dev/testing.md` | 545 | ✅ Complete |
| Contributing Guide | `docs/dev/contributing.md` | 327 | ✅ Complete |
| HTTP API Reference | `docs/api/http.md` | 573 | ✅ Complete |

**Total: 3,564 lines, ~88KB**

### Phase 4 (P3) - COMPLETE ✅

Completed: 2024-12-18

| Document | File | Lines | Status |
|----------|------|-------|--------|
| Auth0 Integration | `docs/integrations/auth0.md` | 228 | ✅ Complete |
| GitHub OAuth Integration | `docs/integrations/github-oauth.md` | 258 | ✅ Complete |
| Splunk Integration | `docs/integrations/splunk.md` | 302 | ✅ Complete |
| Jaeger Integration | `docs/integrations/jaeger.md` | 350 | ✅ Complete |
| Style Guide | `docs/STYLE_GUIDE.md` | 320 | ✅ Complete |

**Total: 1,458 lines, ~35KB**

---

## Next Steps

1. ~~**Phase 1 (P0):** Create Quick Start, Configuration Reference, CLI Reference, Authentication Guide~~ ✅ DONE
2. ~~**Phase 2 (P1):** Create Transport Guide, Multi-Server, Rate Limiting, Observability, Deployment, Troubleshooting~~ ✅ DONE
3. ~~**Phase 3 (P2):** Create developer documentation (Architecture, Traits, Testing, Contributing)~~ ✅ DONE
4. ~~**Phase 4 (P3):** Create integration guides and style guide~~ ✅ DONE

**ALL DOCUMENTATION PHASES COMPLETE**

Total documentation: ~11,300 lines across 23 documents

---

## Documentation Summary

All four phases of documentation are now complete:

| Phase | Focus | Documents | Lines |
|-------|-------|-----------|-------|
| P0 | Core user docs | 4 | 2,558 |
| P1 | Operations & troubleshooting | 6 | 3,720 |
| P2 | Developer documentation | 8 | 3,564 |
| P3 | Integration guides | 5 | 1,458 |
| **Total** | | **23** | **~11,300** |

### Directory Structure

```
docs/
├── STYLE_GUIDE.md
├── quickstart.md
├── cli.md
├── configuration.md
├── authentication.md
├── transports.md
├── multi-server.md
├── rate-limiting.md
├── observability.md
├── deployment.md
├── troubleshooting.md
├── ARCHITECTURE.md
├── SECURITY.md
├── api/
│   └── http.md
├── dev/
│   ├── architecture.md
│   ├── auth-provider.md
│   ├── transport.md
│   ├── middleware.md
│   ├── rate-limiting-internals.md
│   ├── testing.md
│   └── contributing.md
└── integrations/
    ├── auth0.md
    ├── github-oauth.md
    ├── splunk.md
    └── jaeger.md
```

---

*This documentation plan is complete. Update as the project evolves.*
