//! Authorization logic for mcp-guard

use crate::auth::Identity;
use crate::transport::Message;

/// Check if an identity is authorized to call a specific tool
pub fn authorize_tool_call(identity: &Identity, tool_name: &str) -> bool {
    match &identity.allowed_tools {
        None => true, // No restrictions
        Some(tools) => tools.iter().any(|t| t == tool_name || t == "*"),
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
