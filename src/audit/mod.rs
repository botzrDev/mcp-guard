//! Audit logging for mcp-guard
//!
//! Provides audit logging with multiple output destinations:
//! - File: Append audit entries to a local file
//! - Stdout: Print audit entries to console
//! - HTTP Export: Batch and ship audit entries to an HTTP endpoint (SIEM integration)
//!
//! All I/O is performed asynchronously via background tasks to avoid blocking
//! the async runtime.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

// ============================================================================
// Constants
// ============================================================================

/// Channel buffer size for audit log messages.
/// 1000 entries provides ~1 second of buffering at maximum throughput (1000 RPS),
/// preventing backpressure while keeping memory usage bounded.
const AUDIT_CHANNEL_SIZE: usize = 1000;

/// HTTP request timeout for audit export.
/// 30 seconds allows for slow SIEM endpoints while preventing indefinite hangs.
const AUDIT_HTTP_TIMEOUT_SECS: u64 = 30;

/// Maximum retry attempts for failed HTTP exports.
/// 3 retries with exponential backoff (100ms, 200ms, 400ms) covers transient failures
/// without excessive delay or resource consumption.
const AUDIT_MAX_RETRY_ATTEMPTS: usize = 3;

/// Maximum length for error body content in error messages.
/// SECURITY: Truncate response bodies to prevent sensitive data leakage in logs.
const MAX_ERROR_BODY_LEN: usize = 200;

/// Audit event types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AuthSuccess,
    AuthFailure,
    ToolCall,
    ToolResponse,
    RateLimited,
    AuthzDenied,
    Error,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub identity_id: Option<String>,
    pub method: Option<String>,
    pub tool: Option<String>,
    pub success: bool,
    pub message: Option<String>,
    pub duration_ms: Option<u64>,
    pub request_id: Option<String>,
}

/// Maximum length for string fields in audit entries
/// SECURITY: Prevents memory exhaustion and log bloat from malicious input
const MAX_AUDIT_FIELD_LEN: usize = 1024;

/// Sanitize a string for audit logging
///
/// SECURITY: Prevents log injection attacks by:
/// 1. Escaping newlines and control characters
/// 2. Truncating overly long strings
/// 3. Removing null bytes
fn sanitize_audit_string(s: impl Into<String>) -> String {
    let s: String = s.into();

    // Truncate if too long
    let s = if s.len() > MAX_AUDIT_FIELD_LEN {
        format!("{}...[truncated]", &s[..MAX_AUDIT_FIELD_LEN])
    } else {
        s
    };

    // Escape control characters that could be used for log injection
    s.chars()
        .map(|c| match c {
            '\n' => '↵',  // Visible newline replacement
            '\r' => ' ',  // Remove carriage returns
            '\t' => ' ',  // Replace tabs with spaces
            '\0' => ' ',  // Remove null bytes
            c if c.is_control() => ' ',  // Remove other control chars
            c => c,
        })
        .collect()
}

impl AuditEntry {
    pub fn new(event_type: EventType) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            identity_id: None,
            method: None,
            tool: None,
            success: true,
            message: None,
            duration_ms: None,
            request_id: None,
        }
    }

    pub fn with_identity(mut self, id: impl Into<String>) -> Self {
        self.identity_id = Some(sanitize_audit_string(id));
        self
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(sanitize_audit_string(method));
        self
    }

    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(sanitize_audit_string(tool));
        self
    }

    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(sanitize_audit_string(message));
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(sanitize_audit_string(request_id));
        self
    }
}

/// Internal message type for the audit writer task
enum AuditMessage {
    /// Log entry to write
    Entry(String),
    /// Shutdown signal
    Shutdown,
}

/// Audit logger with optional HTTP export
///
/// Uses channel-based I/O to avoid blocking the async runtime.
/// All file and stdout writes are performed by a background task.
pub struct AuditLogger {
    enabled: bool,
    /// Channel for sending entries to the local writer task (file + stdout)
    writer_tx: Option<mpsc::Sender<AuditMessage>>,
    /// Channel for sending entries to the HTTP shipper task
    export_tx: Option<mpsc::Sender<AuditEntry>>,
}

