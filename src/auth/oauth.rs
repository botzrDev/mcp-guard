//! OAuth 2.1 authentication provider for mcp-guard
//!
//! Supports multiple OAuth providers with token validation via:
//! - Token introspection (RFC 7662) for opaque tokens
//! - UserInfo endpoint as fallback

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::auth::{map_scopes_to_tools, AuthError, AuthProvider, Identity};
use crate::config::{OAuthConfig, OAuthProvider as OAuthProviderType};

/// Well-known OAuth provider endpoints
struct ProviderEndpoints {
    authorization_url: &'static str,
    token_url: &'static str,
    userinfo_url: &'static str,
    introspection_url: Option<&'static str>,
}

impl ProviderEndpoints {
    fn for_provider(provider: &OAuthProviderType) -> Option<Self> {
        match provider {
            OAuthProviderType::GitHub => Some(Self {
                authorization_url: "https://github.com/login/oauth/authorize",
                token_url: "https://github.com/login/oauth/access_token",
                userinfo_url: "https://api.github.com/user",
                introspection_url: None, // GitHub doesn't support introspection
            }),
            OAuthProviderType::Google => Some(Self {
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth",
                token_url: "https://oauth2.googleapis.com/token",
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo",
                introspection_url: Some("https://oauth2.googleapis.com/tokeninfo"),
            }),
            OAuthProviderType::Okta => None, // Requires tenant-specific URLs
            OAuthProviderType::Custom => None,
        }
    }
}

/// Token info from introspection or userinfo response
#[derive(Debug, Clone, Default)]
struct TokenInfo {
    active: bool,
    user_id: Option<String>,
    username: Option<String>,
    scopes: Vec<String>,
    expires_at: Option<i64>,
    claims: HashMap<String, serde_json::Value>,
}

/// Token cache duration (5 minutes)
const TOKEN_CACHE_DURATION_SECS: u64 = 300;

/// HTTP request timeout (10 seconds)
const HTTP_REQUEST_TIMEOUT_SECS: u64 = 10;

/// Maximum cache entries before forced cleanup
const CACHE_CLEANUP_THRESHOLD: usize = 100;

/// Maximum cache entries (hard limit)
const CACHE_MAX_ENTRIES: usize = 500;

/// Cached token info to avoid repeated introspection calls
struct TokenCache {
    entries: HashMap<String, CachedToken>,
    cache_duration: Duration,
    insert_count: usize, // Track inserts for periodic cleanup
}

struct CachedToken {
    info: TokenInfo,
    cached_at: Instant,
}

impl TokenCache {
    fn new(cache_duration: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            cache_duration,
            insert_count: 0,
        }
    }

    fn get(&self, token_hash: &str) -> Option<&TokenInfo> {
        self.entries.get(token_hash).and_then(|cached| {
            if cached.cached_at.elapsed() < self.cache_duration {
                Some(&cached.info)
            } else {
                None
            }
        })
    }

    fn insert(&mut self, token_hash: String, info: TokenInfo) {
        self.entries.insert(
            token_hash,
            CachedToken {
                info,
                cached_at: Instant::now(),
            },
        );
        self.insert_count += 1;

        // Periodic cleanup based on insert count
        if self.insert_count >= CACHE_CLEANUP_THRESHOLD {
            self.cleanup_expired();
            self.insert_count = 0;
        }

        // Hard limit - if still too many entries, remove oldest
        if self.entries.len() > CACHE_MAX_ENTRIES {
            self.evict_oldest();
        }
    }

    fn cleanup_expired(&mut self) {
        let before = self.entries.len();
        self.entries
            .retain(|_, cached| cached.cached_at.elapsed() < self.cache_duration);
        let removed = before - self.entries.len();
        if removed > 0 {
            tracing::debug!(removed = removed, remaining = self.entries.len(), "Token cache cleanup");
        }
    }

    /// Remove oldest entries to enforce hard limit
    fn evict_oldest(&mut self) {
        // Collect entries with their ages
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.cached_at))
            .collect();

        // Sort by age (oldest first)
        entries.sort_by(|a, b| a.1.cmp(&b.1));

        // Remove oldest entries until we're under the limit
        let to_remove = self.entries.len() - CACHE_MAX_ENTRIES + 50; // Remove 50 extra to avoid frequent eviction
        for (key, _) in entries.into_iter().take(to_remove) {
            self.entries.remove(&key);
        }

        tracing::debug!(
            removed = to_remove,
            remaining = self.entries.len(),
            "Token cache evicted oldest entries"
        );
    }
}

/// OAuth 2.1 authentication provider
pub struct OAuthAuthProvider {
    config: OAuthConfig,
    authorization_url: String,
    token_url: String,
    userinfo_url: String,
    introspection_url: Option<String>,
    http_client: reqwest::Client,
    token_cache: Arc<RwLock<TokenCache>>,
}

