//! MCP JSON-RPC handler tests for mcp-guard
//!
//! Tests MCP protocol handling including tools/list, tools/call, authorization,
//! and response filtering.

use reqwest::{header, StatusCode};
use serde_json::{json, Value};
use std::fs;

mod common;

// =============================================================================
// Test Fixtures and Helpers
// =============================================================================

/// Spawn a test server with a specific configuration
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

/// Config with admin and restricted users
fn config_with_user_permissions() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let admin_hash = mcp_guard_core::cli::hash_api_key("admin-key");
    let restricted_hash = mcp_guard_core::cli::hash_api_key("restricted-key");
    let readonly_hash = mcp_guard_core::cli::hash_api_key("readonly-key");

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
id = "admin"
key_hash = "{}"

[[auth.api_keys]]
id = "restricted"
key_hash = "{}"
allowed_tools = ["read_file", "list_directory"]

[[auth.api_keys]]
id = "readonly"
key_hash = "{}"
allowed_tools = ["read_file"]
"#,
        script_path.display(),
        admin_hash,
        restricted_hash,
        readonly_hash
    )
}

/// Send an MCP request and return the response
async fn send_mcp_request(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    request: &Value,
) -> (StatusCode, Value) {
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(request)
        .send()
        .await
        .unwrap();

    let status = resp.status();
    let body: Value = resp.json().await.unwrap_or(json!({}));
    (status, body)
}

// =============================================================================
// JSON-RPC Protocol Tests
// =============================================================================

#[tokio::test]
async fn test_jsonrpc_request_with_id() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 123
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["jsonrpc"], "2.0");
    assert_eq!(body["id"], 123);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_jsonrpc_request_with_string_id() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": "my-request-id"
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], "my-request-id");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_jsonrpc_notification_no_id() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    // Notification (no id field)
    let request = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    // Should be OK - notifications are valid
    assert_eq!(status, StatusCode::OK);
    // Echo server returns the same, which won't have id
    assert!(body.get("id").is_none() || body["id"].is_null());

    child.kill().unwrap();
}

// =============================================================================
// Tools/List Tests
// =============================================================================

#[tokio::test]
async fn test_tools_list_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["method"], "tools/list");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_tools_list_with_cursor() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "params": {
            "cursor": "next-page-token"
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["params"]["cursor"], "next-page-token");

    child.kill().unwrap();
}

// =============================================================================
// Tools/Call Tests
// =============================================================================

#[tokio::test]
async fn test_tools_call_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

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

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["method"], "tools/call");
    assert_eq!(body["params"]["name"], "read_file");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_tools_call_with_complex_arguments() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "search",
            "arguments": {
                "query": "test query",
                "options": {
                    "limit": 10,
                    "offset": 0,
                    "filters": ["type:file", "ext:rs"]
                }
            }
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["params"]["arguments"]["options"]["limit"], 10);

    child.kill().unwrap();
}

// =============================================================================
// Authorization Tests
// =============================================================================

#[tokio::test]
async fn test_admin_can_call_any_tool() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Admin should be able to call any tool
    for tool in ["read_file", "write_file", "execute_command", "delete_file"] {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool,
                "arguments": {}
            },
            "id": 1
        });

        let (status, _) = send_mcp_request(&client, &base_url, "admin-key", &request).await;
        assert_eq!(status, StatusCode::OK, "Admin should be able to call {}", tool);
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_restricted_user_allowed_tools() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Restricted user can call allowed tools
    for tool in ["read_file", "list_directory"] {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool,
                "arguments": {}
            },
            "id": 1
        });

        let (status, _) = send_mcp_request(&client, &base_url, "restricted-key", &request).await;
        assert_eq!(status, StatusCode::OK, "Restricted user should be able to call {}", tool);
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_restricted_user_denied_tools() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Restricted user cannot call unauthorized tools
    for tool in ["write_file", "execute_command", "delete_file"] {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool,
                "arguments": {}
            },
            "id": 1
        });

        let (status, _) = send_mcp_request(&client, &base_url, "restricted-key", &request).await;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "Restricted user should NOT be able to call {}",
            tool
        );
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_readonly_user_single_tool() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Readonly user can only call read_file
    let allowed_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "read_file",
            "arguments": {}
        },
        "id": 1
    });

    let (status, _) = send_mcp_request(&client, &base_url, "readonly-key", &allowed_request).await;
    assert_eq!(status, StatusCode::OK);

    // Cannot call list_directory
    let denied_request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "list_directory",
            "arguments": {}
        },
        "id": 1
    });

    let (status, _) = send_mcp_request(&client, &base_url, "readonly-key", &denied_request).await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    child.kill().unwrap();
}

// =============================================================================
// Initialize/Handshake Tests
// =============================================================================

#[tokio::test]
async fn test_initialize_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["method"], "initialize");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_initialized_notification() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });

    let (status, _) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Resources Tests
// =============================================================================

#[tokio::test]
async fn test_resources_list_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/list",
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["method"], "resources/list");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_resources_read_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "resources/read",
        "params": {
            "uri": "file:///tmp/test.txt"
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["params"]["uri"], "file:///tmp/test.txt");

    child.kill().unwrap();
}

// =============================================================================
// Prompts Tests
// =============================================================================

#[tokio::test]
async fn test_prompts_list_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/list",
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["method"], "prompts/list");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_prompts_get_request() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "prompts/get",
        "params": {
            "name": "greeting",
            "arguments": {
                "name": "World"
            }
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["params"]["name"], "greeting");

    child.kill().unwrap();
}

// =============================================================================
// Error Response Tests
// =============================================================================

#[tokio::test]
async fn test_authorization_error_returns_json_rpc_error() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "forbidden_tool",
            "arguments": {}
        },
        "id": 42
    });

    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, "Bearer readonly-key")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let body: Value = resp.json().await.unwrap();
    // Should have error structure
    assert!(body.get("error").is_some() || body.get("message").is_some());

    child.kill().unwrap();
}

// =============================================================================
// Large Payload Tests
// =============================================================================

#[tokio::test]
async fn test_handles_large_arguments() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Create a large content string (10KB)
    let large_content = "x".repeat(10 * 1024);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "write_file",
            "arguments": {
                "path": "/tmp/large.txt",
                "content": large_content
            }
        },
        "id": 1
    });

    let (status, _) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    // Should succeed (admin has no tool restrictions)
    assert_eq!(status, StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Batch Request Tests (if supported)
// =============================================================================

#[tokio::test]
async fn test_single_request_not_array() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();

    // Single request (not an array) should work
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 1
    });

    let (status, _) = send_mcp_request(&client, &base_url, "admin-key", &request).await;
    assert_eq!(status, StatusCode::OK);

    child.kill().unwrap();
}

// =============================================================================
// Unicode and Special Characters
// =============================================================================

#[tokio::test]
async fn test_handles_unicode_in_arguments() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "search",
            "arguments": {
                "query": "こんにちは世界 🌍 Привет мир"
            }
        },
        "id": 1
    });

    let (status, body) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["params"]["arguments"]["query"], "こんにちは世界 🌍 Привет мир");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_handles_special_characters_in_path() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_permissions()).await;

    let client = reqwest::Client::new();
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "read_file",
            "arguments": {
                "path": "/tmp/file with spaces & special \"chars\".txt"
            }
        },
        "id": 1
    });

    let (status, _) = send_mcp_request(&client, &base_url, "admin-key", &request).await;

    assert_eq!(status, StatusCode::OK);

    child.kill().unwrap();
}
