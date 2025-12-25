//! OAuth authentication integration tests with mock endpoints
//!
//! Tests OAuth token introspection, userinfo validation, and caching using wiremock.

use std::collections::HashMap;
use wiremock::{
    matchers::{method, path, header, body_string_contains},
    Mock, MockServer, ResponseTemplate,
};

use mcp_guard::{
    auth::{AuthProvider, OAuthAuthProvider},
    config::{OAuthConfig, OAuthProvider},
};

/// Create an OAuth config pointing to a mock server
fn create_oauth_config(mock_server_uri: &str) -> OAuthConfig {
    OAuthConfig {
        provider: OAuthProvider::Custom,
        client_id: "test-client-id".to_string(),
        client_secret: Some("test-client-secret".to_string()),
        authorization_url: Some(format!("{}/authorize", mock_server_uri)),
        token_url: Some(format!("{}/token", mock_server_uri)),
        introspection_url: Some(format!("{}/introspect", mock_server_uri)),
        userinfo_url: Some(format!("{}/userinfo", mock_server_uri)),
        redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
        token_cache_ttl_secs: 300,
    }
}

/// Create an OAuth config without introspection (userinfo fallback only)
fn create_oauth_config_userinfo_only(mock_server_uri: &str) -> OAuthConfig {
    OAuthConfig {
        provider: OAuthProvider::Custom,
        client_id: "test-client-id".to_string(),
        client_secret: None,
        authorization_url: Some(format!("{}/authorize", mock_server_uri)),
        token_url: Some(format!("{}/token", mock_server_uri)),
        introspection_url: None, // No introspection
        userinfo_url: Some(format!("{}/userinfo", mock_server_uri)),
        redirect_uri: "http://localhost:3000/oauth/callback".to_string(),
        scopes: vec!["openid".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
        token_cache_ttl_secs: 300,
    }
}

// =============================================================================
// Token Introspection Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_introspect_token_success() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .and(body_string_contains("token=valid-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "user123",
            "username": "testuser",
            "scope": "read:user write:user"
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("valid-token").await.unwrap();
    
    assert_eq!(identity.id, "user123");
    assert_eq!(identity.name, Some("testuser".to_string()));
}

#[tokio::test]
async fn test_oauth_introspect_token_inactive() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": false
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let result = provider.authenticate("inactive-token").await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_oauth_introspect_http_error_falls_back_to_userinfo() {
    let mock_server = MockServer::start().await;
    
    // Introspection fails
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    
    // UserInfo succeeds
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .and(header("Authorization", "Bearer fallback-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "sub": "fallback-user",
            "name": "Fallback User"
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("fallback-token").await.unwrap();
    
    assert_eq!(identity.id, "fallback-user");
    assert_eq!(identity.name, Some("Fallback User".to_string()));
}

// =============================================================================
// UserInfo Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_userinfo_success() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "sub": "userinfo-user",
            "name": "UserInfo User"
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config_userinfo_only(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("userinfo-token").await.unwrap();
    
    assert_eq!(identity.id, "userinfo-user");
}

#[tokio::test]
async fn test_oauth_userinfo_401_returns_expired() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config_userinfo_only(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let result = provider.authenticate("expired-token").await;
    
    assert!(result.is_err());
    // Should be a TokenExpired error
}

#[tokio::test]
async fn test_oauth_userinfo_500_error() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config_userinfo_only(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let result = provider.authenticate("error-token").await;
    
    assert!(result.is_err());
}

// =============================================================================
// Token Caching Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_token_caching() {
    let mock_server = MockServer::start().await;
    
    // Mock should only be called once due to caching
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "cached-user",
            "username": "CachedUser"
        })))
        .expect(1) // Should only be called once
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    // First call - should hit the mock
    let identity1 = provider.authenticate("cacheable-token").await.unwrap();
    assert_eq!(identity1.id, "cached-user");
    
    // Second call - should use cache
    let identity2 = provider.authenticate("cacheable-token").await.unwrap();
    assert_eq!(identity2.id, "cached-user");
    
    // Third call - still cached
    let identity3 = provider.authenticate("cacheable-token").await.unwrap();
    assert_eq!(identity3.id, "cached-user");
}

