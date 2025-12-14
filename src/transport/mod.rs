//! MCP transport implementations

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

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
}

/// MCP JSON-RPC message
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

impl Message {
    pub fn request(id: impl Into<serde_json::Value>, method: &str, params: Option<serde_json::Value>) -> Self {
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
}

/// Stdio transport for communicating with a subprocess
pub struct StdioTransport {
    tx: mpsc::Sender<Message>,
    rx: tokio::sync::Mutex<mpsc::Receiver<Message>>,
    _child: tokio::sync::Mutex<Child>,
}

impl StdioTransport {
    pub async fn spawn(command: &str, args: &[String]) -> Result<Self, TransportError> {
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

        let (to_process_tx, mut to_process_rx) = mpsc::channel::<Message>(32);
        let (from_process_tx, from_process_rx) = mpsc::channel::<Message>(32);

        // Writer task
        tokio::spawn(async move {
            let mut stdin = stdin;
            while let Some(msg) = to_process_rx.recv().await {
                let json = match serde_json::to_string(&msg) {
                    Ok(j) => j,
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to serialize MCP message, dropping");
                        continue;
                    }
                };
                if stdin.write_all(json.as_bytes()).await.is_err() {
                    break;
                }
                if stdin.write_all(b"\n").await.is_err() {
                    break;
                }
                if stdin.flush().await.is_err() {
                    break;
                }
            }
        });

        // Reader task
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                    if from_process_tx.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            tx: to_process_tx,
            rx: tokio::sync::Mutex::new(from_process_rx),
            _child: tokio::sync::Mutex::new(child),
        })
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
        let mut child = self._child.lock().await;
        child.kill().await?;
        Ok(())
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
pub struct HttpTransport {
    /// HTTP client
    client: reqwest::Client,
    /// Base URL of the upstream MCP server
    url: String,
    /// Additional headers to include in requests
    headers: HashMap<String, String>,
    /// Request timeout
    timeout: std::time::Duration,
    /// Pending responses (for async receive pattern)
    pending_responses: tokio::sync::Mutex<Vec<Message>>,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            headers: HashMap::new(),
            timeout: std::time::Duration::from_secs(30),
            pending_responses: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    /// Create a new HTTP transport with custom configuration
    pub fn with_config(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            headers,
            timeout: std::time::Duration::from_secs(timeout_secs),
            pending_responses: tokio::sync::Mutex::new(Vec::new()),
        }
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

        let response = request
            .json(message)
            .send()
            .await
            .map_err(|e| {
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
                status, body
            )));
        }

        let response_message: Message = response
            .json()
            .await
            .map_err(|e| TransportError::InvalidMessage(e.to_string()))?;

        Ok(response_message)
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        // For HTTP transport, we send and immediately queue the response
        let response = self.send_request(&message).await?;
        self.pending_responses.lock().await.push(response);
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
/// The SSE format follows the standard:
/// ```text
/// event: message
/// data: {"jsonrpc": "2.0", "id": 1, "result": {...}}
/// ```
pub struct SseTransport {
    /// HTTP client
    client: reqwest::Client,
    /// Base URL of the upstream MCP server
    url: String,
    /// Additional headers to include in requests
    headers: HashMap<String, String>,
    /// Request timeout for the initial connection
    timeout: std::time::Duration,
    /// Channel for receiving SSE messages
    rx: tokio::sync::Mutex<mpsc::Receiver<Message>>,
    /// Channel for sending messages to SSE stream handler
    tx: mpsc::Sender<Message>,
}

impl SseTransport {
    /// Create a new SSE transport
    pub async fn connect(url: String) -> Result<Self, TransportError> {
        Self::connect_with_config(url, HashMap::new(), 30).await
    }

    /// Create a new SSE transport with custom configuration
    pub async fn connect_with_config(
        url: String,
        headers: HashMap<String, String>,
        timeout_secs: u64,
    ) -> Result<Self, TransportError> {
        let (tx, rx) = mpsc::channel::<Message>(32);

        Ok(Self {
            client: reqwest::Client::new(),
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

        let response = request
            .json(message)
            .send()
            .await
            .map_err(|e| {
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
                status, body
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
                    bytes_stream.map(|r| r.map_err(std::io::Error::other))
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
                                data_buffer.push_str(data.trim());
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
}
