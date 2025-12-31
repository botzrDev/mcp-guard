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
//! Enterprise Guard Tools - Admin MCP tools for runtime management
//!
//! These tools require Enterprise license and admin authentication:
//!
//! - `guard/keys/list` - List configured API keys
//! - `guard/keys/create` - Generate a new API key
//! - `guard/keys/revoke` - Revoke an existing API key
//! - `guard/audit/query` - Query audit logs
//! - `guard/config/reload` - Hot-reload configuration
//! - `guard/config/validate` - Validate configuration file

use async_trait::async_trait;
use mcp_guard_core::auth::Identity;
use mcp_guard_core::guard_tools::{GuardToolError, GuardToolsProvider, ToolDefinition, ToolResult};
use serde_json::Value;
use tracing::{debug, info};

/// Enterprise guard tools provider
///
/// Provides admin-level guard/* tools that require authentication.
pub struct EnterpriseGuardTools {
    /// Whether admin auth is required (true in HTTP mode, false in stdio mode)
    #[allow(dead_code)]
    require_admin_auth: bool,
}

impl EnterpriseGuardTools {
    pub fn new(require_admin_auth: bool) -> Self {
        Self { require_admin_auth }
    }

    /// Check if the identity has admin privileges
    #[allow(dead_code)]
    fn check_admin(&self, identity: Option<&Identity>) -> Result<(), GuardToolError> {
        if !self.require_admin_auth {
            // Stdio mode - trust the parent process
            return Ok(());
        }

        // HTTP mode - require admin identity
        let identity = identity.ok_or_else(|| {
            GuardToolError::Unauthorized("Admin authentication required".to_string())
        })?;

        // Check for admin claim or admin scope
        let is_admin = identity
            .claims
            .get("admin")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
            || identity.id == "admin";

        if !is_admin {
            return Err(GuardToolError::Unauthorized(
                "Admin privileges required".to_string(),
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl GuardToolsProvider for EnterpriseGuardTools {
    fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![
            // Key Management
            ToolDefinition {
                name: "guard/keys/list".to_string(),
                description: "List configured API keys (names only, not secrets)".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            ToolDefinition {
                name: "guard/keys/create".to_string(),
                description: "Generate a new API key".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "user_id": {
                            "type": "string",
                            "description": "Unique identifier for the key owner"
                        },
                        "allowed_tools": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "List of allowed tools (empty for all)"
                        },
                        "rate_limit": {
                            "type": "integer",
                            "description": "Requests per second limit"
                        }
                    },
                    "required": ["user_id"],
                    "additionalProperties": false
                }),
            },
            ToolDefinition {
                name: "guard/keys/revoke".to_string(),
                description: "Revoke an existing API key".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "user_id": {
                            "type": "string",
                            "description": "ID of the key to revoke"
                        }
                    },
                    "required": ["user_id"],
                    "additionalProperties": false
                }),
            },
            // Audit
            ToolDefinition {
                name: "guard/audit/query".to_string(),
                description: "Query audit logs".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "since": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Start time for query (ISO 8601)"
                        },
                        "event_type": {
                            "type": "string",
                            "description": "Filter by event type"
                        },
                        "identity_id": {
                            "type": "string",
                            "description": "Filter by identity ID"
                        },
                        "limit": {
                            "type": "integer",
                            "default": 100,
                            "description": "Maximum number of results"
                        }
                    },
                    "additionalProperties": false
                }),
            },
            // Config
            ToolDefinition {
                name: "guard/config/reload".to_string(),
                description: "Hot-reload configuration without restart".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            ToolDefinition {
                name: "guard/config/validate".to_string(),
                description: "Validate a configuration file".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to configuration file to validate"
                        }
                    },
                    "additionalProperties": false
                }),
            },
        ]
    }

    async fn call_tool(&self, name: &str, args: Value) -> Result<ToolResult, GuardToolError> {
        // Note: In a real implementation, identity would be passed here
        // For now, we skip auth check in this method signature
        // The actual auth check happens at the HTTP layer

        match name {
            "guard/keys/list" => self.handle_keys_list().await,
            "guard/keys/create" => self.handle_keys_create(args).await,
            "guard/keys/revoke" => self.handle_keys_revoke(args).await,
            "guard/audit/query" => self.handle_audit_query(args).await,
            "guard/config/reload" => self.handle_config_reload().await,
            "guard/config/validate" => self.handle_config_validate(args).await,
            _ => Err(GuardToolError::NotFound(name.to_string())),
        }
    }
}

impl EnterpriseGuardTools {
    async fn handle_keys_list(&self) -> Result<ToolResult, GuardToolError> {
        debug!("Listing API keys");

        // In a real implementation, this would read from the config
        // For now, return a placeholder response
        let response = serde_json::json!({
            "keys": [
                {
                    "id": "example-user",
                    "created_at": "2024-01-01T00:00:00Z",
                    "allowed_tools": ["*"],
                    "rate_limit": null
                }
            ],
            "note": "This is a placeholder. In production, this reads from config."
        });

        Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
    }

    async fn handle_keys_create(&self, args: Value) -> Result<ToolResult, GuardToolError> {
        let user_id = args
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GuardToolError::InvalidArguments("Missing user_id".to_string()))?;

