# Transport Trait Guide

This guide explains how to implement custom transport adapters for connecting mcp-guard to upstream MCP servers.

## Overview

Transports handle the bidirectional communication between mcp-guard and upstream MCP servers. The `Transport` trait provides a unified interface for different communication methods (stdio, HTTP, SSE, WebSocket, etc.).

## The Transport Trait

```rust
// src/transport/mod.rs:58-74

#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message to the upstream server
    async fn send(&self, message: Message) -> Result<(), TransportError>;

    /// Receive a message from the upstream server
    async fn receive(&self) -> Result<Message, TransportError>;

    /// Close the transport gracefully
    async fn close(&self) -> Result<(), TransportError>;

    /// Check if the transport is healthy
    fn is_healthy(&self) -> bool {
        true // Default implementation
    }
}
```

## The Message Struct

Messages follow the JSON-RPC 2.0 specification used by MCP:

```rust
// src/transport/mod.rs:30-56

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub jsonrpc: String,                  // Always "2.0"
    pub id: Option<serde_json::Value>,    // Request ID (null for notifications)
    pub method: Option<String>,           // Method name (requests only)
    pub params: Option<serde_json::Value>,// Parameters (requests only)
    pub result: Option<serde_json::Value>,// Result (responses only)
    pub error: Option<serde_json::Value>, // Error (error responses only)
}

impl Message {
    /// Create a new request message
    pub fn request(id: u64, method: &str, params: Option<serde_json::Value>) -> Self;

    /// Create a response message
    pub fn response(id: u64, result: serde_json::Value) -> Self;

    /// Create an error response
    pub fn error_response(id: u64, code: i32, message: &str) -> Self;

    /// Check if this is a request (has method)
    pub fn is_request(&self) -> bool;

    /// Check if this is a response (has result or error)
    pub fn is_response(&self) -> bool;
}
```

## TransportError Types

```rust
// src/transport/mod.rs:76-95

pub enum TransportError {
    Io(String),           // I/O operation failed
    Serialization(String),// JSON serialization/deserialization error
    ConnectionClosed,     // Remote end closed connection
    Timeout,              // Operation timed out
    InvalidMessage(String),// Malformed message
    Ssrf(String),         // SSRF validation failed
    CommandInjection(String), // Command validation failed
    Internal(String),     // Internal error
}
```

## Implementing a Custom Transport

### Step 1: Define Your Transport Struct

```rust
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::transport::{Message, Transport, TransportError};

pub struct WebSocketTransport {
    url: String,
    connection: Mutex<Option<WebSocketStream>>,
    max_reconnect_attempts: usize,
}
```

### Step 2: Implement Construction with Security

```rust
impl WebSocketTransport {
    /// Create a new WebSocket transport with SSRF validation
    pub fn new(url: String) -> Result<Self, TransportError> {
        // Validate URL to prevent SSRF attacks
        validate_url_for_ssrf(&url)?;

        Ok(Self {
            url,
            connection: Mutex::new(None),
            max_reconnect_attempts: 3,
        })
    }

    /// Create without SSRF validation (for testing only)
    #[cfg(test)]
    pub fn new_unchecked(url: String) -> Self {
        Self {
            url,
            connection: Mutex::new(None),
            max_reconnect_attempts: 3,
        }
    }

    /// Connect to the WebSocket server
    async fn connect(&self) -> Result<(), TransportError> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.url)
            .await
            .map_err(|e| TransportError::Io(e.to_string()))?;

        let mut conn = self.connection.lock().await;
        *conn = Some(ws_stream);
        Ok(())
    }
}
```

### Step 3: Implement the Trait

