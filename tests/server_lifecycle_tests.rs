//! Server lifecycle integration tests for mcp-guard
//!
//! These tests verify app state creation and key component behavior.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use mcp_guard::{
    audit::AuditLogger,
    auth::{ApiKeyProvider, AuthProvider},
    cli::hash_api_key,
    config::{
        ApiKeyConfig, AuditConfig, Config, RateLimitConfig, ServerConfig, TracingConfig,
        TransportType, UpstreamConfig,
    },
    observability::init_metrics,
    rate_limit::RateLimitService,
    server::{new_oauth_state_store, AppState, PkceState},
};

/// Create a minimal test configuration
fn create_test_config(port: u16) -> Config {
    Config {
        server: ServerConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            // Use 'cat' directly as the command (no shell needed)
            command: Some("cat".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
        auth: mcp_guard::config::AuthConfig {
            api_keys: vec![ApiKeyConfig {
                id: "test-client".to_string(),
                key_hash: hash_api_key("test-api-key"),
                allowed_tools: vec![],
                rate_limit: None,
            }],
            jwt: None,
            oauth: None,
            mtls: None,
        },
        rate_limit: RateLimitConfig {
            enabled: false,
            requests_per_second: 10,
            burst_size: 20,
            tool_limits: Vec::new(),
        },
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
    }
}

/// Get a free port for testing
async fn get_free_port() -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().port()
}

// =============================================================================
// App State Creation Tests
// =============================================================================

#[tokio::test]
async fn test_app_state_creation_with_api_key() {
    let port = get_free_port().await;
    let config = create_test_config(port);

    // Create auth provider
    let auth_provider: Arc<dyn AuthProvider> =
        Arc::new(ApiKeyProvider::new(config.auth.api_keys.clone()));

    // Create rate limiter
    let rate_limiter = RateLimitService::new(&config.rate_limit);

    // Create audit logger (disabled for tests)
    let audit_logger = Arc::new(AuditLogger::disabled());

    // Create metrics handle
    let metrics_handle = init_metrics();

    // Create OAuth state store
    let oauth_state_store = new_oauth_state_store();

    // Create readiness state
    let ready = Arc::new(RwLock::new(true));

    // Create app state
    let state = Arc::new(AppState {
        config,
        auth_provider,
        rate_limiter,
        audit_logger,
        transport: None, // No transport for this test
        router: None,
        metrics_handle,
        oauth_provider: None,
        oauth_state_store,
        started_at: Instant::now(),
        ready,
        mtls_provider: None,
    });

    // Verify state is created correctly
    assert!(*state.ready.read().await);
    assert!(state.transport.is_none());
    assert!(state.router.is_none());
}

#[tokio::test]
async fn test_app_state_with_rate_limiting() {
    let port = get_free_port().await;
    let mut config = create_test_config(port);
    config.rate_limit.enabled = true;
    config.rate_limit.requests_per_second = 5;
    config.rate_limit.burst_size = 10;

    let rate_limiter = RateLimitService::new(&config.rate_limit);

    // Verify rate limiter is configured
    let result = rate_limiter.check("test-identity", None);
    assert!(result.allowed);
    assert_eq!(result.limit, 5);
}

#[tokio::test]
async fn test_app_state_readiness_transition() {
    let ready = Arc::new(RwLock::new(false));

    // Start not ready
    assert!(!*ready.read().await);

    // Transition to ready
    *ready.write().await = true;
    assert!(*ready.read().await);

    // Transition back to not ready (e.g., during shutdown)
    *ready.write().await = false;
    assert!(!*ready.read().await);
}

// =============================================================================
// Multi-Provider Authentication Tests
// =============================================================================

#[tokio::test]
async fn test_multi_provider_auth_fallback() {
    use mcp_guard::auth::MultiProvider;

    // Create two API key providers with different keys
    let provider1 = Arc::new(ApiKeyProvider::new(vec![ApiKeyConfig {
        id: "user1".to_string(),
        key_hash: hash_api_key("key1"),
        allowed_tools: vec![],
        rate_limit: None,
    }])) as Arc<dyn AuthProvider>;

    let provider2 = Arc::new(ApiKeyProvider::new(vec![ApiKeyConfig {
        id: "user2".to_string(),
        key_hash: hash_api_key("key2"),
        allowed_tools: vec![],
        rate_limit: None,
    }])) as Arc<dyn AuthProvider>;

    let multi_provider = MultiProvider::new(vec![provider1, provider2]);

    // First provider's key should work
    let result1 = multi_provider.authenticate("key1").await;
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().id, "user1");

    // Second provider's key should also work (fallback)
    let result2 = multi_provider.authenticate("key2").await;
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap().id, "user2");

    // Unknown key should fail
    let result3 = multi_provider.authenticate("unknown").await;
    assert!(result3.is_err());
}

// =============================================================================
// OAuth State Store Tests
// =============================================================================

#[tokio::test]
async fn test_oauth_state_store_operations() {
    let store = new_oauth_state_store();

    // Insert a state
    store.insert(
        "state1".to_string(),
        PkceState {
            code_verifier: "verifier1".to_string(),
            created_at: Instant::now(),
            client_ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
        },
    );

    // Verify it exists
    assert!(store.contains_key("state1"));

    // Retrieve and remove
    let removed = store.remove("state1");
    assert!(removed.is_some());
    let (key, pkce) = removed.unwrap();
    assert_eq!(key, "state1");
    assert_eq!(pkce.code_verifier, "verifier1");

    // Should be gone now
    assert!(!store.contains_key("state1"));
}

