// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: LicenseRef-Commercial
//
// This file is part of MCP-Guard Enterprise, a commercial product.
//
// MCP-Guard Enterprise requires a valid commercial license for use.
// Unauthorized use, modification, or distribution is prohibited.
//
// For licensing information, visit: https://mcp-guard.io/pricing
// For support, contact: austin@botzr.dev
//! MCP Guard Enterprise - Enterprise tier features for mcp-guard
//!
//! This crate provides Enterprise tier features that require a valid Enterprise license:
//!
//! - **mTLS Authentication**: Client certificate authentication via reverse proxy
//! - **Multi-Server Routing**: Path-based routing to multiple upstream MCP servers
//! - **SIEM Integration**: Audit log shipping to external systems
//! - **Per-Tool Rate Limiting**: Fine-grained rate limits per tool
//! - **OpenTelemetry Tracing**: Distributed tracing support
//! - **Admin Guard Tools**: Runtime management via MCP tools
//!   - `guard/keys/*` - API key management
//!   - `guard/audit/*` - Audit log queries
//!   - `guard/config/*` - Configuration management
//!
//! # License
//!
//! This crate requires a valid Enterprise license key with online validation.
//! Get one at https://mcp-guard.io/pricing
//!
//! # Usage
//!
//! ```rust,ignore
//! use mcp_guard_enterprise::license::EnterpriseLicense;
//!
//! // Validate license at startup (online + cached)
//! let license = EnterpriseLicense::validate().await?;
//!
//! // Use Enterprise features
//! use mcp_guard_enterprise::guard_tools::EnterpriseGuardTools;
//! ```

pub mod guard_tools;
pub mod license;

// Re-export Enterprise features from core (these will be gated in Phase 5)
pub mod auth {
    //! Enterprise authentication providers

    pub use mcp_guard_core::auth::MtlsAuthProvider;
}

pub mod router {
    //! Enterprise routing features

    pub use mcp_guard_core::router::ServerRouter;
}

pub mod audit {
    //! Enterprise audit features

    pub use mcp_guard_core::audit::AuditShipperHandle;
    pub use mcp_guard_core::audit::AuditLogger;
}

// Re-export Pro features (Enterprise includes Pro)
pub use mcp_guard_pro::auth as pro_auth;
pub use mcp_guard_pro::rate_limit as pro_rate_limit;
pub use mcp_guard_pro::transport as pro_transport;

/// Result type for Enterprise operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for Enterprise tier operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("License error: {0}")]
    License(#[from] license::LicenseError),

    #[error("Pro error: {0}")]
    Pro(#[from] mcp_guard_pro::Error),

    #[error("Core error: {0}")]
    Core(#[from] mcp_guard_core::Error),

    #[error("Guard tool error: {0}")]
    GuardTool(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}
