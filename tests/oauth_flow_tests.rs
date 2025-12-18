//! OAuth Authorization Code Flow Integration Tests
//!
//! Tests the complete OAuth 2.1 authorization code flow with PKCE,
//! including the /oauth/authorize and /oauth/callback endpoints.
//! Uses wiremock to simulate OAuth provider token endpoints.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use mcp_guard::{
    audit::AuditLogger,
    auth::{ApiKeyProvider, OAuthAuthProvider},
    config::{AuditConfig, Config, OAuthConfig, OAuthProvider as OAuthProviderType, RateLimitConfig, TracingConfig, TransportType, UpstreamConfig},
    observability::create_metrics_handle,
    rate_limit::RateLimitService,
    server::{build_router, new_oauth_state_store, AppState, PkceState},
    transport::StdioTransport,
};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tower::ServiceExt;
use wiremock::{
    matchers::{body_string_contains, method, path},
    Mock, MockServer, ResponseTemplate,
};

/// Helper to create a minimal valid Config
fn create_test_config() -> Config {
    Config {
        server: Default::default(),
        auth: Default::default(),
        rate_limit: RateLimitConfig::default(),
        audit: AuditConfig::default(),
        tracing: TracingConfig::default(),
        upstream: UpstreamConfig {
            transport: TransportType::Stdio,
            command: Some("echo".to_string()),
            args: vec![],
            url: None,
            servers: vec![],
        },
    }
}

/// Helper to create an OAuthConfig pointing to a mock server
fn create_oauth_config(mock_server_uri: &str) -> OAuthConfig {
    OAuthConfig {
        provider: OAuthProviderType::Custom,
        client_id: "test_client_id".to_string(),
        client_secret: Some("test_client_secret".to_string()),
        authorization_url: Some(format!("{}/authorize", mock_server_uri)),
        token_url: Some(format!("{}/token", mock_server_uri)),
        introspection_url: Some(format!("{}/introspect", mock_server_uri)),
        userinfo_url: Some(format!("{}/userinfo", mock_server_uri)),
        redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string()],
        user_id_claim: "sub".to_string(),
        scope_tool_mapping: HashMap::new(),
    }
}

// =============================================================================
// OAuth State Management Tests
// =============================================================================

/// Test that oauth_authorize stores state correctly and can be retrieved
#[tokio::test]
async fn test_oauth_authorize_stores_pkce_state() {
    let mock_server = MockServer::start().await;

    let config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: oauth_state_store.clone(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Call /oauth/authorize
    let mut request = Request::builder()
        .uri("/oauth/authorize")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should redirect
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    // State should be stored
    assert_eq!(oauth_state_store.len(), 1);

    // Extract state from redirect location and verify it exists in store
    let location = response.headers().get("location").unwrap().to_str().unwrap();
    let state_param = location
        .split("state=")
        .nth(1)
        .unwrap()
        .split('&')
        .next()
        .unwrap();

    assert!(oauth_state_store.contains_key(state_param));
}

/// Test that oauth_callback with valid state and code exchanges tokens successfully
#[tokio::test]
async fn test_oauth_callback_successful_token_exchange() {
    let mock_server = MockServer::start().await;

    // Mock the token endpoint
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("grant_type=authorization_code"))
        .and(body_string_contains("code=valid_auth_code"))
        .and(body_string_contains("code_verifier="))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "test_access_token_12345",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "test_refresh_token",
            "scope": "openid profile"
        })))
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    // Pre-populate the state store with a valid PKCE state
    let test_state = "test_oauth_state_parameter";
    let test_verifier = "test_code_verifier_12345678901234567890123456789012345678901234";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: test_verifier.to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: oauth_state_store.clone(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Call /oauth/callback with valid state and code
    let uri = format!(
        "/oauth/callback?code=valid_auth_code&state={}",
        test_state
    );
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should return 200 with token response
    assert_eq!(response.status(), StatusCode::OK);

    // Parse response body
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["access_token"], "test_access_token_12345");
    assert_eq!(json["token_type"], "Bearer");
    assert_eq!(json["expires_in"], 3600);
    assert_eq!(json["refresh_token"], "test_refresh_token");
    assert_eq!(json["scope"], "openid profile");

    // State should be consumed (removed from store)
    assert!(!oauth_state_store.contains_key(test_state));
}

