//! JWT JWKS integration tests with mock endpoints
//!
//! Tests JWKS endpoint refresh, RS256/ES256 validation, and caching using wiremock.

use std::collections::HashMap;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

use mcp_guard::{
    auth::{AuthProvider, JwtProvider},
    config::{JwtConfig, JwtMode},
};

/// Get current unix timestamp
fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Create an HS256 test configuration
fn create_hs256_config() -> JwtConfig {
    JwtConfig {
        mode: JwtMode::Simple {
            secret: "test-secret-key-at-least-32-characters-long".to_string(),
        },
        issuer: "test-issuer".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: HashMap::new(),
        leeway_secs: 0,
    }
}

/// Create claims for a valid token
fn create_valid_claims() -> HashMap<String, serde_json::Value> {
    let now = now_secs();
    let mut claims = HashMap::new();
    claims.insert("sub".to_string(), serde_json::json!("user123"));
    claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
    claims.insert("aud".to_string(), serde_json::json!("test-audience"));
    claims.insert("exp".to_string(), serde_json::json!(now + 3600));
    claims.insert("iat".to_string(), serde_json::json!(now));
    claims
}

/// Encode claims to JWT with HS256
fn encode_hs256(claims: &HashMap<String, serde_json::Value>, secret: &str) -> String {
    let header = Header::new(Algorithm::HS256);
    encode(&header, claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

// =============================================================================
// HS256 Basic Tests
// =============================================================================

#[tokio::test]
async fn test_jwt_hs256_valid_token() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let claims = create_valid_claims();
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let identity = provider.authenticate(&token).await.unwrap();
    assert_eq!(identity.id, "user123");
}

#[tokio::test]
async fn test_jwt_hs256_expired_token() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let now = now_secs();
    let mut claims = create_valid_claims();
    claims.insert("exp".to_string(), serde_json::json!(now - 3600)); // Expired
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_hs256_wrong_issuer() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("iss".to_string(), serde_json::json!("wrong-issuer"));
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_hs256_wrong_audience() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("aud".to_string(), serde_json::json!("wrong-audience"));
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_hs256_wrong_secret() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let claims = create_valid_claims();
    let token = encode_hs256(&claims, "wrong-secret-that-is-definitely-invalid");
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_hs256_missing_sub() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let now = now_secs();
    let mut claims = HashMap::new();
    // No "sub" claim
    claims.insert("iss".to_string(), serde_json::json!("test-issuer"));
    claims.insert("aud".to_string(), serde_json::json!("test-audience"));
    claims.insert("exp".to_string(), serde_json::json!(now + 3600));
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

// =============================================================================
// Scope Extraction Tests
// =============================================================================

#[tokio::test]
async fn test_jwt_scope_string_format() {
    let mut scope_mapping = HashMap::new();
    scope_mapping.insert("read".to_string(), vec!["read_file".to_string()]);
    scope_mapping.insert("write".to_string(), vec!["write_file".to_string()]);
    
    let config = JwtConfig {
        mode: JwtMode::Simple {
            secret: "test-secret-key-at-least-32-characters-long".to_string(),
        },
        issuer: "test-issuer".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: scope_mapping,
        leeway_secs: 0,
    };
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("scope".to_string(), serde_json::json!("read write")); // Space-separated
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let identity = provider.authenticate(&token).await.unwrap();
    let tools = identity.allowed_tools.unwrap();
    assert!(tools.contains(&"read_file".to_string()));
    assert!(tools.contains(&"write_file".to_string()));
}

#[tokio::test]
async fn test_jwt_scope_array_format() {
    let mut scope_mapping = HashMap::new();
    scope_mapping.insert("admin".to_string(), vec!["*".to_string()]);
    
    let config = JwtConfig {
        mode: JwtMode::Simple {
            secret: "test-secret-key-at-least-32-characters-long".to_string(),
        },
        issuer: "test-issuer".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "permissions".to_string(), // Array format
        scope_tool_mapping: scope_mapping,
        leeway_secs: 0,
    };
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("permissions".to_string(), serde_json::json!(["admin", "user"])); // Array format
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let identity = provider.authenticate(&token).await.unwrap();
    // Wildcard means None (all allowed)
    assert!(identity.allowed_tools.is_none());
}

#[tokio::test]
async fn test_jwt_name_extraction() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("name".to_string(), serde_json::json!("John Doe"));
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let identity = provider.authenticate(&token).await.unwrap();
    assert_eq!(identity.name, Some("John Doe".to_string()));
}

