use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DbApiKey {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub key_hash: String,
    pub name: Option<String>,
    pub allowed_tools: Option<serde_json::Value>,
    pub rate_limit: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn users(&self) -> UserRepository {
        UserRepository { pool: self.pool.clone() }
    }

    pub fn api_keys(&self) -> ApiKeyRepository {
        ApiKeyRepository { pool: self.pool.clone() }
    }
}

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn create(&self, id: &str, email: &str, role: &str) -> Result<DbUser, sqlx::Error> {
        let user = sqlx::query_as::<_, DbUser>(
            r#"
            INSERT INTO users (id, email, role)
            VALUES ($1, $2, $3)
            RETURNING id, email, name, role, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(email)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<DbUser>, sqlx::Error> {
        sqlx::query_as::<_, DbUser>(
            r#"SELECT id, email, name, role, created_at, updated_at FROM users WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }
}

pub struct ApiKeyRepository {
    pool: PgPool,
}

impl ApiKeyRepository {
    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<DbApiKey>, sqlx::Error> {
        sqlx::query_as::<_, DbApiKey>(
            r#"SELECT id, user_id, key_hash, name, allowed_tools, rate_limit, expires_at, created_at FROM api_keys WHERE key_hash = $1"#
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await
    }
}
