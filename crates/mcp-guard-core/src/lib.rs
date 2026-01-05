// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.
//! MCP Guard - A lightweight security gateway for MCP servers
//!
//! This crate provides authentication, authorization, rate limiting,
//! and observability for Model Context Protocol (MCP) servers.

pub mod audit;
pub mod auth;
pub mod authz;
pub mod cli;
pub mod config;
pub mod guard_tools;
pub mod mcp_server;
pub mod observability;
pub mod rate_limit;
pub mod db;
pub mod router;
pub mod server;
pub mod tier;
pub mod transport;

#[cfg(test)]
pub mod mocks;

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

    #[error("Router error: {0}")]
    Router(#[from] router::RouterError),

    #[error("Server error: {0}")]
    Server(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