```rust
#[async_trait]
impl Transport for WebSocketTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        let json = serde_json::to_string(&message)
            .map_err(|e| TransportError::Serialization(e.to_string()))?;

        let mut conn = self.connection.lock().await;
        let ws = conn.as_mut()
            .ok_or(TransportError::ConnectionClosed)?;

        ws.send(tungstenite::Message::Text(json))
            .await
            .map_err(|e| TransportError::Io(e.to_string()))?;

        Ok(())
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        let mut conn = self.connection.lock().await;
        let ws = conn.as_mut()
            .ok_or(TransportError::ConnectionClosed)?;

        let msg = ws.next()
            .await
            .ok_or(TransportError::ConnectionClosed)?
            .map_err(|e| TransportError::Io(e.to_string()))?;

        match msg {
            tungstenite::Message::Text(text) => {
                serde_json::from_str(&text)
                    .map_err(|e| TransportError::Serialization(e.to_string()))
            }
            _ => Err(TransportError::InvalidMessage("Expected text frame".into()))
        }
    }

    async fn close(&self) -> Result<(), TransportError> {
        let mut conn = self.connection.lock().await;
        if let Some(ws) = conn.take() {
            // Send close frame
            ws.close(None).await.ok();
        }
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        // Check if connection exists and is open
        // Note: This is a sync method, so we can't lock the mutex
        // Consider using an AtomicBool for the health flag
        true
    }
}
```

### Step 4: Add Configuration

```rust
// In src/config/mod.rs

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    Stdio,
    Http,
    Sse,
    WebSocket,  // New transport type
}
```

### Step 5: Wire Into Bootstrap

```rust
// In src/main.rs bootstrap()

TransportType::WebSocket => {
    let url = config.upstream.url.as_ref()
        .ok_or_else(|| anyhow!("WebSocket transport requires 'url'"))?
        .clone();
    tracing::info!(url = %url, "Using WebSocket transport");
    Arc::new(WebSocketTransport::new(url)?)
}
```

## Existing Transport Implementations

### StdioTransport

Spawns a subprocess and communicates via stdin/stdout:

```rust
// src/transport/mod.rs:254-420

pub struct StdioTransport {
    write_tx: mpsc::Sender<String>,    // Channel to writer task
    read_rx: Mutex<mpsc::Receiver<String>>,  // Channel from reader task
    writer_handle: JoinHandle<()>,      // For task supervision
    reader_handle: JoinHandle<()>,
    is_closed: AtomicBool,
}

impl StdioTransport {
    pub async fn spawn(command: &str, args: &[String]) -> Result<Self, TransportError> {
        // 1. Validate command for injection attacks
        validate_command_for_injection(command)?;

        // 2. Spawn the subprocess
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // 3. Start reader/writer background tasks
        // ... channel setup and task spawning
    }
}
```

**Key features:**
- Command injection prevention
- Background reader/writer tasks for non-blocking I/O
- `JoinHandle` storage for task supervision
- Graceful shutdown on close

### HttpTransport

Sends JSON-RPC messages as HTTP POST requests:

```rust
// src/transport/mod.rs:422-500

pub struct HttpTransport {
    url: String,
    client: reqwest::Client,
    pending_responses: DashMap<u64, oneshot::Sender<Message>>,
}

impl HttpTransport {
    pub fn new(url: String) -> Result<Self, TransportError> {
        validate_url_for_ssrf(&url)?;
        // ...
    }
}
```

**Key features:**
- SSRF validation on construction
- Request/response correlation via message ID
- Configurable timeout (default: 30s)

### SseTransport

Server-Sent Events for streaming responses:

```rust
// src/transport/mod.rs:502-620

pub struct SseTransport {
    url: String,
    client: reqwest::Client,
    event_rx: Mutex<mpsc::Receiver<Message>>,
    event_task: JoinHandle<()>,
}

impl SseTransport {
    pub async fn connect(url: String) -> Result<Self, TransportError> {
        validate_url_for_ssrf(&url)?;
        // Start SSE event listener task
        // ...
    }
}
```

**Key features:**
- SSRF validation
- Background event listener task
- Automatic reconnection (configurable)

## Security Considerations

### SSRF Protection

HTTP-based transports must validate URLs to prevent Server-Side Request Forgery:

