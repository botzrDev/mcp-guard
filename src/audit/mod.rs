//! Audit logging for mcp-guard
//!
//! Provides audit logging with multiple output destinations:
//! - File: Append audit entries to a local file (with optional rotation)
//! - Stdout: Print audit entries to console
//! - HTTP Export: Batch and ship audit entries to an HTTP endpoint (SIEM integration)
//!
//! Features:
//! - Secret redaction: Configurable regex patterns to prevent credential leakage
//! - Log rotation: Size and time-based rotation with optional gzip compression
//!
//! All I/O is performed asynchronously via background tasks to avoid blocking
//! the async runtime.

use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use crate::config::{LogRotationConfig, RedactionRule};

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

// ============================================================================
// Secret Redaction
// ============================================================================

/// Compiled redaction rules for efficient secret matching
///
/// Pre-compiles regex patterns at startup for optimal runtime performance.
/// Patterns are applied in order, so more specific patterns should come first.
#[derive(Clone)]
pub struct CompiledRedactionRules {
    /// Compiled rules: (name, pattern, replacement)
    rules: Vec<(String, Regex, String)>,
}

impl std::fmt::Debug for CompiledRedactionRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledRedactionRules")
            .field("rules_count", &self.rules.len())
            .field("rule_names", &self.rules.iter().map(|(name, _, _)| name.as_str()).collect::<Vec<_>>())
            .finish()
    }
}

impl CompiledRedactionRules {
    /// Create compiled rules from configuration
    ///
    /// Returns an error if any regex pattern is invalid.
    pub fn new(rules: &[RedactionRule]) -> Result<Self, regex::Error> {
        let compiled = rules
            .iter()
            .map(|r| {
                let regex = Regex::new(&r.pattern)?;
                Ok((r.name.clone(), regex, r.replacement.clone()))
            })
            .collect::<Result<Vec<_>, regex::Error>>()?;

        if !compiled.is_empty() {
            tracing::info!(
                rule_count = compiled.len(),
                rules = ?compiled.iter().map(|(n, _, _)| n.as_str()).collect::<Vec<_>>(),
                "Compiled secret redaction rules"
            );
        }

        Ok(Self { rules: compiled })
    }

    /// Create empty rules (no redaction)
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Check if any rules are configured
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Redact secrets from a string using configured patterns
    ///
    /// Applies all rules in order. Each rule's replacement can use
    /// regex capture groups (e.g., "$1" to preserve matched group).
    pub fn redact(&self, input: &str) -> String {
        if self.rules.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for (name, pattern, replacement) in &self.rules {
            let before = result.clone();
            result = pattern.replace_all(&result, replacement.as_str()).into_owned();
            if result != before {
                tracing::trace!(rule = %name, "Applied redaction rule");
            }
        }
        result
    }
}

// ============================================================================
// Log Rotation
// ============================================================================

/// Rotating file writer that handles size and time-based log rotation
///
/// Automatically rotates log files when they exceed size or age limits.
/// Supports optional gzip compression of rotated files.
pub struct RotatingFileWriter {
    /// Path to the main log file
    path: PathBuf,
    /// Current file handle
    file: BufWriter<File>,
    /// Current file size in bytes
    current_size: u64,
    /// When the current file was created
    created_at: Instant,
    /// Rotation configuration
    config: LogRotationConfig,
}

impl RotatingFileWriter {
    /// Create a new rotating file writer
    pub fn new(path: PathBuf, config: LogRotationConfig) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        let current_size = file.metadata()?.len();

