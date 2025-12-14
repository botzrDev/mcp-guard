<div align="center">
  <h1>🛡️ mcp-guard</h1>
  <h3>Secure your MCP servers in 5 minutes.</h3>
  <p>No Docker. No Kubernetes. No DevOps team.</p>
</div>

## The Problem
Model Context Protocol (MCP) is powerful, but most servers are deployed with **zero authentication**. If your agent can access it, so can anyone else.

## The Solution
`mcp-guard` is a lightweight security gateway that wraps your MCP server protection:
- **Authentication**: API Keys, OAuth 2.1, JWT
- **Control**: Per-tool permissions & Rate limiting
- **Visibility**: Audit logging for every request

## Quick Start (Coming Soon)
```bash
# The dream:
curl -fsSL https://mcp.guard/install.sh | sh
mcp-guard run --upstream http://localhost:8080
```

## Status
🚧 **Under Construction** - Currently building the core proxy logic.
Follow development on [Twitter](https://twitter.com/YOUR_HANDLE).
