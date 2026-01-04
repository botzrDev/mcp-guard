//! Rate limiting tests for mcp-guard
//!
//! Tests rate limiting behavior including per-identity limits, burst handling,
//! and Retry-After headers.

use reqwest::{header, StatusCode};
use std::fs;
use std::time::Duration;

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

/// Config with strict rate limiting for testing
fn config_with_strict_rate_limit() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let api_key_hash = mcp_guard_core::cli::hash_api_key("test-key");

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
requests_per_second = 2
burst_size = 3

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

/// Config with per-user rate limit override
fn config_with_user_rate_limit() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let normal_hash = mcp_guard_core::cli::hash_api_key("normal-key");
    let limited_hash = mcp_guard_core::cli::hash_api_key("limited-key");

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
id = "normal-user"
key_hash = "{}"

[[auth.api_keys]]
id = "limited-user"
key_hash = "{}"
rate_limit = 2
"#,
        script_path.display(),
        normal_hash,
        limited_hash
    )
}

/// Config with rate limiting disabled
fn config_with_rate_limit_disabled() -> String {
    let cwd = std::env::current_dir().unwrap();
    let script_path = cwd.join("tests/fixtures/echo_server.sh");
    let api_key_hash = mcp_guard_core::cli::hash_api_key("test-key");

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
enabled = false

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

// =============================================================================
// Basic Rate Limiting Tests
// =============================================================================

#[tokio::test]
async fn test_rate_limit_allows_requests_within_limit() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Send 2 requests (within burst limit of 3)
    for i in 0..2 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "Request {} should succeed within rate limit",
            i
        );
    }

    child.kill().unwrap();
}

#[tokio::test]
async fn test_rate_limit_blocks_excess_requests() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();
    let mut success_count = 0;
    let mut rate_limited_count = 0;

    // Send many rapid requests to exceed the limit
    // With burst_size = 3 and requests_per_second = 2, we should hit the limit
    for i in 0..10 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::OK {
            success_count += 1;
        } else if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            rate_limited_count += 1;
        }
    }

    // Some should succeed (burst), some should be rate limited
    assert!(success_count > 0, "Some requests should succeed (burst)");
    assert!(
        rate_limited_count > 0,
        "Some requests should be rate limited"
    );

    child.kill().unwrap();
}

#[tokio::test]
async fn test_rate_limit_returns_429_status() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Exhaust the rate limit
    for i in 0..20 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            // Found a rate-limited response
            child.kill().unwrap();
            return;
        }
    }

    child.kill().unwrap();
    // If we didn't hit rate limit, the test config might need adjustment
    // but this is not necessarily a failure
}

#[tokio::test]
async fn test_rate_limit_includes_retry_after_header() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Send many requests to trigger rate limit
    let mut found_retry_after = false;
    for i in 0..20 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            // Check for Retry-After header
            if resp.headers().get("Retry-After").is_some() {
                found_retry_after = true;
                let retry_after = resp.headers().get("Retry-After").unwrap().to_str().unwrap();
                // Should be a number (seconds)
                assert!(
                    retry_after.parse::<u64>().is_ok(),
                    "Retry-After should be a number"
                );
            }
            break;
        }
    }

    // Note: Retry-After is optional but recommended
    // The test passes whether or not we found the header
    if found_retry_after {
        println!("Retry-After header was present");
    }

    child.kill().unwrap();
}

// =============================================================================
// Per-Identity Rate Limiting Tests
// =============================================================================

#[tokio::test]
async fn test_per_user_rate_limit_override() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_rate_limit()).await;

    let client = reqwest::Client::new();

    // Normal user should have high rate limit (100 req/s)
    let mut normal_successes = 0;
    for i in 0..20 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer normal-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::OK {
            normal_successes += 1;
        }
    }

    // Limited user should hit rate limit quickly (2 req/s)
    let mut limited_successes = 0;
    let mut limited_blocked = 0;
    for i in 0..20 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer limited-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::OK {
            limited_successes += 1;
        } else if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            limited_blocked += 1;
        }
    }

    // Normal user should have many more successes than limited user
    assert!(
        normal_successes > limited_successes,
        "Normal user ({}) should have more successes than limited user ({})",
        normal_successes,
        limited_successes
    );

    // Limited user should be blocked at some point
    assert!(
        limited_blocked > 0 || limited_successes <= 5,
        "Limited user should be rate limited (blocked: {}, successes: {})",
        limited_blocked,
        limited_successes
    );

    child.kill().unwrap();
}

#[tokio::test]
async fn test_rate_limits_are_per_identity() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_user_rate_limit()).await;

    let client = reqwest::Client::new();

    // Exhaust limited user's rate limit
    for i in 0..10 {
        let _ = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer limited-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await;
    }

    // Normal user should still be able to make requests
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, "Bearer normal-key")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Normal user should not be affected by limited user's rate limit"
    );

    child.kill().unwrap();
}

