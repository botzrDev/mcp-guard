//! Multi-server routing tests for mcp-guard
//!
//! Tests path-based routing to multiple upstream MCP servers.
//! These tests require the Enterprise feature to be enabled.

#![cfg(feature = "enterprise")]

use reqwest::{header, StatusCode};
use serde_json::{json, Value};
use std::fs;

mod common;

// =============================================================================
// Test Fixtures and Helpers
// =============================================================================

async fn spawn_server_with_config(config_content: &str) -> (std::process::Child, String, u16) {
    let port = common::get_free_port().await;
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config = config_content.replace("PORT_PLACEHOLDER", &port.to_string());
    fs::write(&config_path, config).unwrap();

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("run")
        .arg("--config")
        .arg(config_path.to_str().unwrap());

    let child = cmd.spawn().expect("Failed to spawn server");
    let base_url = format!("http://127.0.0.1:{}", port);

    if !common::wait_for_server(port).await {
        panic!("Server failed to start on port {}", port);
    }

    std::mem::forget(temp_dir);
    (child, base_url, port)
}

/// Config with multiple upstream servers
fn config_with_multi_server() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let api_key_hash = mcp_guard_core::cli::hash_api_key("test-key");

    format!(
        r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[rate_limit]
enabled = false

[audit]
enabled = true
stdout = false

[[auth.api_keys]]
id = "test-user"
key_hash = "{}"

[[upstream.servers]]
name = "server1"
path_prefix = "/server1"
transport = "stdio"
command = "{}"
args = []

[[upstream.servers]]
name = "server2"
path_prefix = "/server2"
transport = "stdio"
command = "{}"
args = []
"#,
        api_key_hash,
        script_path.display(),
        script_path.display()
    )
}

/// Config with servers having different path prefixes
fn config_with_path_prefixes() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let api_key_hash = mcp_guard_core::cli::hash_api_key("test-key");

    format!(
        r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[rate_limit]
enabled = false

[audit]
enabled = true
stdout = false

[[auth.api_keys]]
id = "test-user"
key_hash = "{}"

[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "{}"
args = []

[[upstream.servers]]
name = "database"
path_prefix = "/db"
transport = "stdio"
command = "{}"
args = []

[[upstream.servers]]
name = "api"
path_prefix = "/api"
transport = "stdio"
command = "{}"
args = []
"#,
        api_key_hash,
        script_path.display(),
        script_path.display(),
        script_path.display()
    )
}

// =============================================================================
// Routes Endpoint Tests
// =============================================================================

#[tokio::test]
async fn test_routes_endpoint_lists_servers() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/routes", base_url))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = resp.json().await.unwrap();

    // Should list the configured servers
    assert!(body.is_array() || body.is_object());
    let routes = if body.is_array() {
        body.as_array().unwrap().clone()
    } else if let Some(routes) = body.get("routes") {
        routes.as_array().unwrap().clone()
    } else {
        vec![]
    };

    // Should have at least 2 routes
    assert!(
        routes.len() >= 2,
        "Should have at least 2 routes configured"
    );

    child.kill().unwrap();
}

#[tokio::test]
async fn test_routes_endpoint_no_auth_required() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    // No Authorization header
    let resp = client
        .get(format!("{}/routes", base_url))
        .send()
        .await
        .unwrap();

    // Routes endpoint should be public (for discovery)
    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Path-Based Routing Tests
// =============================================================================

#[tokio::test]
async fn test_route_to_server1() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    let resp = client
        .post(format!("{}/mcp/server1", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_route_to_server2() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 2
    });

    let resp = client
        .post(format!("{}/mcp/server2", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_route_to_nonexistent_server_returns_404() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    let resp = client
        .post(format!("{}/mcp/nonexistent", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    child.kill().unwrap();
}

// =============================================================================
// Path Prefix Routing Tests
// =============================================================================

#[tokio::test]
async fn test_route_by_path_prefix() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_path_prefixes()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    // Route to filesystem server
    let resp = client
        .post(format!("{}/mcp/fs", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // Route to database server
    let resp = client
        .post(format!("{}/mcp/db", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // Route to api server
    let resp = client
        .post(format!("{}/mcp/api", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Authentication in Multi-Server Mode
// =============================================================================

#[tokio::test]
async fn test_routed_request_requires_auth() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    // No auth header
    let resp = client
        .post(format!("{}/mcp/server1", base_url))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_routed_request_rejects_invalid_auth() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    // Invalid key
    let resp = client
        .post(format!("{}/mcp/server1", base_url))
        .header(header::AUTHORIZATION, "Bearer wrong-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

// =============================================================================
// Tool Calls with Routing
// =============================================================================

#[tokio::test]
async fn test_tool_call_via_routed_server() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let request = json!({
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
        .post(format!("{}/mcp/server1", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["method"], "tools/call");

    child.kill().unwrap();
}

// =============================================================================
// Concurrent Requests to Different Servers
// =============================================================================

#[tokio::test]
async fn test_concurrent_requests_to_different_servers() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let mut handles = vec![];

    // Spawn requests to different servers concurrently
    for i in 0..10 {
        let client = client.clone();
        let url = if i % 2 == 0 {
            format!("{}/mcp/server1", base_url)
        } else {
            format!("{}/mcp/server2", base_url)
        };

        let handle = tokio::spawn(async move {
            let request = json!({
                "jsonrpc": "2.0",
                "method": "tools/list",
                "id": i
            });

            let resp = client
                .post(&url)
                .header(header::AUTHORIZATION, "Bearer test-key")
                .header(header::CONTENT_TYPE, "application/json")
                .json(&request)
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
        assert_eq!(status, StatusCode::OK, "Request {} failed", i);
    }

    child.kill().unwrap();
}

// =============================================================================
// Error Handling in Multi-Server Mode
// =============================================================================

#[tokio::test]
async fn test_invalid_json_to_routed_server() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/mcp/server1", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .body("not valid json")
        .send()
        .await
        .unwrap();

    // Should return 400 or 422
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::UNPROCESSABLE_ENTITY
    );

    child.kill().unwrap();
}

#[tokio::test]
async fn test_method_not_allowed_on_routed_endpoint() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_multi_server()).await;

    let client = reqwest::Client::new();

    // GET to /mcp/server1 should fail (only POST allowed)
    let resp = client
        .get(format!("{}/mcp/server1", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    child.kill().unwrap();
}
