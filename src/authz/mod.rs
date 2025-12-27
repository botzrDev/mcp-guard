//! Authorization logic for mcp-guard
//!
//! This module implements tool-level authorization for MCP requests.
//!
//! Authorization model:
//! - Each identity can have an optional `allowed_tools` list
//! - `None` = unrestricted access to all tools
//! - `Some(["*"])` = wildcard, equivalent to unrestricted
//! - `Some(["tool1", "tool2"])` = only these specific tools
//!
//! Key functions:
//! - [`authorize_tool_call`] - Check if identity can call a specific tool
//! - [`filter_tools_list_response`] - Filter `tools/list` to show only authorized tools (FR-AUTHZ-03)

use crate::auth::Identity;
use crate::transport::Message;
use serde_json::Value;

// ============================================================================
// Authorization Functions
// ============================================================================

/// Check if an identity is authorized to call a specific tool
///
/// Supports glob patterns in allowed_tools:
/// - `*` - matches all tools (wildcard)
/// - `read_*` - matches tools starting with "read_"
/// - `fs/*` - matches tools like "fs/read", "fs/write"
/// - Exact matches also work
pub fn authorize_tool_call(identity: &Identity, tool_name: &str) -> bool {
    match &identity.allowed_tools {
        None => true, // No restrictions
        Some(tools) => tools.iter().any(|pattern| {
            // Exact match or wildcard
            if pattern == tool_name || pattern == "*" {
                return true;
            }
            // Try glob pattern matching for patterns containing wildcards
            if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
                if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
                    return glob_pattern.matches(tool_name);
                }
            }
            false
        }),
    }
}

/// Extract tool name from a MCP request message
pub fn extract_tool_name(message: &Message) -> Option<&str> {
    if let Some(method) = &message.method {
        if method == "tools/call" {
            if let Some(params) = &message.params {
                return params.get("name").and_then(|v| v.as_str());
            }
        }
    }
    None
}

// ============================================================================
// Types
// ============================================================================

/// Authorization decision
#[derive(Debug, Clone)]
pub enum AuthzDecision {
    Allow,
    Deny(String),
}

/// Authorize a request based on identity and message
pub fn authorize_request(identity: &Identity, message: &Message) -> AuthzDecision {
    // Check tool-level authorization for tool calls
    if let Some(tool_name) = extract_tool_name(message) {
        if !authorize_tool_call(identity, tool_name) {
            return AuthzDecision::Deny(format!(
                "Identity '{}' is not authorized to call tool '{}'",
                identity.id, tool_name
            ));
        }
    }

    AuthzDecision::Allow
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if a request is a tools/list request
pub fn is_tools_list_request(message: &Message) -> bool {
    message.method.as_deref() == Some("tools/list")
}

/// Filter a tools/list response to only include tools the identity is authorized to call (FR-AUTHZ-03)
///
/// The tools/list response has this structure:
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "result": {
///     "tools": [
///       { "name": "read_file", "description": "...", "inputSchema": {...} },
///       { "name": "write_file", "description": "...", "inputSchema": {...} }
///     ]
///   }
/// }
/// ```
///
/// This function filters the tools array to only include tools the identity can call.
pub fn filter_tools_list_response(mut response: Message, identity: &Identity) -> Message {
    // If identity has unrestricted access, return as-is
    if identity.allowed_tools.is_none() {
        return response;
    }

    // If identity has wildcard access, return as-is
    if let Some(tools) = &identity.allowed_tools {
        if tools.iter().any(|t| t == "*") {
            return response;
        }
    }

    // Try to filter the tools array in the result
    if let Some(ref mut result) = response.result {
        if let Some(tools) = result.get_mut("tools") {
            if let Some(tools_array) = tools.as_array() {
                let filtered: Vec<Value> = tools_array
                    .iter()
                    .filter(|tool| {
                        if let Some(name) = tool.get("name").and_then(|n| n.as_str()) {
                            authorize_tool_call(identity, name)
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect();

                *tools = Value::Array(filtered);
            }
        }
    }

    response
}

#[cfg(test)]
mod tests {
    //! Unit tests for authorization logic.
    //!
    //! Tests cover:
    //! - Unrestricted access (allowed_tools = None)
    //! - Restricted access to specific tools
    //! - Wildcard tool permissions
    //! - tools/list response filtering (FR-AUTHZ-03)

    use super::*;

    /// Verify unrestricted identity can call any tool
    #[test]
    fn test_authorize_tool_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }

    /// Verify restricted identity can only call allowed tools
    #[test]
    fn test_authorize_tool_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read".to_string(), "list".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "read"));
        assert!(authorize_tool_call(&identity, "list"));
        assert!(!authorize_tool_call(&identity, "write"));
    }

    /// Verify wildcard "*" grants access to all tools
    #[test]
    fn test_authorize_tool_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        assert!(authorize_tool_call(&identity, "any_tool"));
    }

    /// Verify tools/list request detection
    #[test]
    fn test_is_tools_list_request() {
        let request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/list".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(is_tools_list_request(&request));

        let other_request = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };
        assert!(!is_tools_list_request(&other_request));
    }

    /// Verify unrestricted identity sees all tools in list response
    #[test]
    fn test_filter_tools_list_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }

    /// Verify restricted identity only sees allowed tools in list response (FR-AUTHZ-03)
    #[test]
    fn test_filter_tools_list_restricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "delete_file", "description": "Delete a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "read_file");
    }

    #[test]
    fn test_filter_tools_list_wildcard() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["*".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }

    #[test]
    fn test_filter_tools_list_multiple_allowed() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string(), "list_files".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let response = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": [
                    {"name": "read_file", "description": "Read a file"},
                    {"name": "write_file", "description": "Write a file"},
                    {"name": "list_files", "description": "List files"}
                ]
            })),
            error: None,
        };

        let filtered = filter_tools_list_response(response, &identity);
        let result = filtered.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 2);

        let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"list_files"));
        assert!(!names.contains(&"write_file"));
    }

    /// Verify extract_tool_name extracts tool name from tools/call request
    #[test]
    fn test_extract_tool_name() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "read_file", "arguments": {}})),
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), Some("read_file"));
    }

    #[test]
    fn test_extract_tool_name_returns_none_for_non_tool_call() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }

    #[test]
    fn test_extract_tool_name_returns_none_without_params() {
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: None,
            result: None,
            error: None,
        };

        assert_eq!(extract_tool_name(&message), None);
    }

    #[test]
    fn test_authorize_request_allows_unrestricted() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: None,
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "any_tool"})),
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => {}
            AuthzDecision::Deny(_) => panic!("Expected Allow"),
        }
    }

    #[test]
    fn test_authorize_request_denies_unauthorized_tool() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("tools/call".to_string()),
            params: Some(serde_json::json!({"name": "delete_file"})),
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => panic!("Expected Deny"),
            AuthzDecision::Deny(reason) => {
                assert!(reason.contains("delete_file"));
            }
        }
    }

    #[test]
    fn test_authorize_request_allows_non_tool_calls() {
        let identity = Identity {
            id: "test".to_string(),
            name: None,
            allowed_tools: Some(vec!["read_file".to_string()]),
            rate_limit: None,
            claims: std::collections::HashMap::new(),
        };

        // This is not a tools/call request, so authorization should pass
        let message = Message {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: Some("resources/list".to_string()),
            params: None,
            result: None,
            error: None,
        };

        match authorize_request(&identity, &message) {
            AuthzDecision::Allow => {}
            AuthzDecision::Deny(_) => panic!("Expected Allow for non-tool call"),
        }
    }
}