/// Test that oauth_callback rejects requests from different IP (state fixation protection)
#[tokio::test]
async fn test_oauth_callback_rejects_ip_mismatch() {
    let mock_server = MockServer::start().await;

    let config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    // Pre-populate state from a different IP address
    let test_state = "test_state_different_ip";
    let original_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)); // Different IP

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: original_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: oauth_state_store.clone(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Call from a different IP (127.0.0.1 vs 192.168.1.100)
    let uri = format!("/oauth/callback?code=code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)), // Different IP
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should be unauthorized due to IP mismatch
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test that oauth_callback rejects missing authorization code
#[tokio::test]
async fn test_oauth_callback_rejects_missing_code() {
    let mock_server = MockServer::start().await;

    let config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_no_code";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: oauth_state_store.clone(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    // Call without code parameter
    let uri = format!("/oauth/callback?state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should be unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test token exchange failure (provider returns error)
#[tokio::test]
async fn test_oauth_callback_token_exchange_failure() {
    let mock_server = MockServer::start().await;

    // Mock token endpoint to return an error
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "invalid_grant",
            "error_description": "The authorization code has expired"
        })))
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_token_fail";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store: oauth_state_store.clone(),
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let uri = format!("/oauth/callback?code=expired_code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should return unauthorized due to token exchange failure
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// Test token response with minimal fields (only access_token)
#[tokio::test]
async fn test_oauth_callback_minimal_token_response() {
    let mock_server = MockServer::start().await;

    // Mock token endpoint with minimal response
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "minimal_token"
            // No token_type, expires_in, refresh_token, or scope
        })))
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_minimal";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store,
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let uri = format!("/oauth/callback?code=code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["access_token"], "minimal_token");
    assert_eq!(json["token_type"], "Bearer"); // Default
    assert!(json.get("expires_in").is_none() || json["expires_in"].is_null());
}

/// Test token response missing access_token returns error
#[tokio::test]
async fn test_oauth_callback_missing_access_token() {
    let mock_server = MockServer::start().await;

    // Mock token endpoint with invalid response (no access_token)
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "token_type": "Bearer",
            "expires_in": 3600
            // Missing access_token!
        })))
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_no_token";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store,
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let uri = format!("/oauth/callback?code=code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    // Should return 500 (internal server error) due to missing access_token
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

/// Test confidential client (with client_secret) token exchange
#[tokio::test]
async fn test_oauth_callback_confidential_client() {
    let mock_server = MockServer::start().await;

    // Mock token endpoint expecting client_secret
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("client_secret=test_client_secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "confidential_token"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let oauth_config = create_oauth_config(&mock_server.uri());
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_confidential";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store,
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let uri = format!("/oauth/callback?code=code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["access_token"], "confidential_token");
}

/// Test public client (without client_secret) token exchange
#[tokio::test]
async fn test_oauth_callback_public_client() {
    let mock_server = MockServer::start().await;

    // Mock token endpoint NOT expecting client_secret
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "public_token"
        })))
        .mount(&mock_server)
        .await;

    let mut config = create_test_config();
    let mut oauth_config = create_oauth_config(&mock_server.uri());
    oauth_config.client_secret = None; // Public client
    config.auth.oauth = Some(oauth_config.clone());

    let oauth_provider = OAuthAuthProvider::new(oauth_config).unwrap();
    let oauth_state_store = new_oauth_state_store();

    let test_state = "test_state_public";
    let client_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    oauth_state_store.insert(
        test_state.to_string(),
        PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip,
        },
    );

    let state = Arc::new(AppState {
        config: config.clone(),
        auth_provider: Arc::new(ApiKeyProvider::new(vec![])),
        rate_limiter: RateLimitService::new(&config.rate_limit),
        audit_logger: Arc::new(AuditLogger::new(&config.audit).unwrap()),
        transport: Some(Arc::new(StdioTransport::spawn("echo", &[]).await.unwrap())),
        router: None,
        metrics_handle: create_metrics_handle(),
        oauth_provider: Some(Arc::new(oauth_provider)),
        oauth_state_store,
        started_at: Instant::now(),
        ready: Arc::new(RwLock::new(true)),
        mtls_provider: None,
    });

    let app = build_router(state);

    let uri = format!("/oauth/callback?code=code&state={}", test_state);
    let mut request = Request::builder()
        .uri(&uri)
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(axum::extract::ConnectInfo(
        SocketAddr::from(([127, 0, 0, 1], 3000)),
    ));

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
