import { Injectable } from '@angular/core';

export interface DocPage {
  slug: string;
  title: string;
  description: string;
  content: string;
  category: string;
}

export interface DocCategory {
  name: string;
  slug: string;
  pages: { slug: string; title: string }[];
}

@Injectable({
  providedIn: 'root'
})
export class DocsService {
  private docs: Map<string, DocPage> = new Map();

  constructor() {
    this.initializeDocs();
  }

  private initializeDocs(): void {
    // Quick Start Guide
    this.docs.set('quickstart', {
      slug: 'quickstart',
      title: 'Quick Start Guide',
      description: 'Get from zero to a secured MCP server in under 10 minutes.',
      category: 'Getting Started',
      content: QUICKSTART_CONTENT
    });

    // Configuration Reference
    this.docs.set('configuration', {
      slug: 'configuration',
      title: 'Configuration Reference',
      description: 'Complete reference for all MCP Guard configuration options.',
      category: 'Reference',
      content: CONFIGURATION_CONTENT
    });

    // CLI Reference
    this.docs.set('cli', {
      slug: 'cli',
      title: 'CLI Reference',
      description: 'Complete reference for all MCP Guard command-line commands and options.',
      category: 'Reference',
      content: CLI_CONTENT
    });

    // Authentication Guide
    this.docs.set('authentication', {
      slug: 'authentication',
      title: 'Authentication Guide',
      description: 'Deep dive into MCP Guard\'s authentication providers.',
      category: 'Guides',
      content: AUTHENTICATION_CONTENT
    });

    // HTTP API Reference
    this.docs.set('api', {
      slug: 'api',
      title: 'HTTP API Reference',
      description: 'All HTTP endpoints exposed by MCP Guard.',
      category: 'Reference',
      content: API_CONTENT
    });

    // Rate Limiting Guide
    this.docs.set('rate-limiting', {
      slug: 'rate-limiting',
      title: 'Rate Limiting Guide',
      description: 'Configure and tune per-identity rate limiting.',
      category: 'Guides',
      content: RATE_LIMITING_CONTENT
    });

    // Transports Guide
    this.docs.set('transports', {
      slug: 'transports',
      title: 'Transport Guide',
      description: 'Deep dive into MCP Guard transport types.',
      category: 'Guides',
      content: TRANSPORTS_CONTENT
    });

    // Observability Guide
    this.docs.set('observability', {
      slug: 'observability',
      title: 'Observability Guide',
      description: 'Monitor MCP Guard with Prometheus metrics, OpenTelemetry tracing, and audit logging.',
      category: 'Guides',
      content: OBSERVABILITY_CONTENT
    });
  }

  getDoc(slug: string): DocPage | undefined {
    return this.docs.get(slug);
  }

  getAllDocs(): DocPage[] {
    return Array.from(this.docs.values());
  }

  getCategories(): DocCategory[] {
    return [
      {
        name: 'Getting Started',
        slug: 'getting-started',
        pages: [
          { slug: 'quickstart', title: 'Quick Start' }
        ]
      },
      {
        name: 'Guides',
        slug: 'guides',
        pages: [
          { slug: 'authentication', title: 'Authentication' },
          { slug: 'rate-limiting', title: 'Rate Limiting' },
          { slug: 'transports', title: 'Transports' },
          { slug: 'observability', title: 'Observability' }
        ]
      },
      {
        name: 'Reference',
        slug: 'reference',
        pages: [
          { slug: 'configuration', title: 'Configuration' },
          { slug: 'cli', title: 'CLI' },
          { slug: 'api', title: 'HTTP API' }
        ]
      }
    ];
  }
}

