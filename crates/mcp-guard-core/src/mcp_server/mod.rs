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
//! MCP Server - Stdio-based MCP server for mcp-guard
//!
//! This module implements an MCP server that communicates over stdin/stdout,
//! allowing mcp-guard to be used as a subprocess by MCP clients like Claude Desktop.
//!
//! The server:
//! - Exposes guard/* tools for monitoring and management
//! - Proxies other requests to upstream MCP servers
//! - Merges guard tools into tools/list responses

use crate::guard_tools::{is_guard_tool_method, FreeGuardTools, GuardToolsProvider, ToolDefinition};
use crate::transport::{Message, Transport};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// MCP Server configuration
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    /// Server name to advertise
    pub server_name: String,
    /// Server version
    pub server_version: String,
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            server_name: "mcp-guard".to_string(),
            server_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// MCP Server that communicates over stdio
pub struct McpServer {
    config: McpServerConfig,
    guard_tools: Arc<dyn GuardToolsProvider>,
    upstream: Option<Arc<dyn Transport>>,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Server info for initialize response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

impl McpServer {
    pub fn new(
        config: McpServerConfig,
        upstream: Option<Arc<dyn Transport>>,
        metrics: Option<Arc<PrometheusHandle>>,
    ) -> Self {
        let guard_tools = Arc::new(FreeGuardTools::new(metrics));
        Self {
            config,
            guard_tools,
            upstream,
        }
    }

    /// Run the MCP server, reading from stdin and writing to stdout
    pub async fn run(&self) -> io::Result<()> {
        info!("Starting MCP server in stdio mode");

        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            debug!("Received message: {}", line);

            match serde_json::from_str::<Message>(&line) {
                Ok(message) => {
                    let response = self.handle_message(message).await;
                    if let Some(resp) = response {
                        let json = serde_json::to_string(&resp)?;
                        debug!("Sending response: {}", json);
                        writeln!(stdout, "{}", json)?;
                        stdout.flush()?;
                    }
                }
                Err(e) => {
                    error!("Failed to parse message: {}", e);
                    let error_response = Message {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        method: None,
                        params: None,
                        result: None,
                        error: Some(serde_json::json!({
                            "code": -32700,
                            "message": format!("Parse error: {}", e)
                        })),
                    };
                    let json = serde_json::to_string(&error_response)?;
                    writeln!(stdout, "{}", json)?;
                    stdout.flush()?;
                }
            }
        }

        info!("MCP server shutting down");
        Ok(())
    }

    /// Handle an incoming MCP message
    async fn handle_message(&self, message: Message) -> Option<Message> {
        let method = message.method.as_deref()?;
        let id = message.id.clone();

        debug!("Handling method: {}", method);

        let result = match method {
            "initialize" => self.handle_initialize(message.params.clone()).await,
            "initialized" => {
                // Notification, no response
                return None;
            }
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(message.params.clone()).await,
            _ if is_guard_tool_method(method) => {
                // Direct guard tool call (non-standard but useful)
                self.handle_guard_tool(method, message.params.clone())
                    .await
            }
            _ => {
                // Proxy to upstream
                self.proxy_to_upstream(message).await
            }
        };

        match result {
            Ok(value) => Some(Message {
                jsonrpc: "2.0".to_string(),
                id,
                method: None,
                params: None,
                result: Some(value),
                error: None,
            }),
            Err(error) => Some(Message {
                jsonrpc: "2.0".to_string(),
                id,
                method: None,
                params: None,
                result: None,
                error: Some(error),
            }),
        }
    }

