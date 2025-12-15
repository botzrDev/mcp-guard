//! Integration tests for mcp-guard

use mcp_guard::{
    auth::Identity,
    authz::{filter_tools_list_response, is_tools_list_request},
    cli::{generate_api_key, hash_api_key},
    config::{ApiKeyConfig, Config, RateLimitConfig, TracingConfig, TransportType, UpstreamConfig},
    transport::Message,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

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
            servers: vec![],
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
            servers: vec![],
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
            servers: vec![],
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
            servers: vec![],
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
            servers: vec![],
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
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("url"));
}

#[test]
fn test_config_validation_port_zero() {
    use mcp_guard::config::ServerConfig;

    let config = Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0, // Invalid: port 0
            tls: None,
        },
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("port"));
}

#[test]
fn test_config_validation_rate_limit_zero_rps() {
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig {
            enabled: true,
            requests_per_second: 0, // Invalid: zero RPS
            burst_size: 10,
        },
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("requests_per_second"));
}

#[test]
fn test_config_validation_rate_limit_zero_burst() {
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig {
            enabled: true,
            requests_per_second: 100,
            burst_size: 0, // Invalid: zero burst
        },
        audit: Default::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("burst_size"));
}

#[test]
fn test_config_validation_audit_invalid_export_url() {
    use mcp_guard::config::AuditConfig;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: AuditConfig {
            enabled: true,
            file: None,
            stdout: true,
            export_url: Some("not-a-valid-url".to_string()), // Invalid URL
            export_batch_size: 100,
            export_interval_secs: 30,
            export_headers: Default::default(),
        },
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("export_url"));
}

#[test]
fn test_config_validation_audit_zero_batch_size() {
    use mcp_guard::config::AuditConfig;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: AuditConfig {
            enabled: true,
            file: None,
            stdout: true,
            export_url: Some("https://siem.example.com/logs".to_string()),
            export_batch_size: 0, // Invalid: zero batch size
            export_interval_secs: 30,
            export_headers: Default::default(),
        },
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("export_batch_size"));
}

#[test]
fn test_config_validation_tracing_invalid_sample_rate() {
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: Default::default(),
        audit: Default::default(),
        tracing: TracingConfig {
            enabled: true,
            service_name: "test".to_string(),
            otlp_endpoint: None,
            sample_rate: 1.5, // Invalid: > 1.0
            propagate_context: true,
        },
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("sample_rate"));
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

// =============================================================================
// Health Check Endpoint Tests (Sprint 6)
// =============================================================================

#[tokio::test]
async fn test_health_endpoint_response_structure() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    // Create minimal config
    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    // Create minimal app state
    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Test /health endpoint
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "healthy");
    assert!(json["version"].is_string());
    assert!(json["uptime_secs"].is_number());

    // Verify security headers are present
    let headers = {
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        response.headers().clone()
    };

    assert_eq!(
        headers.get("x-content-type-options").map(|v| v.to_str().unwrap()),
        Some("nosniff"),
        "X-Content-Type-Options header should be nosniff"
    );
    assert_eq!(
        headers.get("x-frame-options").map(|v| v.to_str().unwrap()),
        Some("DENY"),
        "X-Frame-Options header should be DENY"
    );
    assert_eq!(
        headers.get("x-xss-protection").map(|v| v.to_str().unwrap()),
        Some("1; mode=block"),
        "X-XSS-Protection header should be 1; mode=block"
    );
    assert_eq!(
        headers.get("content-security-policy").map(|v| v.to_str().unwrap()),
        Some("default-src 'none'"),
        "Content-Security-Policy header should be default-src 'none'"
    );
}

#[tokio::test]
async fn test_live_endpoint() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Test /live endpoint
    let request = Request::builder()
        .uri("/live")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "alive");
}

#[tokio::test]
async fn test_ready_endpoint_when_ready() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)), // Ready = true
        mtls_provider: None,
    });

    let app = build_router(state);

    // Test /ready endpoint when server is ready
    let request = Request::builder()
        .uri("/ready")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["ready"], true);
    assert!(json["version"].is_string());
    assert!(json["reason"].is_null() || json.get("reason").is_none());
}

#[tokio::test]
async fn test_ready_endpoint_when_not_ready() {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(false)), // Ready = false
        mtls_provider: None,
    });

    let app = build_router(state);

    // Test /ready endpoint when server is NOT ready
    let request = Request::builder()
        .uri("/ready")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["ready"], false);
    assert!(json["version"].is_string());
    assert!(json["reason"].is_string());
}

// =============================================================================
// CLI Command Tests (Sprint 6)
// =============================================================================

#[test]
fn test_cli_version_command() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let mut cmd = Command::cargo_bin("mcp-guard").unwrap();
    cmd.arg("version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("mcp-guard"))
        .stdout(predicate::str::contains("1.0.0"))
        .stdout(predicate::str::contains("Build Information"))
        .stdout(predicate::str::contains("Features"))
        .stdout(predicate::str::contains("Auth providers"));
}

