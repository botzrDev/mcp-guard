//! Mock implementations for testing
//!
//! This module provides mock implementations of core traits for unit testing
//! without requiring real network connections or subprocess spawning.

use crate::auth::{AuthError, AuthProvider, Identity};
use crate::transport::{Message, Transport, TransportError};
use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// ============================================================================
// MockTransport
// ============================================================================

/// A mock transport for testing that records sent messages and returns
/// pre-configured responses.
#[derive(Clone)]
pub struct MockTransport {
    sent_messages: Arc<Mutex<Vec<Message>>>,
    pending_responses: Arc<Mutex<VecDeque<Result<Message, TransportError>>>>,
}

impl MockTransport {
    /// Create a new mock transport with no pending responses.
    pub fn new() -> Self {
        Self {
            sent_messages: Arc::new(Mutex::new(Vec::new())),
            pending_responses: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Queue a successful response to be returned by the next `receive()` call.
    pub fn push_response(&self, message: Message) {
        self.pending_responses
            .lock()
            .unwrap()
            .push_back(Ok(message));
    }

    /// Queue an error to be returned by the next `receive()` call.
    pub fn push_error(&self, error: TransportError) {
        self.pending_responses
            .lock()
            .unwrap()
            .push_back(Err(error));
    }

    /// Take all sent messages, clearing the internal buffer.
    pub fn take_sent_messages(&self) -> Vec<Message> {
        let mut sent = self.sent_messages.lock().unwrap();
        std::mem::take(&mut *sent)
    }

    /// Get the count of messages sent through this transport.
    pub fn sent_count(&self) -> usize {
        self.sent_messages.lock().unwrap().len()
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn send(&self, message: Message) -> Result<(), TransportError> {
        self.sent_messages.lock().unwrap().push(message);
        Ok(())
    }

    async fn receive(&self) -> Result<Message, TransportError> {
        let mut responses = self.pending_responses.lock().unwrap();
        if let Some(response) = responses.pop_front() {
            response
        } else {
            Err(TransportError::ConnectionClosed)
        }
    }

    async fn close(&self) -> Result<(), TransportError> {
        Ok(())
    }

    fn transport_type(&self) -> &'static str {
        "mock"
    }
}

// ============================================================================
// MockAuthProvider
// ============================================================================

/// A mock auth provider for testing authentication flows.
#[derive(Clone)]
pub struct MockAuthProvider {
    /// If Some, all authenticate calls return this identity. If None, returns error.
    valid_identity: Arc<Mutex<Option<Identity>>>,
    /// Custom error message to return when authentication fails
    error_message: Arc<Mutex<String>>,
}

impl MockAuthProvider {
    /// Create a mock provider that rejects all tokens.
    pub fn rejecting() -> Self {
        Self {
            valid_identity: Arc::new(Mutex::new(None)),
            error_message: Arc::new(Mutex::new("Invalid token".to_string())),
        }
    }

    /// Create a mock provider that accepts all tokens with the given identity.
    pub fn accepting(identity: Identity) -> Self {
        Self {
            valid_identity: Arc::new(Mutex::new(Some(identity))),
            error_message: Arc::new(Mutex::new(String::new())),
        }
    }

    /// Set the identity to return for successful authentication.
    pub fn set_identity(&self, identity: Identity) {
        *self.valid_identity.lock().unwrap() = Some(identity);
    }

    /// Clear the identity, causing all authentication to fail.
    pub fn clear_identity(&self) {
        *self.valid_identity.lock().unwrap() = None;
    }

    /// Set the error message to return on failed authentication.
    pub fn set_error_message(&self, msg: impl Into<String>) {
        *self.error_message.lock().unwrap() = msg.into();
    }
}

#[async_trait]
impl AuthProvider for MockAuthProvider {
    async fn authenticate(&self, _token: &str) -> Result<Identity, AuthError> {
        let identity = self.valid_identity.lock().unwrap().clone();
        match identity {
            Some(id) => Ok(id),
            None => {
                let msg = self.error_message.lock().unwrap().clone();
                Err(AuthError::InvalidJwt(msg))
            }
        }
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_transport_send_receive() {
        let transport = MockTransport::new();

        // Queue a response
        let response = Message::response(serde_json::json!(1), serde_json::json!({"status": "ok"}));
        transport.push_response(response);

        // Send a message
        let request = Message::request(1, "test/method", None);
        transport.send(request).await.unwrap();

        // Verify sent message
        let sent = transport.take_sent_messages();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].method, Some("test/method".to_string()));

        // Receive the queued response
        let received = transport.receive().await.unwrap();
        assert!(received.result.is_some());
    }

    #[tokio::test]
    async fn test_mock_transport_connection_closed() {
        let transport = MockTransport::new();

        // No responses queued, should return ConnectionClosed
        let result = transport.receive().await;
        assert!(matches!(result, Err(TransportError::ConnectionClosed)));
    }

    #[tokio::test]
    async fn test_mock_transport_error_response() {
        let transport = MockTransport::new();

        // Queue an error
        transport.push_error(TransportError::Timeout);

        let result = transport.receive().await;
        assert!(matches!(result, Err(TransportError::Timeout)));
    }

    #[tokio::test]
    async fn test_mock_auth_provider_accepting() {
        let identity = Identity {
            id: "test-user".to_string(),
            name: Some("Test User".to_string()),
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };
        let provider = MockAuthProvider::accepting(identity.clone());

        let result = provider.authenticate("any-token").await.unwrap();
        assert_eq!(result.id, "test-user");
    }

    #[tokio::test]
    async fn test_mock_auth_provider_rejecting() {
        let provider = MockAuthProvider::rejecting();

        let result = provider.authenticate("any-token").await;
        assert!(result.is_err());
    }
}