#[tokio::test]
async fn test_oauth_different_tokens_not_confused() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .and(body_string_contains("token=token-a"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "user-a"
        })))
        .mount(&mock_server)
        .await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .and(body_string_contains("token=token-b"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "user-b"
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity_a = provider.authenticate("token-a").await.unwrap();
    let identity_b = provider.authenticate("token-b").await.unwrap();
    
    assert_eq!(identity_a.id, "user-a");
    assert_eq!(identity_b.id, "user-b");
}

// =============================================================================
// GitHub-style UserInfo Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_github_userinfo_format() {
    let mock_server = MockServer::start().await;
    
    // GitHub returns numeric ID and login instead of sub/username
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 12345,
            "login": "octocat",
            "name": "The Octocat"
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config_userinfo_only(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("github-token").await.unwrap();
    
    // Should extract numeric ID as string
    assert_eq!(identity.id, "12345");
    // Should use "name" for the display name
    assert_eq!(identity.name, Some("The Octocat".to_string()));
}

// =============================================================================
// Scope to Tool Mapping Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_scope_to_tool_mapping() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "scoped-user",
            "scope": "read:files write:files"
        })))
        .mount(&mock_server)
        .await;
    
    let mut scope_mapping = HashMap::new();
    scope_mapping.insert("read:files".to_string(), vec!["read_file".to_string(), "list_files".to_string()]);
    scope_mapping.insert("write:files".to_string(), vec!["write_file".to_string()]);
    
    let mut config = create_oauth_config(&mock_server.uri());
    config.scope_tool_mapping = scope_mapping;
    
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("scoped-token").await.unwrap();
    
    assert!(identity.allowed_tools.is_some());
    let tools = identity.allowed_tools.unwrap();
    assert!(tools.contains(&"read_file".to_string()));
    assert!(tools.contains(&"list_files".to_string()));
    assert!(tools.contains(&"write_file".to_string()));
}

#[tokio::test]
async fn test_oauth_wildcard_scope() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "admin-user",
            "scope": "admin"
        })))
        .mount(&mock_server)
        .await;
    
    let mut scope_mapping = HashMap::new();
    scope_mapping.insert("admin".to_string(), vec!["*".to_string()]);
    
    let mut config = create_oauth_config(&mock_server.uri());
    config.scope_tool_mapping = scope_mapping;
    
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let identity = provider.authenticate("admin-token").await.unwrap();
    
    // Wildcard means all tools allowed (represented as None)
    assert!(identity.allowed_tools.is_none());
}

// =============================================================================
// Provider Name Test
// =============================================================================

#[test]
fn test_oauth_provider_name() {
    let config = OAuthConfig {
        provider: OAuthProvider::GitHub,
        client_id: "test".to_string(),
        client_secret: None,
        authorization_url: None,
        token_url: None,
        introspection_url: None,
        userinfo_url: None,
        redirect_uri: "http://localhost:3000/callback".to_string(),
        scopes: vec![],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
        token_cache_ttl_secs: 300,
    };

    let provider = OAuthAuthProvider::new(config).unwrap();
    assert_eq!(provider.name(), "oauth");
}

// =============================================================================
// Token Expiration Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_expired_token_in_response() {
    let mock_server = MockServer::start().await;
    
    // Return a token with an expiration in the past
    let past_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64 - 3600; // 1 hour ago
    
    Mock::given(method("POST"))
        .and(path("/introspect"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "active": true,
            "sub": "expired-user",
            "exp": past_timestamp
        })))
        .mount(&mock_server)
        .await;
    
    let config = create_oauth_config(&mock_server.uri());
    let provider = OAuthAuthProvider::new(config).unwrap();
    
    let result = provider.authenticate("past-exp-token").await;
    
    // Should fail due to expiration
    assert!(result.is_err());
}
