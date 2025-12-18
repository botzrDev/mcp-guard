# AuthProvider Trait Guide

This guide explains how to implement custom authentication providers for mcp-guard.

## Overview

mcp-guard uses a trait-based authentication system that allows multiple providers to be composed together. Each provider implements the `AuthProvider` trait and returns an `Identity` on successful authentication.

## The AuthProvider Trait

```rust
// src/auth/mod.rs:122-129

#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate a request and return the identity
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError>;

    /// Provider name for logging and metrics
    fn name(&self) -> &str;
}
```

### Requirements

- **`Send + Sync`**: Providers must be thread-safe for use across async tasks
- **`async_trait`**: The `authenticate` method is async to support network operations
- **Stateless preferred**: Providers should minimize internal state for scalability

## The Identity Struct

```rust
// src/auth/mod.rs:57-74

pub struct Identity {
    /// Unique identifier for the user/service
    pub id: String,

    /// Display name (optional)
    pub name: Option<String>,

    /// Allowed tools (None = unrestricted, Some([]) = no tools)
    pub allowed_tools: Option<Vec<String>>,

    /// Custom rate limit for this identity (requests per second)
    pub rate_limit: Option<u32>,

    /// Additional claims/metadata from the authentication token
    pub claims: HashMap<String, serde_json::Value>,
}
```

### Authorization Semantics

The `allowed_tools` field controls tool-level access:

| Value | Meaning |
|-------|---------|
| `None` | Unrestricted - can call any tool |
| `Some(vec!["*"])` | Wildcard - equivalent to unrestricted |
| `Some(vec!["read", "list"])` | Restricted to specified tools only |
| `Some(vec![])` | No tools allowed |

## AuthError Types

```rust
// src/auth/mod.rs:29-51

pub enum AuthError {
    MissingCredentials,           // No token provided
    InvalidApiKey,                // API key not found
    InvalidJwt(String),           // JWT validation failed
    TokenExpired,                 // Token past expiration
    OAuth(String),                // OAuth validation error
    InvalidClientCert(String),    // mTLS certificate error
    Internal(String),             // Internal error (e.g., JWKS fetch)
}
```

## Implementing a Custom Provider

### Step 1: Define Your Provider Struct

```rust
use async_trait::async_trait;
use crate::auth::{AuthError, AuthProvider, Identity};

pub struct LdapProvider {
    server_url: String,
    base_dn: String,
    // Connection pool, cache, etc.
}

impl LdapProvider {
    pub fn new(server_url: String, base_dn: String) -> Self {
        Self { server_url, base_dn }
    }
}
```

### Step 2: Implement the Trait

```rust
#[async_trait]
impl AuthProvider for LdapProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        // 1. Parse the token (e.g., "username:password" for basic auth)
        let (username, password) = parse_credentials(token)
            .map_err(|_| AuthError::MissingCredentials)?;

        // 2. Validate against LDAP server
        let user_entry = self.ldap_bind(&username, &password).await
            .map_err(|e| AuthError::Internal(e.to_string()))?;

        // 3. Extract groups/permissions from LDAP attributes
        let groups = user_entry.get_groups();
        let allowed_tools = self.map_groups_to_tools(&groups);

        // 4. Build and return Identity
        Ok(Identity {
            id: username,
            name: user_entry.display_name,
            allowed_tools,
            rate_limit: self.rate_limit_for_groups(&groups),
            claims: user_entry.to_claims(),
        })
    }

    fn name(&self) -> &str {
        "ldap"
    }
}
```

### Step 3: Add Configuration Types

```rust
// In src/config/mod.rs

#[derive(Debug, Clone, Deserialize)]
pub struct LdapConfig {
    pub server_url: String,
    pub base_dn: String,
    pub bind_dn: Option<String>,
    pub bind_password: Option<String>,
    #[serde(default)]
    pub group_tool_mapping: HashMap<String, Vec<String>>,
}
```

### Step 4: Wire Into Bootstrap

```rust
// In src/main.rs bootstrap()

if let Some(ldap_config) = &config.auth.ldap {
    tracing::info!("Enabling LDAP authentication");
    providers.push(Arc::new(LdapProvider::new(
        ldap_config.server_url.clone(),
        ldap_config.base_dn.clone(),
    )));
}
```

## The MultiProvider Composition

`MultiProvider` combines multiple auth providers with fallback behavior:

```rust
// src/auth/mod.rs:224-273

pub struct MultiProvider {
    providers: Vec<Arc<dyn AuthProvider>>,
}

#[async_trait]
impl AuthProvider for MultiProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        let mut last_error: Option<AuthError> = None;

        for provider in &self.providers {
            match provider.authenticate(token).await {
                Ok(identity) => return Ok(identity),
                Err(e) => {
                    // Prioritize more informative errors
                    // TokenExpired > InvalidJwt > InvalidApiKey
                    if should_replace_error(&last_error, &e) {
                        last_error = Some(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or(AuthError::MissingCredentials))
    }
}
```