// Documentation content
const QUICKSTART_CONTENT = `# Quick Start Guide

Get from zero to a secured MCP server in under 10 minutes.

## What is MCP Guard?

MCP Guard is a lightweight security gateway that wraps any Model Context Protocol (MCP) server with production-grade protection. It provides authentication, authorization, rate limiting, and observability without requiring Docker, Kubernetes, or a DevOps team.

## Prerequisites

### Required

- **mcp-guard binary** (see Installation below)

### For the Demo Setup

- **Node.js & npm** - The default config uses \`npx\` to run the demo MCP filesystem server

> **Don't have Node.js?** Either:
> - Install Node.js from [nodejs.org](https://nodejs.org)
> - Use your own MCP server (edit the \`[upstream]\` section in config)
> - Use HTTP transport with an existing MCP server URL

### Installation Speed

| Method | Time | Best For |
|--------|------|----------|
| cargo install | 2-3 min | Rust developers |
| Build from source | 3-5 min | Contributors |

## Installation

Choose one of three installation methods:

### Option 1: Install from crates.io

\`\`\`bash
cargo install mcp-guard
\`\`\`

### Option 2: Download Prebuilt Binary

Download the latest release for your platform from [GitHub Releases](https://github.com/botzrdev/mcp-guard/releases).

### Option 3: Build from Source

\`\`\`bash
git clone https://github.com/botzrdev/mcp-guard
cd mcp-guard
cargo build --release
\`\`\`

The binary will be at \`./target/release/mcp-guard\`.

### Verify Installation

\`\`\`bash
mcp-guard version
\`\`\`

---

## Step 1: Generate Configuration

Create a configuration file template:

\`\`\`bash
mcp-guard init
\`\`\`

This creates \`mcp-guard.toml\` in your current directory with example configurations for all features.

## Step 2: Generate an API Key

Generate an API key for your first client:

\`\`\`bash
# Option A: Auto-add to config (recommended)
mcp-guard keygen --user-id my-agent --apply-to-config

# Option B: Manual (prints TOML to copy)
mcp-guard keygen --user-id my-agent
\`\`\`

**Important:**
- The **API Key** goes to your client application (store it securely)
- The **key_hash** is stored in the config file (safe to commit)

## Step 3: Configure Upstream

Edit \`mcp-guard.toml\` to point to your MCP server:

\`\`\`toml
[server]
host = "127.0.0.1"
port = 3000

# Your MCP server
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# Rate limiting
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

# Audit logging
[audit]
enabled = true
stdout = true
\`\`\`

Validate your configuration:

\`\`\`bash
mcp-guard validate
\`\`\`

## Step 4: Start the Gateway

Start MCP Guard:

\`\`\`bash
mcp-guard run
\`\`\`

Verify it's running:

\`\`\`bash
curl http://localhost:3000/health
\`\`\`

**Response:**

\`\`\`json
{"status": "healthy", "version": "1.0.0", "uptime_secs": 5}
\`\`\`

## Step 5: Test Authentication

### Unauthenticated Request (should fail)

\`\`\`bash
curl -X POST http://localhost:3000/mcp \\
  -H "Content-Type: application/json" \\
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
\`\`\`

**Response: 401 Unauthorized**

### Authenticated Request (should succeed)

\`\`\`bash
curl -X POST http://localhost:3000/mcp \\
  -H "Content-Type: application/json" \\
  -H "Authorization: Bearer mcp_YOUR_API_KEY_HERE" \\
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
\`\`\`

**Response: 200 OK**

---

## Next Steps

Now that you have a working gateway, explore these topics:

- **Configuration Reference** - Complete reference for all configuration options
- **Authentication Guide** - Deep dive into JWT, OAuth 2.1, and mTLS
- **CLI Reference** - All CLI commands and options
`;