        Ok(Self {
            path,
            file: BufWriter::new(file),
            current_size,
            created_at: Instant::now(),
            config,
        })
    }

    /// Write data to the log file, rotating if necessary
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        // Check if rotation is needed before writing
        if self.should_rotate() {
            self.rotate()?;
        }

        self.file.write_all(data)?;
        self.current_size += data.len() as u64;
        Ok(())
    }

    /// Write a line to the log file
    pub fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.write(line.as_bytes())?;
        self.write(b"\n")
    }

    /// Flush buffered data to disk
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }

    /// Check if rotation is needed based on size or age
    fn should_rotate(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Check size limit
        if let Some(max_size) = self.config.max_size_bytes {
            if self.current_size >= max_size {
                return true;
            }
        }

        // Check age limit
        if let Some(max_age) = self.config.max_age_secs {
            if self.created_at.elapsed().as_secs() >= max_age {
                return true;
            }
        }

        false
    }

    /// Rotate the current log file
    fn rotate(&mut self) -> io::Result<()> {
        // Flush and close current file
        self.file.flush()?;

        // Generate backup filename with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
        let backup_name = format!(
            "{}.{}",
            self.path.file_name().unwrap_or_default().to_string_lossy(),
            timestamp
        );
        let backup_path = self.path.with_file_name(&backup_name);

        // Rename current file to backup
        fs::rename(&self.path, &backup_path)?;

        tracing::info!(
            original = %self.path.display(),
            backup = %backup_path.display(),
            size_bytes = self.current_size,
            "Rotated audit log file"
        );

        // Compress if configured
        if self.config.compress {
            let compressed_path = backup_path.with_extension(
                format!("{}.gz", backup_path.extension().unwrap_or_default().to_string_lossy())
            );
            if let Err(e) = Self::compress_file(&backup_path, &compressed_path) {
                tracing::warn!(
                    error = %e,
                    path = %backup_path.display(),
                    "Failed to compress rotated log file"
                );
            } else {
                // Remove uncompressed backup after successful compression
                let _ = fs::remove_file(&backup_path);
                tracing::debug!(path = %compressed_path.display(), "Compressed rotated log file");
            }
        }

        // Clean up old backups
        if let Err(e) = self.cleanup_old_backups() {
            tracing::warn!(error = %e, "Failed to clean up old backup files");
        }

        // Create new file
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        self.file = BufWriter::new(new_file);
        self.current_size = 0;
        self.created_at = Instant::now();

        Ok(())
    }

    /// Compress a file using gzip
    fn compress_file(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        let input = File::open(source)?;
        let output = File::create(dest)?;
        let mut encoder = GzEncoder::new(output, Compression::default());

        io::copy(&mut io::BufReader::new(input), &mut encoder)?;
        encoder.finish()?;

        Ok(())
    }

    /// Clean up old backup files, keeping only max_backups
    fn cleanup_old_backups(&self) -> io::Result<()> {
        let parent = self.path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let base_name = self.path.file_name().unwrap_or_default().to_string_lossy();

        // Find all backup files
        let mut backups: Vec<_> = fs::read_dir(parent)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                // Match files like "audit.log.20251225-120000" or "audit.log.20251225-120000.gz"
                name.starts_with(&format!("{}.", base_name)) && name != base_name.as_ref()
            })
            .collect();

        // Sort by modification time (oldest first)
        backups.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        // Remove oldest backups if we have too many
        let to_remove = backups.len().saturating_sub(self.config.max_backups);
        for entry in backups.into_iter().take(to_remove) {
            let path = entry.path();
            if let Err(e) = fs::remove_file(&path) {
                tracing::warn!(error = %e, path = %path.display(), "Failed to remove old backup");
            } else {
                tracing::debug!(path = %path.display(), "Removed old backup file");
            }
        }

        Ok(())
    }
}

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
    /// Compiled redaction rules for secret filtering
    redaction_rules: CompiledRedactionRules,
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
        // Compile redaction rules
        let redaction_rules = CompiledRedactionRules::new(&config.redaction_rules)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        // For backward compatibility, create a synchronous logger
        // This is used in tests and simple cases
        Ok(Self {
            enabled: config.enabled,
            writer_tx: None, // No background task in sync mode
            export_tx: None,
            redaction_rules,
        })
    }

    /// Create a new audit logger with background tasks for async I/O
    ///
    /// This is the preferred constructor for production use. All file and stdout
    /// writes are performed by background tasks, avoiding blocking the async runtime.
    pub fn with_tasks(config: &crate::config::AuditConfig) -> std::io::Result<(Self, AuditLoggerHandle)> {
        // Compile redaction rules first (before checking enabled)
        let redaction_rules = CompiledRedactionRules::new(&config.redaction_rules)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        if !config.enabled {
            return Ok((
                Self {
                    enabled: false,
                    writer_tx: None,
                    export_tx: None,
                    redaction_rules: CompiledRedactionRules::empty(),
                },
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

        // Create file writer (with or without rotation)
        let file_writer: Option<FileWriter> = if let Some(path) = &config.file {
            if let Some(ref rotation_config) = config.rotation {
                if rotation_config.enabled {
                    // Use rotating file writer
                    Some(FileWriter::Rotating(RotatingFileWriter::new(
                        path.clone(),
                        rotation_config.clone(),
                    )?))
                } else {
                    // Rotation configured but disabled - use simple file
                    Some(FileWriter::Simple(
                        OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(path)?,
                    ))
                }
            } else {
                // No rotation config - use simple file
                Some(FileWriter::Simple(
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)?,
                ))
            }
        } else {
            None
        };

        let stdout_enabled = config.stdout;

        // Spawn writer task
        let writer_task = tokio::spawn(async move {
            run_audit_writer(writer_rx, file_writer, stdout_enabled).await;
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
                redaction_rules,
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
            redaction_rules: CompiledRedactionRules::empty(),
        }
    }

    /// Log an audit entry (non-blocking)
    ///
    /// This method never blocks the async runtime. Entries are sent to background
    /// tasks for writing. If the channel is full, entries may be dropped.
    ///
    /// Secret redaction is applied before serialization if redaction rules are configured.
    pub fn log(&self, entry: &AuditEntry) {
        if !self.enabled {
            return;
        }

        // Apply redaction to the entry before serializing
        let redacted_entry = if self.redaction_rules.is_empty() {
            entry.clone()
        } else {
            self.redact_entry(entry)
        };

        let json = match serde_json::to_string(&redacted_entry) {
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

        // Send to HTTP shipper if configured (use redacted entry)
        if let Some(ref tx) = self.export_tx {
            let _ = tx.try_send(redacted_entry);
        }
    }

    /// Apply redaction rules to an audit entry
    fn redact_entry(&self, entry: &AuditEntry) -> AuditEntry {
        AuditEntry {
            timestamp: entry.timestamp,
            event_type: entry.event_type.clone(),
            identity_id: entry.identity_id.as_ref().map(|s| self.redaction_rules.redact(s)),
            method: entry.method.as_ref().map(|s| self.redaction_rules.redact(s)),
            tool: entry.tool.as_ref().map(|s| self.redaction_rules.redact(s)),
            success: entry.success,
            message: entry.message.as_ref().map(|s| self.redaction_rules.redact(s)),
            duration_ms: entry.duration_ms,
            request_id: entry.request_id.as_ref().map(|s| self.redaction_rules.redact(s)),
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

/// File writer abstraction supporting both simple and rotating writes
enum FileWriter {
    /// Simple append-only file
    Simple(File),
    /// Rotating file with size/time-based rotation
    Rotating(RotatingFileWriter),
}

impl FileWriter {
    /// Write a line to the file
    fn write_line(&mut self, line: &str) -> io::Result<()> {
        match self {
            FileWriter::Simple(f) => {
                writeln!(f, "{}", line)
            }
            FileWriter::Rotating(r) => {
                r.write_line(line)
            }
        }
    }

    /// Flush buffered data
    fn flush(&mut self) -> io::Result<()> {
        match self {
            FileWriter::Simple(f) => f.flush(),
            FileWriter::Rotating(r) => r.flush(),
        }
    }
}

/// Background task that writes audit entries to file and/or stdout
///
/// Supports both simple file writes and rotating file writes.
async fn run_audit_writer(
    mut rx: mpsc::Receiver<AuditMessage>,
    mut file_writer: Option<FileWriter>,
    stdout_enabled: bool,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            AuditMessage::Entry(json) => {
                // Write to stdout (quick, unlikely to block significantly)
                if stdout_enabled {
                    println!("{}", json);
                }

                // Write to file (with or without rotation)
                if let Some(ref mut writer) = file_writer {
                    if let Err(e) = writer.write_line(&json) {
                        tracing::error!(error = %e, "Failed to write audit entry to file");
                    }
                }
            }
            AuditMessage::Shutdown => {
                tracing::debug!("Audit writer received shutdown signal");
                // Flush file before exiting
                if let Some(ref mut writer) = file_writer {
                    let _ = writer.flush();
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
    use tempfile::{NamedTempFile, TempDir};

    /// Helper to create a default test config
    fn test_config() -> AuditConfig {
        AuditConfig {
            enabled: true,
            file: None,
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
            redaction_rules: Vec::new(),
            rotation: None,
        }
    }

    // ========================================================================
    // Redaction Tests
    // ========================================================================

    #[test]
    fn test_redaction_rules_empty() {
        let rules = CompiledRedactionRules::empty();
        assert!(rules.is_empty());
        assert_eq!(rules.redact("test"), "test");
    }

    #[test]
    fn test_redaction_rules_bearer_token() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer_tokens".to_string(),
                pattern: r"Bearer\s+[A-Za-z0-9\-_.]+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz";
        let output = rules.redact(input);
        assert!(!output.contains("eyJ"));
        assert!(output.contains("Bearer [REDACTED]"));
    }

    #[test]
    fn test_redaction_rules_api_key() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "api_keys".to_string(),
                pattern: r"(?i)(api[_-]?key)[=:]\s*([a-zA-Z0-9_\-]{20,})".to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Config: api_key=sk-1234567890abcdefghij1234567890";
        let output = rules.redact(input);
        assert!(!output.contains("sk-1234567890"));
        assert!(output.contains("api_key=[REDACTED]"));
    }

    #[test]
    fn test_redaction_rules_password() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r#"(?i)(password|passwd|secret)["\s:=]+["\']?([^"\'`,\s}{]+)"#.to_string(),
                replacement: "$1=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = r#"{"password": "super_secret_123"}"#;
        let output = rules.redact(input);
        assert!(!output.contains("super_secret_123"));
    }

    #[test]
    fn test_redaction_rules_multiple_patterns() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "bearer".to_string(),
                pattern: r"Bearer\s+\S+".to_string(),
                replacement: "Bearer [REDACTED]".to_string(),
            },
            RedactionRule {
                name: "api_key".to_string(),
                pattern: r"api_key=\S+".to_string(),
                replacement: "api_key=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "Auth: Bearer xyz123 and api_key=abc456";
        let output = rules.redact(input);
        assert!(!output.contains("xyz123"));
        assert!(!output.contains("abc456"));
        assert!(output.contains("Bearer [REDACTED]"));
        assert!(output.contains("api_key=[REDACTED]"));
    }

    #[test]
    fn test_redaction_rules_invalid_regex() {
        let result = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "invalid".to_string(),
                pattern: "[invalid(regex".to_string(), // Invalid regex
                replacement: "[REDACTED]".to_string(),
            },
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_redaction_preserves_non_sensitive_data() {
        let rules = CompiledRedactionRules::new(&[
            RedactionRule {
                name: "passwords".to_string(),
                pattern: r"password=\S+".to_string(),
                replacement: "password=[REDACTED]".to_string(),
            },
        ]).expect("Should compile");

        let input = "user=john tool=read_file status=success";
        let output = rules.redact(input);
        assert_eq!(input, output); // No changes
    }

    #[tokio::test]
    async fn test_audit_logger_with_redaction() {
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
            redaction_rules: vec![
                RedactionRule {
                    name: "bearer".to_string(),
                    pattern: r"Bearer\s+\S+".to_string(),
                    replacement: "Bearer [REDACTED]".to_string(),
                },
            ],
            rotation: None,
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Log an entry with sensitive data
        let entry = AuditEntry::new(EventType::AuthFailure)
            .with_message("Invalid token: Bearer eyJhbGciOiJIUzI1NiJ9.xyz");
        logger.log(&entry);

        tokio::time::sleep(Duration::from_millis(100)).await;
        handle.shutdown().await;

        // Verify redaction was applied
        let contents = std::fs::read_to_string(&file_path).expect("Should read file");
        assert!(!contents.contains("eyJ"), "Token should be redacted");
        assert!(contents.contains("[REDACTED]"), "Should contain redaction marker");
    }

    // ========================================================================
    // Log Rotation Tests
    // ========================================================================

    #[test]
    fn test_rotating_file_writer_creation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(1024),
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let writer = RotatingFileWriter::new(log_path.clone(), config);
        assert!(writer.is_ok());
    }

    #[test]
    fn test_rotating_file_writer_size_trigger() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(100), // Small size to trigger rotation
            max_age_secs: None,
            max_backups: 3,
            compress: false,
        };

        let mut writer = RotatingFileWriter::new(log_path.clone(), config).expect("Should create");

        // Write enough data to trigger rotation
        for i in 0..20 {
            writer.write_line(&format!("Log line number {} with some padding", i)).expect("Should write");
        }
        writer.flush().expect("Should flush");

        // Check that backup files were created
        let files: Vec<_> = std::fs::read_dir(temp_dir.path())
            .expect("Should read dir")
            .filter_map(|e| e.ok())
            .collect();

        // Should have at least the current log file and one backup
        assert!(files.len() >= 2, "Should have rotated files, got {}", files.len());
    }

    #[test]
    fn test_rotating_file_writer_backup_cleanup() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("test.log");

        let config = LogRotationConfig {
            enabled: true,
            max_size_bytes: Some(50), // Very small to force multiple rotations
            max_age_secs: None,
            max_backups: 2, // Only keep 2 backups
            compress: false,
        };

        let mut writer = RotatingFileWriter::new(log_path.clone(), config).expect("Should create");

        // Write lots of data to trigger multiple rotations
        for i in 0..50 {
            writer.write_line(&format!("Log line {}", i)).expect("Should write");
        }
        writer.flush().expect("Should flush");

        // Count backup files (excluding main log file)
        let backup_count = std::fs::read_dir(temp_dir.path())
            .expect("Should read dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".2"))
            .count();

        // Should have at most max_backups backup files
        assert!(backup_count <= 2, "Should have at most 2 backups, got {}", backup_count);
    }

    #[tokio::test]
    async fn test_audit_logger_with_rotation() {
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let log_path = temp_dir.path().join("audit.log");

        let config = AuditConfig {
            enabled: true,
            file: Some(log_path.clone()),
            stdout: false,
            export_url: None,
            export_headers: HashMap::new(),
            export_batch_size: 100,
            export_interval_secs: 30,
            redaction_rules: Vec::new(),
            rotation: Some(LogRotationConfig {
                enabled: true,
                max_size_bytes: Some(500), // Small for testing
                max_age_secs: None,
                max_backups: 3,
                compress: false,
            }),
        };

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Log many entries to trigger rotation
        for i in 0..50 {
            logger.log_auth_success(&format!("user{}", i));
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
        handle.shutdown().await;

        // Verify that rotation happened (multiple files exist)
        let files: Vec<_> = std::fs::read_dir(temp_dir.path())
            .expect("Should read dir")
            .filter_map(|e| e.ok())
            .collect();

        assert!(files.len() >= 2, "Should have rotated files");
    }

    // ========================================================================
    // Original Tests (updated with new config fields)
    // ========================================================================

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
        let mut config = test_config();
        config.enabled = false;

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
        let mut config = test_config();
        config.enabled = false;

        let (logger, handle) = AuditLogger::with_tasks(&config).expect("Should create logger");

        // Should not panic when logging
        logger.log_auth_success("user1");

        // Shutdown should complete immediately for disabled logger
        handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_audit_logger_with_tasks_stdout_only() {
        let mut config = test_config();
        config.stdout = true;

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

        let mut config = test_config();
        config.file = Some(file_path.clone());

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
        let config = test_config();

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
        let mut config = test_config();
        config.enabled = false;

        // Test legacy with_export API
        let (logger, handle) = AuditLogger::with_export(&config).expect("Should create logger");
        assert!(handle.is_none()); // No shipper for disabled config

        logger.log_auth_success("user1");
    }

    #[tokio::test]
    async fn test_audit_logger_channel_full_behavior() {
        let temp_file = NamedTempFile::new().expect("Should create temp file");

        let mut config = test_config();
        config.file = Some(temp_file.path().to_path_buf());

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