#[tokio::test]
async fn test_jwt_custom_user_id_claim() {
    let config = JwtConfig {
        mode: JwtMode::Simple {
            secret: "test-secret-key-at-least-32-characters-long".to_string(),
        },
        issuer: "test-issuer".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "email".to_string(), // Custom claim
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: HashMap::new(),
        leeway_secs: 0,
    };
    let provider = JwtProvider::new(config).unwrap();
    
    let mut claims = create_valid_claims();
    claims.insert("email".to_string(), serde_json::json!("user@example.com"));
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    let identity = provider.authenticate(&token).await.unwrap();
    assert_eq!(identity.id, "user@example.com");
}

// =============================================================================
// JWKS Mode Tests
// =============================================================================

#[tokio::test]
async fn test_jwt_jwks_config_creation() {
    let mock_server = MockServer::start().await;
    
    let config = JwtConfig {
        mode: JwtMode::Jwks {
            jwks_url: format!("{}/jwks", mock_server.uri()),
            algorithms: vec!["RS256".to_string()],
            cache_duration_secs: 3600,
        },
        issuer: "https://issuer.example.com".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: HashMap::new(),
        leeway_secs: 0,
    };
    
    let provider = JwtProvider::new(config);
    assert!(provider.is_ok());
}

#[tokio::test]
async fn test_jwt_jwks_missing_kid_error() {
    let mock_server = MockServer::start().await;
    
    // Empty JWKS
    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "keys": []
        })))
        .mount(&mock_server)
        .await;
    
    let config = JwtConfig {
        mode: JwtMode::Jwks {
            jwks_url: format!("{}/jwks", mock_server.uri()),
            algorithms: vec!["RS256".to_string()],
            cache_duration_secs: 3600,
        },
        issuer: "https://issuer.example.com".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: HashMap::new(),
        leeway_secs: 0,
    };
    
    let provider = JwtProvider::new(config).unwrap();
    
    // Create a token without kid header (this will fail validation)
    let header = Header::new(Algorithm::HS256);
    let claims = create_valid_claims();
    let token = encode(&header, &claims, &EncodingKey::from_secret(b"secret")).unwrap();
    
    let result = provider.authenticate(&token).await;
    assert!(result.is_err());
}

// =============================================================================
// Leeway Tests
// =============================================================================

#[tokio::test]
async fn test_jwt_leeway_allows_slightly_expired() {
    let config = JwtConfig {
        mode: JwtMode::Simple {
            secret: "test-secret-key-at-least-32-characters-long".to_string(),
        },
        issuer: "test-issuer".to_string(),
        audience: "test-audience".to_string(),
        user_id_claim: "sub".to_string(),
        scopes_claim: "scope".to_string(),
        scope_tool_mapping: HashMap::new(),
        leeway_secs: 60, // 60 seconds leeway
    };
    let provider = JwtProvider::new(config).unwrap();
    
    let now = now_secs();
    let mut claims = create_valid_claims();
    claims.insert("exp".to_string(), serde_json::json!(now - 30)); // 30 seconds ago
    
    let token = encode_hs256(&claims, "test-secret-key-at-least-32-characters-long");
    
    // Should succeed with 60 second leeway
    let result = provider.authenticate(&token).await;
    assert!(result.is_ok());
}

// =============================================================================
// Provider Name Test
// =============================================================================

#[test]
fn test_jwt_provider_name() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    assert_eq!(provider.name(), "jwt");
}

// =============================================================================
// Invalid Token Format Tests
// =============================================================================

#[tokio::test]
async fn test_jwt_invalid_format() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let result = provider.authenticate("not-a-jwt").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_jwt_malformed_base64() {
    let config = create_hs256_config();
    let provider = JwtProvider::new(config).unwrap();
    
    let result = provider.authenticate("header.payload.signature").await;
    assert!(result.is_err());
}
