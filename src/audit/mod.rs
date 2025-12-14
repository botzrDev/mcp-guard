//! Audit logging for mcp-guard
//!
//! Provides audit logging with multiple output destinations:
//! - File: Append audit entries to a local file
//! - Stdout: Print audit entries to console
//! - HTTP Export: Batch and ship audit entries to an HTTP endpoint (SIEM integration)

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;

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
        self.identity_id = Some(id.into());
        self
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
}

/// Audit logger with optional HTTP export
pub struct AuditLogger {
    enabled: bool,
    stdout: bool,
    file: Option<Mutex<std::fs::File>>,
    /// Channel for sending entries to the shipper task
    export_tx: Option<mpsc::Sender<AuditEntry>>,
}

/// Handle for the audit log shipper background task
pub struct AuditShipperHandle {
    /// Handle to the background task
    _task: tokio::task::JoinHandle<()>,
}

impl AuditLogger {
    /// Create a new audit logger from configuration
    pub fn new(config: &crate::config::AuditConfig) -> std::io::Result<Self> {
        let file = if let Some(path) = &config.file {
            Some(Mutex::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)?,
            ))
        } else {
            None
        };

        Ok(Self {
            enabled: config.enabled,
            stdout: config.stdout,
            file,
            export_tx: None,
        })
    }

    /// Create a new audit logger with HTTP export enabled
    pub fn with_export(config: &crate::config::AuditConfig) -> std::io::Result<(Self, Option<AuditShipperHandle>)> {
        let mut logger = Self::new(config)?;

        let handle = if let Some(ref export_url) = config.export_url {
            let (tx, rx) = mpsc::channel::<AuditEntry>(1000);
            logger.export_tx = Some(tx);

            let shipper = AuditShipper::new(
                export_url.clone(),
                config.export_headers.clone(),
                config.export_batch_size,
                config.export_interval_secs,
            );

            let task = tokio::spawn(async move {
                shipper.run(rx).await;
            });

            Some(AuditShipperHandle { _task: task })
        } else {
            None
        };

        Ok((logger, handle))
    }

    /// Create a disabled audit logger
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            stdout: false,
            file: None,
            export_tx: None,
        }
    }

    /// Log an audit entry
    pub fn log(&self, entry: &AuditEntry) {
        if !self.enabled {
            return;
        }

        let json = match serde_json::to_string(entry) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("Failed to serialize audit entry: {}", e);
                return;
            }
        };

        if self.stdout {
            println!("{}", json);
        }

        if let Some(file) = &self.file {
            if let Ok(mut f) = file.lock() {
                let _ = writeln!(f, "{}", json);
            }
        }

        // Send to shipper if configured
        if let Some(ref tx) = self.export_tx {
            // Use try_send to avoid blocking; if channel is full, we'll drop the entry
            // This is acceptable for audit logs as we don't want to block the main request path
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
        Self {
            url,
            headers,
            batch_size,
            flush_interval: Duration::from_secs(flush_interval_secs),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
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
        for attempt in 0..3 {
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
            return Err(format!("HTTP {}: {}", status, body));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
