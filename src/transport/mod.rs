//! MCP transport implementations

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

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
    _child: tokio::sync::Mutex<Child>,
    /// Background task writing messages to subprocess stdin
    writer_task: tokio::task::JoinHandle<()>,
    /// Background task reading messages from subprocess stdout
    reader_task: tokio::task::JoinHandle<()>,
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

        let (to_process_tx, mut to_process_rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);
        let (from_process_tx, from_process_rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);

        // Writer task with error tracking
        let writer_task = tokio::spawn(async move {
            let mut stdin = stdin;
            while let Some(msg) = to_process_rx.recv().await {
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
            tracing::debug!("Writer task exiting");
        });

        // Reader task with error tracking
        let reader_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
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
            tracing::debug!("Reader task exiting");
        });

        Ok(Self {
            tx: to_process_tx,
            rx: tokio::sync::Mutex::new(from_process_rx),
            _child: tokio::sync::Mutex::new(child),
            writer_task,
            reader_task,
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
    /// Reusable HTTP client with connection pooling
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
    /// Create a new HTTP transport
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            headers: HashMap::new(),
            timeout: std::time::Duration::from_secs(HTTP_REQUEST_TIMEOUT_SECS),
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
    /// Reusable HTTP client with connection pooling
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
        let (tx, rx) = mpsc::channel::<Message>(TRANSPORT_CHANNEL_SIZE);

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
    async fn test_http_transport_new() {
        let transport = HttpTransport::new("http://localhost:8080/mcp".to_string());
        assert_eq!(transport.url, "http://localhost:8080/mcp");
        assert!(transport.headers.is_empty());
    }

    #[tokio::test]
    async fn test_http_transport_with_config() {
        let mut headers = HashMap::new();
        headers.insert("X-Api-Key".to_string(), "secret".to_string());
        let transport = HttpTransport::with_config(
            "http://localhost:8080/mcp".to_string(),
            headers,
            60,
        );
        assert_eq!(transport.url, "http://localhost:8080/mcp");
        assert_eq!(transport.headers.get("X-Api-Key"), Some(&"secret".to_string()));
        assert_eq!(transport.timeout, std::time::Duration::from_secs(60));
    }

    #[tokio::test]
    async fn test_http_transport_success() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

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

        let transport = HttpTransport::new(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);
        
        transport.send(request).await.unwrap();
        let response = transport.receive().await.unwrap();
        
        assert!(response.result.is_some());
        assert_eq!(response.id, Some(serde_json::json!(1)));
    }

    #[tokio::test]
    async fn test_http_transport_server_error() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);
        
        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::Http(_))));
    }

    #[tokio::test]
    async fn test_http_transport_not_found() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);
        
        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::Http(_))));
        if let Err(TransportError::Http(msg)) = result {
            assert!(msg.contains("404"));
        }
    }

    #[tokio::test]
    async fn test_http_transport_invalid_json_response() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&mock_server)
            .await;

        let transport = HttpTransport::new(format!("{}/mcp", mock_server.uri()));
        let request = Message::request(1, "tools/list", None);
        
        let result = transport.send(request).await;
        assert!(matches!(result, Err(TransportError::InvalidMessage(_))));
    }

    #[tokio::test]
    async fn test_http_transport_receive_when_empty() {
        let transport = HttpTransport::new("http://localhost:8080/mcp".to_string());
        let result = transport.receive().await;
        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_http_transport_close() {
        let transport = HttpTransport::new("http://localhost:8080/mcp".to_string());
        let result = transport.close().await;
        assert!(result.is_ok());
    }

    // ------------------------------------------------------------------------
    // SseTransport Tests
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_sse_transport_connect() {
        let transport = SseTransport::connect("http://localhost:8080/sse".to_string()).await;
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_sse_transport_connect_with_config() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());
        let transport = SseTransport::connect_with_config(
            "http://localhost:8080/sse".to_string(),
            headers,
            60,
        ).await;
        assert!(transport.is_ok());
    }

    #[tokio::test]
    async fn test_sse_transport_json_fallback() {
        use wiremock::{Mock, MockServer, ResponseTemplate};
        use wiremock::matchers::{method, path};

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
                    .insert_header("Content-Type", "application/json")
            )
            .mount(&mock_server)
            .await;

        let transport = SseTransport::connect(format!("{}/sse", mock_server.uri())).await.unwrap();
        let request = Message::request(1, "test/method", None);
        
        transport.send(request).await.unwrap();
        let response = transport.receive().await.unwrap();
        
        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_sse_transport_close() {
        let transport = SseTransport::connect("http://localhost:8080/sse".to_string()).await.unwrap();
        let result = transport.close().await;
        assert!(result.is_ok());
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
}
