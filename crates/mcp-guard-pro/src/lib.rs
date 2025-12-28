//! MCP Guard Pro - Commercial features for mcp-guard
//!
//! This crate provides Pro tier features that require a valid license:
//!
//! - **OAuth 2.1 + PKCE**: Full OAuth authentication with PKCE support
//! - **JWT JWKS Mode**: Auto-refreshing JWKS key validation
//! - **HTTP Transport**: Connect to upstream MCP servers over HTTP
//! - **SSE Transport**: Server-Sent Events for streaming responses
//! - **Per-Identity Rate Limiting**: Rate limits tied to authenticated users
//!
//! # License
//!
//! This crate requires a valid Pro license key. Get one at https://mcp-guard.io/pricing
//!
//! # Usage
//!
//! ```rust,ignore
//! use mcp_guard_pro::license::ProLicense;
//!
//! // Validate license at startup
//! let license = ProLicense::validate("pro_xxx...")?;
//!
//! // Use Pro features
//! use mcp_guard_pro::auth::OAuthProvider;
//! use mcp_guard_pro::transport::{HttpTransport, SseTransport};
//! ```

pub mod license;

// Re-export Pro features from core (these will be gated in Phase 5)
pub mod auth {
    //! Pro authentication providers

    pub use mcp_guard_core::auth::OAuthAuthProvider;
    pub use mcp_guard_core::auth::JwtProvider;
}

pub mod transport {
    //! Pro transport types

    pub use mcp_guard_core::transport::HttpTransport;
    pub use mcp_guard_core::transport::SseTransport;
}

pub mod rate_limit {
    //! Pro rate limiting features

    // Per-identity rate limiting is already in core's RateLimitService
    pub use mcp_guard_core::rate_limit::RateLimitService;
}

/// Result type for Pro operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Pro tier operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("License error: {0}")]
    License(#[from] license::LicenseError),

    #[error("Core error: {0}")]
    Core(#[from] mcp_guard_core::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}