impl OAuthAuthProvider {
    /// Create a new OAuth provider from configuration
    pub fn new(config: OAuthConfig) -> Result<Self, AuthError> {
        // Resolve endpoints from provider type or config
        let endpoints = ProviderEndpoints::for_provider(&config.provider);

        let authorization_url = config
            .authorization_url
            .clone()
            .or_else(|| endpoints.as_ref().map(|e| e.authorization_url.to_string()))
            .ok_or_else(|| {
                AuthError::OAuth("authorization_url required for this provider".into())
            })?;

        let token_url = config
            .token_url
            .clone()
            .or_else(|| endpoints.as_ref().map(|e| e.token_url.to_string()))
            .ok_or_else(|| AuthError::OAuth("token_url required for this provider".into()))?;

        let userinfo_url = config
            .userinfo_url
            .clone()
            .or_else(|| endpoints.as_ref().map(|e| e.userinfo_url.to_string()))
            .ok_or_else(|| AuthError::OAuth("userinfo_url required for this provider".into()))?;

        let introspection_url = config
            .introspection_url
            .clone()
            .or_else(|| endpoints.as_ref().and_then(|e| e.introspection_url.map(String::from)));

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(HTTP_REQUEST_TIMEOUT_SECS))
            .build()
            .map_err(|e| AuthError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        // Cache tokens for 5 minutes by default
        let token_cache = Arc::new(RwLock::new(TokenCache::new(Duration::from_secs(TOKEN_CACHE_DURATION_SECS))));

        Ok(Self {
            config,
            authorization_url,
            token_url,
            userinfo_url,
            introspection_url,
            http_client,
            token_cache,
        })
    }

