//! Integration tests for mcp-guard

use mcp_guard::{
    auth::Identity,
    authz::{filter_tools_list_response, is_tools_list_request},
    cli::{generate_api_key, hash_api_key},
    config::{ApiKeyConfig, Config, RateLimitConfig, TracingConfig, TransportType, UpstreamConfig},
    transport::Message,
};
use std::collections::HashMap;

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
        tracing: TracingConfig::default(),
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
        tracing: TracingConfig::default(),
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
        tracing: TracingConfig::default(),
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
    assert!(limiter.check("user1", None).allowed);
    assert!(limiter.check("user1", None).allowed);

    // Third should be rate limited
    let result = limiter.check("user1", None);
    assert!(!result.allowed);
    assert!(result.retry_after_secs.is_some());
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
        assert!(limiter.check("user1", None).allowed);
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

#[test]
fn test_metrics_initialization_and_rendering() {
    use mcp_guard::observability::init_metrics;

    // Initialize metrics (can only be done once per process, so this test
    // must be careful). We use a different approach - test that the handle
    // renders valid Prometheus format
    let handle = init_metrics();

    // Render should return valid Prometheus format (even if empty)
    let output = handle.render();
    // Output should be valid text (may be empty if no metrics recorded yet)
    assert!(output.is_empty() || output.contains("# ") || output.contains("mcp_guard"));
}

#[test]
fn test_metrics_prometheus_format() {
    use mcp_guard::observability::{
        record_auth, record_rate_limit, record_request, set_active_identities,
    };

    // Record some metrics (these use the global recorder)
    record_request("POST", 200, std::time::Duration::from_millis(50));
    record_request("GET", 404, std::time::Duration::from_millis(10));
    record_auth("api_key", true);
    record_auth("jwt", false);
    record_rate_limit(true);
    record_rate_limit(false);
    set_active_identities(10);

    // These should not panic even without a recorder installed
    // (metrics crate uses no-op by default)
}

// =============================================================================
// HTTP/SSE Transport Tests (Sprint 5)
// =============================================================================

#[test]
fn test_config_validation_http_valid() {
    // Valid HTTP config
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080/mcp".to_string()),
        },
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_sse_valid() {
    // Valid SSE config
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080/mcp/stream".to_string()),
        },
    };

    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_sse_missing_url() {
    // Invalid: SSE without URL
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Sse,
            command: None,
            args: vec![],
            url: None,
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("url"));
}

// =============================================================================
// Tools/List Filtering Tests (FR-AUTHZ-03)
// =============================================================================

#[test]
fn test_tools_list_request_detection() {
    // tools/list request
    let tools_list = Message {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: Some("tools/list".to_string()),
        params: None,
        result: None,
        error: None,
    };
    assert!(is_tools_list_request(&tools_list));

    // tools/call request (not tools/list)
    let tools_call = Message {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(2)),
        method: Some("tools/call".to_string()),
        params: Some(serde_json::json!({"name": "read_file"})),
        result: None,
        error: None,
    };
    assert!(!is_tools_list_request(&tools_call));

    // Response (no method)
    let response = Message {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: None,
        params: None,
        result: Some(serde_json::json!({})),
        error: None,
    };
    assert!(!is_tools_list_request(&response));
}

#[test]
fn test_tools_list_filtering_integration() {
    // Create a mock tools/list response with multiple tools
    let response = Message {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: None,
        params: None,
        result: Some(serde_json::json!({
            "tools": [
                {
                    "name": "read_file",
                    "description": "Read contents of a file",
                    "inputSchema": {"type": "object", "properties": {"path": {"type": "string"}}}
                },
                {
                    "name": "write_file",
                    "description": "Write contents to a file",
                    "inputSchema": {"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}}
                },
                {
                    "name": "delete_file",
                    "description": "Delete a file",
                    "inputSchema": {"type": "object", "properties": {"path": {"type": "string"}}}
                },
                {
                    "name": "list_directory",
                    "description": "List directory contents",
                    "inputSchema": {"type": "object", "properties": {"path": {"type": "string"}}}
                }
            ]
        })),
        error: None,
    };

    // Identity with only read permissions
    let read_only = Identity {
        id: "read_only_user".to_string(),
        name: Some("Read Only User".to_string()),
        allowed_tools: Some(vec!["read_file".to_string(), "list_directory".to_string()]),
        rate_limit: None,
        claims: HashMap::new(),
    };

    let filtered = filter_tools_list_response(response.clone(), &read_only);
    let result = filtered.result.unwrap();
    let tools = result["tools"].as_array().unwrap();

    // Should only have 2 tools: read_file and list_directory
    assert_eq!(tools.len(), 2);
    let tool_names: Vec<&str> = tools
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    assert!(tool_names.contains(&"read_file"));
    assert!(tool_names.contains(&"list_directory"));
    assert!(!tool_names.contains(&"write_file"));
    assert!(!tool_names.contains(&"delete_file"));

    // Identity with unrestricted access
    let admin = Identity {
        id: "admin".to_string(),
        name: Some("Admin User".to_string()),
        allowed_tools: None, // No restrictions
        rate_limit: None,
        claims: HashMap::new(),
    };

    let unfiltered = filter_tools_list_response(response.clone(), &admin);
    let result = unfiltered.result.unwrap();
    let tools = result["tools"].as_array().unwrap();

    // Should have all 4 tools
    assert_eq!(tools.len(), 4);
}

#[test]
fn test_http_transport_instantiation() {
    use mcp_guard::transport::HttpTransport;

    // Should be able to create HTTP transport
    let transport = HttpTransport::new("http://localhost:8080/mcp".to_string());

    // Transport should implement the Transport trait
    fn _assert_transport<T: mcp_guard::transport::Transport>(_t: &T) {}
    _assert_transport(&transport);
}

#[tokio::test]
async fn test_sse_transport_instantiation() {
    use mcp_guard::transport::SseTransport;

    // Should be able to create SSE transport
    let transport = SseTransport::connect("http://localhost:8080/mcp/stream".to_string())
        .await
        .expect("Should create SSE transport");

    // Transport should implement the Transport trait
    fn _assert_transport<T: mcp_guard::transport::Transport>(_t: &T) {}
    _assert_transport(&transport);
}