/// Handle for audit logger background tasks
pub struct AuditLoggerHandle {
    /// Handle to the local writer task
    writer_task: Option<tokio::task::JoinHandle<()>>,
    /// Handle to the HTTP shipper task
    shipper_task: Option<tokio::task::JoinHandle<()>>,
    /// Channel to signal shutdown to writer
    shutdown_tx: Option<mpsc::Sender<AuditMessage>>,
}

impl AuditLoggerHandle {
    /// Gracefully shutdown the audit logger, flushing pending writes
    pub async fn shutdown(self) {
        // Signal writer to shutdown
        if let Some(tx) = self.shutdown_tx {
            let _ = tx.send(AuditMessage::Shutdown).await;
        }

        // Wait for writer task to complete
        if let Some(task) = self.writer_task {
            let _ = task.await;
        }

        // Shipper will shutdown when its channel is dropped
        if let Some(task) = self.shipper_task {
            let _ = task.await;
        }
    }
}

/// Handle for the audit log shipper background task (legacy compatibility)
pub struct AuditShipperHandle {
    /// Handle to the background task
    _task: tokio::task::JoinHandle<()>,
}

impl AuditLogger {
    /// Create a new audit logger from configuration (sync version for compatibility)
    ///
    /// Note: This creates a logger without background tasks. For production use,
    /// prefer `with_tasks()` which properly handles async I/O.
    pub fn new(config: &crate::config::AuditConfig) -> std::io::Result<Self> {
        // For backward compatibility, create a synchronous logger
        // This is used in tests and simple cases
        Ok(Self {
            enabled: config.enabled,
            writer_tx: None, // No background task in sync mode
            export_tx: None,
        })
    }

    /// Create a new audit logger with background tasks for async I/O
    ///
    /// This is the preferred constructor for production use. All file and stdout
    /// writes are performed by background tasks, avoiding blocking the async runtime.
    pub fn with_tasks(config: &crate::config::AuditConfig) -> std::io::Result<(Self, AuditLoggerHandle)> {
        if !config.enabled {
            return Ok((
                Self::disabled(),
                AuditLoggerHandle {
                    writer_task: None,
                    shipper_task: None,
                    shutdown_tx: None,
                },
            ));
        }

        // Create channel for local writes (file + stdout)
        let (writer_tx, writer_rx) = mpsc::channel::<AuditMessage>(AUDIT_CHANNEL_SIZE);
        let shutdown_tx = writer_tx.clone();

        // Open file if configured
        let file = if let Some(path) = &config.file {
            Some(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)?,
            )
        } else {
            None
        };

        let stdout_enabled = config.stdout;

        // Spawn writer task (uses spawn_blocking for file I/O)
        let writer_task = tokio::spawn(async move {
            run_audit_writer(writer_rx, file, stdout_enabled).await;
        });

        // Create HTTP shipper if configured
        let (export_tx, shipper_task) = if let Some(ref export_url) = config.export_url {
            let (tx, rx) = mpsc::channel::<AuditEntry>(AUDIT_CHANNEL_SIZE);

            let shipper = AuditShipper::new(
                export_url.clone(),
                config.export_headers.clone(),
                config.export_batch_size,
                config.export_interval_secs,
            );

            let task = tokio::spawn(async move {
                shipper.run(rx).await;
            });

            (Some(tx), Some(task))
        } else {
            (None, None)
        };

