//! CLI commands for mcp-guard
//!
//! This module provides the command-line interface for mcp-guard.
//!
//! Available commands:
//! - `init` - Generate a new configuration file (TOML or YAML)
//! - `validate` - Validate configuration file syntax and semantics
//! - `keygen` - Generate a new API key with its hash for configuration
//! - `hash-key` - Hash an existing API key for configuration
//! - `run` - Start the MCP Guard server
//! - `version` - Show version and build information
//! - `check-upstream` - Test upstream MCP server connectivity
//!
//! # Example
//!
//! ```bash
//! # Generate config and start server
//! mcp-guard init
//! mcp-guard validate
//! mcp-guard run
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;

// ============================================================================
// CLI Definition
// ============================================================================

/// MCP Guard - Security gateway for MCP servers
#[derive(Debug, Parser)]
#[command(name = "mcp-guard")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, global = true, default_value = "mcp-guard.toml")]
    pub config: PathBuf,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new configuration file
    Init {
        /// Output format (toml or yaml)
        #[arg(long, default_value = "toml")]
        format: String,

        /// Force overwrite existing file
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// Validate configuration file
    Validate,

    /// Generate a new API key
    Keygen {
        /// User/service identifier
        #[arg(long)]
        user_id: String,

        /// Rate limit for this key (requests per second)
        #[arg(long)]
        rate_limit: Option<u32>,

        /// Comma-separated list of allowed tools
        #[arg(long)]
        tools: Option<String>,
    },

    /// Run the MCP Guard server
    Run {
        /// Override listen host
        #[arg(long)]
        host: Option<String>,

        /// Override listen port
        #[arg(long)]
        port: Option<u16>,
    },

    /// Hash an API key for configuration
    HashKey {
        /// The API key to hash
        key: String,
    },

    /// Show version and build information
    Version,

    /// Check upstream MCP server connectivity
    CheckUpstream {
        /// Timeout in seconds for the connectivity check
        #[arg(short, long, default_value = "10")]
        timeout: u64,
    },
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a new random API key
///
/// Creates a 32-byte random key encoded as base64url with an "mcp_" prefix.
/// Example output: `mcp_AbCdEf123456...`
pub fn generate_api_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    format!(
        "mcp_{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, bytes)
    )
}

/// Hash an API key for storage
///
/// Uses SHA-256 and encodes the result as base64. This hash should be stored
/// in the configuration file instead of the plaintext key.
pub fn hash_api_key(key: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hasher.finalize())
}

// ============================================================================
// Config Generation
// ============================================================================

/// Generate default configuration (deprecated - use generate_config_with_demo_key instead)
///
/// Returns a configuration template in either TOML or YAML format.
/// Note: This generates a config with no API keys, so authentication will reject all requests.
#[deprecated(since = "1.0.0", note = "Use generate_config_with_demo_key instead")]
pub fn generate_config(format: &str) -> String {
    let config = r#"# MCP Guard Configuration

[server]
host = "127.0.0.1"
port = 3000

[auth]
# API keys are configured here
# api_keys = [
#   { id = "user1", key_hash = "<hash>", allowed_tools = ["read", "write"] }
# ]

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
"#;

    if format == "yaml" {
        // Convert to YAML format
        r#"# MCP Guard Configuration

server:
  host: "127.0.0.1"
  port: 3000

auth:
  api_keys: []
  # - id: "user1"
  #   key_hash: "<hash>"
  #   allowed_tools:
  #     - read
  #     - write

rate_limit:
  enabled: true
  requests_per_second: 100
  burst_size: 50

audit:
  enabled: true
  stdout: true

upstream:
  transport: stdio
  command: npx
  args:
    - "-y"
    - "@modelcontextprotocol/server-filesystem"
    - "/tmp"
"#
        .to_string()
    } else {
        config.to_string()
    }
}

/// Generate default configuration with a working demo API key
///
/// Returns a tuple of (config_string, demo_api_key) where:
/// - config_string: A complete configuration in TOML or YAML format with a demo API key
/// - demo_api_key: The plaintext API key that can be used to test the configuration
///
/// This allows `mcp-guard init && mcp-guard run` to work immediately without manual editing.
pub fn generate_config_with_demo_key(format: &str) -> (String, String) {
    let demo_key = generate_api_key();
    let demo_hash = hash_api_key(&demo_key);

    if format == "yaml" {
        let config = format!(
            r#"# MCP Guard Configuration
# Generated by: mcp-guard init
# Documentation: https://github.com/botzrdev/mcp-guard

server:
  host: "127.0.0.1"
  port: 3000

# Authentication - Demo API key (replace in production!)
auth:
  api_keys:
    - id: "demo-user"
      key_hash: "{demo_hash}"
      allowed_tools: []  # Empty = all tools allowed
      rate_limit: 100

# Generate production keys with: mcp-guard keygen --user-id <name>

rate_limit:
  enabled: true
  requests_per_second: 100
  burst_size: 50

audit:
  enabled: true
  stdout: true

upstream:
  transport: stdio
  command: npx
  args:
    - "-y"
    - "@modelcontextprotocol/server-filesystem"
    - "/tmp"

# Full configuration reference with all options:
# https://github.com/botzrdev/mcp-guard/blob/main/templates/mcp-guard.toml
"#
        );
        (config, demo_key)
    } else {
        let config = format!(
            r#"# MCP Guard Configuration
# Generated by: mcp-guard init
# Documentation: https://github.com/botzrdev/mcp-guard

[server]
host = "127.0.0.1"
port = 3000

# Authentication - Demo API key (replace in production!)
[[auth.api_keys]]
id = "demo-user"
key_hash = "{demo_hash}"
allowed_tools = []  # Empty = all tools allowed
rate_limit = 100

# Generate production keys with: mcp-guard keygen --user-id <name>

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# Full configuration reference with all options:
# https://github.com/botzrdev/mcp-guard/blob/main/templates/mcp-guard.toml
"#
        );
        (config, demo_key)
    }
}