const CONFIGURATION_CONTENT = `# Configuration Reference

Complete reference for all MCP Guard configuration options.

## Overview

MCP Guard uses a configuration file to define server settings, authentication providers, rate limiting, audit logging, and upstream connections.

### Supported Formats

- **TOML** (recommended): \`mcp-guard.toml\`
- **YAML**: \`mcp-guard.yaml\` or \`mcp-guard.yml\`

### File Locations

1. Explicit path: \`mcp-guard run --config /path/to/config.toml\`
2. Current directory: \`./mcp-guard.toml\`

### Validation

Always validate configuration before deployment:

\`\`\`bash
mcp-guard validate
mcp-guard validate --config production.toml
\`\`\`

---

## [server] Section

Server and TLS configuration.

### Basic Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`host\` | string | \`"127.0.0.1"\` | Host to bind to |
| \`port\` | integer | \`3000\` | Port to listen on (1-65535) |

**Example:**

\`\`\`toml
[server]
host = "0.0.0.0"  # Listen on all interfaces
port = 3000
\`\`\`

### TLS Configuration

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| \`tls.cert_path\` | string | Yes (for TLS) | Path to server certificate (PEM) |
| \`tls.key_path\` | string | Yes (for TLS) | Path to server private key (PEM) |
| \`tls.client_ca_path\` | string | No | Path to CA for client cert validation (enables mTLS) |

---

## [auth] Section

Authentication configuration. Multiple providers can be enabled simultaneously.

### API Keys [[auth.api_keys]]

Pre-shared API keys with SHA-256 hashing.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| \`id\` | string | Yes | Unique identifier for the key holder |
| \`key_hash\` | string | Yes | Base64-encoded SHA-256 hash of the API key |
| \`allowed_tools\` | array | No | List of allowed tool names (empty = all) |
| \`rate_limit\` | integer | No | Custom rate limit (requests/second) |

**Generate keys:**

\`\`\`bash
mcp-guard keygen --user-id alice
\`\`\`

### JWT [auth.jwt]

JSON Web Token authentication with two modes: simple (HS256) and JWKS (RS256/ES256).

#### Common Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`mode\` | string | Required | \`"simple"\` or \`"jwks"\` |
| \`issuer\` | string | Required | Expected \`iss\` claim value |
| \`audience\` | string | Required | Expected \`aud\` claim value |

### OAuth [auth.oauth]

OAuth 2.1 authentication with PKCE support.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`provider\` | string | Required | \`"github"\`, \`"google"\`, \`"okta"\`, or \`"custom"\` |
| \`client_id\` | string | Required | OAuth client ID |
| \`client_secret\` | string | No | OAuth client secret |

### mTLS [auth.mtls]

Client certificate authentication via reverse proxy headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`enabled\` | boolean | \`false\` | Enable mTLS authentication |
| \`identity_source\` | string | \`"cn"\` | Certificate field for identity |

---

## [rate_limit] Section

Per-identity rate limiting using token bucket algorithm.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`enabled\` | boolean | \`true\` | Enable rate limiting |
| \`requests_per_second\` | integer | \`100\` | Default rate limit |
| \`burst_size\` | integer | \`50\` | Burst allowance |

---

## [audit] Section

Audit logging configuration with file, stdout, and HTTP export options.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`enabled\` | boolean | \`true\` | Enable audit logging |
| \`stdout\` | boolean | \`false\` | Log to stdout |
| \`file\` | string | None | File path for audit logs |
| \`export_url\` | string | None | HTTP endpoint for SIEM integration |

---

## [tracing] Section

OpenTelemetry distributed tracing with W3C trace context propagation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`enabled\` | boolean | \`false\` | Enable OpenTelemetry tracing |
| \`service_name\` | string | \`"mcp-guard"\` | Service name in traces |
| \`otlp_endpoint\` | string | Required | OTLP gRPC endpoint |
| \`sample_rate\` | float | \`0.1\` | Sampling rate (0.0-1.0) |

---

## [upstream] Section

Upstream MCP server configuration.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| \`transport\` | string | Yes | \`"stdio"\`, \`"http"\`, or \`"sse"\` |
| \`command\` | string | For stdio | Command to execute |
| \`args\` | array | No | Command arguments |
| \`url\` | string | For http/sse | Upstream URL |
`;