```rust
// src/transport/mod.rs:133-188

pub fn validate_url_for_ssrf(url: &str) -> Result<(), TransportError> {
    let parsed = Url::parse(url)
        .map_err(|e| TransportError::Ssrf(format!("Invalid URL: {}", e)))?;

    // Only allow http/https schemes
    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(TransportError::Ssrf("Only http/https allowed".into()))
    }

    // Block private/internal IP ranges
    if let Some(host) = parsed.host_str() {
        if is_internal_host(host) {
            return Err(TransportError::Ssrf("Internal hosts blocked".into()));
        }
    }

    // Block cloud metadata endpoints
    if is_cloud_metadata_url(url) {
        return Err(TransportError::Ssrf("Cloud metadata blocked".into()));
    }

    Ok(())
}

fn is_internal_host(host: &str) -> bool {
    // Block: localhost, 127.x.x.x, 10.x.x.x, 172.16-31.x.x, 192.168.x.x
    // Block: 169.254.x.x (link-local), ::1, fe80::, etc.
}

fn is_cloud_metadata_url(url: &str) -> bool {
    // Block: 169.254.169.254, metadata.google.internal, etc.
}
```

### Command Injection Prevention

Stdio transport validates commands to prevent shell injection:

```rust
// src/transport/mod.rs:190-252

pub fn validate_command_for_injection(command: &str) -> Result<(), TransportError> {
    // Block shell metacharacters
    let dangerous_chars = ['|', '&', ';', '$', '`', '(', ')', '{', '}', '<', '>', '\n', '\r'];
    for c in dangerous_chars {
        if command.contains(c) {
            return Err(TransportError::CommandInjection(
                format!("Command contains dangerous character: {}", c)
            ));
        }
    }

    // Block direct shell execution
    let shells = ["sh", "bash", "zsh", "fish", "ksh", "csh", "dash", "ash"];
    let base_command = Path::new(command)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(command);

    if shells.contains(&base_command) {
        return Err(TransportError::CommandInjection(
            "Direct shell execution not allowed".into()
        ));
    }

    Ok(())
}
```

## Task Supervision Pattern

For transports that spawn background tasks:

```rust
pub struct SupervisedTransport {
    // Store JoinHandles to monitor task health
    reader_handle: JoinHandle<()>,
    writer_handle: JoinHandle<()>,
}

impl Transport for SupervisedTransport {
    fn is_healthy(&self) -> bool {
        // Check if tasks are still running
        !self.reader_handle.is_finished() && !self.writer_handle.is_finished()
    }
}
```

## Testing Patterns

### MockTransport

```rust
// src/mocks.rs

pub struct MockTransport {
    pub messages_to_receive: Mutex<VecDeque<Message>>,
    pub sent_messages: Mutex<Vec<Message>>,
}

#[async_trait]
impl Transport for MockTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        self.sent_messages.lock().await.push(message);
        Ok(())
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        self.messages_to_receive.lock().await
            .pop_front()
            .ok_or(TransportError::ConnectionClosed)
    }

    async fn close(&self) -> Result<(), TransportError> {
        Ok(())
    }
}
```

### Integration Test Example

```rust
#[tokio::test]
async fn test_http_transport_roundtrip() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, body_json};

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "ping"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "pong"
        })))
        .mount(&mock_server)
        .await;

    let transport = HttpTransport::new_unchecked(mock_server.uri());
    transport.send(Message::request(1, "ping", None)).await.unwrap();

    let response = transport.receive().await.unwrap();
    assert_eq!(response.result, Some(serde_json::json!("pong")));
}
```

## Best Practices

1. **Always validate URLs**: Use `validate_url_for_ssrf()` for any HTTP-based transport
2. **Always validate commands**: Use `validate_command_for_injection()` for subprocess transports
3. **Provide `*_unchecked` variants**: For testing with localhost/mock servers
4. **Store task handles**: Enable `is_healthy()` checks and supervision
5. **Use channels for async I/O**: Prevent blocking the async runtime
6. **Implement graceful shutdown**: Clean up resources in `close()`
7. **Log transport errors**: But sanitize messages (no internal paths/credentials)
