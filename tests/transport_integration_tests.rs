use mcp_guard::transport::{Transport, StdioTransport};
use std::time::Duration;
use tokio::time::timeout;

mod common;

#[tokio::test]
async fn test_stdio_transport_echo() {
    let script_path = std::env::current_dir()
        .unwrap()
        .join("tests/fixtures/echo_server.sh");

    // Call the script directly (it has a shebang #!/bin/sh)
    // The script must be executable (chmod +x)
    let command = script_path.to_str().unwrap();
    let args: Vec<String> = vec![];

    // Use spawn_unchecked for test scripts (they are trusted test fixtures)
    let transport = StdioTransport::spawn_unchecked(command, &args).await.expect("Failed to create transport");
    
    // Start currently returns (tx, rx). But StdioTransport implements Transport trait?
    // Let's check line 263: impl Transport for StdioTransport.
    // Transport trait has send() and receive().
    
    // Line 149 struct shows it holds tx, rx.
    // Wait, StdioTransport::spawn returns `Result<Self, ...>`.
    // It starts the tasks internally in `spawn` (lines 186 and 213).
    // So I don't need to call `.start()`.
    // And `tx` and `rx` are private fields of `StdioTransport`.
    // I should use `Transport` trait methods `send` and `receive`.
    
    use mcp_guard::transport::Message;

    // Send a message
    // Message::request(id, method, params)
    let msg = Message::request(1, "ping", None);
    
    transport.send(msg.clone()).await.expect("Failed to send");

    // Expect echo back
    // Echo server echoes the line. 
    // The reader task reads the line and parses as JSON.
    // If echo server just echoes the JSON string, it should parse back as the same message.
    
    let received = timeout(Duration::from_secs(2), transport.receive())
        .await
        .expect("Timeout waiting for response")
        .expect("Channel closed");
        
    assert_eq!(received.id, msg.id);
    assert_eq!(received.method, msg.method);
}

#[tokio::test]
async fn test_stdio_transport_bad_command() {
    // This should fail at spawn time or shortly after?
    // spawn calls Command::spawn(). If command not found, it returns Error (line 169).
    let result = StdioTransport::spawn("/path/to/nonexistent/command", &[]).await;
    
    assert!(result.is_err());
}