const CLI_CONTENT = `# CLI Reference

Complete reference for all MCP Guard command-line commands and options.

## Overview

MCP Guard provides a command-line interface for configuration, key management, and server operation.

### Global Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| \`--config\` | \`-c\` | \`mcp-guard.toml\` | Path to configuration file |
| \`--verbose\` | \`-v\` | false | Enable verbose logging output |
| \`--help\` | \`-h\` | - | Show help for command |

---

## Commands

### init

Generate a new configuration file template.

\`\`\`bash
mcp-guard init [OPTIONS]
\`\`\`

| Option | Default | Description |
|--------|---------|-------------|
| \`--format\` | \`toml\` | Output format: \`toml\` or \`yaml\` |
| \`--force\` | false | Overwrite existing file |

### validate

Parse and validate a configuration file without starting the server.

\`\`\`bash
mcp-guard validate [OPTIONS]
\`\`\`

### keygen

Generate a new API key with its hash for use in configuration.

\`\`\`bash
mcp-guard keygen --user-id <ID> [OPTIONS]
\`\`\`

| Option | Type | Description |
|--------|------|-------------|
| \`--rate-limit\` | u32 | Custom rate limit in requests per second |
| \`--tools\` | string | Comma-separated list of allowed tools |
| \`--apply-to-config\` | flag | Automatically add the key to the config file |

### run

Start the MCP Guard server.

\`\`\`bash
mcp-guard run [OPTIONS]
\`\`\`

| Option | Type | Description |
|--------|------|-------------|
| \`--host\` | string | Override listen host from config |
| \`--port\` | u16 | Override listen port from config |

### version

Display version, build information, and available features.

\`\`\`bash
mcp-guard version
\`\`\`

### check-upstream

Test upstream MCP server connectivity without starting the full gateway.

\`\`\`bash
mcp-guard check-upstream [OPTIONS]
\`\`\`

| Option | Default | Description |
|--------|---------|-------------|
| \`--timeout\` | 10 | Timeout in seconds |

---

## Common Workflows

### Initial Setup

\`\`\`bash
# 1. Generate config template
mcp-guard init

# 2. Generate API key
mcp-guard keygen --user-id my-agent

# 3. Validate configuration
mcp-guard validate

# 4. Test upstream connectivity
mcp-guard check-upstream

# 5. Start server
mcp-guard run
\`\`\`
`;

const AUTHENTICATION_CONTENT = `# Authentication Guide

Deep dive into MCP Guard's authentication providers, including setup, configuration, and troubleshooting.

## Overview

MCP Guard supports four authentication providers that can be used individually or combined:

| Provider | Use Case | Token Type |
|----------|----------|------------|
| **API Keys** | Service-to-service, CLI tools | Pre-shared key |
| **JWT** | Enterprise SSO, existing IdPs | JSON Web Token |
| **OAuth 2.1** | User authentication, third-party apps | Access token |
| **mTLS** | High-security, zero-trust | Client certificate |

### How Authentication Works

1. Client sends request with credentials
2. MCP Guard extracts credentials (mTLS headers or Bearer token)
3. Credentials are validated against configured providers
4. On success, an \`Identity\` is created with user info and permissions
5. Identity is used for authorization and rate limiting

---

## API Key Authentication

### Overview

API keys are pre-shared secrets for simple, secure authentication. They're ideal for:

- Service-to-service communication
- CLI tools and scripts
- Simple deployments without an IdP

### Security Model

- Keys are SHA-256 hashed before storage
- Constant-time comparison prevents timing attacks
- Keys cannot be recovered from configuration (hash only)

### Configuration

\`\`\`toml
[[auth.api_keys]]
id = "my-service"
key_hash = "base64-encoded-sha256-hash"
allowed_tools = ["read_file", "list_directory"]
rate_limit = 100
\`\`\`

### Client Usage

\`\`\`bash
curl -X POST http://localhost:3000/mcp \\
  -H "Content-Type: application/json" \\
  -H "Authorization: Bearer mcp_AbCdEf123456789XYZ..." \\
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
\`\`\`

---

## JWT Authentication

### Overview

JWT (JSON Web Token) authentication integrates with existing identity providers like Auth0, Keycloak, Okta, and AWS Cognito. It supports two modes:

- **Simple (HS256)**: Symmetric key, shared secret
- **JWKS (RS256/ES256)**: Asymmetric keys from IdP

### Simple Mode Configuration

\`\`\`toml
[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-here-minimum-32-characters"
issuer = "https://your-app.com"
audience = "mcp-guard"
\`\`\`

### JWKS Mode Configuration

\`\`\`toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://your-idp.com/.well-known/jwks.json"
algorithms = ["RS256", "ES256"]
issuer = "https://your-idp.com/"
audience = "mcp-guard"
\`\`\`

### Scope-to-Tool Mapping

\`\`\`toml
[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]  # Wildcard = all tools
\`\`\`

---

## OAuth 2.1 Authentication

### Overview

OAuth 2.1 enables user authentication through external providers like GitHub and Google. MCP Guard implements:

- Authorization Code flow with PKCE (RFC 7636)
- Token introspection (RFC 7662)
- UserInfo endpoint fallback

### GitHub Configuration

\`\`\`toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["read:user", "repo"]
\`\`\`

### Google Configuration

\`\`\`toml
[auth.oauth]
provider = "google"
client_id = "your-client-id.apps.googleusercontent.com"
client_secret = "your-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["openid", "profile", "email"]
\`\`\`

---

## mTLS Authentication

### Overview

Mutual TLS (mTLS) authenticates clients using X.509 certificates. It's ideal for:

- Zero-trust architectures
- Service mesh integration
- High-security environments

### Configuration

\`\`\`toml
[auth.mtls]
enabled = true
identity_source = "cn"
allowed_tools = ["read_file", "write_file"]
rate_limit = 1000
trusted_proxy_ips = ["10.0.0.0/8", "172.16.0.0/12"]
\`\`\`

**Security Critical:** You **must** configure \`trusted_proxy_ips\` when enabling mTLS.
`;

