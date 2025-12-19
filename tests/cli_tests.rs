use assert_cmd::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;

mod common;

#[test]
fn test_version() {
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mcp-guard"));
}

#[test]
fn test_init_creates_config() {
    let temp = tempfile::tempdir().unwrap();
    // We don't set --path, we just run in the temp dir.
    // The init command writes to mcp-guard.toml in CWD.

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("init")
        .current_dir(&temp)
        .assert()
        .success();

    let config_path = temp.path().join("mcp-guard.toml");
    assert!(config_path.exists());
    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("[server]"));
}

#[test]
fn test_init_fails_if_exists_without_force() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("mcp-guard.toml");
    fs::write(&config_path, "existing content").unwrap();

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("init")
        .current_dir(&temp)
        .assert()
        .failure();
    
    // Content should be unchanged
    let content = fs::read_to_string(config_path.clone()).unwrap();
    assert_eq!(content, "existing content");

    // With force it should succeed
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("init")
        .arg("--force")
        .current_dir(&temp)
        .assert()
        .success();
        
    let content = fs::read_to_string(config_path).unwrap();
    assert!(content.contains("[server]"));
}

#[test]
fn test_validate_valid_config() {
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("validate")
        .arg("--config")
        .arg("tests/fixtures/valid_config.toml")
        .assert()
        .success();
}

#[test]
fn test_validate_invalid_config() {
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("validate")
        .arg("--config")
        .arg("tests/fixtures/invalid_config.toml")
        .assert()
        .failure();
}

#[test]
fn test_keygen() {
    let mut cmd = common::cargo_bin("mcp-guard");
    let output = cmd.arg("keygen")
        .arg("--user-id")
        .arg("test-user")
        .assert()
        .success()
        .get_output()
        .clone();
        
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("API Key"));
    assert!(stdout.contains("key_hash"));
    assert!(stdout.contains("id = \"test-user\""));
}

#[test]
fn test_hash_key() {
    // Generate a key first
    let mut cmd = common::cargo_bin("mcp-guard");
    let output = cmd.arg("keygen")
        .arg("--user-id")
        .arg("test-user")
        .assert()
        .success()
        .get_output()
        .clone();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Extract the key (it's on the line after "API Key...")
    let lines: Vec<&str> = stdout.lines().collect();
    let key_idx = lines.iter().position(|l| l.contains("API Key")).unwrap();
    let key = lines[key_idx + 1].trim();

    // Extract the hash from config snippet
    let hash_line = lines.iter().find(|l| l.contains("key_hash =")).unwrap();
    // content is like:   key_hash = "..."
    let original_hash = hash_line.split('"').nth(1).unwrap();

    // Now hash it back
    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("hash-key")
        .arg(key)
        .assert()
        .success()
        .stdout(predicate::str::contains(original_hash));
}

#[tokio::test]
async fn test_check_upstream_http_success() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::method;

    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let config_content = format!(
        r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#,
        mock_server.uri()
    );

    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("mcp-guard.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("check-upstream")
        .arg("--config")
        .arg(config_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Upstream is reachable"));
}

#[tokio::test]
async fn test_check_upstream_http_failure() {
    // Config pointing to a dead port (hopefully)
    let config_content = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "http://127.0.0.1:1"

[rate_limit]
enabled = false
"#;

    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("mcp-guard.toml");
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = common::cargo_bin("mcp-guard");
    cmd.arg("check-upstream")
        .arg("--config")
        .arg(config_path)
        .arg("--timeout")
        .arg("1") // Fast fail
        .assert()
        .failure()
        .stderr(predicate::str::contains("Upstream check failed"));
}
