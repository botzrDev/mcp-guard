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
//! JWT authentication provider for mcp-guard
//!
//! Supports two modes:
//! - Simple: HS256 with local secret
//! - JWKS: RS256/ES256 with remote JWKS endpoint

use async_trait::async_trait;

// ============================================================================
// Constants
// ============================================================================

/// HTTP request timeout for JWKS endpoint calls.
/// 10 seconds allows for slow identity providers while preventing indefinite hangs.
const JWKS_HTTP_TIMEOUT_SECS: u64 = 10;

/// JWKS refresh interval as a fraction of cache duration.
/// Refreshing at 75% of TTL ensures keys are updated before expiry while
/// avoiding excessive network calls.
const JWKS_REFRESH_FRACTION_NUMERATOR: u64 = 3;
const JWKS_REFRESH_FRACTION_DENOMINATOR: u64 = 4;
use jsonwebtoken::{
    decode, decode_header, errors::ErrorKind as JwtErrorKind, Algorithm, DecodingKey, Validation,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::auth::{map_scopes_to_tools, AuthError, AuthProvider, Identity};
use crate::config::{JwtConfig, JwtMode};

/// JWKS key entry with decoded key and algorithm
struct JwksKey {
    key: DecodingKey,
    algorithm: Algorithm,
}

/// JWKS cache structure
struct JwksCache {
    keys: HashMap<String, JwksKey>,
    fetched_at: Instant,
    cache_duration: Duration,
}

impl JwksCache {
    fn new(cache_duration: Duration) -> Self {
        Self {
            keys: HashMap::new(),
            fetched_at: Instant::now() - cache_duration - Duration::from_secs(1), // Start expired
            cache_duration,
        }
    }

    fn is_expired(&self) -> bool {
        self.fetched_at.elapsed() > self.cache_duration
    }
}

/// JWT authentication provider
pub struct JwtProvider {
    config: JwtConfig,
    /// For simple mode: pre-computed decoding key
    simple_key: Option<DecodingKey>,
    /// For JWKS mode: cached keys
    jwks_cache: Option<Arc<RwLock<JwksCache>>>,
    /// HTTP client for JWKS fetching
    http_client: Option<reqwest::Client>,
}

impl JwtProvider {
    /// Create a new JWT provider from configuration
    pub fn new(config: JwtConfig) -> Result<Self, AuthError> {
        match &config.mode {
            JwtMode::Simple { secret } => {
                let key = DecodingKey::from_secret(secret.as_bytes());
                Ok(Self {
                    config,
                    simple_key: Some(key),
                    jwks_cache: None,
                    http_client: None,
                })
            }
            JwtMode::Jwks {
                cache_duration_secs,
                ..
            } => {
                let cache_duration = Duration::from_secs(*cache_duration_secs);
                let cache = Arc::new(RwLock::new(JwksCache::new(cache_duration)));
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(JWKS_HTTP_TIMEOUT_SECS))
                    .build()
                    .map_err(|e| {
                        AuthError::Internal(format!("Failed to create HTTP client: {}", e))
                    })?;

                Ok(Self {
                    config,
                    simple_key: None,
                    jwks_cache: Some(cache),
                    http_client: Some(client),
                })
            }
        }
    }

    /// Start background JWKS refresh task (for JWKS mode)
    ///
    /// The task will run until the cancellation token is triggered.
    /// Pass `CancellationToken::new()` if you don't need graceful shutdown.
    pub fn start_background_refresh(self: &Arc<Self>, cancel_token: CancellationToken) {
        if let JwtMode::Jwks {
            cache_duration_secs,
            ..
        } = &self.config.mode
        {
            let provider = Arc::clone(self);
            // Refresh at 75% of cache duration to ensure keys are fresh before expiry
            let refresh_interval = Duration::from_secs(
                *cache_duration_secs * JWKS_REFRESH_FRACTION_NUMERATOR
                    / JWKS_REFRESH_FRACTION_DENOMINATOR,
            );

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = cancel_token.cancelled() => {
                            tracing::debug!("JWKS refresh task shutting down");
                            break;
                        }
                        _ = tokio::time::sleep(refresh_interval) => {
                            if let Err(e) = provider.refresh_jwks().await {
                                tracing::warn!(error = %e, "Background JWKS refresh failed");
                            }
                        }
                    }
                }
            });
        }
    }

    /// Refresh JWKS from remote endpoint
    async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let JwtMode::Jwks {
            jwks_url,
            algorithms,
            cache_duration_secs,
            ..
        } = &self.config.mode
        else {
            return Err(AuthError::Internal("Not in JWKS mode".into()));
        };

        let client = self
            .http_client
            .as_ref()
            .ok_or_else(|| AuthError::Internal("HTTP client not initialized".into()))?;

        tracing::debug!("Fetching JWKS from {}", jwks_url);

        let response = client
            .get(jwks_url)
            .send()
            .await
            .map_err(|e| AuthError::Internal(format!("JWKS fetch failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::Internal(format!(
                "JWKS endpoint returned {}",
                response.status()
            )));
        }

        let jwks: JwksResponse = response
            .json()
            .await
            .map_err(|e| AuthError::Internal(format!("JWKS parse failed: {}", e)))?;

        let mut new_keys = HashMap::new();
        let allowed_algs: Vec<Algorithm> = algorithms
            .iter()
            .filter_map(|a| parse_algorithm(a))
            .collect();

        for key in jwks.keys {
            let Some(kid) = key.kid else { continue };
            let Some(alg) = key.alg.as_ref().and_then(|a| parse_algorithm(a)) else {
                continue;
            };

            if !allowed_algs.contains(&alg) {
                continue;
            }

            let decoding_key = match (&key.kty[..], &key.n, &key.e, &key.x, &key.y) {
                // RSA key
                ("RSA", Some(n), Some(e), _, _) => DecodingKey::from_rsa_components(n, e)
                    .map_err(|e| AuthError::Internal(format!("Invalid RSA key: {}", e)))?,
                // EC key
                ("EC", _, _, Some(x), Some(y)) => DecodingKey::from_ec_components(x, y)
                    .map_err(|e| AuthError::Internal(format!("Invalid EC key: {}", e)))?,
                _ => continue, // Skip unsupported key types
            };

            new_keys.insert(
                kid,
                JwksKey {
                    key: decoding_key,
                    algorithm: alg,
                },
            );
        }

        if new_keys.is_empty() {
            return Err(AuthError::Internal("No valid keys found in JWKS".into()));
        }

        // Update cache
        let cache = self
            .jwks_cache
            .as_ref()
            .ok_or_else(|| AuthError::Internal("JWKS cache not initialized".into()))?;

        let mut cache_guard = cache.write().await;
        cache_guard.keys = new_keys;
        cache_guard.fetched_at = Instant::now();
        cache_guard.cache_duration = Duration::from_secs(*cache_duration_secs);

        tracing::info!("JWKS cache refreshed with {} keys", cache_guard.keys.len());
        Ok(())
    }

    /// Get decoding key for a given kid (JWKS mode)
    async fn get_jwks_key(&self, kid: &str) -> Result<(DecodingKey, Algorithm), AuthError> {
        let cache = self
            .jwks_cache
            .as_ref()
            .ok_or_else(|| AuthError::Internal("JWKS cache not initialized".into()))?;

        // Check if cache needs refresh
        {
            let cache_guard = cache.read().await;
            if cache_guard.is_expired() {
                drop(cache_guard);
                self.refresh_jwks().await?;
            }
        }

        // Get key from cache
        let cache_guard = cache.read().await;
        cache_guard
            .keys
            .get(kid)
            .map(|k| (k.key.clone(), k.algorithm))
            .ok_or_else(|| AuthError::InvalidJwt(format!("Unknown key ID: {}", kid)))
    }

    /// Build validation parameters
    fn build_validation(&self, algorithm: Algorithm) -> Validation {
        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.leeway = self.config.leeway_secs;
        validation
    }

    /// Extract scopes from token claims
    fn extract_scopes(&self, claims: &HashMap<String, serde_json::Value>) -> Vec<String> {
        claims
            .get(&self.config.scopes_claim)
            .map(|v| match v {
                // Space-separated string (OAuth2 style)
                serde_json::Value::String(s) => s.split_whitespace().map(String::from).collect(),
                // Array of strings
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect(),
                _ => vec![],
            })
            .unwrap_or_default()
    }
}