const API_CONTENT = `# HTTP API Reference

This document describes all HTTP endpoints exposed by MCP Guard.

## Base URL

\`\`\`
http://{host}:{port}
\`\`\`

Default: \`http://127.0.0.1:3000\`

## Authentication

Protected endpoints require a Bearer token in the Authorization header:

\`\`\`http
Authorization: Bearer <token>
\`\`\`

Supported token types:
- **API Key**: Raw API key string
- **JWT**: Signed JWT token
- **OAuth**: Access token from OAuth 2.1 flow

---

## Health Endpoints

### GET /health

Comprehensive health check with version and uptime.

**Response:** \`200 OK\`

\`\`\`json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_secs": 3600
}
\`\`\`

### GET /live

Kubernetes liveness probe.

**Response:** \`200 OK\`

\`\`\`json
{
  "status": "alive"
}
\`\`\`

### GET /ready

Kubernetes readiness probe.

**Response:** \`200 OK\` or \`503 Service Unavailable\`

---

## Metrics Endpoint

### GET /metrics

Prometheus-formatted metrics.

**Content-Type:** \`text/plain; charset=utf-8\`

\`\`\`prometheus
mcp_guard_requests_total{method="POST",status="200"} 1523
mcp_guard_active_identities 25
\`\`\`

---

## MCP Endpoints

### POST /mcp

Forward an MCP JSON-RPC request to the upstream server.

**Authentication:** Required

**Request Body:**

\`\`\`json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "read_file",
    "arguments": {
      "path": "/tmp/test.txt"
    }
  }
}
\`\`\`

**Response:** \`200 OK\`

\`\`\`json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "File contents here..."
      }
    ]
  }
}
\`\`\`

---

## Error Responses

### 401 Unauthorized

Missing or invalid authentication credentials.

\`\`\`json
{
  "error": "Authentication failed: InvalidApiKey",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
\`\`\`

### 403 Forbidden

Authenticated but not authorized for the requested action.

### 429 Too Many Requests

Rate limit exceeded.

**Headers:**
- \`Retry-After\`: Seconds until retry allowed

---

## Rate Limiting

Rate limits are applied per-identity. Successful responses include:

\`\`\`http
x-ratelimit-limit: 100
x-ratelimit-remaining: 42
x-ratelimit-reset: 1702900000
\`\`\`
`;

const RATE_LIMITING_CONTENT = `# Rate Limiting Guide

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

\`\`\`toml
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50
\`\`\`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| \`enabled\` | boolean | \`true\` | Enable rate limiting |
| \`requests_per_second\` | integer | \`100\` | Requests per second limit |
| \`burst_size\` | integer | \`50\` | Maximum burst allowance |

### Per-Identity Overrides

\`\`\`toml
[[auth.api_keys]]
id = "high-volume-service"
key_hash = "..."
rate_limit = 1000  # 1000 RPS instead of default 100
\`\`\`

---

## How It Works

### Token Bucket Algorithm

1. **Bucket fills** at \`requests_per_second\` rate
2. **Bucket capacity** is \`burst_size\`
3. **Each request** removes one token
4. **If empty**, request is rejected with 429

---

## HTTP Headers

### Successful Requests (2xx)

\`\`\`
x-ratelimit-limit: 100
x-ratelimit-remaining: 47
x-ratelimit-reset: 1702656789
\`\`\`

### Rate Limited Requests (429)

**Status:** \`429 Too Many Requests\`

**Headers:**
\`\`\`
Retry-After: 1
\`\`\`

---

## Client Handling

Clients should implement backoff based on \`Retry-After\`:

\`\`\`python
import requests
import time

def call_mcp(endpoint, data, headers):
    response = requests.post(endpoint, json=data, headers=headers)

    if response.status_code == 429:
        retry_after = int(response.headers.get('Retry-After', 1))
        time.sleep(retry_after)
        return call_mcp(endpoint, data, headers)

    return response
\`\`\`
`;

