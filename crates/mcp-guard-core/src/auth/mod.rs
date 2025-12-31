// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.
//! Authentication providers for mcp-guard
//!
//! This module provides pluggable authentication for MCP requests:
//! - API Key: Simple hash-based key validation
//! - JWT: HS256 (simple) or RS256/ES256 (JWKS) token validation
//! - OAuth 2.1: Token introspection and userinfo validation with PKCE
//! - mTLS: Client certificate authentication via reverse proxy headers
//!
//! All providers implement the [`AuthProvider`] trait, allowing them to be
//! combined via [`MultiProvider`] for fallback authentication.

mod jwt;
mod mtls;
mod oauth;

pub use jwt::JwtProvider;
pub use mtls::{
    ClientCertInfo, MtlsAuthProvider, HEADER_CLIENT_CERT_CN, HEADER_CLIENT_CERT_VERIFIED,
};
pub use oauth::OAuthAuthProvider;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Error Types
// ============================================================================

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

// ============================================================================
// Types
// ============================================================================

/// Authenticated identity representing a user or service that has been verified
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

    /// Additional claims/metadata from the authentication token
    pub claims: std::collections::HashMap<String, serde_json::Value>,
}

// ============================================================================
// Utility Functions
// ============================================================================

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

// ============================================================================
// Traits
// ============================================================================

/// Authentication provider trait
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate a request and return the identity
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError>;

    /// Provider name for logging and metrics
    fn name(&self) -> &str;
}

// ============================================================================
// Providers
// ============================================================================

/// API key authentication provider
///
/// Validates requests using pre-shared API keys. Keys are stored as SHA-256
/// hashes to prevent exposure of plaintext keys in configuration.
///
/// SECURITY: Uses constant-time comparison to prevent timing attacks.
pub struct ApiKeyProvider {
    keys: Vec<crate::config::ApiKeyConfig>,
}

impl ApiKeyProvider {
    pub fn new(configs: Vec<crate::config::ApiKeyConfig>) -> Self {
        Self { keys: configs }
    }

    fn hash_key(key: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            hasher.finalize(),
        )
    }

    /// Constant-time comparison of two hash strings.
    ///
    /// SECURITY: Prevents timing attacks by ensuring comparison takes the same
    /// amount of time regardless of where the hashes differ.
    fn constant_time_compare(a: &str, b: &str) -> bool {
        use subtle::ConstantTimeEq;

        // First, compare lengths in constant time
        let len_eq = a.len().ct_eq(&b.len());

        // If lengths match, compare bytes in constant time
        // If lengths differ, still compare to maintain constant time
        let bytes_eq = if a.len() == b.len() {
            a.as_bytes().ct_eq(b.as_bytes())
        } else {
            // Compare with dummy to maintain timing
            let dummy = vec![0u8; a.len()];
            a.as_bytes().ct_eq(&dummy)
        };

        // Both length and content must match
        (len_eq & bytes_eq).into()
    }
}

#[async_trait]
impl AuthProvider for ApiKeyProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        let provided_hash = Self::hash_key(token);

        // SECURITY: Iterate through ALL keys to prevent timing-based enumeration.
        // The loop always runs for the same number of iterations regardless of
        // which key matches (or if any matches at all).
        let mut matched_config: Option<&crate::config::ApiKeyConfig> = None;

        for config in &self.keys {
            if Self::constant_time_compare(&provided_hash, &config.key_hash) {
                matched_config = Some(config);
                // Don't break - continue iterating to maintain constant time
            }
        }

        matched_config
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

/// Combined authentication provider that tries multiple providers in sequence
///
/// Attempts authentication against each configured provider until one succeeds.
/// Returns the most informative error if all providers fail (e.g., prefers
/// "token expired" over "invalid API key").
pub struct MultiProvider {
    /// List of providers to try, in order of precedence
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare_equal() {
        let a = "abc123XYZ";
        let b = "abc123XYZ";
        assert!(ApiKeyProvider::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different_content() {
        let a = "abc123XYZ";
        let b = "abc123XYy"; // Last char different
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different_length() {
        let a = "abc123";
        let b = "abc123XYZ";
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_empty() {
        assert!(ApiKeyProvider::constant_time_compare("", ""));
        assert!(!ApiKeyProvider::constant_time_compare("", "a"));
        assert!(!ApiKeyProvider::constant_time_compare("a", ""));
    }

    #[test]
    fn test_constant_time_compare_first_char_different() {
        let a = "Xbc123XYZ";
        let b = "abc123XYZ";
        assert!(!ApiKeyProvider::constant_time_compare(a, b));
    }

    #[tokio::test]
    async fn test_api_key_provider_valid_key() {
        let key = "test-api-key-12345";
        let hash = ApiKeyProvider::hash_key(key);

        let config = crate::config::ApiKeyConfig {
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
    }

    #[tokio::test]
    async fn test_api_key_provider_invalid_key() {
        let valid_key = "valid-key";
        let hash = ApiKeyProvider::hash_key(valid_key);

        let config = crate::config::ApiKeyConfig {
            id: "test-user".to_string(),
            key_hash: hash,
            allowed_tools: vec![],
            rate_limit: None,
        };

        let provider = ApiKeyProvider::new(vec![config]);
        let result = provider.authenticate("wrong-key").await;

        assert!(matches!(result, Err(AuthError::InvalidApiKey)));
    }
}
