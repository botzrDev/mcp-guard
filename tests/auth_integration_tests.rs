use reqwest::StatusCode;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;
use std::fs;

mod common;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    scope: String,
    iss: String,
    aud: String,
}

// Helper specific to this file to spawn server with custom auth config
async fn spawn_auth_server(config_str: &str) -> (std::process::Child, String) {
    let port = common::get_free_port().await;
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("auth_config.toml");
    
    // Inject port
    let mut full_config = config_str.replace("PORT_PLACEHOLDER", &port.to_string());
    
    // Valid upstream needed
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    full_config = full_config.replace("SCRIPT_PATH_PLACEHOLDER", script_path.to_str().unwrap());

    fs::write(&config_path, full_config).unwrap();
    
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("run").arg("--config").arg(config_path.to_str().unwrap());
    
    let child = cmd.spawn().expect("Failed to spawn server");
    let base_url = format!("http://127.0.0.1:{}", port);
    
    if !common::wait_for_server(port).await {
        panic!("Server failed to start");
    }
    
    std::mem::forget(temp_dir);
    (child, base_url)
}

#[tokio::test]
async fn test_api_key_auth_failures() {
    let config = r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[upstream]
transport = "stdio"
command = "/bin/sh"
args = ["SCRIPT_PATH_PLACEHOLDER"]

[[auth.api_keys]]
id = "test"
key_hash = "2bb80d537b1da3e38bd30361aa855686bde0eacd7162fef6a25fe97bf527a25b" # hash of "secret"
"#;
    let (mut child, base_url) = spawn_auth_server(config).await;
    let client = reqwest::Client::new();

    // No header
    let resp = client.post(format!("{}/mcp", base_url)).send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // Wrong key
    let resp = client.post(format!("{}/mcp", base_url))
        .header("Authorization", "Bearer wrong")
        .send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}

#[tokio::test]
async fn test_jwt_auth_hs256() {
    let secret = "my_super_secret_key_at_least_32_chars_long";
    let config = format!(r#"
[server]
host = "127.0.0.1"
port = PORT_PLACEHOLDER

[upstream]
transport = "stdio"
command = "/bin/sh"
args = ["SCRIPT_PATH_PLACEHOLDER"]

[auth.jwt]
mode = "simple"
secret = "{}"
issuer = "test-issuer"
audience = "test-audience"
"#, secret);

    let (mut child, base_url) = spawn_auth_server(&config).await;
    let client = reqwest::Client::new();

    // Generate valid token
    let claims = Claims {
        sub: "user123".to_string(),
        exp: 9999999999,
        scope: "read".to_string(),
        iss: "test-issuer".to_string(),
        aud: "test-audience".to_string(),
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();

    // Test success
    let resp = client.post(format!("{}/mcp", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send().await.unwrap();
        
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Test invalid signature
    let bad_token = encode(&Header::default(), &claims, &EncodingKey::from_secret("wrong_secret".as_bytes())).unwrap();
    let resp = client.post(format!("{}/mcp", base_url))
        .header("Authorization", format!("Bearer {}", bad_token))
        .send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    child.kill().unwrap();
}