const TRANSPORTS_CONTENT = `# Transport Guide

Deep dive into MCP Guard transport types for connecting to upstream MCP servers.

## Overview

Transports define how MCP Guard communicates with upstream MCP servers.

### Available Transports

| Transport | Use Case | Communication |
|-----------|----------|---------------|
| **Stdio** | Local processes (npx, python) | stdin/stdout |
| **HTTP** | Remote servers, microservices | POST JSON-RPC |
| **SSE** | Streaming responses | Server-Sent Events |

---

## Stdio Transport

### Overview

The stdio transport spawns a local process and communicates via stdin/stdout using newline-delimited JSON-RPC messages.

**Best for:**
- npx-based MCP servers
- Python MCP servers
- Local development

### Configuration

\`\`\`toml
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
\`\`\`

---

## HTTP Transport

### Overview

The HTTP transport sends JSON-RPC requests as POST requests to an HTTP endpoint.

**Best for:**
- Cloud-hosted MCP servers
- Microservice architectures
- Load-balanced deployments

### Configuration

\`\`\`toml
[upstream]
transport = "http"
url = "http://mcp-server.internal:8080/mcp"
\`\`\`

---

## SSE Transport

### Overview

The SSE (Server-Sent Events) transport is designed for streaming responses from upstream MCP servers.

**Best for:**
- Long-running tool operations
- Streaming LLM responses
- Progress updates

### Configuration

\`\`\`toml
[upstream]
transport = "sse"
url = "http://mcp-server.internal:8080/mcp/stream"
\`\`\`

---

## Transport Comparison

| Feature | Stdio | HTTP | SSE |
|---------|-------|------|-----|
| **Location** | Local only | Remote | Remote |
| **Streaming** | No | No | Yes |
| **Scalability** | Single instance | Load balanced | Load balanced |
| **Latency** | Lowest | Low | Low (streaming) |
`;

const OBSERVABILITY_CONTENT = `# Observability Guide

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

\`\`\`bash
curl http://localhost:3000/metrics
\`\`\`

### Available Metrics

#### mcp_guard_requests_total

Total HTTP requests by method and status code.

| Label | Values | Description |
|-------|--------|-------------|
| \`method\` | GET, POST | HTTP method |
| \`status\` | 200, 401, 403, 429, 500 | HTTP status code |

#### mcp_guard_request_duration_seconds

Request latency histogram.

**Buckets:** 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0

#### mcp_guard_auth_total

Authentication attempts by provider and result.

#### mcp_guard_rate_limit_total

Rate limiting decisions.

#### mcp_guard_active_identities

Current number of tracked identities (gauge).

---

## OpenTelemetry Tracing

### Configuration

\`\`\`toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"
sample_rate = 0.1
propagate_context = true
\`\`\`

### W3C Trace Context

When \`propagate_context = true\`, MCP Guard:

1. **Extracts** \`traceparent\` headers from incoming requests
2. **Creates** child spans within the same trace
3. **Injects** trace context into upstream requests
4. **Includes** \`trace_id\` in audit logs

---

## Audit Logging

### Configuration

\`\`\`toml
[audit]
enabled = true
stdout = false
file = "/var/log/mcp-guard/audit.log"
export_url = "https://siem.example.com/api/logs"
export_batch_size = 100
\`\`\`

### Event Types

| Event Type | Description |
|------------|-------------|
| \`AuthSuccess\` | Successful authentication |
| \`AuthFailure\` | Failed authentication |
| \`ToolCall\` | Tool invocation |
| \`RateLimited\` | Rate limit exceeded |
| \`AuthzDenied\` | Authorization denied |

---

## Health Endpoints

### GET /health

Detailed health status with version and uptime.

### GET /live

Kubernetes liveness probe.

### GET /ready

Kubernetes readiness probe.
`;
