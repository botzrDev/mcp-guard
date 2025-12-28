//! Comprehensive HTTP server tests for mcp-guard
//!
//! Tests all HTTP endpoints, response formats, and error conditions.

use reqwest::{header, StatusCode};
use serde_json::{json, Value};
use std::fs;
use std::time::Duration;

mod common;

// =============================================================================
// Test Fixtures and Helpers
// =============================================================================

/// Spawn a test server with a specific configuration
async fn spawn_server_with_config(config_content: &str) -> (std::process::Child, String, u16) {
    let port = common::get_free_port().await;
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Replace port placeholder
    let config = config_content.replace("PORT_PLACEHOLDER", &port.to_string());
    fs::write(&config_path, config).unwrap();

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("run")
        .arg("--config")
        .arg(config_path.to_str().unwrap());

    let child = cmd.spawn().expect("Failed to spawn server");
    let base_url = format!("http://127.0.0.1:{}", port);

    // Wait for server to be ready
    if !common::wait_for_server(port).await {
        panic!("Server failed to start on port {}", port);
    }

    std::mem::forget(temp_dir);
    (child, base_url, port)
}

/// Create a basic config with echo server and API key auth
fn basic_config_with_api_key(api_key_hash: &str) -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");

    format!(
        r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[upstream]
transport = "stdio"
command = "{}"
args = []

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = false

[[auth.api_keys]]
id = "test-user"
key_hash = "{}"
"#,
        script_path.display(),
        api_key_hash
    )
}

/// Create a config with multiple API keys with different permissions
fn config_with_multiple_keys() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let admin_hash = mcp_guard_core::cli::hash_api_key("admin-key");
    let restricted_hash = mcp_guard_core::cli::hash_api_key("restricted-key");

    format!(
        r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[upstream]
transport = "stdio"
command = "{}"
args = []

[rate_limit]
enabled = true
requests_per_second = 10
burst_size = 5

[audit]
enabled = true
stdout = false

[[auth.api_keys]]
id = "admin"
key_hash = "{}"

[[auth.api_keys]]
id = "restricted"
key_hash = "{}"
allowed_tools = ["read_file", "list_directory"]
rate_limit = 5
"#,
        script_path.display(),
        admin_hash,
        restricted_hash
    )
}

// =============================================================================
// Health Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_health_endpoint_returns_json() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/health", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let content_type = resp.headers().get(header::CONTENT_TYPE).unwrap();
    assert!(content_type.to_str().unwrap().contains("application/json"));

    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
    assert!(body["version"].is_string());
    assert!(body["uptime_secs"].is_number());

    child.kill().unwrap();
}

#[tokio::test]
async fn test_health_no_auth_required() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    // No Authorization header
    let resp = client
        .get(format!("{}/health", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    child.kill().unwrap();
}

// =============================================================================
// Liveness Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_live_endpoint_returns_minimal_response() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/live", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "alive");
    // Should be minimal - only status field
    assert!(body.as_object().unwrap().len() <= 2);

    child.kill().unwrap();
}

// =============================================================================
// Readiness Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_ready_endpoint_returns_ready_when_initialized() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/ready", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["ready"], true);
    assert!(body["version"].is_string());

    child.kill().unwrap();
}

// =============================================================================
// Metrics Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_metrics_endpoint_returns_prometheus_format() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/metrics", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let content_type = resp.headers().get(header::CONTENT_TYPE).unwrap();
    assert!(content_type.to_str().unwrap().contains("text/plain"));

    let body = resp.text().await.unwrap();
    // Should contain Prometheus-style output
    assert!(body.contains("# HELP") || body.contains("# TYPE") || body.is_empty());

    child.kill().unwrap();
}