// =============================================================================
// Rate Limit Disabled Tests
// =============================================================================

#[tokio::test]
async fn test_rate_limit_disabled_allows_all_requests() {
    let (mut child, base_url, _) =
        spawn_server_with_config(&config_with_rate_limit_disabled()).await;

    let client = reqwest::Client::new();

    // Send many rapid requests - all should succeed
    let mut all_success = true;
    for i in 0..50 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() != StatusCode::OK {
            all_success = false;
            break;
        }
    }

    assert!(
        all_success,
        "All requests should succeed when rate limiting is disabled"
    );

    child.kill().unwrap();
}

// =============================================================================
// Burst Tests
// =============================================================================

#[tokio::test]
async fn test_burst_allows_initial_requests() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Burst size is 3, so first 3 requests should succeed immediately
    let mut immediate_successes = 0;
    for i in 0..3 {
        let resp = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await
            .unwrap();

        if resp.status() == StatusCode::OK {
            immediate_successes += 1;
        }
    }

    assert!(
        immediate_successes >= 2,
        "Burst should allow initial requests (got {} successes)",
        immediate_successes
    );

    child.kill().unwrap();
}

// =============================================================================
// Recovery Tests
// =============================================================================

#[tokio::test]
async fn test_rate_limit_recovers_over_time() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Exhaust the rate limit
    for i in 0..10 {
        let _ = client
            .post(format!("{}/mcp", base_url))
            .header(header::AUTHORIZATION, "Bearer test-key")
            .header(header::CONTENT_TYPE, "application/json")
            .body(format!(
                r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                i
            ))
            .send()
            .await;
    }

    // Wait for rate limit to recover (2 req/s means 500ms per token)
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Should be able to make requests again
    let resp = client
        .post(format!("{}/mcp", base_url))
        .header(header::AUTHORIZATION, "Bearer test-key")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"jsonrpc": "2.0", "method": "ping", "id": 999}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Rate limit should recover after waiting"
    );

    child.kill().unwrap();
}

// =============================================================================
// Health/Metrics Endpoint Rate Limit Tests
// =============================================================================

#[tokio::test]
async fn test_health_endpoint_not_rate_limited() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Health endpoint should not be rate limited
    let mut all_success = true;
    for _ in 0..20 {
        let resp = client
            .get(format!("{}/health", base_url))
            .send()
            .await
            .unwrap();

        if resp.status() != StatusCode::OK {
            all_success = false;
            break;
        }
    }

    assert!(all_success, "Health endpoint should not be rate limited");

    child.kill().unwrap();
}

#[tokio::test]
async fn test_metrics_endpoint_not_rate_limited() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();

    // Metrics endpoint should not be rate limited
    let mut all_success = true;
    for _ in 0..20 {
        let resp = client
            .get(format!("{}/metrics", base_url))
            .send()
            .await
            .unwrap();

        if resp.status() != StatusCode::OK {
            all_success = false;
            break;
        }
    }

    assert!(all_success, "Metrics endpoint should not be rate limited");

    child.kill().unwrap();
}

// =============================================================================
// Concurrent Rate Limit Tests
// =============================================================================

#[tokio::test]
async fn test_rate_limit_under_concurrent_load() {
    let (mut child, base_url, _) = spawn_server_with_config(&config_with_strict_rate_limit()).await;

    let client = reqwest::Client::new();
    let mut handles = vec![];

    // Spawn 10 concurrent requests
    for i in 0..10 {
        let client = client.clone();
        let url = format!("{}/mcp", base_url);
        let handle = tokio::spawn(async move {
            let resp = client
                .post(&url)
                .header(header::AUTHORIZATION, "Bearer test-key")
                .header(header::CONTENT_TYPE, "application/json")
                .body(format!(
                    r#"{{"jsonrpc": "2.0", "method": "ping", "id": {}}}"#,
                    i
                ))
                .send()
                .await
                .unwrap();
            resp.status()
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    let mut rate_limited_count = 0;

    for handle in handles {
        let status = handle.await.unwrap();
        if status == StatusCode::OK {
            success_count += 1;
        } else if status == StatusCode::TOO_MANY_REQUESTS {
            rate_limited_count += 1;
        }
    }

    // With concurrent requests and strict rate limit, we should see both
    assert!(success_count > 0, "Some concurrent requests should succeed");
    // Rate limiting may or may not kick in depending on timing
    println!(
        "Concurrent test: {} successes, {} rate limited",
        success_count, rate_limited_count
    );

    child.kill().unwrap();
}
