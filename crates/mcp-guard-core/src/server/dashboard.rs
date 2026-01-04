use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::server::AppState;

#[derive(Debug, Serialize)]
pub struct Stats {
    pub active_connections: usize,
    pub total_requests: usize,
    pub uptime_secs: u64,
    pub api_calls: usize,
    pub success_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct LicenseInfo {
    pub tier: String,
    pub status: String,
    pub expires_at: Option<String>,
    pub max_users: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub rate_limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub provider: String,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct AuditLog {
    pub id: String,
    pub timestamp: String,
    pub actor: String,
    pub action: String,
    pub resource: String,
    pub status: String,
    pub ip_address: String,
}

pub fn dashboard_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/stats", get(get_stats))
        .route("/license", get(get_license))
        .route("/api-keys", get(list_api_keys).post(create_api_key))
        .route("/api-keys/:id", delete(revoke_api_key))
        .route("/users", get(list_users))
        .route("/audit", get(list_audit_logs))
}

async fn get_stats(State(state): State<Arc<AppState>>) -> Json<Stats> {
    Json(Stats {
        active_connections: 42, // Mocked
        total_requests: 1234567, // Mocked
        uptime_secs: state.started_at.elapsed().as_secs(),
        api_calls: 85432, // Mocked
        success_rate: 99.9, // Mocked
    })
}

async fn get_license() -> Json<LicenseInfo> {
    Json(LicenseInfo {
        tier: "pro".to_string(), // Mocked
        status: "active".to_string(),
        expires_at: Some("2026-12-31T23:59:59Z".to_string()),
        max_users: Some(10),
    })
}

async fn list_api_keys() -> Json<Vec<ApiKey>> {
    Json(vec![
        ApiKey {
            id: "1".to_string(),
            name: "Production API".to_string(),
            prefix: "mcp_prod...".to_string(),
            created_at: "2025-12-01T10:00:00Z".to_string(),
            last_used_at: Some("2026-01-04T10:30:00Z".to_string()),
            status: "active".to_string(),
        },
        ApiKey {
            id: "2".to_string(),
            name: "Development".to_string(),
            prefix: "mcp_dev...".to_string(),
            created_at: "2025-12-15T14:20:00Z".to_string(),
            last_used_at: None,
            status: "active".to_string(),
        },
    ])
}

async fn create_api_key(Json(payload): Json<CreateApiKeyRequest>) -> Json<CreateApiKeyResponse> {
    // In a real implementation, this would generate a key, hash it, and store it
    Json(CreateApiKeyResponse {
        id: uuid::Uuid::new_v4().to_string(),
        key: format!("mcp_{}_{}", payload.name.to_lowercase(), uuid::Uuid::new_v4().to_string().replace("-", "")),
    })
}

async fn revoke_api_key(Path(_id): Path<String>) -> Json<()> {
    // Mock revocation
    Json(())
}

async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: "1".to_string(),
            name: "Austin Green".to_string(),
            email: "austin@example.com".to_string(),
            role: "admin".to_string(),
            provider: "github".to_string(),
            created_at: "2025-11-20T08:00:00Z".to_string(),
            status: "active".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
            role: "user".to_string(),
            provider: "google".to_string(),
            created_at: "2025-12-05T11:30:00Z".to_string(),
            status: "active".to_string(),
        },
    ])
}

async fn list_audit_logs() -> Json<Vec<AuditLog>> {
    Json(vec![
        AuditLog {
            id: "1".to_string(),
            timestamp: "2026-01-04T10:00:00Z".to_string(),
            actor: "austin@example.com".to_string(),
            action: "api_key.create".to_string(),
            resource: "key_123".to_string(),
            status: "success".to_string(),
            ip_address: "192.168.1.1".to_string(),
        },
        AuditLog {
            id: "2".to_string(),
            timestamp: "2026-01-04T09:45:00Z".to_string(),
            actor: "system".to_string(),
            action: "rate_limit.exceeded".to_string(),
            resource: "user_456".to_string(),
            status: "warning".to_string(),
            ip_address: "10.0.0.5".to_string(),
        },
    ])
}