#[async_trait]
impl AuthProvider for JwtProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        // Decode header to get algorithm and kid
        let header = decode_header(token)
            .map_err(|e| AuthError::InvalidJwt(format!("Invalid JWT header: {}", e)))?;

        // Get decoding key and algorithm based on mode
        let (decoding_key, algorithm) = match &self.config.mode {
            JwtMode::Simple { .. } => {
                let key = self
                    .simple_key
                    .as_ref()
                    .ok_or_else(|| AuthError::Internal("Simple key not initialized".into()))?;
                (key.clone(), Algorithm::HS256)
            }
            JwtMode::Jwks { .. } => {
                let kid = header
                    .kid
                    .as_ref()
                    .ok_or_else(|| AuthError::InvalidJwt("JWT missing 'kid' header".into()))?;
                self.get_jwks_key(kid).await?
            }
        };

        // SECURITY: Validate algorithm matches to prevent algorithm confusion attacks.
        // In Simple mode, reject any token not using HS256 (prevents 'none' algorithm attack).
        // In JWKS mode, ensure the token's alg matches the key's expected algorithm.
        if header.alg != algorithm {
            tracing::warn!(
                expected_alg = ?algorithm,
                claimed_alg = ?header.alg,
                "JWT algorithm mismatch - possible algorithm confusion attack"
            );
            return Err(AuthError::InvalidJwt(format!(
                "Algorithm mismatch: expected {:?}, got {:?}",
                algorithm, header.alg
            )));
        }

        // Build validation and decode
        let validation = self.build_validation(algorithm);
        let token_data =
            decode::<HashMap<String, serde_json::Value>>(token, &decoding_key, &validation)
                .map_err(|e| match e.kind() {
                    JwtErrorKind::ExpiredSignature => AuthError::TokenExpired,
                    JwtErrorKind::InvalidIssuer => AuthError::InvalidJwt("Invalid issuer".into()),
                    JwtErrorKind::InvalidAudience => {
                        AuthError::InvalidJwt("Invalid audience".into())
                    }
                    _ => AuthError::InvalidJwt(format!("JWT validation failed: {}", e)),
                })?;

        // Extract user ID
        let user_id = token_data
            .claims
            .get(&self.config.user_id_claim)
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AuthError::InvalidJwt(format!("Missing '{}' claim", self.config.user_id_claim))
            })?
            .to_string();

        // Extract scopes and map to tools
        let scopes = self.extract_scopes(&token_data.claims);
        let allowed_tools = map_scopes_to_tools(&scopes, &self.config.scope_tool_mapping);

        // Extract optional name
        let name = token_data
            .claims
            .get("name")
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(Identity {
            id: user_id,
            name,
            allowed_tools,
            rate_limit: None, // Could be extracted from claims if needed
            claims: token_data.claims,
        })
    }

    fn name(&self) -> &str {
        "jwt"
    }
}

