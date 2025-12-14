//! MCP Guard - A lightweight security gateway for MCP servers
//!
//! This crate provides authentication, authorization, rate limiting,
//! and observability for Model Context Protocol (MCP) servers.

pub mod audit;
pub mod auth;
pub mod authz;
pub mod cli;
pub mod config;
pub mod observability;
pub mod rate_limit;
pub mod server;
pub mod transport;

pub use config::Config;

/// Result type alias for mcp-guard operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for mcp-guard
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Authentication error: {0}")]
    Auth(#[from] auth::AuthError),

    #[error("Authorization denied: {0}")]
    Authz(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Transport error: {0}")]
    Transport(#[from] transport::TransportError),

    #[error("Server error: {0}")]
    Server(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
