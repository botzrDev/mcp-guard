# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 1.x     | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it through **GitHub Security Advisories** for private, responsible disclosure:

**[Report a Vulnerability](https://github.com/botzrDev/mcp-guard/security/advisories/new)**

### What to Include

- A clear description of the vulnerability
- Steps to reproduce the issue
- Potential impact assessment
- Any suggested fixes (optional)

### Response Timeline

| Stage | Timeline |
|-------|----------|
| Initial acknowledgment | 48 hours |
| Severity assessment | 7 days |
| Fix for critical/high severity | 30 days |
| Fix for medium/low severity | 90 days |

### What to Expect

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours.
2. **Assessment**: Our team will assess the severity and impact.
3. **Communication**: We will keep you informed of our progress.
4. **Credit**: With your permission, we will credit you in the security advisory.

## Security Best Practices

When deploying mcp-guard in production:

### Authentication

- **API Keys**: Use long, randomly generated API keys (minimum 32 characters)
- **JWT**: Configure JWKS endpoint for public key rotation; avoid HS256 with weak secrets
- **OAuth 2.1**: Always use PKCE for public clients
- **mTLS**: Prefer mTLS for service-to-service authentication

### Network Security

- Run mcp-guard behind a reverse proxy (nginx, Traefik, Caddy)
- Enable TLS 1.2+ for all connections
- Use firewall rules to restrict access to management endpoints
- Consider network segmentation for MCP servers

### Configuration

- Never commit secrets to version control
- Use environment variables or secret managers for sensitive values
- Set restrictive file permissions on config files (`chmod 600`)
- Enable audit logging for compliance and incident response

### Rate Limiting

- Configure appropriate rate limits to prevent abuse
- Use per-identity rate limits for authenticated users
- Set stricter limits for sensitive operations

### Monitoring

- Enable Prometheus metrics for observability
- Configure OpenTelemetry tracing for request correlation
- Ship audit logs to a SIEM for security monitoring
- Set up alerts for authentication failures and rate limit hits

## Security Features

mcp-guard includes the following security features:

| Feature | Description |
|---------|-------------|
| **Multi-provider Authentication** | API keys, JWT (HS256/RS256/ES256), OAuth 2.1, mTLS |
| **Per-identity Rate Limiting** | Token bucket algorithm with configurable limits |
| **Tool Authorization** | Glob-pattern based tool access control |
| **Audit Logging** | JSON Lines format with SIEM export support |
| **Secret Redaction** | Configurable patterns to prevent credential leakage |
| **Request Validation** | Input sanitization and size limits |
| **TLS Support** | Native TLS with configurable certificates |

## Vulnerability Disclosure Policy

- We follow [coordinated vulnerability disclosure](https://en.wikipedia.org/wiki/Coordinated_vulnerability_disclosure)
- Security issues will be fixed before public disclosure
- We will publish security advisories for significant vulnerabilities
- We do not pursue legal action against good-faith security researchers

## Contact

For general security questions (not vulnerability reports), please open a GitHub Discussion.
