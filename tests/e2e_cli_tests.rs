//! End-to-end CLI tests for mcp-guard binary
//!
//! These tests run the actual binary as a subprocess to verify CLI behavior.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to get the mcp-guard binary command
fn mcp_guard() -> Command {
    Command::cargo_bin("mcp-guard").unwrap()
}

/// Helper to create a temp directory with a valid config file
fn create_temp_config(content: &str) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, content).unwrap();
    (temp_dir, config_path)
}

/// Minimal valid config for testing
const VALID_CONFIG: &str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false
"#;

// =============================================================================
// Version Command Tests
// =============================================================================

#[test]
fn test_version_command() {
    mcp_guard()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")))
        .stdout(predicate::str::contains("mcp-guard"));
}

#[test]
fn test_version_shows_features() {
    mcp_guard()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Auth providers:"))
        .stdout(predicate::str::contains("Transports:"))
        .stdout(predicate::str::contains("Rate limiting:"))
        .stdout(predicate::str::contains("Observability:"));
}

// =============================================================================
// Init Command Tests
// =============================================================================

#[test]
fn test_init_creates_toml() {
    let temp_dir = TempDir::new().unwrap();
    
    mcp_guard()
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration file: mcp-guard.toml"));
    
    assert!(temp_dir.path().join("mcp-guard.toml").exists());
}

#[test]
fn test_init_creates_yaml() {
    let temp_dir = TempDir::new().unwrap();
    
    mcp_guard()
        .arg("init")
        .arg("--format")
        .arg("yaml")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration file: mcp-guard.yaml"));
    
    assert!(temp_dir.path().join("mcp-guard.yaml").exists());
}

#[test]
fn test_init_fails_if_exists_without_force() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("mcp-guard.toml");
    fs::write(&config_path, "existing content").unwrap();
    
    mcp_guard()
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"))
        .stderr(predicate::str::contains("--force"));
}

#[test]
fn test_init_force_overwrites() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("mcp-guard.toml");
    fs::write(&config_path, "old content").unwrap();
    
    mcp_guard()
        .arg("init")
        .arg("--force")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration file"));
    
    // Verify it was overwritten with actual config content
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("[server]") || content.contains("server:"));
}

// =============================================================================
// Validate Command Tests
// =============================================================================