#[test]
fn test_cli_help_includes_new_commands() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let mut cmd = Command::cargo_bin("mcp-guard").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("version"))
        .stdout(predicate::str::contains("Show version and build information"))
        .stdout(predicate::str::contains("check-upstream"))
        .stdout(predicate::str::contains("Check upstream MCP server connectivity"));
}

#[test]
fn test_cli_check_upstream_help() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let mut cmd = Command::cargo_bin("mcp-guard").unwrap();
    cmd.args(["check-upstream", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--timeout"))
        .stdout(predicate::str::contains("Timeout in seconds"));
}

#[test]
fn test_cli_check_upstream_missing_config() {
    use assert_cmd::Command;
    use predicates::prelude::*;

    let mut cmd = Command::cargo_bin("mcp-guard").unwrap();
    cmd.args(["--config", "nonexistent.toml", "check-upstream"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error loading config"));
}

// =============================================================================
// OAuth E2E Tests
// =============================================================================

/// Test that /oauth/authorize endpoint returns 401 when OAuth is not configured
/// (route not added, so falls through to auth middleware which rejects unauthenticated requests)
#[tokio::test]
async fn test_oauth_authorize_not_configured() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: None, // OAuth not configured
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // /oauth/authorize returns 401 when OAuth not configured because the route
    // is not added, so requests fall through to auth middleware
    let request = Request::builder()
        .uri("/oauth/authorize")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test OAuth authorize endpoint generates proper redirect with PKCE
#[tokio::test]
async fn test_oauth_authorize_generates_redirect() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::{ApiKeyProvider, OAuthAuthProvider},
        config::{AuditConfig, Config, OAuthConfig, OAuthProvider as OAuthProviderType},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let oauth_config = OAuthConfig {
        provider: OAuthProviderType::GitHub,
        client_id: "test_client_id".to_string(),
        client_secret: Some("test_client_secret".to_string()),
        authorization_url: None, // Uses GitHub default
        token_url: None,
        introspection_url: None,
        userinfo_url: None,
        redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
        scopes: vec!["read:user".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
    };

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let request = Request::builder()
        .uri("/oauth/authorize")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Should redirect to GitHub authorization URL
    // StatusCode 307 (Temporary Redirect) preserves the request method
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response.headers().get("location").unwrap().to_str().unwrap();
    assert!(location.starts_with("https://github.com/login/oauth/authorize"));
    assert!(location.contains("client_id=test_client_id"));
    assert!(location.contains("redirect_uri="));
    assert!(location.contains("state="));
    assert!(location.contains("code_challenge=")); // PKCE
    assert!(location.contains("code_challenge_method=S256"));
}

/// Test OAuth callback rejects missing state parameter
#[tokio::test]
async fn test_oauth_callback_rejects_missing_state() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::{ApiKeyProvider, OAuthAuthProvider},
        config::{AuditConfig, Config, OAuthConfig, OAuthProvider as OAuthProviderType},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let oauth_config = OAuthConfig {
        provider: OAuthProviderType::GitHub,
        client_id: "test_client_id".to_string(),
        client_secret: Some("test_client_secret".to_string()),
        authorization_url: None,
        token_url: None,
        introspection_url: None,
        userinfo_url: None,
        redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
        scopes: vec!["read:user".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
    };

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Callback without state parameter
    let request = Request::builder()
        .uri("/oauth/callback?code=test_code")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test OAuth callback rejects invalid state parameter
#[tokio::test]
async fn test_oauth_callback_rejects_invalid_state() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::{ApiKeyProvider, OAuthAuthProvider},
        config::{AuditConfig, Config, OAuthConfig, OAuthProvider as OAuthProviderType},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let oauth_config = OAuthConfig {
        provider: OAuthProviderType::GitHub,
        client_id: "test_client_id".to_string(),
        client_secret: Some("test_client_secret".to_string()),
        authorization_url: None,
        token_url: None,
        introspection_url: None,
        userinfo_url: None,
        redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
        scopes: vec!["read:user".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
    };

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Callback with invalid/unknown state parameter
    let request = Request::builder()
        .uri("/oauth/callback?code=test_code&state=invalid_state_token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test OAuth callback handles provider errors gracefully
#[tokio::test]
async fn test_oauth_callback_handles_provider_error() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::{ApiKeyProvider, OAuthAuthProvider},
        config::{AuditConfig, Config, OAuthConfig, OAuthProvider as OAuthProviderType},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    };

    let oauth_config = OAuthConfig {
        provider: OAuthProviderType::GitHub,
        client_id: "test_client_id".to_string(),
        client_secret: Some("test_client_secret".to_string()),
        authorization_url: None,
        token_url: None,
        introspection_url: None,
        userinfo_url: None,
        redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
        scopes: vec!["read:user".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
    };

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Callback with error from OAuth provider
    let request = Request::builder()
        .uri("/oauth/callback?error=access_denied&error_description=User+denied+access")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// =============================================================================
// Multi-Server Routing Tests
// =============================================================================

/// Test route matcher finds correct route by prefix
#[test]
fn test_route_matcher_basic() {
    use mcp_guard::config::{ServerRouteConfig, TransportType};
    use mcp_guard::router::RouteMatcher;

    let routes = vec![
        ServerRouteConfig {
            name: "github".to_string(),
            path_prefix: "/github".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8081".to_string()),
            strip_prefix: false,
        },
        ServerRouteConfig {
            name: "filesystem".to_string(),
            path_prefix: "/filesystem".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8082".to_string()),
            strip_prefix: false,
        },
    ];

    let matcher = RouteMatcher::new(&routes);

    assert_eq!(matcher.match_path("/github/repos"), Some("github"));
    assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
    assert_eq!(matcher.match_path("/unknown/path"), None);
}

/// Test route matcher uses longest prefix match
#[test]
fn test_route_matcher_longest_prefix() {
    use mcp_guard::config::{ServerRouteConfig, TransportType};
    use mcp_guard::router::RouteMatcher;

    let routes = vec![
        ServerRouteConfig {
            name: "api".to_string(),
            path_prefix: "/api".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8081".to_string()),
            strip_prefix: false,
        },
        ServerRouteConfig {
            name: "api-v2".to_string(),
            path_prefix: "/api/v2".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8082".to_string()),
            strip_prefix: false,
        },
    ];

    let matcher = RouteMatcher::new(&routes);

    // Longer prefix should win
    assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
    assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
}

/// Test /routes endpoint returns configured routes
#[tokio::test]
async fn test_routes_endpoint_lists_servers() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config, ServerRouteConfig},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        router::ServerRouter,
        server::{build_router, new_oauth_state_store, AppState},
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8081".to_string()),
            servers: vec![
                ServerRouteConfig {
                    name: "github".to_string(),
                    path_prefix: "/github".to_string(),
                    transport: TransportType::Http,
                    command: None,
                    args: vec![],
                    url: Some("http://localhost:8081".to_string()),
                    strip_prefix: false,
                },
                ServerRouteConfig {
                    name: "filesystem".to_string(),
                    path_prefix: "/filesystem".to_string(),
                    transport: TransportType::Http,
                    command: None,
                    args: vec![],
                    url: Some("http://localhost:8082".to_string()),
                    strip_prefix: false,
                },
            ],
        },
    };

    // Create router from server routes
    let server_router = Arc::new(ServerRouter::new(config.upstream.servers.clone()).await.unwrap());

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: None, // Using router instead
        router: Some(server_router),
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let request = Request::builder()
        .uri("/routes")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let routes = json["routes"].as_array().unwrap();
    assert_eq!(routes.len(), 2);

    let route_names: Vec<&str> = routes.iter().map(|r| r.as_str().unwrap()).collect();
    assert!(route_names.contains(&"github"));
    assert!(route_names.contains(&"filesystem"));
}

/// Test /routes endpoint is not available when no multi-server routing
/// (route is only added when in multi-server mode, so falls through to auth)
#[tokio::test]
async fn test_routes_endpoint_unavailable_when_single_server() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use mcp_guard::{
        audit::AuditLogger,
        auth::ApiKeyProvider,
        config::{AuditConfig, Config},
        observability::create_metrics_handle,
        rate_limit::RateLimitService,
        server::{build_router, new_oauth_state_store, AppState},
        transport::StdioTransport,
    };
    use tower::ServiceExt;

    let config = Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![], // No multi-server routing
        },
    };

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None, // No router
        metrics_handle: create_metrics_handle(),
        oauth_provider: None,
        oauth_state_store: new_oauth_state_store(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // /routes endpoint is not available in single-server mode
    // (the route is only added when servers are configured)
    let request = Request::builder()
        .uri("/routes")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Returns 401 because the route doesn't exist and falls through to auth middleware
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test ServerRouteConfig validation
#[test]
fn test_server_route_config_validation() {
    use mcp_guard::config::{ServerRouteConfig, TransportType};

    // Valid config
    let valid = ServerRouteConfig {
        name: "test".to_string(),
        path_prefix: "/test".to_string(),
        transport: TransportType::Http,
        command: None,
        args: vec![],
        url: Some("http://localhost:8080".to_string()),
        strip_prefix: false,
    };
    assert!(valid.validate().is_ok());

    // Invalid: path_prefix doesn't start with /
    let invalid_prefix = ServerRouteConfig {
        name: "test".to_string(),
        path_prefix: "test".to_string(), // Missing leading /
        transport: TransportType::Http,
        command: None,
        args: vec![],
        url: Some("http://localhost:8080".to_string()),
        strip_prefix: false,
    };
    assert!(invalid_prefix.validate().is_err());

    // Invalid: empty name
    let invalid_name = ServerRouteConfig {
        name: "".to_string(), // Empty
        path_prefix: "/test".to_string(),
        transport: TransportType::Http,
        command: None,
        args: vec![],
        url: Some("http://localhost:8080".to_string()),
        strip_prefix: false,
    };
    assert!(invalid_name.validate().is_err());
}
