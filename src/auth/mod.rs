//! Authentication providers for mcp-guard

mod jwt;
mod mtls;
mod oauth;

pub use jwt::JwtProvider;
pub use mtls::{ClientCertInfo, MtlsAuthProvider, HEADER_CLIENT_CERT_CN, HEADER_CLIENT_CERT_VERIFIED};
pub use oauth::OAuthAuthProvider;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Authentication error type
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authentication credentials")]
    MissingCredentials,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid JWT: {0}")]
    InvalidJwt(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("Invalid client certificate: {0}")]
    InvalidClientCert(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Authenticated identity
#[derive(Debug, Clone)]
pub struct Identity {
    /// Unique identifier for the user/service
    pub id: String,

    /// Display name
    pub name: Option<String>,

    /// Allowed tools (None means all allowed)
    pub allowed_tools: Option<Vec<String>>,

    /// Custom rate limit for this identity
    pub rate_limit: Option<u32>,

    /// Additional claims/metadata
    pub claims: std::collections::HashMap<String, serde_json::Value>,
}

/// Map OAuth/JWT scopes to allowed tools based on a scope-to-tool mapping
///
/// # Arguments
/// * `scopes` - List of scopes from the token
/// * `scope_tool_mapping` - Mapping from scope names to tool names
///
/// # Returns
/// * `None` - No restrictions (empty mapping or wildcard "*" scope)
/// * `Some(vec![])` - No tools allowed (scopes not in mapping)
/// * `Some(tools)` - Specific tools allowed
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

    if tools.is_empty() {
        Some(vec![]) // Empty = no tools allowed (scope not in mapping)
    } else {
        tools.sort();
        tools.dedup();
        Some(tools)
    }
}

/// Authentication provider trait
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate a request and return the identity
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError>;

    /// Provider name for logging
    fn name(&self) -> &str;
}

/// API key authentication provider
pub struct ApiKeyProvider {
    keys: std::collections::HashMap<String, crate::config::ApiKeyConfig>,
}

impl ApiKeyProvider {
    pub fn new(configs: Vec<crate::config::ApiKeyConfig>) -> Self {
        let keys = configs
            .into_iter()
            .map(|c| (c.key_hash.clone(), c))
            .collect();
        Self { keys }
    }

    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
    }
}

#[async_trait]
impl AuthProvider for ApiKeyProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        let hash = Self::hash_key(token);

        self.keys
            .get(&hash)
            .map(|config| Identity {
                id: config.id.clone(),
                name: Some(config.id.clone()),
                allowed_tools: if config.allowed_tools.is_empty() {
                    None
                } else {
                    Some(config.allowed_tools.clone())
                },
                rate_limit: config.rate_limit,
                claims: std::collections::HashMap::new(),
            })
            .ok_or(AuthError::InvalidApiKey)
    }

    fn name(&self) -> &str {
        "api_key"
    }
}

/// Combined authentication provider that tries multiple providers
pub struct MultiProvider {
    providers: Vec<Arc<dyn AuthProvider>>,
}

impl MultiProvider {
    pub fn new(providers: Vec<Arc<dyn AuthProvider>>) -> Self {
        Self { providers }
    }
}

#[async_trait]
impl AuthProvider for MultiProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        if self.providers.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        let mut last_error: Option<AuthError> = None;

        for provider in &self.providers {
            match provider.authenticate(token).await {
                Ok(identity) => return Ok(identity),
                Err(e) => {
                    // Prioritize more informative errors
                    let should_replace = match (&last_error, &e) {
                        (None, _) => true,
                        // Token expired is more specific than generic errors
                        (Some(AuthError::InvalidApiKey), AuthError::TokenExpired) => true,
                        (Some(AuthError::InvalidApiKey), AuthError::InvalidJwt(_)) => true,
                        (Some(AuthError::InvalidApiKey), AuthError::OAuth(_)) => true,
                        (Some(AuthError::MissingCredentials), _) => true,
                        // Keep the current error in other cases
                        _ => false,
                    };

                    if should_replace {
                        last_error = Some(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or(AuthError::MissingCredentials))
    }

    fn name(&self) -> &str {
        "multi"
    }
}