#[test]
fn test_validate_valid_config() {
    let (_temp_dir, config_path) = create_temp_config(VALID_CONFIG);
    
    mcp_guard()
        .arg("validate")
        .arg("-c")
        .arg(&config_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Configuration is valid"));
}

#[test]
fn test_validate_invalid_config() {
    let (_temp_dir, config_path) = create_temp_config("invalid { toml content");
    
    mcp_guard()
        .arg("validate")
        .arg("-c")
        .arg(&config_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Configuration error"));
}

#[test]
fn test_validate_missing_config() {
    mcp_guard()
        .arg("validate")
        .arg("-c")
        .arg("/nonexistent/path/config.toml")
        .assert()
        .failure();
}

#[test]
fn test_validate_incomplete_config() {
    // Config missing required upstream transport
    let incomplete_config = r#"
[server]
host = "127.0.0.1"
port = 3000
"#;
    let (_temp_dir, config_path) = create_temp_config(incomplete_config);
    
    mcp_guard()
        .arg("validate")
        .arg("-c")
        .arg(&config_path)
        .assert()
        .failure();
}

// =============================================================================
// Keygen Command Tests
// =============================================================================

#[test]
fn test_keygen_basic() {
    mcp_guard()
        .arg("keygen")
        .arg("--user-id")
        .arg("test-user")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated API key for 'test-user'"))
        .stdout(predicate::str::contains("API Key (save this"))
        .stdout(predicate::str::contains("key_hash ="));
}

#[test]
fn test_keygen_with_rate_limit() {
    mcp_guard()
        .arg("keygen")
        .arg("--user-id")
        .arg("limited-user")
        .arg("--rate-limit")
        .arg("100")
        .assert()
        .success()
        .stdout(predicate::str::contains("rate_limit = 100"));
}

#[test]
fn test_keygen_with_tools() {
    mcp_guard()
        .arg("keygen")
        .arg("--user-id")
        .arg("tool-user")
        .arg("--tools")
        .arg("read_file,write_file")
        .assert()
        .success()
        .stdout(predicate::str::contains("allowed_tools ="));
}

#[test]
fn test_keygen_full_options() {
    mcp_guard()
        .arg("keygen")
        .arg("--user-id")
        .arg("full-user")
        .arg("--rate-limit")
        .arg("50")
        .arg("--tools")
        .arg("tool1,tool2,tool3")
        .assert()
        .success()
        .stdout(predicate::str::contains("rate_limit = 50"))
        .stdout(predicate::str::contains("allowed_tools ="));
}

// =============================================================================
// Hash-key Command Tests
// =============================================================================

#[test]
fn test_hash_key_basic() {
    let output = mcp_guard()
        .arg("hash-key")
        .arg("my-secret-key")
        .assert()
        .success();
    
    // Should output only the hash (base64 encoded)
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(!stdout.is_empty());
    // Should be a valid base64 string (SHA-256 hash)
    assert!(stdout.trim().len() > 20);
}

#[test]
fn test_hash_key_consistent() {
    // Same input should produce same hash
    let output1 = mcp_guard()
        .arg("hash-key")
        .arg("consistent-key")
        .output()
        .unwrap();
    
    let output2 = mcp_guard()
        .arg("hash-key")
        .arg("consistent-key")
        .output()
        .unwrap();
    
    assert_eq!(output1.stdout, output2.stdout);
}

#[test]
fn test_hash_key_different_inputs() {
    let output1 = mcp_guard()
        .arg("hash-key")
        .arg("key-one")
        .output()
        .unwrap();
    
    let output2 = mcp_guard()
        .arg("hash-key")
        .arg("key-two")
        .output()
        .unwrap();
    
    assert_ne!(output1.stdout, output2.stdout);
}

// =============================================================================
// Check-upstream Command Tests
// =============================================================================

#[test]
fn test_check_upstream_missing_config() {
    mcp_guard()
        .arg("check-upstream")
        .arg("-c")
        .arg("/nonexistent/config.toml")
        .assert()
        .failure();
}

#[test]
fn test_check_upstream_stdio_command_not_found() {
    let config = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/nonexistent/command/that/does/not/exist"
args = []

[rate_limit]
enabled = false
"#;
    let (_temp_dir, config_path) = create_temp_config(config);
    
    mcp_guard()
        .arg("check-upstream")
        .arg("-c")
        .arg(&config_path)
        .arg("--timeout")
        .arg("2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Upstream check failed").or(predicate::str::contains("timed out")));
}

#[test]
fn test_check_upstream_http_invalid_url() {
    let config = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "http://localhost:99999/invalid"

[rate_limit]
enabled = false
"#;
    let (_temp_dir, config_path) = create_temp_config(config);
    
    mcp_guard()
        .arg("check-upstream")
        .arg("-c")
        .arg(&config_path)
        .arg("--timeout")
        .arg("2")
        .assert()
        .failure();
}

#[test]
fn test_check_upstream_sse_invalid_url() {
    let config = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "sse"
url = "http://localhost:99998/sse/invalid"

[rate_limit]
enabled = false
"#;
    let (_temp_dir, config_path) = create_temp_config(config);
    
    mcp_guard()
        .arg("check-upstream")
        .arg("-c")
        .arg(&config_path)
        .arg("--timeout")
        .arg("2")
        .assert()
        .failure();
}

// =============================================================================
// Run Command Tests (basic error cases)
// =============================================================================

#[test]
fn test_run_missing_config() {
    mcp_guard()
        .arg("run")
        .arg("-c")
        .arg("/nonexistent/config.toml")
        .assert()
        .failure();
}

#[test]
fn test_run_invalid_config() {
    let (_temp_dir, config_path) = create_temp_config("not valid toml {");
    
    mcp_guard()
        .arg("run")
        .arg("-c")
        .arg(&config_path)
        .assert()
        .failure();
}

// =============================================================================
// Help Command Tests
// =============================================================================

#[test]
fn test_help_command() {
    mcp_guard()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("mcp-guard"))
        .stdout(predicate::str::contains("run"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("validate"))
        .stdout(predicate::str::contains("keygen"))
        .stdout(predicate::str::contains("check-upstream"));
}

#[test]
fn test_run_help() {
    mcp_guard()
        .arg("run")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--host"))
        .stdout(predicate::str::contains("--port"))
        .stdout(predicate::str::contains("--config"));
}

#[test]
fn test_keygen_help() {
    mcp_guard()
        .arg("keygen")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--user-id"))
        .stdout(predicate::str::contains("--rate-limit"))
        .stdout(predicate::str::contains("--tools"));
}

// =============================================================================
// Verbose Flag Tests
// =============================================================================

#[test]
fn test_verbose_flag_validate() {
    let (_temp_dir, config_path) = create_temp_config(VALID_CONFIG);
    
    mcp_guard()
        .arg("-v")
        .arg("validate")
        .arg("-c")
        .arg(&config_path)
        .assert()
        .success();
}

// Note: CLI does not support multiple -v flags (-vv), only single -v