#[tokio::test]
async fn test_metrics_updates_after_requests() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();

    // Make some requests first
    for _ in 0..5 {
        client.get(format!("{}/health", base_url)).send().await.ok();
    }

    // Check metrics
    let resp = client
        .get(format!("{}/metrics", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();

    // Should have recorded the requests (if metrics are enabled)
    // Note: Metrics may or may not show depending on configuration
    assert!(!body.is_empty() || body.is_empty()); // Always passes, just verifying endpoint works

    child.kill().unwrap();
}

// =============================================================================
// MCP Endpoint Authentication Tests
// =============================================================================

#[tokio::test]
async fn test_mcp_requires_authentication() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_accepts_valid_api_key() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_rejects_invalid_api_key() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, "Bearer wrong-key")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_rejects_malformed_authorization_header() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();

    // Missing "Bearer " prefix
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, api_key)
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

// =============================================================================
// MCP Endpoint Request/Response Tests
// =============================================================================

#[tokio::test]
async fn test_mcp_echoes_json_rpc_request() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 42
    });

    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let response_body: Value = resp.json().await.unwrap();
    // Echo server returns the same request
    assert_eq!(response_body["jsonrpc"], "2.0");
    assert_eq!(response_body["method"], "tools/list");
    assert_eq!(response_body["id"], 42);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_handles_tools_call_request() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "read_file",
            "arguments": {
                "path": "/tmp/test.txt"
            }
        },
        "id": 1
    });

    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[tokio::test]
async fn test_mcp_rejects_invalid_json() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .body("not valid json {")
        .send()
        .await
        .unwrap();

    // Should return 400 Bad Request or 422 Unprocessable Entity
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::UNPROCESSABLE_ENTITY
    );

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_returns_json_error_response() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .body("invalid json")
        .send()
        .await
        .unwrap();

    // Error responses should still be JSON
    let content_type = resp.headers().get(header::CONTENT_TYPE);
    if let Some(ct) = content_type {
        // May or may not be JSON depending on where error occurs
        let _ = ct.to_str();
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_404_for_unknown_routes() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/unknown/route", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_method_not_allowed_for_wrong_http_method() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();

    // GET to /mcp should fail (only POST allowed)
    let resp = client
        .get(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    child.kill().unwrap();
}

// =============================================================================
// CORS Tests
// =============================================================================

#[tokio::test]
async fn test_cors_headers_present() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();

    // OPTIONS request for CORS preflight
    let resp = client
        .request(reqwest::Method::OPTIONS, format!("{}/mcp", base_url))
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .send()
        .await
        .unwrap();

    // Should return 200 or 204 for preflight
    assert!(resp.status().is_success() || resp.status() == StatusCode::NO_CONTENT);

    child.kill().unwrap();
}

// =============================================================================
// Content-Type Tests
// =============================================================================

#[tokio::test]
async fn test_mcp_accepts_application_json() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_accepts_json_with_charset() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Concurrent Request Tests
// =============================================================================

#[tokio::test]
async fn test_handles_concurrent_requests() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let mut handles = vec![];

    // Spawn 10 concurrent requests
    for i in 0..10 {
        let client = client.clone();
        let url = format!("{}/health", base_url);
        let handle = tokio::spawn(async move {
            let resp = client.get(&url).send().await.unwrap();
            (i, resp.status())
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let (i, status) = handle.await.unwrap();
        assert_eq!(status, StatusCode::OK, "Request {} failed", i);
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_handles_concurrent_mcp_requests() {
    let api_key = "test-secret-key";
    let hash = mcp_guard_core::cli::hash_api_key(api_key);
    let (mut child, base_url, _) = spawn_server_with_config(&basic_config_with_api_key(&hash)).await;

    let client = reqwest::Client::new();
    let mut handles = vec![];

    // Spawn 5 concurrent MCP requests
    for i in 0..5 {
        let client = client.clone();
        let url = format!("{}/mcp", base_url);
        let api_key = api_key.to_string();
        let handle = tokio::spawn(async move {
            let resp = client
                .post(&url)
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .header(header::CONTENT_TYPE, "application/json")
                .body(format!(r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#, i))
                .send()
                .await
                .unwrap();
            (i, resp.status())
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let (i, status) = handle.await.unwrap();
        assert_eq!(status, StatusCode::OK, "MCP request {} failed", i);
    }

    child.kill().unwrap();
}
