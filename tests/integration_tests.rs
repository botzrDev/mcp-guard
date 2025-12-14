//! Integration tests for mcp-guard

use mcp_guard::{
    cli::{generate_api_key, hash_api_key},
    config::{ApiKeyConfig, Config, RateLimitConfig, TransportType, UpstreamConfig},
};

#[test]
fn test_api_key_generation() {
    let key = generate_api_key();
    assert!(key.starts_with("mcp_"));
    assert!(key.len() > 10);
}

#[test]
fn test_api_key_hashing() {
    let key = "test_key_123";
    let hash1 = hash_api_key(key);
    let hash2 = hash_api_key(key);

    // Same key should produce same hash
    assert_eq!(hash1, hash2);

    // Hash should be base64 encoded
    assert!(base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &hash1
    )
    .is_ok());
}

#[test]
fn test_api_key_different_inputs() {
    let hash1 = hash_api_key("key1");
    let hash2 = hash_api_key("key2");

    // Different keys should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_config_validation_stdio() {
    // Valid stdio config
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
        },
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_stdio_missing_command() {
    // Invalid: stdio without command
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: None,
            args: vec![],
            url: None,
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("command"));
}

#[test]
fn test_config_validation_http_missing_url() {
    // Invalid: http without url
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: None,
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("url"));
}

#[test]
fn test_rate_limit_config_defaults() {
    let config = RateLimitConfig::default();

    assert!(config.enabled);
    assert_eq!(config.requests_per_second, 100);
    assert_eq!(config.burst_size, 50);
}

#[tokio::test]
async fn test_auth_provider_api_key() {
    use mcp_guard::auth::{ApiKeyProvider, AuthProvider};

    let key = "test_secret_key";
    let hash = hash_api_key(key);

    let config = ApiKeyConfig {
        id: "test_user".to_string(),
        key_hash: hash,
        allowed_tools: vec!["read".to_string()],
        rate_limit: Some(50),
    };

    let provider = ApiKeyProvider::new(vec![config]);

    // Valid key should authenticate
    let result = provider.authenticate(key).await;
    assert!(result.is_ok());

    let identity = result.unwrap();
    assert_eq!(identity.id, "test_user");
    assert_eq!(identity.allowed_tools, Some(vec!["read".to_string()]));
    assert_eq!(identity.rate_limit, Some(50));
}

#[tokio::test]
async fn test_auth_provider_invalid_key() {
    use mcp_guard::auth::{ApiKeyProvider, AuthProvider};

    let hash = hash_api_key("correct_key");

    let config = ApiKeyConfig {
        id: "test_user".to_string(),
        key_hash: hash,
        allowed_tools: vec![],
        rate_limit: None,
    };

    let provider = ApiKeyProvider::new(vec![config]);

    // Invalid key should fail
    let result = provider.authenticate("wrong_key").await;
    assert!(result.is_err());
}

#[test]
fn test_authz_tool_authorization() {
    use mcp_guard::auth::Identity;
    use mcp_guard::authz::authorize_tool_call;

    // Unrestricted identity
    let unrestricted = Identity {
        id: "user1".to_string(),
        name: None,
        allowed_tools: None,
        rate_limit: None,
        claims: std::collections::HashMap::new(),
    };

    assert!(authorize_tool_call(&unrestricted, "any_tool"));

    // Restricted identity
    let restricted = Identity {
        id: "user2".to_string(),
        name: None,
        allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
        rate_limit: None,
        claims: std::collections::HashMap::new(),
    };

    assert!(authorize_tool_call(&restricted, "read"));
    assert!(authorize_tool_call(&restricted, "list"));
    assert!(!authorize_tool_call(&restricted, "write"));
    assert!(!authorize_tool_call(&restricted, "delete"));
}

#[test]
fn test_rate_limiter() {
    use mcp_guard::rate_limit::RateLimitService;

    let config = RateLimitConfig {
        enabled: true,
        requests_per_second: 1,
        burst_size: 2,
    };

    let limiter = RateLimitService::new(&config);

    // First two requests should succeed (burst)
    assert!(limiter.check("user1", None));
    assert!(limiter.check("user1", None));

    // Third should be rate limited
    assert!(!limiter.check("user1", None));
}

#[test]
fn test_rate_limiter_disabled() {
    use mcp_guard::rate_limit::RateLimitService;

    let config = RateLimitConfig {
        enabled: false,
        requests_per_second: 1,
        burst_size: 1,
    };

    let limiter = RateLimitService::new(&config);

    // Should never rate limit when disabled
    for _ in 0..100 {
        assert!(limiter.check("user1", None));
    }
}

#[test]
fn test_mcp_message_types() {
    use mcp_guard::transport::Message;

    // Request
    let request = Message::request(1, "tools/call", Some(serde_json::json!({"name": "read"})));
    assert!(request.is_request());
    assert!(!request.is_notification());
    assert!(!request.is_response());

    // Response
    let response = Message::response(serde_json::json!(1), serde_json::json!({"content": "data"}));
    assert!(response.is_response());
    assert!(!response.is_request());
    assert!(!response.is_notification());

    // Error response
    let error = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid request");
    assert!(error.is_response());
    assert!(error.error.is_some());
}