### Error Priority

When all providers fail, `MultiProvider` returns the most informative error:

1. `TokenExpired` - User's token is valid but expired
2. `InvalidJwt` - Token looks like a JWT but failed validation
3. `OAuth` - OAuth-specific error
4. `InvalidApiKey` - Generic invalid credentials
5. `MissingCredentials` - Fallback

This helps users understand why authentication failed.

## Existing Provider Implementations

### ApiKeyProvider

Simple hash-based API key validation:

```rust
// src/auth/mod.rs:141-217

impl ApiKeyProvider {
    fn hash_key(key: &str) -> String {
        // SHA-256 hash, base64 encoded
    }

    fn constant_time_compare(a: &str, b: &str) -> bool {
        // Timing-safe comparison to prevent timing attacks
    }
}
```

**Security features:**
- Keys stored as SHA-256 hashes (never plaintext)
- Constant-time comparison prevents timing attacks
- Iterates all keys (no early exit) to prevent enumeration

### JwtProvider

Supports both simple (HS256) and JWKS (RS256/ES256) modes:

```rust
// src/auth/jwt.rs

pub struct JwtProvider {
    mode: JwtMode,
    issuer: Option<String>,
    audience: Option<String>,
    scope_tool_mapping: HashMap<String, Vec<String>>,
    // JWKS cache with TTL
}
```

**Features:**
- JWKS auto-refresh with configurable TTL
- Scope-to-tool mapping for fine-grained authorization
- Background refresh task with `CancellationToken`

### OAuthAuthProvider

Token introspection and userinfo validation:

```rust
// src/auth/oauth.rs

pub struct OAuthAuthProvider {
    config: OAuthConfig,
    introspection_url: Option<String>,
    userinfo_url: Option<String>,
    token_cache: DashMap<String, CachedToken>,  // LRU eviction
}
```

**Features:**
- PKCE support for authorization code flow
- Token caching with TTL
- Fallback to userinfo if introspection unavailable
- Provider presets (GitHub, Google) with auto-configured URLs

### MtlsAuthProvider

Client certificate authentication via reverse proxy headers:

```rust
// src/auth/mtls.rs

pub struct MtlsAuthProvider {
    config: MtlsConfig,
    trusted_proxy_ips: Vec<IpAddr>,
}
```

**Security:**
- Only accepts cert headers from trusted proxy IPs
- Validates `X-Client-Cert-Verified` header
- Extracts identity from CN or SAN fields

## Testing Patterns

### Mock Provider

```rust
// src/mocks.rs

pub struct MockAuthProvider {
    pub should_succeed: bool,
    pub identity: Identity,
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn authenticate(&self, _token: &str) -> Result<Identity, AuthError> {
        if self.should_succeed {
            Ok(self.identity.clone())
        } else {
            Err(AuthError::InvalidApiKey)
        }
    }

    fn name(&self) -> &str {
        "mock"
    }
}
```

### Unit Test Example

```rust
#[tokio::test]
async fn test_api_key_provider_valid_key() {
    let key = "test-api-key-12345";
    let hash = ApiKeyProvider::hash_key(key);

    let config = ApiKeyConfig {
        id: "test-user".to_string(),
        key_hash: hash,
        allowed_tools: vec!["read".to_string()],
        rate_limit: Some(100),
    };

    let provider = ApiKeyProvider::new(vec![config]);
    let result = provider.authenticate(key).await;

    assert!(result.is_ok());
    let identity = result.unwrap();
    assert_eq!(identity.id, "test-user");
    assert_eq!(identity.allowed_tools, Some(vec!["read".to_string()]));
    assert_eq!(identity.rate_limit, Some(100));
}
```

## Best Practices

1. **Return specific errors**: Use `TokenExpired` instead of generic `InvalidJwt` when possible
2. **Log authentication failures**: But never log tokens or secrets
3. **Cache validation results**: For providers that make network calls (JWT JWKS, OAuth introspection)
4. **Implement timeout handling**: External calls should have timeouts
5. **Use constant-time comparison**: For any secret comparison to prevent timing attacks

## Utility Functions

### map_scopes_to_tools

Shared function for mapping OAuth/JWT scopes to tool permissions:

```rust
// src/auth/mod.rs:90-115

pub fn map_scopes_to_tools(
    scopes: &[String],
    scope_tool_mapping: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    if scope_tool_mapping.is_empty() {
        return None; // No mapping = all tools allowed
    }

    let mut tools = Vec::new();
    for scope in scopes {
        if let Some(scope_tools) = scope_tool_mapping.get(scope) {
            if scope_tools.contains(&"*".to_string()) {
                return None; // Wildcard = all tools
            }
            tools.extend(scope_tools.iter().cloned());
        }
    }

    // Deduplicate and sort
    tools.sort();
    tools.dedup();
    Some(tools)
}
```
