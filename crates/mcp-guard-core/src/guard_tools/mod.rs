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
//! Guard Tools - Internal MCP tools exposed by mcp-guard
//!
//! This module provides the `guard/*` tools that mcp-guard exposes as an MCP server.
//! Free tier tools are public, enterprise tools require admin authentication.

use async_trait::async_trait;
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;

/// Error type for guard tool operations
#[derive(Debug, thiserror::Error)]
pub enum GuardToolError {
    #[error("Tool not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("License required: {0}")]
    LicenseRequired(String),
}

/// MCP Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Result of calling a guard tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ToolContent>>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl ToolResult {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: Some(vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }]),
            is_error: None,
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self {
            content: Some(vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }]),
            is_error: Some(true),
        }
    }
}

/// Trait for providing guard tools
#[async_trait]
pub trait GuardToolsProvider: Send + Sync {
    /// List available guard tools
    fn list_tools(&self) -> Vec<ToolDefinition>;

    /// Call a guard tool by name
    async fn call_tool(&self, name: &str, args: Value) -> Result<ToolResult, GuardToolError>;
}

/// Free tier guard tools provider
pub struct FreeGuardTools {
    start_time: Instant,
    metrics: Option<Arc<PrometheusHandle>>,
}

impl FreeGuardTools {
    pub fn new(metrics: Option<Arc<PrometheusHandle>>) -> Self {
        Self {
            start_time: Instant::now(),
            metrics,
        }
    }
}

#[async_trait]
impl GuardToolsProvider for FreeGuardTools {
    fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "guard/health".to_string(),
                description: "Get mcp-guard health status including version and uptime".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            ToolDefinition {
                name: "guard/metrics".to_string(),
                description: "Get Prometheus metrics snapshot in text or JSON format".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string",
                            "enum": ["prometheus", "json"],
                            "default": "prometheus",
                            "description": "Output format for metrics"
                        }
                    },
                    "additionalProperties": false
                }),
            },
            ToolDefinition {
                name: "guard/version".to_string(),
                description: "Get mcp-guard version and build information".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
        ]
    }

    async fn call_tool(&self, name: &str, args: Value) -> Result<ToolResult, GuardToolError> {
        match name {
            "guard/health" => self.handle_health().await,
            "guard/metrics" => self.handle_metrics(args).await,
            "guard/version" => self.handle_version().await,
            _ => Err(GuardToolError::NotFound(name.to_string())),
        }
    }
}

impl FreeGuardTools {
    async fn handle_health(&self) -> Result<ToolResult, GuardToolError> {
        let uptime_secs = self.start_time.elapsed().as_secs();

        let health = serde_json::json!({
            "status": "healthy",
            "version": env!("CARGO_PKG_VERSION"),
            "uptime_secs": uptime_secs
        });

        Ok(ToolResult::text(
            serde_json::to_string_pretty(&health).unwrap(),
        ))
    }

    async fn handle_metrics(&self, args: Value) -> Result<ToolResult, GuardToolError> {
        let format = args
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("prometheus");

        match &self.metrics {
            Some(metrics) => {
                let output = metrics.render();
                match format {
                    "prometheus" => Ok(ToolResult::text(output)),
                    "json" => {
                        // Parse prometheus format to JSON (basic conversion)
                        let lines: Vec<&str> = output
                            .lines()
                            .filter(|l| !l.starts_with('#') && !l.is_empty())
                            .collect();

                        let metrics_json: Vec<Value> = lines
                            .iter()
                            .filter_map(|line| {
                                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                                if parts.len() == 2 {
                                    Some(serde_json::json!({
                                        "name": parts[0],
                                        "value": parts[1].parse::<f64>().unwrap_or(0.0)
                                    }))
                                } else {
                                    None
                                }
                            })
                            .collect();

                        Ok(ToolResult::text(
                            serde_json::to_string_pretty(&metrics_json).unwrap(),
                        ))
                    }
                    _ => Err(GuardToolError::InvalidArguments(format!(
                        "Unknown format: {}. Use 'prometheus' or 'json'",
                        format
                    ))),
                }
            }
            None => Ok(ToolResult::text(
                "Metrics not enabled. Set [observability] enabled = true in config.",
            )),
        }
    }

    async fn handle_version(&self) -> Result<ToolResult, GuardToolError> {
        let version_info = serde_json::json!({
            "package": env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION"),
            "description": env!("CARGO_PKG_DESCRIPTION"),
            "license": "AGPL-3.0",
            "repository": env!("CARGO_PKG_REPOSITORY"),
            "features": {
                "auth_providers": ["API Key", "JWT (HS256/JWKS)", "OAuth 2.1 (PKCE)", "mTLS"],
                "transports": ["Stdio", "HTTP", "SSE"],
                "rate_limiting": ["Per-identity", "Token bucket"],
                "observability": ["Prometheus metrics", "OpenTelemetry tracing"]
            }
        });

        Ok(ToolResult::text(
            serde_json::to_string_pretty(&version_info).unwrap(),
        ))
    }
}

/// Check if a method is a guard tool call
pub fn is_guard_tool_method(method: &str) -> bool {
    method.starts_with("guard/")
}

/// Check if requesting tools/list for guard tools
pub fn is_tools_list_method(method: &str) -> bool {
    method == "tools/list"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_tool() {
        let tools = FreeGuardTools::new(None);
        let result = tools.call_tool("guard/health", Value::Null).await.unwrap();

        let content = result.content.unwrap();
        assert_eq!(content.len(), 1);
        assert!(content[0].text.contains("healthy"));
        assert!(content[0].text.contains("version"));
    }

    #[tokio::test]
    async fn test_version_tool() {
        let tools = FreeGuardTools::new(None);
        let result = tools.call_tool("guard/version", Value::Null).await.unwrap();

        let content = result.content.unwrap();
        assert_eq!(content.len(), 1);
        assert!(content[0].text.contains("mcp-guard-core"));
    }

    #[tokio::test]
    async fn test_metrics_tool_no_metrics() {
        let tools = FreeGuardTools::new(None);
        let result = tools
            .call_tool("guard/metrics", serde_json::json!({}))
            .await
            .unwrap();

        let content = result.content.unwrap();
        assert!(content[0].text.contains("not enabled"));
    }

    #[tokio::test]
    async fn test_unknown_tool() {
        let tools = FreeGuardTools::new(None);
        let result = tools.call_tool("guard/unknown", Value::Null).await;

        assert!(matches!(result, Err(GuardToolError::NotFound(_))));
    }

    #[test]
    fn test_list_tools() {
        let tools = FreeGuardTools::new(None);
        let list = tools.list_tools();

        assert_eq!(list.len(), 3);
        assert!(list.iter().any(|t| t.name == "guard/health"));
        assert!(list.iter().any(|t| t.name == "guard/metrics"));
        assert!(list.iter().any(|t| t.name == "guard/version"));
    }

    #[test]
    fn test_is_guard_tool_method() {
        assert!(is_guard_tool_method("guard/health"));
        assert!(is_guard_tool_method("guard/metrics"));
        assert!(is_guard_tool_method("guard/keys/list"));
        assert!(!is_guard_tool_method("tools/list"));
        assert!(!is_guard_tool_method("resources/read"));
    }
}
