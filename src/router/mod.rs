//! Multi-server routing for mcp-guard
//!
//! Routes requests to different upstream MCP servers based on path prefix.
//! This enables organizations to run multiple MCP servers behind a single gateway.

use std::collections::HashMap;
use std::sync::Arc;

use crate::config::{ServerRouteConfig, TransportType};
use crate::transport::{HttpTransport, Message, SseTransport, StdioTransport, Transport, TransportError};

/// Router error types
#[derive(Debug, thiserror::Error)]
pub enum RouterError {
    #[error("No route found for path: {0}")]
    NoRoute(String),

    #[error("Failed to initialize transport for server '{0}': {1}")]
    TransportInit(String, String),

    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),
}

/// Server route with initialized transport
pub struct ServerRoute {
    /// Route configuration
    pub config: ServerRouteConfig,
    /// Initialized transport
    pub transport: Arc<dyn Transport>,
}

/// Multi-server router that routes requests to different upstreams based on path
pub struct ServerRouter {
    /// Routes indexed by path prefix (sorted by specificity)
    routes: Vec<ServerRoute>,
    /// Default route (optional, used when no path prefix matches)
    default_route: Option<ServerRoute>,
}

impl ServerRouter {
    /// Create a new server router from configuration
    pub async fn new(configs: Vec<ServerRouteConfig>) -> Result<Self, RouterError> {
        let mut routes = Vec::new();

        for config in configs {
            let transport = Self::create_transport(&config).await?;
            routes.push(ServerRoute {
                config,
                transport,
            });
        }

        // Sort routes by path prefix length (longer = more specific = higher priority)
        routes.sort_by(|a, b| b.config.path_prefix.len().cmp(&a.config.path_prefix.len()));

        Ok(Self {
            routes,
            default_route: None,
        })
    }

    /// Create a transport from server route configuration
    async fn create_transport(config: &ServerRouteConfig) -> Result<Arc<dyn Transport>, RouterError> {
        match config.transport {
            TransportType::Stdio => {
                let command = config.command.as_ref().ok_or_else(|| {
                    RouterError::TransportInit(
                        config.name.clone(),
                        "stdio transport requires 'command'".to_string(),
                    )
                })?;
                let transport = StdioTransport::spawn(command, &config.args)
                    .await
                    .map_err(|e| RouterError::TransportInit(config.name.clone(), e.to_string()))?;
                Ok(Arc::new(transport))
            }
            TransportType::Http => {
                let url = config.url.as_ref().ok_or_else(|| {
                    RouterError::TransportInit(
                        config.name.clone(),
                        "http transport requires 'url'".to_string(),
                    )
                })?;
                Ok(Arc::new(HttpTransport::new(url.clone())))
            }
            TransportType::Sse => {
                let url = config.url.as_ref().ok_or_else(|| {
                    RouterError::TransportInit(
                        config.name.clone(),
                        "sse transport requires 'url'".to_string(),
                    )
                })?;
                let transport = SseTransport::connect(url.clone())
                    .await
                    .map_err(|e| RouterError::TransportInit(config.name.clone(), e.to_string()))?;
                Ok(Arc::new(transport))
            }
        }
    }

    /// Set a default route for unmatched requests
    pub fn with_default(mut self, route: ServerRoute) -> Self {
        self.default_route = Some(route);
        self
    }

    /// Find the route for a given path
    pub fn find_route(&self, path: &str) -> Option<&ServerRoute> {
        // Try to match a specific route first
        for route in &self.routes {
            if path.starts_with(&route.config.path_prefix) {
                return Some(route);
            }
        }

        // Fall back to default route
        self.default_route.as_ref()
    }

    /// Get the transport for a given path
    pub fn get_transport(&self, path: &str) -> Option<Arc<dyn Transport>> {
        self.find_route(path).map(|r| r.transport.clone())
    }