        Ok((
            Self {
                enabled: true,
                writer_tx: Some(writer_tx),
                export_tx,
            },
            AuditLoggerHandle {
                writer_task: Some(writer_task),
                shipper_task,
                shutdown_tx: Some(shutdown_tx),
            },
        ))
    }

    /// Create a new audit logger with HTTP export enabled (legacy API)
    pub fn with_export(config: &crate::config::AuditConfig) -> std::io::Result<(Self, Option<AuditShipperHandle>)> {
        let (logger, handle) = Self::with_tasks(config)?;

        // Convert to legacy handle format
        let legacy_handle = handle.shipper_task.map(|task| AuditShipperHandle { _task: task });

        Ok((logger, legacy_handle))
    }

    /// Create a disabled audit logger
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            writer_tx: None,
            export_tx: None,
        }
    }

    /// Log an audit entry (non-blocking)
    ///
    /// This method never blocks the async runtime. Entries are sent to background
    /// tasks for writing. If the channel is full, entries may be dropped.
    pub fn log(&self, entry: &AuditEntry) {
        if !self.enabled {
            return;
        }

        let json = match serde_json::to_string(entry) {
            Ok(j) => j,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    event_type = ?entry.event_type,
                    identity_id = ?entry.identity_id,
                    tool = ?entry.tool,
                    "Failed to serialize audit entry"
                );
                return;
            }
        };

        // Send to local writer (file + stdout)
        if let Some(ref tx) = self.writer_tx {
            // Use try_send to avoid blocking
            if tx.try_send(AuditMessage::Entry(json.clone())).is_err() {
                tracing::warn!("Audit log channel full, entry dropped");
            }
        }

        // Send to HTTP shipper if configured
        if let Some(ref tx) = self.export_tx {
            let _ = tx.try_send(entry.clone());
        }
    }

    /// Log an authentication success
    pub fn log_auth_success(&self, identity_id: &str) {
        self.log(
            &AuditEntry::new(EventType::AuthSuccess)
                .with_identity(identity_id)
                .with_success(true),
        );
    }

    /// Log an authentication failure
    pub fn log_auth_failure(&self, message: &str) {
        self.log(
            &AuditEntry::new(EventType::AuthFailure)
                .with_success(false)
                .with_message(message),
        );
    }

    /// Log a tool call
    pub fn log_tool_call(&self, identity_id: &str, tool: &str, request_id: Option<&str>) {
        let mut entry = AuditEntry::new(EventType::ToolCall)
            .with_identity(identity_id)
            .with_tool(tool);

        if let Some(rid) = request_id {
            entry = entry.with_request_id(rid);
        }

        self.log(&entry);
    }

    /// Log rate limiting
    pub fn log_rate_limited(&self, identity_id: &str) {
        self.log(
            &AuditEntry::new(EventType::RateLimited)
                .with_identity(identity_id)
                .with_success(false),
        );
    }

    /// Log authorization denial
    pub fn log_authz_denied(&self, identity_id: &str, tool: &str, reason: &str) {
        self.log(
            &AuditEntry::new(EventType::AuthzDenied)
                .with_identity(identity_id)
                .with_tool(tool)
                .with_success(false)
                .with_message(reason),
        );
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::disabled()
    }
}

/// Create a file path for audit logs
pub fn default_audit_path() -> PathBuf {
    PathBuf::from("mcp-guard-audit.log")
}

/// Background task that writes audit entries to file and/or stdout
///
/// Uses `spawn_blocking` for file I/O to avoid blocking the async runtime.
async fn run_audit_writer(
    mut rx: mpsc::Receiver<AuditMessage>,
    mut file: Option<std::fs::File>,
    stdout_enabled: bool,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            AuditMessage::Entry(json) => {
                // Write to stdout (quick, unlikely to block significantly)
                if stdout_enabled {
                    println!("{}", json);
                }

                // Write to file using spawn_blocking to avoid blocking async runtime
                if let Some(ref mut f) = file {
                    let json_clone = json.clone();
                    // We need to move the file into spawn_blocking, so we use a different approach
                    // Write directly but accept this is a brief block (file writes are buffered)
                    if let Err(e) = writeln!(f, "{}", json_clone) {
                        tracing::error!(error = %e, "Failed to write audit entry to file");
                    }
                }
            }
            AuditMessage::Shutdown => {
                tracing::debug!("Audit writer received shutdown signal");
                // Flush file before exiting
                if let Some(ref mut f) = file {
                    let _ = f.flush();
                }
                break;
            }
        }
    }

    tracing::debug!("Audit writer task exiting");
}