        let allowed_tools: Vec<String> = args
            .get("allowed_tools")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["*".to_string()]);

        let rate_limit = args
            .get("rate_limit")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        info!(user_id = %user_id, "Creating new API key");

        // Generate a new API key
        let key = mcp_guard_core::cli::generate_api_key();
        let hash = mcp_guard_core::cli::hash_api_key(&key);

        let response = serde_json::json!({
            "user_id": user_id,
            "key": key,
            "key_hash": hash,
            "allowed_tools": allowed_tools,
            "rate_limit": rate_limit,
            "note": "Add the key_hash to your config file. The plaintext key is shown only once."
        });

        Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
    }

    async fn handle_keys_revoke(&self, args: Value) -> Result<ToolResult, GuardToolError> {
        let user_id = args
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GuardToolError::InvalidArguments("Missing user_id".to_string()))?;

        info!(user_id = %user_id, "Revoking API key");

        // In a real implementation, this would modify the config
        let response = serde_json::json!({
            "user_id": user_id,
            "status": "revoked",
            "note": "In production, this would remove the key from config and trigger a reload."
        });

        Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
    }

    async fn handle_audit_query(&self, args: Value) -> Result<ToolResult, GuardToolError> {
        let limit = args
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(100) as usize;

        let event_type = args.get("event_type").and_then(|v| v.as_str());
        let identity_id = args.get("identity_id").and_then(|v| v.as_str());

        debug!(
            limit = limit,
            event_type = ?event_type,
            identity_id = ?identity_id,
            "Querying audit logs"
        );

        // In a real implementation, this would query the audit log storage
        let response = serde_json::json!({
            "events": [],
            "total": 0,
            "query": {
                "event_type": event_type,
                "identity_id": identity_id,
                "limit": limit
            },
            "note": "In production, this queries the audit log storage."
        });

        Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
    }

    async fn handle_config_reload(&self) -> Result<ToolResult, GuardToolError> {
        info!("Config reload requested");

        // In a real implementation, this would trigger a hot reload
        let response = serde_json::json!({
            "status": "pending",
            "note": "Config hot-reload is not yet implemented. Restart the server to apply changes."
        });

        Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
    }

    async fn handle_config_validate(&self, args: Value) -> Result<ToolResult, GuardToolError> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("mcp-guard.toml");

        debug!(path = %path, "Validating config");

        // Try to load and validate the config
        match mcp_guard_core::Config::from_file(&std::path::PathBuf::from(path)) {
            Ok(config) => {
                let response = serde_json::json!({
                    "valid": true,
                    "path": path,
                    "server": {
                        "host": config.server.host,
                        "port": config.server.port
                    },
                    "auth": {
                        "api_keys_count": config.auth.api_keys.len(),
                        "jwt_enabled": config.auth.jwt.is_some(),
                        "oauth_enabled": config.auth.oauth.is_some(),
                        "mtls_enabled": config.auth.mtls.as_ref().map(|m| m.enabled).unwrap_or(false)
                    }
                });
                Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
            }
            Err(e) => {
                let response = serde_json::json!({
                    "valid": false,
                    "path": path,
                    "error": format!("{}", e)
                });
                Ok(ToolResult::text(serde_json::to_string_pretty(&response).unwrap()))
            }
        }
    }
}

/// Check if a method is an Enterprise guard tool
pub fn is_enterprise_guard_tool(method: &str) -> bool {
    method.starts_with("guard/keys/")
        || method.starts_with("guard/audit/")
        || method.starts_with("guard/config/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_tools() {
        let tools = EnterpriseGuardTools::new(false);
        let list = tools.list_tools();

        assert_eq!(list.len(), 6);
        assert!(list.iter().any(|t| t.name == "guard/keys/list"));
        assert!(list.iter().any(|t| t.name == "guard/keys/create"));
        assert!(list.iter().any(|t| t.name == "guard/audit/query"));
        assert!(list.iter().any(|t| t.name == "guard/config/reload"));
    }

    #[tokio::test]
    async fn test_keys_create() {
        let tools = EnterpriseGuardTools::new(false);
        let result = tools
            .call_tool(
                "guard/keys/create",
                serde_json::json!({
                    "user_id": "test-user"
                }),
            )
            .await
            .unwrap();

        let content = result.content.unwrap();
        assert!(content[0].text.contains("test-user"));
        assert!(content[0].text.contains("mcp_")); // Key prefix
    }

    #[tokio::test]
    async fn test_config_validate_missing() {
        let tools = EnterpriseGuardTools::new(false);
        let result = tools
            .call_tool(
                "guard/config/validate",
                serde_json::json!({
                    "path": "/nonexistent/config.toml"
                }),
            )
            .await
            .unwrap();

        let content = result.content.unwrap();
        assert!(content[0].text.contains("\"valid\": false"));
    }

    #[test]
    fn test_is_enterprise_guard_tool() {
        assert!(is_enterprise_guard_tool("guard/keys/list"));
        assert!(is_enterprise_guard_tool("guard/audit/query"));
        assert!(is_enterprise_guard_tool("guard/config/reload"));
        assert!(!is_enterprise_guard_tool("guard/health"));
        assert!(!is_enterprise_guard_tool("tools/list"));
    }
}