    /// Get the authorization URL for initiating OAuth flow
    pub fn get_authorization_url(&self, state: &str, code_challenge: Option<&str>) -> String {
        let mut url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            self.authorization_url,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&self.config.scopes.join(" ")),
            urlencoding::encode(state)
        );

        // Add PKCE code_challenge if provided (OAuth 2.1 requires PKCE)
        if let Some(challenge) = code_challenge {
            url.push_str(&format!(
                "&code_challenge={}&code_challenge_method=S256",
                urlencoding::encode(challenge)
            ));
        }

        url
    }

    /// Get the token URL for reference
    pub fn token_url(&self) -> &str {
        &self.token_url
    }

    /// Hash a token for cache key (don't store raw tokens)
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hasher.finalize())
    }

    /// Validate token via introspection endpoint (RFC 7662)
    async fn introspect_token(&self, token: &str) -> Result<TokenInfo, AuthError> {
        let introspection_url = self
            .introspection_url
            .as_ref()
            .ok_or_else(|| AuthError::OAuth("No introspection endpoint configured".into()))?;

        let mut request = self
            .http_client
            .post(introspection_url)
            .form(&[("token", token)]);

        // Add client credentials if available
        if let Some(ref secret) = self.config.client_secret {
            request = request.basic_auth(&self.config.client_id, Some(secret));
        }

        let response = request
            .send()
            .await
            .map_err(|e| AuthError::OAuth(format!("Introspection request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::OAuth(format!(
                "Introspection endpoint returned {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::OAuth(format!("Failed to parse introspection response: {}", e)))?;

        self.parse_token_info(&body)
    }

    /// Validate token via userinfo endpoint
    async fn get_userinfo(&self, token: &str) -> Result<TokenInfo, AuthError> {
        let response = self
            .http_client
            .get(&self.userinfo_url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AuthError::OAuth(format!("UserInfo request failed: {}", e)))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AuthError::TokenExpired);
        }

        if !response.status().is_success() {
            return Err(AuthError::OAuth(format!(
                "UserInfo endpoint returned {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::OAuth(format!("Failed to parse userinfo response: {}", e)))?;

        // UserInfo doesn't have "active" field, so we assume active if we got a response
        let mut info = self.parse_token_info(&body)?;
        info.active = true;
        Ok(info)
    }

    /// Parse token info from JSON response (works for both introspection and userinfo)
    fn parse_token_info(&self, body: &serde_json::Value) -> Result<TokenInfo, AuthError> {
        let active = body.get("active").and_then(|v| v.as_bool()).unwrap_or(true);

        if !active {
            return Ok(TokenInfo {
                active: false,
                ..Default::default()
            });
        }

        // Extract user ID from configured claim
        let user_id = body
            .get(&self.config.user_id_claim)
            .and_then(|v| v.as_str())
            .map(String::from)
            .or_else(|| body.get("sub").and_then(|v| v.as_str()).map(String::from))
            .or_else(|| {
                // GitHub returns "id" as a number
                body.get("id").and_then(|v| v.as_i64()).map(|id| id.to_string())
            })
            .or_else(|| body.get("id").and_then(|v| v.as_str()).map(String::from));

        // Extract username/name
        let username = body
            .get("username")
            .or_else(|| body.get("name"))
            .or_else(|| body.get("login")) // GitHub
            .and_then(|v| v.as_str())
            .map(String::from);

        // Extract scopes
        let scopes = body
            .get("scope")
            .map(|v| match v {
                serde_json::Value::String(s) => s.split_whitespace().map(String::from).collect(),
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect(),
                _ => vec![],
            })
            .unwrap_or_default();

        // Extract expiration
        let expires_at = body.get("exp").and_then(|v| v.as_i64());

        // Convert body to claims map
        let claims = body
            .as_object()
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(TokenInfo {
            active: true,
            user_id,
            username,
            scopes,
            expires_at,
            claims,
        })
    }

    /// Validate token and return info (with caching)
    async fn validate_token(&self, token: &str) -> Result<TokenInfo, AuthError> {
        let token_hash = Self::hash_token(token);

        // Check cache first
        {
            let cache = self.token_cache.read().await;
            if let Some(info) = cache.get(&token_hash) {
                if info.active {
                    return Ok(info.clone());
                } else {
                    return Err(AuthError::TokenExpired);
                }
            }
        }

        // Try introspection first, fall back to userinfo
        let info = if self.introspection_url.is_some() {
            match self.introspect_token(token).await {
                Ok(info) => info,
                Err(_) => self.get_userinfo(token).await?,
            }
        } else {
            self.get_userinfo(token).await?
        };

        // Cache the result (cleanup handled automatically in insert)
        {
            let mut cache = self.token_cache.write().await;
            cache.insert(token_hash, info.clone());
        }

        if !info.active {
            return Err(AuthError::TokenExpired);
        }

        // Check expiration
        if let Some(exp) = info.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            if now > exp {
                return Err(AuthError::TokenExpired);
            }
        }

        Ok(info)
    }
}

#[async_trait]
impl AuthProvider for OAuthAuthProvider {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        let info = self.validate_token(token).await?;

        let user_id = info
            .user_id
            .ok_or_else(|| AuthError::OAuth("No user ID in token info".into()))?;

        let allowed_tools = map_scopes_to_tools(&info.scopes, &self.config.scope_tool_mapping);

        Ok(Identity {
            id: user_id,
            name: info.username,
            allowed_tools,
            rate_limit: None,
            claims: info.claims,
        })
    }

    fn name(&self) -> &str {
        "oauth"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> OAuthConfig {
        OAuthConfig {
            provider: OAuthProviderType::GitHub,
            client_id: "test-client-id".to_string(),
            client_secret: Some("test-secret".to_string()),
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec!["read:user".to_string()],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
        }
    }

    #[test]
    fn test_provider_creation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(provider.name(), "oauth");
    }

    #[test]
    fn test_github_endpoints() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();
        assert_eq!(
            provider.authorization_url,
            "https://github.com/login/oauth/authorize"
        );
        assert_eq!(
            provider.token_url,
            "https://github.com/login/oauth/access_token"
        );
        assert_eq!(provider.userinfo_url, "https://api.github.com/user");
    }

    #[test]
    fn test_authorization_url_generation() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", None);
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
    }

    #[test]
    fn test_authorization_url_with_pkce() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let url = provider.get_authorization_url("test-state", Some("test-challenge"));
        assert!(url.contains("code_challenge=test-challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }

    #[test]
    fn test_custom_provider_requires_urls() {
        let config = OAuthConfig {
            provider: OAuthProviderType::Custom,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None, // Missing required URL
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
        };

        let result = OAuthAuthProvider::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_token_info_introspection() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user repo"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(info.active);
        assert_eq!(info.user_id, Some("user123".to_string()));
        assert_eq!(info.username, Some("testuser".to_string()));
        assert_eq!(info.scopes, vec!["read:user".to_string(), "repo".to_string()]);
    }

    #[test]
    fn test_parse_token_info_github_userinfo() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert_eq!(info.user_id, Some("12345".to_string()));
        assert_eq!(info.username, Some("The Octocat".to_string()));
    }

    #[test]
    fn test_parse_token_info_inactive() {
        let config = create_test_config();
        let provider = OAuthAuthProvider::new(config).unwrap();

        let body = serde_json::json!({
            "active": false
        });

        let info = provider.parse_token_info(&body).unwrap();
        assert!(!info.active);
    }

    #[test]
    fn test_scope_to_tool_mapping() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string()]);
        scope_mapping.insert("write:files".to_string(), vec!["write_file".to_string()]);

        let tools = map_scopes_to_tools(
            &["read:files".to_string(), "write:files".to_string()],
            &scope_mapping,
        );
        assert!(tools.is_some());
        let tools = tools.unwrap();
        assert!(tools.contains(&"read_file".to_string()));
        assert!(tools.contains(&"write_file".to_string()));
    }

    #[test]
    fn test_scope_to_tool_mapping_wildcard() {
        let mut scope_mapping = HashMap::new();
        scope_mapping.insert("admin".to_string(), vec!["*".to_string()]);

        // Wildcard should return None (all tools allowed)
        let tools = map_scopes_to_tools(&["admin".to_string()], &scope_mapping);
        assert!(tools.is_none());
    }

    #[test]
    fn test_token_hash() {
        let hash1 = OAuthAuthProvider::hash_token("test-token-1");
        let hash2 = OAuthAuthProvider::hash_token("test-token-2");
        let hash1_again = OAuthAuthProvider::hash_token("test-token-1");

        assert_ne!(hash1, hash2);
        assert_eq!(hash1, hash1_again);
    }
}