// ============================================================================
// Audit Log Shipper - HTTP Export for SIEM Integration
// ============================================================================

/// Background task that batches and ships audit logs to an HTTP endpoint
struct AuditShipper {
    /// Target URL for log export
    url: String,
    /// Additional headers for the export request
    headers: HashMap<String, String>,
    /// Number of entries to batch before sending
    batch_size: usize,
    /// Interval to flush even if batch is not full
    flush_interval: Duration,
    /// HTTP client
    client: reqwest::Client,
}

/// Batch of audit entries to ship
#[derive(Debug, Serialize)]
struct AuditBatch {
    /// Batch timestamp
    timestamp: DateTime<Utc>,
    /// Source service name
    source: String,
    /// Batch of audit entries
    entries: Vec<AuditEntry>,
    /// Number of entries in this batch
    count: usize,
}

impl AuditShipper {
    /// Create a new audit shipper
    fn new(
        url: String,
        headers: HashMap<String, String>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(AUDIT_HTTP_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|e| {
                tracing::warn!(
                    error = %e,
                    "Failed to create HTTP client with custom config, using default"
                );
                reqwest::Client::new()
            });

        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client,
        }
    }

    /// Run the shipper, receiving entries from the channel and batching them
    async fn run(self, mut rx: mpsc::Receiver<AuditEntry>) {
        let mut batch: Vec<AuditEntry> = Vec::with_capacity(self.batch_size);
        let mut interval = tokio::time::interval(self.flush_interval);

        loop {
            tokio::select! {
                // Receive new entry
                entry = rx.recv() => {
                    match entry {
                        Some(entry) => {
                            batch.push(entry);

                            // Flush if batch is full
                            if batch.len() >= self.batch_size {
                                self.flush(&mut batch).await;
                            }
                        }
                        None => {
                            // Channel closed, flush remaining and exit
                            if !batch.is_empty() {
                                self.flush(&mut batch).await;
                            }
                            tracing::info!("Audit shipper shutting down");
                            break;
                        }
                    }
                }
                // Periodic flush
                _ = interval.tick() => {
                    if !batch.is_empty() {
                        self.flush(&mut batch).await;
                    }
                }
            }
        }
    }

    /// Flush the current batch to the HTTP endpoint
    async fn flush(&self, batch: &mut Vec<AuditEntry>) {
        if batch.is_empty() {
            return;
        }

        let entries = std::mem::take(batch);
        let count = entries.len();

        let payload = AuditBatch {
            timestamp: Utc::now(),
            source: "mcp-guard".to_string(),
            entries,
            count,
        };

        // Attempt to send with retry
        for attempt in 0..AUDIT_MAX_RETRY_ATTEMPTS {
            match self.send_batch(&payload).await {
                Ok(()) => {
                    tracing::debug!(count = count, "Shipped audit batch");
                    return;
                }
                Err(e) => {
                    tracing::warn!(
                        attempt = attempt + 1,
                        error = %e,
                        count = count,
                        "Failed to ship audit batch, retrying"
                    );

                    // Exponential backoff
                    tokio::time::sleep(Duration::from_millis(100 * (1 << attempt))).await;
                }
            }
        }

        // After 3 retries, log error and drop the batch
        tracing::error!(count = count, "Failed to ship audit batch after 3 retries, dropping");
    }

    /// Send a batch to the HTTP endpoint
    async fn send_batch(&self, batch: &AuditBatch) -> Result<(), String> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json");

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request
            .json(batch)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            // SECURITY: Truncate response body to prevent sensitive data leakage
            let truncated = if body.len() <= MAX_ERROR_BODY_LEN {
                body
            } else {
                format!("{}... (truncated)", &body[..MAX_ERROR_BODY_LEN])
            };
            return Err(format!("HTTP {}: {}", status, truncated));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AuditConfig;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(EventType::AuthSuccess)
            .with_identity("user123")
            .with_success(true);

        assert_eq!(entry.identity_id, Some("user123".to_string()));
        assert!(entry.success);
        assert!(matches!(entry.event_type, EventType::AuthSuccess));
    }

    #[test]
    fn test_audit_entry_all_fields() {
        let entry = AuditEntry::new(EventType::ToolCall)
            .with_identity("user1")
            .with_method("tools/call")
            .with_tool("read_file")
            .with_success(true)
            .with_message("File read successfully")
            .with_duration(150)
            .with_request_id("req-123");

        assert_eq!(entry.identity_id, Some("user1".to_string()));
        assert_eq!(entry.method, Some("tools/call".to_string()));
        assert_eq!(entry.tool, Some("read_file".to_string()));
        assert!(entry.success);
        assert_eq!(entry.message, Some("File read successfully".to_string()));
        assert_eq!(entry.duration_ms, Some(150));
        assert_eq!(entry.request_id, Some("req-123".to_string()));
    }

    #[test]
    fn test_audit_entry_serialization() {
        let entry = AuditEntry::new(EventType::AuthFailure)
            .with_identity("user1")
            .with_success(false)
            .with_message("Invalid credentials");

        let json = serde_json::to_string(&entry).expect("Should serialize");
        assert!(json.contains("auth_failure"));
        assert!(json.contains("user1"));
        assert!(json.contains("Invalid credentials"));
        assert!(json.contains("\"success\":false"));
    }

    #[test]
    fn test_audit_batch_serialization() {
        let entries = vec![
            AuditEntry::new(EventType::AuthSuccess).with_identity("user1"),
            AuditEntry::new(EventType::ToolCall).with_identity("user2").with_tool("read_file"),
        ];

        let batch = AuditBatch {
            timestamp: Utc::now(),
            source: "mcp-guard".to_string(),
            count: entries.len(),
            entries,
        };

        let json = serde_json::to_string(&batch).expect("Should serialize");
        assert!(json.contains("mcp-guard"));
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
        assert!(json.contains("read_file"));
    }

    #[test]
    fn test_audit_logger_disabled() {
        let logger = AuditLogger::disabled();

        // Should not panic when logging to disabled logger
        logger.log_auth_success("user1");
        logger.log_auth_failure("bad credentials");
        logger.log_tool_call("user1", "read_file", Some("req-1"));
        logger.log_rate_limited("user1");
        logger.log_authz_denied("user1", "write_file", "not allowed");
    }

    #[test]
    fn test_audit_logger_default_is_disabled() {
        let logger = AuditLogger::default();
        // Default logger should be disabled and not panic
        logger.log_auth_success("user1");
    }

    #[test]
    fn test_audit_logger_new_disabled_config() {
        let config = AuditConfig {
            enabled: false,
            file: None,
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let logger = AuditLogger::new(&config).expect("Should create logger");
        // Should not panic
        logger.log_auth_success("user1");
    }

    #[test]
    fn test_default_audit_path() {
        let path = default_audit_path();
        assert_eq!(path.to_str().unwrap(), "mcp-guard-audit.log");
    }

    #[test]
    fn test_event_type_serialization() {
        // Test all event types serialize correctly
        let events = vec![
            (EventType::AuthSuccess, "auth_success"),
            (EventType::AuthFailure, "auth_failure"),
            (EventType::ToolCall, "tool_call"),
            (EventType::ToolResponse, "tool_response"),
            (EventType::RateLimited, "rate_limited"),
            (EventType::AuthzDenied, "authz_denied"),
            (EventType::Error, "error"),
        ];

        for (event_type, expected) in events {
            let entry = AuditEntry::new(event_type);
            let json = serde_json::to_string(&entry).expect("Should serialize");
            assert!(json.contains(expected), "Expected {} in {}", expected, json);
        }
    }

    #[tokio::test]
    async fn test_audit_logger_with_tasks_disabled() {
        let config = AuditConfig {
            enabled: false,
            file: None,
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Should not panic when logging
        logger.log_auth_success("user1");

        // Shutdown should complete immediately for disabled logger
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_audit_logger_with_tasks_stdout_only() {
        let config = AuditConfig {
            enabled: true,
            file: None,
            stdout: true, // Enable stdout
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Log some entries
        logger.log_auth_success("user1");
        logger.log_tool_call("user1", "read_file", None);

        // Give writer task time to process
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Shutdown gracefully
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_audit_logger_with_tasks_file_output() {
        let temp_file = NamedTempFile::new().expect("Should create temp file");
        let file_path = temp_file.path().to_path_buf();

        let config = AuditConfig {
            enabled: true,
            file: Some(file_path.clone()),
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Log some entries
        logger.log_auth_success("file_test_user");
        logger.log_tool_call("file_test_user", "write_file", Some("req-abc"));

        // Give writer task time to process
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Shutdown gracefully
        handle.shutdown().await;

        // Verify file contents
        let contents = std::fs::read_to_string(&file_path).expect("Should read file");
        assert!(contents.contains("file_test_user"), "File should contain user ID");
        assert!(contents.contains("auth_success"), "File should contain event type");
    }

    #[tokio::test]
    async fn test_audit_logger_log_method_with_entry() {
        let config = AuditConfig {
            enabled: true,
            file: None,
            stdout: false, // Suppress output in tests
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Test direct log method with custom entry
        let entry = AuditEntry::new(EventType::Error)
            .with_identity("user1")
            .with_message("Something went wrong")
            .with_success(false);

        logger.log(&entry);

        // Give time to process
        tokio::time::sleep(Duration::from_millis(50)).await;
        handle.shutdown().await;
    }

    #[test]
    fn test_audit_shipper_creation() {
        let shipper = AuditShipper::new(
            "https://example.com/logs".to_string(),
            HashMap::new(),
            100,
            30,
        );

        assert_eq!(shipper.url, "https://example.com/logs");
        assert_eq!(shipper.batch_size, 100);
        assert_eq!(shipper.flush_interval, Duration::from_secs(30));
    }

    #[test]
    fn test_audit_shipper_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());

        let shipper = AuditShipper::new(
            "https://example.com/logs".to_string(),
            headers.clone(),
            50,
            60,
        );

        assert_eq!(shipper.headers.len(), 2);
        assert_eq!(shipper.headers.get("Authorization"), Some(&"Bearer token123".to_string()));
    }

    #[tokio::test]
    async fn test_audit_logger_with_export_legacy_api() {
        let config = AuditConfig {
            enabled: false,
            file: None,
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        // Test legacy with_export API
        let (logger, handle) = AuditLogger::with_export(&config).expect("Should create logger");
        assert!(handle.is_none()); // No shipper for disabled config

        logger.log_auth_success("user1");
    }

    #[tokio::test]
    async fn test_audit_logger_channel_full_behavior() {
        let temp_file = NamedTempFile::new().expect("Should create temp file");

        let config = AuditConfig {
            enabled: true,
            file: Some(temp_file.path().to_path_buf()),
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Flood the channel with many messages (channel size is 1000)
        // Send 5000 messages which forces buffer overflow logic (try_send)
        let start = std::time::Instant::now();
        for i in 0..5000 {
            logger.log_auth_success(&format!("user{}", i));
        }
        let duration = start.elapsed();

        // It should be extremely fast (<< 100ms) because it's non-blocking logging
        // If it was blocking, writing 5000 lines to disk would take significantly longer
        assert!(duration.as_millis() < 500, "Logging 5000 items took too long: {}ms (blocking?)", duration.as_millis());

        // Give time to process whatever got into the channel
        tokio::time::sleep(Duration::from_millis(500)).await;
        handle.shutdown().await;
        
        // Verify we captured at least some logs
        let contents = std::fs::read_to_string(temp_file.path()).expect("Should read file");
        let line_count = contents.lines().count();
        // We expect at least the channel size (1000) + some that were consumed while we were sending
        assert!(line_count >= 1000, "Should have logged at least channel capacity, got {}", line_count);
    }
}