    /// Get the route name for a given path (for logging/metrics)
    pub fn get_route_name(&self, path: &str) -> Option<&str> {
        self.find_route(path).map(|r| r.config.name.as_str())
    }

    /// Transform the path if strip_prefix is enabled for the route
    pub fn transform_path(&self, path: &str) -> String {
        if let Some(route) = self.find_route(path) {
            if route.config.strip_prefix {
                return path
                    .strip_prefix(&route.config.path_prefix)
                    .unwrap_or(path)
                    .to_string();
            }
        }
        path.to_string()
    }

    /// Send a message to the appropriate server based on path
    pub async fn send(&self, path: &str, message: Message) -> Result<(), RouterError> {
        let route = self
            .find_route(path)
            .ok_or_else(|| RouterError::NoRoute(path.to_string()))?;

        route.transport.send(message).await.map_err(RouterError::from)
    }

    /// Receive a message from the appropriate server based on path
    pub async fn receive(&self, path: &str) -> Result<Message, RouterError> {
        let route = self
            .find_route(path)
            .ok_or_else(|| RouterError::NoRoute(path.to_string()))?;

        route.transport.receive().await.map_err(RouterError::from)
    }

    /// Get all route names for metrics/debugging
    pub fn route_names(&self) -> Vec<&str> {
        self.routes.iter().map(|r| r.config.name.as_str()).collect()
    }

    /// Check if any routes are configured
    pub fn has_routes(&self) -> bool {
        !self.routes.is_empty() || self.default_route.is_some()
    }

    /// Get the number of configured routes
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}

/// Route matcher for extracting server name from path
pub struct RouteMatcher {
    /// Map of path prefixes to server names
    prefixes: HashMap<String, String>,
}

impl RouteMatcher {
    /// Create a new route matcher from server routes
    pub fn new(routes: &[ServerRouteConfig]) -> Self {
        let mut prefixes = HashMap::new();
        for route in routes {
            prefixes.insert(route.path_prefix.clone(), route.name.clone());
        }
        Self { prefixes }
    }

    /// Match a path to a server name
    pub fn match_path(&self, path: &str) -> Option<&str> {
        // Find the longest matching prefix
        let mut best_match: Option<(&str, &String)> = None;
        for (prefix, name) in &self.prefixes {
            if path.starts_with(prefix)
                && (best_match.is_none() || prefix.len() > best_match.unwrap().0.len())
            {
                best_match = Some((prefix, name));
            }
        }
        best_match.map(|(_, name)| name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TransportType;

    fn create_test_route(name: &str, path_prefix: &str, strip: bool) -> ServerRouteConfig {
        ServerRouteConfig {
            name: name.to_string(),
            path_prefix: path_prefix.to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: strip,
        }
    }

    #[test]
    fn test_route_matcher_exact() {
        let routes = vec![
            create_test_route("github", "/github", false),
            create_test_route("filesystem", "/filesystem", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        assert_eq!(matcher.match_path("/github/repos"), Some("github"));
        assert_eq!(matcher.match_path("/filesystem/read"), Some("filesystem"));
        assert_eq!(matcher.match_path("/unknown/path"), None);
    }

    #[test]
    fn test_route_matcher_longest_prefix() {
        let routes = vec![
            create_test_route("api", "/api", false),
            create_test_route("api-v2", "/api/v2", false),
        ];
        let matcher = RouteMatcher::new(&routes);

        // Longer prefix should win
        assert_eq!(matcher.match_path("/api/v2/users"), Some("api-v2"));
        assert_eq!(matcher.match_path("/api/v1/users"), Some("api"));
    }

    #[test]
    fn test_config_validation() {
        let valid = create_test_route("test", "/test", false);
        assert!(valid.validate().is_ok());

        let mut invalid = create_test_route("test", "no-slash", false);
        assert!(invalid.validate().is_err());

        invalid.path_prefix = "/test".to_string();
        invalid.name = "".to_string();
        assert!(invalid.validate().is_err());
    }
}
