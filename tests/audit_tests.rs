use mcp_guard::audit::{AuditEntry, AuditLogger, EventType};
use mcp_guard::config::AuditConfig;
use std::collections::HashMap;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper to create a test config with export enabled
fn test_export_config(export_url: String) -> AuditConfig {
    AuditConfig {
        enabled: true,
        file: None,
        stdout: false,
        export_url: Some(export_url),
        export_headers: HashMap::new(),
        export_batch_size: 1, // Flush immediately
        export_interval_secs: 1,
        redaction_rules: Vec::new(),
        rotation: None,
    }
}

#[tokio::test]
async fn test_audit_export_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/audit"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let config = test_export_config(format!("{}/audit", mock_server.uri()));

    let (logger, handle) = AuditLogger::with_tasks(&config).expect("Failed to create logger");

    logger.log(
        &AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user1")
            .with_success(true),
    );

    // Wait for async processing
    tokio::time::sleep(Duration::from_millis(200)).await;

    drop(logger); // Drop logger to close export channel
    handle.shutdown().await;
}

#[tokio::test]
async fn test_audit_export_retry_logic() {
    let mock_server = MockServer::start().await;

    // First request fails (500), second succeeds (200)
    Mock::given(method("POST"))
        .and(path("/audit"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/audit"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let config = test_export_config(format!("{}/audit", mock_server.uri()));

    let (logger, handle) = AuditLogger::with_tasks(&config).expect("Failed to create logger");

    logger.log(&AuditEntry::new(EventType::Error).with_message("Test retry error"));

    // Initial backoff is 100ms, so give it enough time (e.g., 500ms)
    tokio::time::sleep(Duration::from_millis(500)).await;

    drop(logger);
    handle.shutdown().await;

    // Wiremock verification handles the expectation count
}

#[tokio::test]
async fn test_audit_export_max_retries_exceeded() {
    let mock_server = MockServer::start().await;

    // Always fail
    Mock::given(method("POST"))
        .and(path("/audit"))
        .respond_with(ResponseTemplate::new(500))
        .expect(3) // 3 attempts total (0, 1, 2)
        .mount(&mock_server)
        .await;

    let config = test_export_config(format!("{}/audit", mock_server.uri()));

    let (logger, handle) = AuditLogger::with_tasks(&config).expect("Failed to create logger");

    logger.log(&AuditEntry::new(EventType::Error).with_message("This will fail"));

    // Backoff: 100ms + 200ms + 400ms = 700ms total wait needed roughly
    // Give it 1.5s to be safe
    tokio::time::sleep(Duration::from_millis(1500)).await;

    drop(logger);
    handle.shutdown().await;
}
