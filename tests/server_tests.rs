use std::time::Duration;
use tokio::time::sleep;
use reqwest::StatusCode;
use std::fs;

mod common;

async fn spawn_server() -> (std::process::Child, String, String) {
    let port = common::get_free_port().await;
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    // Read fixture
    let content = fs::read_to_string("tests/fixtures/valid_config.toml").unwrap();
    
    // Fix paths and port
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    
    // Replace valid config values to make them workable for test
    // Use the script path directly as the command (it has #!/bin/sh shebang and is executable)
    let mut new_config = content.replace("port = 3000", &format!("port = {}", port));
    new_config = new_config.replace("command = \"echo\"", &format!("command = \"{}\"", script_path.display()));
    new_config = new_config.replace("args = [\"hello\"]", "args = []");
    
    // Add an API key for testing
    new_config.push_str("\n[[auth.api_keys]]\n");
    new_config.push_str("id = \"test-client\"\n");
    // SHA256 of "secret" (Base64 encoded)
    let hash = mcp_guard::cli::hash_api_key("secret");
    new_config.push_str(&format!("key_hash = \"{}\"\n", hash));
    
    fs::write(&config_path, new_config).unwrap();
    
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("run").arg("--config").arg(config_path.to_str().unwrap());
    
    let child = cmd.spawn().expect("Failed to spawn server");
    
    let base_url = format!("http://127.0.0.1:{}", port);
    
    // Wait for server to be ready
    if !common::wait_for_server(port).await {
        panic!("Server failed to start");
    }
    
    std::mem::forget(temp_dir);

    (child, base_url, "secret".to_string())
}

#[tokio::test]
async fn test_health_endpoints() {
    let (mut child, base_url, _) = spawn_server().await;
    
    let client = reqwest::Client::new();
    
    // Health
    let resp = client.get(format!("{}/health", base_url)).send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Live
    let resp = client.get(format!("{}/live", base_url)).send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Ready
    let resp = client.get(format!("{}/ready", base_url)).send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Metrics
    let resp = client.get(format!("{}/metrics", base_url)).send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();
    // Check for standard prometheus output header
    assert!(body.contains("# HELP") || body.contains("# TYPE"));

    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_auth_rejection() {
    let (mut child, base_url, _) = spawn_server().await;
    let client = reqwest::Client::new();
    
    let resp = client.post(format!("{}/mcp", base_url))
        .send()
        .await
        .unwrap();
        
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
    child.kill().unwrap();
}

#[tokio::test]
async fn test_mcp_auth_success() {
    let (mut child, base_url, api_key) = spawn_server().await;
    let client = reqwest::Client::new();
    
    let resp = client.post(format!("{}/mcp", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();
        
    // Standard MCP server might not respond to ping if echo server just echoes.
    // Echo server echoes. So we expect 200 OK and body to be echoed.
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.unwrap();
    
    let actual_json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let expected_json: serde_json::Value = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ping",
        "id": 1
    });

    assert_eq!(actual_json, expected_json);
    
    child.kill().unwrap();
}