    async fn handle_initialize(&self, _params: Option<Value>) -> Result<Value, Value> {
        info!("Handling initialize request");

        let capabilities = ServerCapabilities {
            tools: Some(ToolsCapability {
                list_changed: Some(true),
            }),
            resources: if self.upstream.is_some() {
                Some(serde_json::json!({}))
            } else {
                None
            },
            prompts: if self.upstream.is_some() {
                Some(serde_json::json!({}))
            } else {
                None
            },
        };

        let server_info = ServerInfo {
            name: self.config.server_name.clone(),
            version: self.config.server_version.clone(),
        };

        Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": capabilities,
            "serverInfo": server_info
        }))
    }

    async fn handle_tools_list(&self) -> Result<Value, Value> {
        debug!("Handling tools/list request");

        // Start with guard tools
        let mut all_tools: Vec<ToolDefinition> = self.guard_tools.list_tools();

        // If we have an upstream, get its tools and merge
        if let Some(upstream) = &self.upstream {
            let request = Message {
                jsonrpc: "2.0".to_string(),
                id: Some(serde_json::json!(1)),
                method: Some("tools/list".to_string()),
                params: None,
                result: None,
                error: None,
            };

            // Send and receive separately per Transport trait
            if let Err(e) = upstream.send(request).await {
                warn!("Failed to send tools/list to upstream: {}", e);
            } else {
                match upstream.receive().await {
                    Ok(response) => {
                        if let Some(result) = response.result {
                            if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                                for tool in tools {
                                    if let Ok(tool_def) =
                                        serde_json::from_value::<ToolDefinition>(tool.clone())
                                    {
                                        all_tools.push(tool_def);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to receive upstream tools: {}", e);
                    }
                }
            }
        }

        Ok(serde_json::json!({
            "tools": all_tools
        }))
    }

    async fn handle_tools_call(&self, params: Option<Value>) -> Result<Value, Value> {
        let params = params.ok_or_else(|| {
            serde_json::json!({
                "code": -32602,
                "message": "Missing params"
            })
        })?;

        let tool_name = params.get("name").and_then(|n| n.as_str()).ok_or_else(|| {
            serde_json::json!({
                "code": -32602,
                "message": "Missing tool name"
            })
        })?;

        let arguments = params
            .get("arguments")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        debug!("Calling tool: {} with args: {:?}", tool_name, arguments);

        // Check if it's a guard tool
        if is_guard_tool_method(tool_name) {
            return self.handle_guard_tool(tool_name, Some(arguments)).await;
        }

        // Proxy to upstream
        if let Some(upstream) = &self.upstream {
            let request = Message {
                jsonrpc: "2.0".to_string(),
                id: Some(serde_json::json!(1)),
                method: Some("tools/call".to_string()),
                params: Some(params),
                result: None,
                error: None,
            };

            upstream.send(request).await.map_err(|e| {
                serde_json::json!({
                    "code": -32603,
                    "message": format!("Upstream send error: {}", e)
                })
            })?;

            let response = upstream.receive().await.map_err(|e| {
                serde_json::json!({
                    "code": -32603,
                    "message": format!("Upstream receive error: {}", e)
                })
            })?;

            if let Some(error) = response.error {
                Err(error)
            } else {
                response.result.ok_or_else(|| {
                    serde_json::json!({
                        "code": -32603,
                        "message": "No result from upstream"
                    })
                })
            }
        } else {
            Err(serde_json::json!({
                "code": -32601,
                "message": format!("Unknown tool: {}", tool_name)
            }))
        }
    }

    async fn handle_guard_tool(&self, name: &str, params: Option<Value>) -> Result<Value, Value> {
        let args = params.unwrap_or(serde_json::json!({}));

        match self.guard_tools.call_tool(name, args).await {
            Ok(result) => Ok(serde_json::to_value(result).unwrap()),
            Err(e) => Err(serde_json::json!({
                "code": -32603,
                "message": format!("{}", e)
            })),
        }
    }

    async fn proxy_to_upstream(&self, message: Message) -> Result<Value, Value> {
        if let Some(upstream) = &self.upstream {
            upstream.send(message.clone()).await.map_err(|e| {
                serde_json::json!({
                    "code": -32603,
                    "message": format!("Upstream send error: {}", e)
                })
            })?;

            let response = upstream.receive().await.map_err(|e| {
                serde_json::json!({
                    "code": -32603,
                    "message": format!("Upstream receive error: {}", e)
                })
            })?;

            if let Some(error) = response.error {
                Err(error)
            } else {
                response.result.ok_or_else(|| {
                    serde_json::json!({
                        "code": -32603,
                        "message": "No result from upstream"
                    })
                })
            }
        } else {
            let method = message.method.unwrap_or_default();
            Err(serde_json::json!({
                "code": -32601,
                "message": format!("Method not found: {}", method)
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() {
        let server = McpServer::new(McpServerConfig::default(), None, None);

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("initialize".to_string()),
            params: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            })),
            result: None,
            error: None,
        };

        let response = server.handle_message(message).await.unwrap();

        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], "2024-11-05");
        assert!(result["capabilities"]["tools"].is_object());
    }

    #[tokio::test]
    async fn test_tools_list_guard_only() {
        let server = McpServer::new(McpServerConfig::default(), None, None);

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        let response = server.handle_message(message).await.unwrap();

        assert!(response.result.is_some());
        let result = response.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 3); // health, metrics, version
        assert!(tools.iter().any(|t| t["name"] == "guard/health"));
    }

    #[tokio::test]
    async fn test_tools_call_guard_health() {
        let server = McpServer::new(McpServerConfig::default(), None, None);

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({
                "name": "guard/health",
                "arguments": {}
            })),
            result: None,
            error: None,
        };

        let response = server.handle_message(message).await.unwrap();

        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_unknown_method_no_upstream() {
        let server = McpServer::new(McpServerConfig::default(), None, None);

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("unknown/method".to_string()),
            params: None,
            result: None,
            error: None,
        };

        let response = server.handle_message(message).await.unwrap();

        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error["code"], -32601);
    }

    #[tokio::test]
    async fn test_initialized_notification() {
        let server = McpServer::new(McpServerConfig::default(), None, None);

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: None, // Notifications have no id
            method: Some("initialized".to_string()),
            params: None,
            result: None,
            error: None,
        };

        // Notifications return None
        let response = server.handle_message(message).await;
        assert!(response.is_none());
    }
}
