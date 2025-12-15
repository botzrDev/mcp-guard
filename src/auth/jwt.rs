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
    decode, decode_header, Algorithm, DecodingKey, Validation,
    errors::ErrorKind as JwtErrorKind,
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
            JwtMode::Jwks { cache_duration_secs, .. } => {
                let cache_duration = Duration::from_secs(*cache_duration_secs);
                let cache = Arc::new(RwLock::new(JwksCache::new(cache_duration)));
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(JWKS_HTTP_TIMEOUT_SECS))
                    .build()
                    .map_err(|e| AuthError::Internal(format!("Failed to create HTTP client: {}", e)))?;

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
        if let JwtMode::Jwks { cache_duration_secs, .. } = &self.config.mode {
            let provider = Arc::clone(self);
            // Refresh at 75% of cache duration to ensure keys are fresh before expiry
            let refresh_interval = Duration::from_secs(
                *cache_duration_secs * JWKS_REFRESH_FRACTION_NUMERATOR / JWKS_REFRESH_FRACTION_DENOMINATOR
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
        let JwtMode::Jwks { jwks_url, algorithms, cache_duration_secs, .. } = &self.config.mode else {
            return Err(AuthError::Internal("Not in JWKS mode".into()));
        };

        let client = self.http_client.as_ref()
            .ok_or_else(|| AuthError::Internal("HTTP client not initialized".into()))?;

        tracing::debug!("Fetching JWKS from {}", jwks_url);

        let response = client.get(jwks_url)
            .send()
            .await
            .map_err(|e| AuthError::Internal(format!("JWKS fetch failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::Internal(format!(
                "JWKS endpoint returned {}", response.status()
            )));
        }

        let jwks: JwksResponse = response.json().await
            .map_err(|e| AuthError::Internal(format!("JWKS parse failed: {}", e)))?;

        let mut new_keys = HashMap::new();
        let allowed_algs: Vec<Algorithm> = algorithms.iter()
            .filter_map(|a| parse_algorithm(a))
            .collect();

        for key in jwks.keys {
            let Some(kid) = key.kid else { continue };
            let Some(alg) = key.alg.as_ref().and_then(|a| parse_algorithm(a)) else { continue };

            if !allowed_algs.contains(&alg) {
                continue;
            }

            let decoding_key = match (&key.kty[..], &key.n, &key.e, &key.x, &key.y) {
                // RSA key
                ("RSA", Some(n), Some(e), _, _) => {
                    DecodingKey::from_rsa_components(n, e)
                        .map_err(|e| AuthError::Internal(format!("Invalid RSA key: {}", e)))?
                }
                // EC key
                ("EC", _, _, Some(x), Some(y)) => {
                    DecodingKey::from_ec_components(x, y)
                        .map_err(|e| AuthError::Internal(format!("Invalid EC key: {}", e)))?
                }
                _ => continue, // Skip unsupported key types
            };

            new_keys.insert(kid, JwksKey {
                key: decoding_key,
                algorithm: alg,
            });
        }

        if new_keys.is_empty() {
            return Err(AuthError::Internal("No valid keys found in JWKS".into()));
        }

        // Update cache
        let cache = self.jwks_cache.as_ref()
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
        let cache = self.jwks_cache.as_ref()
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
        cache_guard.keys.get(kid)
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
                serde_json::Value::String(s) => {
                    s.split_whitespace().map(String::from).collect()
                }
                // Array of strings
                serde_json::Value::Array(arr) => {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
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
                let key = self.simple_key.as_ref()
                    .ok_or_else(|| AuthError::Internal("Simple key not initialized".into()))?;
                (key.clone(), Algorithm::HS256)
            }
            JwtMode::Jwks { .. } => {
                let kid = header.kid.as_ref()
                    .ok_or_else(|| AuthError::InvalidJwt("JWT missing 'kid' header".into()))?;
                self.get_jwks_key(kid).await?
            }
        };

        // Validate algorithm matches (for JWKS mode, this ensures the token's alg matches the key's alg)
        if let JwtMode::Jwks { .. } = &self.config.mode {
            if header.alg != algorithm {
                return Err(AuthError::InvalidJwt(format!(
                    "Algorithm mismatch: expected {:?}, got {:?}",
                    algorithm, header.alg
                )));
            }
        }

        // Build validation and decode
        let validation = self.build_validation(algorithm);
        let token_data = decode::<HashMap<String, serde_json::Value>>(
            token,
            &decoding_key,
            &validation,
        ).map_err(|e| {
            match e.kind() {
                JwtErrorKind::ExpiredSignature => AuthError::TokenExpired,
                JwtErrorKind::InvalidIssuer => AuthError::InvalidJwt("Invalid issuer".into()),
                JwtErrorKind::InvalidAudience => AuthError::InvalidJwt("Invalid audience".into()),
                _ => AuthError::InvalidJwt(format!("JWT validation failed: {}", e)),
            }
        })?;

        // Extract user ID
        let user_id = token_data.claims
            .get(&self.config.user_id_claim)
            .and_then(|v| v.as_str())
            .ok_or_else(|| AuthError::InvalidJwt(format!(
                "Missing '{}' claim", self.config.user_id_claim
            )))?
            .to_string();

        // Extract scopes and map to tools
        let scopes = self.extract_scopes(&token_data.claims);
        let allowed_tools = map_scopes_to_tools(&scopes, &self.config.scope_tool_mapping);

        // Extract optional name
        let name = token_data.claims
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
        encode(&header, claims, &EncodingKey::from_secret(TEST_SECRET.as_bytes())).unwrap()
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
        claims.insert("scope".to_string(), serde_json::json!("read:files write:files"));

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
        claims.insert("permissions".to_string(), serde_json::json!(["admin", "read"]));

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
}
