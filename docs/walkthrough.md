# MCP Guard: A Beginner's Complete Walkthrough

> **Welcome!** This guide will teach you everything you need to know about MCP Guard, from core concepts to advanced configuration. No prior knowledge required.

---

## Table of Contents

1. [What is MCP Guard?](#what-is-mcp-guard)
2. [The Problem It Solves](#the-problem-it-solves)
3. [Core Concepts](#core-concepts)
4. [Architecture Overview](#architecture-overview)
5. [Installation](#installation)
6. [Quick Start (5 Minutes)](#quick-start-5-minutes)
7. [Understanding the Configuration File](#understanding-the-configuration-file)
8. [Authentication Deep Dive](#authentication-deep-dive)
9. [Authorization: Who Can Do What](#authorization-who-can-do-what)
10. [Rate Limiting: Preventing Abuse](#rate-limiting-preventing-abuse)
11. [Audit Logging: Track Everything](#audit-logging-track-everything)
12. [The Request Pipeline](#the-request-pipeline)
13. [Transport Types](#transport-types)
14. [Multi-Server Routing](#multi-server-routing)
15. [Observability & Metrics](#observability--metrics)
16. [Production Deployment](#production-deployment)
17. [Troubleshooting](#troubleshooting)
18. [Feature Comparison by Tier](#feature-comparison-by-tier)
19. [Glossary](#glossary)

---

## What is MCP Guard?

**MCP Guard** is a lightweight security gateway that protects Model Context Protocol (MCP) servers. Think of it as a **security guard for AI tools**.

### The Analogy 🏢

Imagine you have a building (your MCP server) where valuable things happen — AI tools that can read files, access databases, send emails, etc. Right now, **anyone can walk in and use everything**. That's dangerous!

**MCP Guard** is like hiring a security guard who:

| Security Task | MCP Guard Feature |
|---------------|-------------------|
| 🔑 Checks ID at the door | **Authentication** |
| 🚧 Decides who can go where | **Authorization** |
| ⏱️ Limits how often someone can enter | **Rate Limiting** |
| 📋 Keeps a log of everyone who comes and goes | **Audit Logging** |
| 📊 Reports statistics to management | **Observability** |

---

## The Problem It Solves

### What is MCP?

**MCP (Model Context Protocol)** is a standard that lets AI assistants (like Claude, ChatGPT, Cursor, etc.) use "tools" — things like:

- 📂 Reading and writing files
- 🗄️ Querying databases
- 🌐 Making API calls
- 📧 Sending emails
- 🔧 Running commands

### The Security Gap

**The Problem:** Most MCP servers are deployed with **zero authentication**. If your AI agent can access it, **so can anyone else on the internet!** 😱

```
Without MCP Guard:

    Hacker ─────────┐
                    │
    Your AI ────────┼──────▶ MCP Server ──▶ Your Files! 
                    │              ▲
    Random Bot ─────┘              │
                              No protection!
```

### The Solution

MCP Guard sits **between** clients and your MCP server, acting as a protective layer:

```
With MCP Guard:

    Hacker ─────────┐
                    │   ❌ Rejected (no API key)
    Your AI ────────┼──▶ MCP Guard ──▶ MCP Server ──▶ Your Files
                    │   ✅ Allowed   (Security)
    Random Bot ─────┘
                    ❌ Rejected (rate limited)
```

---

## Core Concepts

Before diving in, let's understand the key terms:

### 1️⃣ Authentication (Who are you?)

Verifying the **identity** of whoever is making a request. Like showing your ID at a club.

**Supported methods:**
- **API Keys** — Simple secret tokens (like a password)
- **JWT** — Signed tokens with expiration (from Auth0, Keycloak, etc.)
- **OAuth 2.1** — "Login with Google/GitHub" style authentication
- **mTLS** — Client certificates for high-security environments

### 2️⃣ Authorization (What can you do?)

After proving identity, controlling **which tools** that identity can use. Like having a keycard that only opens certain doors.

**Example:** User "alice" can use `read_file` but not `delete_file`.

### 3️⃣ Rate Limiting (Don't overdo it!)

Preventing any single user from overwhelming the system with too many requests.

**Example:** Maximum 100 requests per second per user.

### 4️⃣ Audit Logging (What happened?)

Recording every request for security analysis, compliance, and debugging.

**Example:** Log that user "bob" called `write_file` at 3:42 PM.

### 5️⃣ Identity

The internal representation of an authenticated user:

```rust
Identity {
    id: "alice",                              // Unique ID
    name: "Alice Smith",                      // Display name
    allowed_tools: ["read_file", "write_file"], // What they can do
    rate_limit: 100,                          // Custom limit
}
```

---

## Architecture Overview

### High-Level Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              MCP Guard                                       │
│  ┌─────────────────────────────────────────────────────────────────────────┐│
│  │                         Request Pipeline                                 ││
│  │                                                                          ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────────────────┐   ││
│  │  │ Security │→│   Auth   │→│   Rate   │→│      Transport          │   ││
│  │  │ Headers  │  │Middleware│  │  Limit   │  │   (send to upstream)   │   ││
│  │  └──────────┘  └──────────┘  └──────────┘  └────────────────────────┘   ││
│  │       ↓              ↓             ↓                   ↓                 ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────────────────┐   ││
│  │  │ Metrics  │  │  Audit   │  │ Identity │  │   Tools Filtering      │   ││
│  │  │ Record   │  │  Logger  │  │  Store   │  │   (authorization)      │   ││
│  │  └──────────┘  └──────────┘  └──────────┘  └────────────────────────┘   ││
│  └─────────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────────┘
```

### System Context

```
  ┌─────────────┐      ┌─────────────────┐      ┌─────────────────┐
  │   AI Agent  │ ───▶ │    MCP Guard    │ ───▶ │   MCP Server    │
  │   (Claude)  │ ◀─── │   (Security)    │ ◀─── │   (Tools)       │
  └─────────────┘      └─────────────────┘      └─────────────────┘
                              │
                              ▼
                       ┌─────────────┐
                       │  Audit Log  │
                       │  Metrics    │
                       │  Traces     │
                       └─────────────┘
```

---

## Installation

Choose one of three installation methods:

### Option 1: From crates.io (Recommended)

```bash
cargo install mcp-guard
```

> **Note:** Requires Rust to be installed. Get it from [rustup.rs](https://rustup.rs).

### Option 2: Download Prebuilt Binary

```bash
# Linux x86_64
curl -fsSL https://github.com/botzrdev/mcp-guard/releases/latest/download/mcp-guard-x86_64-linux.tar.gz | tar -xz

# Or use the install script
curl -fsSL https://raw.githubusercontent.com/botzrdev/mcp-guard/main/install.sh | bash
```

### Option 3: Build from Source

```bash
git clone https://github.com/botzrdev/mcp-guard
cd mcp-guard
cargo build --release
# Binary at: ./target/release/mcp-guard
```

### Verify Installation

```bash
mcp-guard version
```

You should see version information and enabled features.

---

## Quick Start (5 Minutes)

Let's get from zero to a protected MCP server in 5 minutes!

### Prerequisites

- MCP Guard installed (see above)
- Node.js & npm (for the demo MCP server)

> **Don't have Node.js?** Install from [nodejs.org](https://nodejs.org) or configure your own MCP server.

### Step 1: Generate Configuration

```bash
mcp-guard init
```

This creates `mcp-guard.toml` with example configurations.

### Step 2: Generate an API Key

```bash
# Auto-add to config (recommended)
mcp-guard keygen --user-id my-agent --apply-to-config
```

**Output:**

```
✓ API key for 'my-agent' added to mcp-guard.toml

API Key (save this, shown only once):
  mcp_AbCdEf123456...

Next steps:
  mcp-guard validate
  mcp-guard run
```

> ⚠️ **Important:** Save this API key! It's only shown once.

### Step 3: Validate Configuration

```bash
mcp-guard validate
```

Expected output: `Configuration is valid: mcp-guard.toml`

### Step 4: Verify Upstream Connectivity

```bash
mcp-guard check-upstream
```

**Expected output:**

```
Checking upstream connectivity...

Transport: stdio
Command:   npx
Args:      ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

Server: @modelcontextprotocol/server-filesystem v0.6.2
✓ Upstream is reachable and responding
```

### Step 5: Start the Gateway

```bash
mcp-guard run
```

**Output:**

```
╭──────────────────────────────────────────────╮
│  MCP Guard v1.0.0                            │
╰──────────────────────────────────────────────╯

✓ Server:     http://127.0.0.1:3000
✓ Auth:       API Keys (1)
✓ Transport:  stdio → npx
✓ Rate Limit: 100 req/s, burst 50
✓ Audit:      Enabled

Ready for requests!
```

### Step 6: Test It!

**Health check (no auth needed):**

```bash
curl http://localhost:3000/health
```

**Response:**

```json
{"status": "healthy", "version": "1.0.0", "uptime_secs": 5}
```

**Unauthenticated request (should fail):**

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Response: 401 Unauthorized** ✓ Security working!

**Authenticated request (should succeed):**

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer mcp_YOUR_API_KEY_HERE" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {"name": "read_file", "description": "Read a file..."},
      {"name": "write_file", "description": "Write a file..."}
    ]
  }
}
```

🎉 **Congratulations!** Your MCP server is now protected!

---

## Understanding the Configuration File

MCP Guard uses a TOML configuration file (`mcp-guard.toml`). Let's break down each section:

### Complete Example with Comments

```toml
#═══════════════════════════════════════════════════════════════════════════════
# SERVER CONFIGURATION
# Where MCP Guard listens for incoming requests
#═══════════════════════════════════════════════════════════════════════════════
[server]
host = "127.0.0.1"    # IP address to bind to (0.0.0.0 = all interfaces)
port = 3000           # Port number (1-65535)

# Optional: TLS configuration for HTTPS
# [server.tls]
# cert_path = "/etc/ssl/server.crt"
# key_path = "/etc/ssl/server.key"

#═══════════════════════════════════════════════════════════════════════════════
# UPSTREAM CONFIGURATION
# Your actual MCP server that MCP Guard protects
#═══════════════════════════════════════════════════════════════════════════════
[upstream]
transport = "stdio"   # Options: "stdio", "http", "sse"
command = "npx"       # Command to run (for stdio transport)
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# For HTTP transport:
# transport = "http"
# url = "http://localhost:8080/mcp"

#═══════════════════════════════════════════════════════════════════════════════
# AUTHENTICATION
# How users prove their identity
#═══════════════════════════════════════════════════════════════════════════════

# API Keys - Simple pre-shared secrets
[[auth.api_keys]]
id = "my-service"                              # Unique identifier
key_hash = "abc123..."                         # SHA-256 hash (from keygen)
allowed_tools = ["read_file", "list_directory"] # Optional: restrict tools
rate_limit = 100                               # Optional: custom rate limit

# Add more API keys as needed:
[[auth.api_keys]]
id = "admin-service"
key_hash = "def456..."
# No allowed_tools = all tools allowed

# Optional: JWT Authentication
# [auth.jwt]
# mode = "simple"  # or "jwks" for JWKS
# secret = "your-256-bit-secret-key-here-minimum-32-characters"
# issuer = "https://your-app.com"
# audience = "mcp-guard"

#═══════════════════════════════════════════════════════════════════════════════
# RATE LIMITING
# Prevent abuse by limiting request frequency
#═══════════════════════════════════════════════════════════════════════════════
[rate_limit]
enabled = true              # Master switch
requests_per_second = 100   # Default limit per identity
burst_size = 50             # Allow temporary bursts

#═══════════════════════════════════════════════════════════════════════════════
# AUDIT LOGGING
# Track all requests for security and debugging
#═══════════════════════════════════════════════════════════════════════════════
[audit]
enabled = true
stdout = true               # Log to console (disable in production)
# file = "/var/log/mcp-guard/audit.log"  # Log to file
# export_url = "https://siem.example.com/logs"  # Send to SIEM

#═══════════════════════════════════════════════════════════════════════════════
# DISTRIBUTED TRACING (Optional)
# OpenTelemetry integration for request tracing
#═══════════════════════════════════════════════════════════════════════════════
# [tracing]
# enabled = true
# service_name = "mcp-guard"
# otlp_endpoint = "http://localhost:4317"
# sample_rate = 0.1  # 10% of requests
```

### Section Reference

| Section | Purpose |
|---------|---------|
| `[server]` | Where MCP Guard listens |
| `[upstream]` | Your MCP server to protect |
| `[[auth.api_keys]]` | API key authentication |
| `[auth.jwt]` | JWT authentication |
| `[auth.oauth]` | OAuth 2.1 authentication |
| `[auth.mtls]` | Mutual TLS authentication |
| `[rate_limit]` | Request rate limiting |
| `[audit]` | Audit logging |
| `[tracing]` | OpenTelemetry tracing |

---

## Authentication Deep Dive

MCP Guard supports four authentication methods. Let's explore each one:

### API Key Authentication

**Best for:** Service-to-service communication, CLI tools, simple setups.

**How it works:**
1. You generate a secret key (like a password)
2. MCP Guard stores only the hash (SHA-256)
3. Clients send the key in the `Authorization` header
4. MCP Guard hashes what they sent and compares

**Security features:**
- Keys are never stored in plain text
- Constant-time comparison prevents timing attacks
- Each key has a unique identifier for auditing

**Generate a key:**

```bash
mcp-guard keygen --user-id alice
```

**Client usage:**

```bash
curl -H "Authorization: Bearer mcp_AbCdEf..." http://localhost:3000/mcp
```

---

### JWT Authentication

**Best for:** Integration with existing identity providers (Auth0, Keycloak, Okta, AWS Cognito).

**How it works:**
1. User authenticates with your identity provider
2. Provider issues a signed JWT token
3. Client sends the JWT to MCP Guard
4. MCP Guard validates the signature and claims

**Two modes:**

| Mode | Algorithm | Use Case |
|------|-----------|----------|
| **Simple** | HS256 (symmetric) | Development, internal services |
| **JWKS** | RS256/ES256 (asymmetric) | Production with IdP |

**Simple mode configuration:**

```toml
[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-here-minimum-32-characters"
issuer = "https://your-app.com"
audience = "mcp-guard"
```

**JWKS mode configuration (Auth0 example):**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://YOUR_DOMAIN.auth0.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://YOUR_DOMAIN.auth0.com/"
audience = "mcp-guard"
scopes_claim = "permissions"  # Auth0 uses 'permissions' for RBAC
```

**Scope-to-tool mapping:**

Map JWT scopes to MCP tools:

```toml
[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]  # Wildcard = all tools
```

---

### OAuth 2.1 Authentication

**Best for:** User authentication, "Login with GitHub/Google" flows.

**How it works:**
1. User clicks "Login with GitHub"
2. MCP Guard redirects to the OAuth provider
3. User authenticates with provider
4. Provider redirects back with an authorization code
5. MCP Guard exchanges code for access token
6. Token is validated and user identity extracted

**PKCE Security:**

MCP Guard uses Proof Key for Code Exchange (PKCE) to prevent authorization code interception attacks. This is automatic — no configuration needed!

**GitHub example:**

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["read:user", "repo"]
user_id_claim = "id"

[auth.oauth.scope_tool_mapping]
"read:user" = ["read_file", "list_directory"]
"repo" = ["read_file", "write_file"]
```

**Test the flow:**

```bash
# Open in browser
open "http://localhost:3000/oauth/authorize"
```

---

### mTLS Authentication

**Best for:** Zero-trust architectures, service mesh, high-security environments.

**How it works:**
1. Client has an X.509 certificate signed by a trusted CA
2. TLS handshake validates the certificate
3. Reverse proxy (nginx/HAProxy) extracts certificate info into headers
4. MCP Guard reads headers and creates identity

**Configuration:**

```toml
[auth.mtls]
enabled = true
identity_source = "cn"  # Common Name from certificate
trusted_proxy_ips = ["10.0.0.0/8", "172.16.0.0/12"]  # REQUIRED!
```

> ⚠️ **Critical:** You MUST configure `trusted_proxy_ips` to prevent header spoofing attacks!

---

### Multi-Provider Support

You can enable multiple authentication methods simultaneously:

```toml
# API keys for scripts
[[auth.api_keys]]
id = "ci-pipeline"
key_hash = "..."

# JWT for web users
[auth.jwt]
mode = "jwks"
jwks_url = "https://auth.example.com/.well-known/jwks.json"

# mTLS for internal services
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.0/8"]
```

**Priority order:**
1. mTLS (if configured and headers present)
2. Bearer token → tries API Key → JWT → OAuth

The first successful authentication wins.

---

## Authorization: Who Can Do What

After authentication (proving who someone is), authorization controls what they can do.

### How It Works

Every identity has an `allowed_tools` list:

| Value | Meaning |
|-------|---------|
| `None` (not set) | All tools allowed |
| `[]` (empty array) | No tools allowed |
| `["read_file", "write_file"]` | Only listed tools allowed |

### Configuration Examples

**Full access:**

```toml
[[auth.api_keys]]
id = "admin"
key_hash = "..."
# No allowed_tools = all tools allowed
```

**Read-only access:**

```toml
[[auth.api_keys]]
id = "read-only-client"
key_hash = "..."
allowed_tools = ["read_file", "list_directory"]
```

**No access (blocked):**

```toml
[[auth.api_keys]]
id = "blocked-client"
key_hash = "..."
allowed_tools = []  # Empty = nothing allowed
```

### Tools/List Filtering

When a client calls `tools/list`, MCP Guard automatically filters the response to only show tools the user is authorized to use.

**Example:**

Upstream returns:
```json
{"tools": ["read_file", "write_file", "delete_file"]}
```

User with `allowed_tools = ["read_file"]` sees:
```json
{"tools": ["read_file"]}
```

---

## Rate Limiting: Preventing Abuse

Rate limiting prevents any single user from overwhelming your server.

### How It Works: Token Bucket Algorithm

MCP Guard uses the **token bucket** algorithm:

1. Each user has a "bucket" of tokens
2. The bucket fills at `requests_per_second` rate
3. Each request removes one token
4. If the bucket is empty, request is rejected (429)
5. Burst allowance lets users temporarily exceed the rate

**Visual example:**

```
Bucket Capacity: 50 tokens (burst_size)
Fill Rate: 100 tokens/second

Time 0:    [████████████████████████] 50/50 tokens
Request:   [███████████████████████ ] 49/50
Request:   [██████████████████████  ] 48/50
...
Time 0.5s: [████████████████████████] 50/50 (refilled)
```

### Configuration

```toml
[rate_limit]
enabled = true
requests_per_second = 100   # Default limit
burst_size = 50              # Temporary burst allowance
```

### Per-Identity Overrides

Different users can have different limits:

```toml
# Free tier: 10 RPS
[[auth.api_keys]]
id = "free-user"
key_hash = "..."
rate_limit = 10

# Premium tier: 500 RPS
[[auth.api_keys]]
id = "premium-user"
key_hash = "..."
rate_limit = 500

# Internal service: 1000 RPS
[[auth.api_keys]]
id = "internal-service"
key_hash = "..."
rate_limit = 1000
```

### HTTP Headers

**Successful responses include:**

```
x-ratelimit-limit: 100
x-ratelimit-remaining: 47
x-ratelimit-reset: 1702656789
```

**Rate-limited responses (429):**

```
Retry-After: 1
```

```json
{
  "error": "Rate limit exceeded",
  "error_id": "550e8400-e29b-41d4-a716-446655440000",
  "retry_after": 1
}
```

---

## Audit Logging: Track Everything

Audit logging records every request for security, compliance, and debugging.

### What Gets Logged

| Event Type | When | Details |
|------------|------|---------|
| `AuthSuccess` | Authentication succeeds | Identity ID, method |
| `AuthFailure` | Authentication fails | Error reason |
| `Authorized` | Authorization passes | Tool, identity |
| `AuthorizationDenied` | Authorization fails | Tool, identity |
| `RateLimited` | Rate limit exceeded | Identity, retry time |
| `ToolCall` | Tool execution | Tool, duration, result |

### Example Audit Entry

```json
{
  "timestamp": "2024-12-31T14:30:00Z",
  "event_type": "ToolCall",
  "identity_id": "alice",
  "tool": "read_file",
  "path": "/home/user/documents/secret.txt",
  "duration_ms": 42,
  "status": "success",
  "trace_id": "abc123def456"
}
```

### Configuration

**Log to console (development):**

```toml
[audit]
enabled = true
stdout = true
```

**Log to file (production):**

```toml
[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
```

**Export to SIEM (Splunk example):**

```toml
[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 15
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
```

**Multiple outputs:**

```toml
[audit]
enabled = true
stdout = true
file = "/var/log/mcp-guard/audit.log"
export_url = "https://siem.example.com/logs"
```

---

## The Request Pipeline

Every request flows through these stages in order:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Request Processing                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. Connection        Incoming TCP connection on configured host:port       │
│         │                                                                   │
│         ▼                                                                   │
│  2. Security Headers  X-Content-Type-Options, X-Frame-Options, CSP added   │
│         │                                                                   │
│         ▼                                                                   │
│  3. Trace Context     W3C traceparent/tracestate extracted/propagated      │
│         │                                                                   │
│         ▼                                                                   │
│  4. Metrics           Request counter incremented, timer started           │
│         │                                                                   │
│         ▼                                                                   │
│  5. Auth Middleware   Token extracted, provider.authenticate() called      │
│         │             Identity stored in request extensions                │
│         │             Audit: auth_success or auth_failure                  │
│         ▼                                                                   │
│  6. Rate Limit        Per-identity check via RateLimitService             │
│         │             429 with Retry-After if exceeded                     │
│         │             Audit: rate_limited                                  │
│         ▼                                                                   │
│  7. Handler           MCP message parsed, authorization checked            │
│         │             tools/list filtered per identity.allowed_tools       │
│         ▼                                                                   │
│  8. Transport         Message forwarded to upstream via transport         │
│         │             Response received and returned to client            │
│         ▼                                                                   │
│  9. Response          Status recorded in metrics, duration histogram      │
│                       x-ratelimit-* headers added                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Transport Types

MCP Guard supports three ways to communicate with upstream MCP servers:

### 1. Stdio Transport

Runs an MCP server as a subprocess, communicating via stdin/stdout.

**Best for:** Local development, Node.js/Python MCP servers.

```toml
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
```

**Python example:**

```toml
[upstream]
transport = "stdio"
command = "python"
args = ["-m", "my_mcp_server"]
```

**Security:** MCP Guard validates commands to prevent shell injection attacks.

### 2. HTTP Transport

Connects to an HTTP-based MCP server.

**Best for:** Remote MCP servers, microservices.

```toml
[upstream]
transport = "http"
url = "http://localhost:8080/mcp"
```

**Security:** MCP Guard validates URLs to prevent SSRF attacks (blocks private IPs, cloud metadata endpoints).

### 3. SSE Transport

Connects via Server-Sent Events for streaming responses.

**Best for:** Long-running operations, real-time updates.

```toml
[upstream]
transport = "sse"
url = "http://localhost:8080/mcp/stream"
```

---

## Multi-Server Routing

MCP Guard can route requests to different upstream servers based on path.

### Configuration

```toml
[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[upstream.servers]]
name = "github"
path_prefix = "/github"
transport = "http"
url = "https://github-mcp.example.com/api"

[[upstream.servers]]
name = "database"
path_prefix = "/db"
transport = "http"
url = "http://localhost:8081/mcp"
```

### Accessing Different Servers

```bash
# Access filesystem server
curl -X POST http://localhost:3000/mcp/fs \
  -H "Authorization: Bearer ..." \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# Access GitHub server
curl -X POST http://localhost:3000/mcp/github \
  -H "Authorization: Bearer ..." \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# List available routes
curl http://localhost:3000/routes
```

---

## Observability & Metrics

### Prometheus Metrics

MCP Guard exposes metrics at `/metrics`:

```bash
curl http://localhost:3000/metrics
```

**Key metrics:**

| Metric | Description |
|--------|-------------|
| `mcp_guard_requests_total` | Total requests by status |
| `mcp_guard_request_duration_seconds` | Request latency histogram |
| `mcp_guard_rate_limit_total` | Rate limit allow/block count |
| `mcp_guard_active_identities` | Currently tracked identities |

### OpenTelemetry Tracing

Integrate with Jaeger, Zipkin, or Grafana Tempo:

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"
sample_rate = 1.0  # 100% for development
propagate_context = true
```

### Health Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /health` | Quick health check |
| `GET /ready` | Readiness probe (for k8s) |
| `GET /metrics` | Prometheus metrics |

---

## Production Deployment

### Minimal Production Config

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
algorithms = ["RS256"]
issuer = "https://auth.example.com/"
audience = "mcp-guard"

[rate_limit]
enabled = true
requests_per_second = 500
burst_size = 100

[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
```

### Docker Deployment

```dockerfile
FROM rust:latest as builder
RUN cargo install mcp-guard

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/mcp-guard /usr/local/bin/
COPY mcp-guard.toml /etc/mcp-guard/config.toml
EXPOSE 3000
CMD ["mcp-guard", "run", "--config", "/etc/mcp-guard/config.toml"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mcp-guard
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: mcp-guard
        image: your-registry/mcp-guard:latest
        ports:
        - containerPort: 3000
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
```

---

## Troubleshooting

### Common Issues

#### "Config file not found"

```bash
# Specify config path explicitly
mcp-guard run --config /path/to/config.toml
```

#### "Invalid API key"

1. Check you're using the full key (starts with `mcp_`)
2. Verify the hash matches: `mcp-guard hash-key "mcp_YOUR_KEY"`
3. Ensure no whitespace in the key

#### "Upstream communication error"

```bash
# Test upstream connectivity
mcp-guard check-upstream --timeout 30
```

For stdio transport, verify the command works:

```bash
npx -y @modelcontextprotocol/server-filesystem /tmp
```

#### Rate limit issues

Check current limits in response headers:

```bash
curl -v -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer mcp_KEY" \
  -d '{...}'
# Look for x-ratelimit-* headers
```

### Enable Verbose Logging

```bash
mcp-guard run -v      # Verbose
mcp-guard run -vv     # Very verbose
```

---

## Feature Comparison by Tier

| Feature | Free | Pro | Enterprise |
|---------|:----:|:---:|:----------:|
| **Authentication** | | | |
| API Key Authentication | ✅ | ✅ | ✅ |
| JWT (HS256) | ✅ | ✅ | ✅ |
| JWT (JWKS/RS256/ES256) | | ✅ | ✅ |
| OAuth 2.1 with PKCE | | ✅ | ✅ |
| mTLS Client Certificates | | | ✅ |
| **Transport** | | | |
| Stdio Transport | ✅ | ✅ | ✅ |
| HTTP Transport | | ✅ | ✅ |
| SSE Transport | | ✅ | ✅ |
| Multi-Server Routing | | | ✅ |
| **Security** | | | |
| Per-Tool Authorization | ✅ | ✅ | ✅ |
| Tools/List Filtering | ✅ | ✅ | ✅ |
| Global Rate Limiting | ✅ | ✅ | ✅ |
| Per-Identity Rate Limiting | | ✅ | ✅ |
| **Observability** | | | |
| Prometheus Metrics | ✅ | ✅ | ✅ |
| Health Check Endpoints | ✅ | ✅ | ✅ |
| Audit Logging (file/console) | ✅ | ✅ | ✅ |
| OpenTelemetry Tracing | | | ✅ |
| Audit Log Shipping (SIEM) | | | ✅ |

### Pricing

| Tier | Price | Best For |
|------|-------|----------|
| **Free** | $0 | Individual developers, hobbyists |
| **Pro** | $12/month | Small teams, production apps |
| **Enterprise** | $29+/user/month | Large teams, compliance requirements |

---

## Glossary

| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol — standard for AI tools |
| **API Key** | Pre-shared secret for authentication |
| **JWT** | JSON Web Token — signed, self-contained token |
| **OAuth** | Open standard for authorization delegation |
| **mTLS** | Mutual TLS — both client and server authenticate |
| **PKCE** | Proof Key for Code Exchange — OAuth security extension |
| **JWKS** | JSON Web Key Set — public keys for JWT verification |
| **Identity** | Authenticated user/service representation |
| **Rate Limiting** | Restricting request frequency |
| **Token Bucket** | Algorithm for rate limiting with burst support |
| **Audit Log** | Record of security-relevant events |
| **SSRF** | Server-Side Request Forgery — attack type |
| **SIEM** | Security Information and Event Management |
| **OTLP** | OpenTelemetry Protocol — for tracing |

---

## Next Steps

Now that you understand MCP Guard, here are some paths forward:

1. **[Quick Start Guide](quickstart.md)** — Get started in 5 minutes
2. **[Configuration Reference](configuration.md)** — All configuration options
3. **[Authentication Guide](authentication.md)** — Deep dive into auth providers
4. **[CLI Reference](cli.md)** — All command-line options
5. **[Troubleshooting Guide](troubleshooting.md)** — Common issues and solutions

---

<div align="center">

**Questions?** Open an issue on [GitHub](https://github.com/botzrdev/mcp-guard/issues)

**Happy securing!** 🔐

</div>