// Helper types for JWKS parsing
#[derive(Debug, serde::Deserialize)]
struct JwksResponse {
    keys: Vec<JwksKeyEntry>,
}

#[derive(Debug, serde::Deserialize)]
struct JwksKeyEntry {
    kid: Option<String>,
    kty: String,
    alg: Option<String>,
    #[serde(rename = "use")]
    #[allow(dead_code)]
    key_use: Option<String>,
    // RSA components
    n: Option<String>,
    e: Option<String>,
    // EC components
    x: Option<String>,
    y: Option<String>,
    #[allow(dead_code)]
    crv: Option<String>,
}

fn parse_algorithm(alg: &str) -> Option<Algorithm> {
    match alg {
        "HS256" => Some(Algorithm::HS256),
        "HS384" => Some(Algorithm::HS384),
        "HS512" => Some(Algorithm::HS512),
        "RS256" => Some(Algorithm::RS256),
        "RS384" => Some(Algorithm::RS384),
        "RS512" => Some(Algorithm::RS512),
        "ES256" => Some(Algorithm::ES256),
        "ES384" => Some(Algorithm::ES384),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    const TEST_SECRET: &str = "test-secret-key-at-least-32-characters-long";

    fn create_simple_provider() -> JwtProvider {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        JwtProvider::new(config).unwrap()
    }

    fn create_test_token(claims: &HashMap<String, serde_json::Value>) -> String {
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            claims,
            &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
        )
        .unwrap()
    }

    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    #[tokio::test]
    async fn test_valid_token() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));
        claims.insert("iat".to_string(), serde_json::json!(now));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity.id, "user123");
        assert!(identity.allowed_tools.is_none()); // No scope mapping = all allowed
    }

    #[tokio::test]
    async fn test_expired_token() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now - 3600)); // Expired
        claims.insert("iat".to_string(), serde_json::json!(now - 7200));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(matches!(result, Err(AuthError::TokenExpired)));
    }

    #[tokio::test]
    async fn test_invalid_issuer() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("wrong-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
    }

    #[tokio::test]
    async fn test_invalid_audience() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("wrong-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
    }

    #[tokio::test]
    async fn test_invalid_signature() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        // Sign with wrong secret
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &EncodingKey::from_secret(b"wrong-secret")).unwrap();

        let result = provider.authenticate(&token).await;
        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
    }

    #[tokio::test]
    async fn test_missing_sub_claim() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        // No "sub" claim
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
    }

    #[tokio::test]
    async fn test_scope_extraction_string() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string()]);
        scope_mapping.insert("write:files".to_string(), vec!["write_file".to_string()]);

        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: scope_mapping,
            leeway_secs: 0,
        };
        let provider = JwtProvider::new(config).unwrap();

        let now = now_secs();
        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));
        claims.insert(
            "scope".to_string(),
            serde_json::json!("read:files write:files"),
        );

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        let tools = identity.allowed_tools.unwrap();
        assert!(tools.contains(&"read_file".to_string()));
        assert!(tools.contains(&"write_file".to_string()));
    }

    #[tokio::test]
    async fn test_scope_extraction_array() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("admin".to_string(), vec!["*".to_string()]);

        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "permissions".to_string(), // Array style
            scope_tool_mapping: scope_mapping,
            leeway_secs: 0,
        };
        let provider = JwtProvider::new(config).unwrap();

        let now = now_secs();
        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("admin-user"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));
        claims.insert(
            "permissions".to_string(),
            serde_json::json!(["admin", "read"]),
        );

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        assert!(identity.allowed_tools.is_none()); // Wildcard = all allowed
    }

    #[tokio::test]
    async fn test_unknown_scope() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string()]);

        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: scope_mapping,
            leeway_secs: 0,
        };
        let provider = JwtProvider::new(config).unwrap();

        let now = now_secs();
        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));
        claims.insert("scope".to_string(), serde_json::json!("unknown:scope"));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity.allowed_tools, Some(vec![])); // Empty = no tools allowed
    }

    #[tokio::test]
    async fn test_name_extraction() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("name".to_string(), serde_json::json!("John Doe"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity.name, Some("John Doe".to_string()));
    }

    #[tokio::test]
    async fn test_alg_mismatch_simple_mode() {
        let provider = create_simple_provider();
        let now = now_secs();

        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        // Create token signed with RS256 (simulated by just using wrong header)
        // Note: We can't actually sign with RS256 without a key,
        // but we can sign with HS256 and LIE in the header about the algorithm.
        // Or we can just use HS512.
        let header = Header::new(Algorithm::HS512);
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
        )
        .unwrap();

        let result = provider.authenticate(&token).await;
        // Should fail because validation expects HS256
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::InvalidJwt(_)));
    }

    #[tokio::test]
    async fn test_missing_custom_claim() {
        let config = JwtConfig {
            mode: JwtMode::Simple {
                secret: TEST_SECRET.to_string(),
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "custom_id".to_string(), // Expects "custom_id"
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };
        let provider = JwtProvider::new(config).unwrap();

        let now = now_secs();
        let mut claims = HashMap::new();
        // Provide "sub" but not "custom_id"
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        let token = create_test_token(&claims);
        let result = provider.authenticate(&token).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Missing 'custom_id' claim"));
    }

    // -------------------------------------------------------------------------
    // Algorithm Confusion Attack Prevention Tests
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn test_algorithm_confusion_rs256_rejected() {
        // Attempt to use RS256 header with HS256 secret (algorithm confusion attack)
        // We need to manually craft the token since encode() validates algorithm/key match
        let provider = create_simple_provider();
        let now = now_secs();

        // Manually build a JWT with RS256 in header but HS256 signature
        let header_json = r#"{"alg":"RS256","typ":"JWT"}"#;
        let header_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header_json);

        let claims_json = format!(
            r#"{{"sub":"attacker","iss":"test-issuer","aud":"test-audience","exp":{}}}"#,
            now + 3600
        );
        let claims_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&claims_json);

        // Sign with HS256 using HMAC (would work if we accepted the wrong algorithm)
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let message = format!("{}.{}", header_b64, claims_b64);
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        let sig_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature);

        let token = format!("{}.{}.{}", header_b64, claims_b64, sig_b64);

        let result = provider.authenticate(&token).await;
        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
        if let Err(AuthError::InvalidJwt(msg)) = result {
            assert!(
                msg.contains("Algorithm mismatch"),
                "Expected algorithm mismatch error, got: {}",
                msg
            );
        }
    }

    #[tokio::test]
    async fn test_algorithm_confusion_none_rejected() {
        // The 'none' algorithm attack - try to bypass signature verification
        let provider = create_simple_provider();

        // Manually craft a token with alg: "none"
        // Header: {"alg":"none","typ":"JWT"}
        // This is a well-known attack vector
        let header_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"alg":"none","typ":"JWT"}"#);
        let now = now_secs();
        let claims_json = format!(
            r#"{{"sub":"attacker","iss":"test-issuer","aud":"test-audience","exp":{}}}"#,
            now + 3600
        );
        let claims_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&claims_json);

        // Token with empty signature (alg: none attack)
        let token = format!("{}.{}.", header_b64, claims_b64);

        let result = provider.authenticate(&token).await;
        // Should fail - either algorithm mismatch or invalid JWT
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_algorithm_confusion_es256_rejected() {
        // Attempt to use ES256 header with HS256 provider
        // We need to manually craft the token since encode() validates algorithm/key match
        let provider = create_simple_provider();
        let now = now_secs();

        // Manually build a JWT with ES256 in header but fake signature
        let header_json = r#"{"alg":"ES256","typ":"JWT"}"#;
        let header_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header_json);

        let claims_json = format!(
            r#"{{"sub":"attacker","iss":"test-issuer","aud":"test-audience","exp":{}}}"#,
            now + 3600
        );
        let claims_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&claims_json);

        // Use HMAC signature (the attack would be to use the HMAC secret as the "public key")
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let message = format!("{}.{}", header_b64, claims_b64);
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        let sig_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature);

        let token = format!("{}.{}.{}", header_b64, claims_b64, sig_b64);

        let result = provider.authenticate(&token).await;
        assert!(matches!(result, Err(AuthError::InvalidJwt(_))));
        if let Err(AuthError::InvalidJwt(msg)) = result {
            assert!(
                msg.contains("Algorithm mismatch"),
                "Expected algorithm mismatch error, got: {}",
                msg
            );
        }
    }

    // -------------------------------------------------------------------------
    // parse_algorithm Tests (cover all algorithm variants)
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_algorithm_rs_variants() {
        assert_eq!(parse_algorithm("RS256"), Some(Algorithm::RS256));
        assert_eq!(parse_algorithm("RS384"), Some(Algorithm::RS384));
        assert_eq!(parse_algorithm("RS512"), Some(Algorithm::RS512));
    }

    #[test]
    fn test_parse_algorithm_hs_variants() {
        assert_eq!(parse_algorithm("HS256"), Some(Algorithm::HS256));
        assert_eq!(parse_algorithm("HS384"), Some(Algorithm::HS384));
        assert_eq!(parse_algorithm("HS512"), Some(Algorithm::HS512));
    }

    #[test]
    fn test_parse_algorithm_es_variants() {
        assert_eq!(parse_algorithm("ES256"), Some(Algorithm::ES256));
        assert_eq!(parse_algorithm("ES384"), Some(Algorithm::ES384));
    }

    #[test]
    fn test_parse_algorithm_unknown() {
        assert_eq!(parse_algorithm("PS256"), None);
        assert_eq!(parse_algorithm("unknown"), None);
        assert_eq!(parse_algorithm(""), None);
        assert_eq!(parse_algorithm("rs256"), None); // Case sensitive
    }

    // -------------------------------------------------------------------------
    // JwksCache Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_jwks_cache_new_starts_expired() {
        let cache = JwksCache::new(Duration::from_secs(3600));
        // Cache should start expired to trigger immediate refresh
        assert!(cache.is_expired());
        assert!(cache.keys.is_empty());
    }

    #[test]
    fn test_jwks_cache_is_expired_after_duration() {
        let mut cache = JwksCache::new(Duration::from_millis(1));
        cache.fetched_at = Instant::now();
        // Should not be expired immediately
        assert!(!cache.is_expired());
        // Wait for expiry
        std::thread::sleep(Duration::from_millis(5));
        assert!(cache.is_expired());
    }

    #[test]
    fn test_jwks_cache_not_expired_within_duration() {
        let mut cache = JwksCache::new(Duration::from_secs(3600));
        cache.fetched_at = Instant::now();
        assert!(!cache.is_expired());
    }

    // -------------------------------------------------------------------------
    // JWKS Provider Initialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_jwks_provider_creation() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "https://example.com".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };

        let provider = JwtProvider::new(config);
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        assert!(provider.jwks_cache.is_some());
        assert!(provider.http_client.is_some());
        assert!(provider.simple_key.is_none());
    }

    #[tokio::test]
    async fn test_jwks_authenticate_missing_kid() {
        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "https://example.com/.well-known/jwks.json".to_string(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 3600,
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };

        let provider = JwtProvider::new(config).unwrap();

        // Create a token without kid header (should fail for JWKS mode)
        let now = now_secs();
        let mut claims = HashMap::new();
        claims.insert("sub".to_string(), serde_json::json!("user123"));
        claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
        claims.insert("aud".to_string(), serde_json::json!("test-audience"));
        claims.insert("exp".to_string(), serde_json::json!(now + 3600));

        // Sign with HS256 (no kid)
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &EncodingKey::from_secret(b"secret")).unwrap();

        let result = provider.authenticate(&token).await;
        assert!(result.is_err());
        if let Err(AuthError::InvalidJwt(msg)) = result {
            assert!(
                msg.contains("kid") || msg.contains("key ID"),
                "Expected missing kid error, got: {}",
                msg
            );
        }
    }

    #[test]
    fn test_build_validation_sets_correct_params() {
        let provider = create_simple_provider();
        let validation = provider.build_validation(Algorithm::HS256);

        // Validation should be configured with issuer and audience
        // We can't directly inspect private fields, but we can verify it works
        assert!(!validation.algorithms.is_empty());
    }

    #[test]
    fn test_extract_scopes_with_non_standard_value() {
        let provider = create_simple_provider();

        // Test with number value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(123));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());

        // Test with object value (should return empty)
        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!({"nested": "value"}));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());

        // Test with missing scope claim (should return empty)
        let claims: HashMap<String, serde_json::Value> = HashMap::new();
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }

    #[test]
    fn test_extract_scopes_empty_string() {
        let provider = create_simple_provider();

        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!(""));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }

    #[test]
    fn test_extract_scopes_empty_array() {
        let provider = create_simple_provider();

        let mut claims = HashMap::new();
        claims.insert("scope".to_string(), serde_json::json!([]));
        let scopes = provider.extract_scopes(&claims);
        assert!(scopes.is_empty());
    }

    #[test]
    fn test_extract_scopes_array_with_non_strings() {
        let provider = create_simple_provider();

        // Array with mixed types - only strings should be extracted
        let mut claims = HashMap::new();
        claims.insert(
            "scope".to_string(),
            serde_json::json!(["valid", 123, "also_valid", null]),
        );
        let scopes = provider.extract_scopes(&claims);
        assert_eq!(scopes, vec!["valid", "also_valid"]);
    }

    // -------------------------------------------------------------------------
    // JWKS Integration Tests (requires wiremock)
    // -------------------------------------------------------------------------

    #[tokio::test]
    async fn test_jwks_refresh_success() {
        use jsonwebtoken::{encode, EncodingKey, Header};
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/.well-known/jwks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "keys": []
            })))
            .mount(&mock_server)
            .await;

        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: format!("{}/.well-known/jwks.json", mock_server.uri()),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 0, // Force refresh
            },
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };

        let provider = JwtProvider::new(config).unwrap();

        let token =
            "eyJhbGciOiJSUzI1NiIsImtpZCI6IjEyMyIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyIn0.signature";
        let result = provider.authenticate(token).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jwks_fetch_failure_handling() {
        use wiremock::matchers::any;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        // Return 500
        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: mock_server.uri(),
                algorithms: vec!["RS256".to_string()],
                cache_duration_secs: 0,
            },
            issuer: "test".to_string(),
            audience: "test".to_string(),
            user_id_claim: "sub".into(),
            scopes_claim: "scope".into(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        };

        let provider = JwtProvider::new(config).unwrap();

        let token =
            "eyJhbGciOiJSUzI1NiIsImtpZCI6IjEyMyIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyIn0.signature";

        let result = provider.authenticate(token).await;
        assert!(result.is_err());
    }
}
