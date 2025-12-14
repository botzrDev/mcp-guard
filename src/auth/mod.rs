//! Authentication providers for mcp-guard

use async_trait::async_trait;
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
        for provider in &self.providers {
            match provider.authenticate(token).await {
                Ok(identity) => return Ok(identity),
                Err(_) => continue,
            }
        }
        Err(AuthError::MissingCredentials)
    }

    fn name(&self) -> &str {
        "multi"
    }
}