#[tokio::test]
async fn test_oauth_state_store_multiple_entries() {
    let store = new_oauth_state_store();

    // Insert multiple states
    for i in 0..5 {
        store.insert(
            format!("state{}", i),
            PkceState {
                code_verifier: format!("verifier{}", i),
                created_at: Instant::now(),
                client_ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            },
        );
    }

    // Verify all exist
    assert_eq!(store.len(), 5);

    // Remove one
    store.remove("state2");
    assert_eq!(store.len(), 4);
    assert!(!store.contains_key("state2"));
}

// =============================================================================
// Audit Logger Tests
// =============================================================================

#[tokio::test]
async fn test_audit_logger_disabled() {
    let logger = AuditLogger::disabled();

    // These should not panic even when disabled
    logger.log_auth_success("test-user");
    logger.log_auth_failure("test error");
    logger.log_rate_limited("test-user");
}

#[tokio::test]
async fn test_audit_logger_with_config() {
    let config = AuditConfig {
        enabled: false, // Keep disabled for test
        ..Default::default()
    };

    let (logger, handle) = AuditLogger::with_tasks(&config).unwrap();

    // Log some events
    logger.log_auth_success("user1");
    logger.log_auth_failure("error1");

    // Shutdown should not panic
    handle.shutdown().await;
}

// =============================================================================
// Config Validation Tests
// =============================================================================

#[test]
fn test_config_is_multi_server() {
    let mut config = create_test_config(3000);

    // Single server mode
    assert!(!config.is_multi_server());

    // Add servers for multi-server mode
    config
        .upstream
        .servers
        .push(mcp_guard::config::ServerRouteConfig {
            name: "server1".to_string(),
            path_prefix: "/api".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: false,
        });

    assert!(config.is_multi_server());
}

// =============================================================================
// Health Response Serialization Tests
// =============================================================================

#[test]
fn test_health_response_serialization() {
    #[derive(serde::Serialize)]
    struct HealthResponse {
        status: &'static str,
        version: &'static str,
        uptime_secs: u64,
    }

    let response = HealthResponse {
        status: "healthy",
        version: "1.0.0",
        uptime_secs: 3600,
    };

    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("\"status\":\"healthy\""));
    assert!(json.contains("\"version\":\"1.0.0\""));
    assert!(json.contains("\"uptime_secs\":3600"));
}

#[test]
fn test_ready_response_serialization() {
    #[derive(serde::Serialize)]
    struct ReadyResponse {
        ready: bool,
        version: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    }

    // Ready response (no reason)
    let ready = ReadyResponse {
        ready: true,
        version: "1.0.0",
        reason: None,
    };
    let json = serde_json::to_string(&ready).unwrap();
    assert!(json.contains("\"ready\":true"));
    assert!(!json.contains("reason")); // Skip when None

    // Not ready response (with reason)
    let not_ready = ReadyResponse {
        ready: false,
        version: "1.0.0",
        reason: Some("Transport not initialized".to_string()),
    };
    let json = serde_json::to_string(&not_ready).unwrap();
    assert!(json.contains("\"ready\":false"));
    assert!(json.contains("\"reason\":\"Transport not initialized\""));
}

// =============================================================================
// Rate Limiter Integration Tests
// =============================================================================

#[test]
fn test_rate_limiter_respects_identity_override() {
    let config = RateLimitConfig {
        enabled: true,
        requests_per_second: 10,
        burst_size: 20,
        tool_limits: Vec::new(),
    };

    let rate_limiter = RateLimitService::new(&config);

    // With identity-specific limit
    let result = rate_limiter.check("user1", Some(5));
    assert!(result.allowed);
    assert_eq!(result.limit, 5); // Should use override

    // Without identity-specific limit
    let result = rate_limiter.check("user2", None);
    assert!(result.allowed);
    assert_eq!(result.limit, 10); // Should use global
}

#[test]
fn test_rate_limiter_tracks_identities() {
    let config = RateLimitConfig {
        enabled: true,
        requests_per_second: 10,
        burst_size: 20,
        tool_limits: Vec::new(),
    };

    let rate_limiter = RateLimitService::new(&config);

    // Make requests from different identities
    rate_limiter.check("user1", None);
    rate_limiter.check("user2", None);
    rate_limiter.check("user3", None);

    // Should track 3 identities
    assert!(rate_limiter.tracked_identities() >= 3);
}

// =============================================================================
// Server Config Default Tests
// =============================================================================

#[test]
fn test_server_config_defaults() {
    let config = ServerConfig::default();
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 3000);
    assert!(config.tls.is_none());
}

#[test]
fn test_upstream_config_multi_server() {
    let config = UpstreamConfig {
        transport: TransportType::Stdio,
        command: None,
        args: vec![],
        url: None,
        servers: vec![
            mcp_guard::config::ServerRouteConfig {
                name: "server1".to_string(),
                path_prefix: "/api1".to_string(),
                transport: TransportType::Http,
                command: None,
                args: vec![],
                url: Some("http://localhost:8081".to_string()),
                strip_prefix: true,
            },
            mcp_guard::config::ServerRouteConfig {
                name: "server2".to_string(),
                path_prefix: "/api2".to_string(),
                transport: TransportType::Http,
                command: None,
                args: vec![],
                url: Some("http://localhost:8082".to_string()),
                strip_prefix: false,
            },
        ],
    };

    assert_eq!(config.servers.len(), 2);
    assert!(config.servers[0].strip_prefix);
    assert!(!config.servers[1].strip_prefix);
}
