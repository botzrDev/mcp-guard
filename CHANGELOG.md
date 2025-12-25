# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-26

### Added

#### Authentication
- API Key authentication with SHA256 hashing and constant-time comparison
- JWT authentication with dual-mode support:
  - Simple mode: HS256 with local secret
  - JWKS mode: RS256/ES256 with remote JWKS endpoint and automatic refresh
- OAuth 2.1 authentication with PKCE support for GitHub, Google, Okta, and custom providers
- mTLS client certificate authentication via reverse proxy headers

#### Rate Limiting
- Per-identity rate limiting with token bucket algorithm
- Configurable requests per second and burst size
- Custom rate limits per identity (via API key config or token claims)
- TTL-based eviction to prevent memory growth from abandoned connections
- Retry-After header in 429 responses (FR-RATE-05)

#### Authorization
- Per-tool authorization based on identity permissions
- Scope-to-tool mapping for JWT and OAuth tokens
- Wildcard support (`*`) for granting access to all tools
- Tools/list response filtering to hide unauthorized tools (FR-AUTHZ-03)

#### Observability
- Prometheus metrics endpoint (`/metrics`) with:
  - `mcp_guard_requests_total` (counter) - method, status labels
  - `mcp_guard_request_duration_seconds` (histogram) - method label
  - `mcp_guard_auth_total` (counter) - provider, result labels
  - `mcp_guard_rate_limit_total` (counter) - allowed label
  - `mcp_guard_active_identities` (gauge)
- OpenTelemetry distributed tracing with:
  - W3C trace context propagation (traceparent/tracestate headers)
  - OTLP export to Jaeger, Tempo, or other collectors
  - Configurable sampling rates (0.0-1.0)
- Structured logging with trace ID correlation (FR-AUDIT-06)

#### Audit Logging
- Channel-based async audit logging for non-blocking I/O
- Multiple output destinations: file, stdout
- HTTP export for SIEM integration with:
  - Configurable batch size and flush interval
  - Custom headers for authentication
  - Exponential backoff retry (3 attempts)

#### Transport
- Stdio transport for local MCP server processes
- HTTP transport for remote MCP servers via POST requests
- SSE (Server-Sent Events) transport for streaming responses

#### Multi-Server Routing
- Path-based routing to multiple upstream MCP servers
- Longest-prefix matching for route selection
- Support for mixed transport types across servers
- `/routes` endpoint to list available server routes

#### Health Checks
- `/health` endpoint with version and uptime information
- `/live` endpoint for Kubernetes liveness probes
- `/ready` endpoint for Kubernetes readiness probes (503 when not ready)

#### CLI
- `mcp-guard init` - Generate configuration file from template
- `mcp-guard validate` - Validate configuration file
- `mcp-guard keygen --user-id <ID>` - Generate new API key
- `mcp-guard hash-key` - Hash an existing API key
- `mcp-guard run` - Start the server
- `mcp-guard version` - Display version and build information
- `mcp-guard check-upstream` - Test upstream server connectivity

#### Security
- Security headers middleware:
  - X-Content-Type-Options: nosniff
  - X-Frame-Options: DENY
  - X-XSS-Protection: 1; mode=block
  - Content-Security-Policy: default-src 'none'
- Input validation for configuration:
  - Port range validation (1-65535)
  - HTTPS enforcement for JWKS URLs in production
  - Positive rate limit values
  - Valid URL formats for OAuth and audit export
- Graceful shutdown with signal handling (SIGINT/SIGTERM)
- Background task cancellation coordination

### Security
- AGPL-3.0 license for open-source distribution
- API key secrets stored as hashes only
- OAuth token caching with LRU eviction (max 500 entries)
- No plaintext credentials in configuration

## [0.1.0] - 2024-12-14

Initial development release.
