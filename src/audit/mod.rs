//! Audit logging for mcp-guard

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

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

/// Audit logger
pub struct AuditLogger {
    enabled: bool,
    stdout: bool,
    file: Option<Mutex<std::fs::File>>,
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
        })
    }

    /// Create a disabled audit logger
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            stdout: false,
            file: None,
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
