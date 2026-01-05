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
//! Tier validation for mcp-guard feature gating
//!
//! This module provides helpful error messages when users try to use features
//! that require Pro or Enterprise licenses.

use crate::config::{Config, ConfigError};

#[cfg(not(feature = "pro"))]
use crate::config::{JwtMode, TransportType};

/// Pricing page URL for upgrade messages
#[allow(dead_code)]
const PRICING_URL: &str = "https://mcp-guard.io/pricing";

/// Validate that the configuration only uses features available in the current tier.
///
/// This function checks the config against the compiled feature flags and returns
/// helpful error messages directing users to upgrade if they try to use paid features.
pub fn validate_tier(config: &Config) -> Result<(), ConfigError> {
    validate_pro_features(config)?;
    validate_enterprise_features(config)?;
    Ok(())
}

/// Validate Pro tier features
#[cfg_attr(feature = "pro", allow(unused_variables))]
fn validate_pro_features(config: &Config) -> Result<(), ConfigError> {
    // OAuth 2.1 requires Pro
    #[cfg(not(feature = "pro"))]
    if config.auth.oauth.is_some() {
        return Err(ConfigError::Validation(format!(
            "OAuth 2.1 authentication requires a Pro license.\n\n\
             Upgrade to Pro for $12/month:\n\
             → {}\n\n\
             Features included:\n\
             - OAuth 2.1 + PKCE authentication\n\
             - JWT JWKS mode (RS256/ES256)\n\
             - HTTP/SSE transports for upstream servers\n\
             - Per-identity rate limiting",
            PRICING_URL
        )));
    }

    // JWT JWKS mode requires Pro
    #[cfg(not(feature = "pro"))]
    if let Some(ref jwt_config) = config.auth.jwt {
        if matches!(jwt_config.mode, JwtMode::Jwks { .. }) {
            return Err(ConfigError::Validation(format!(
                "JWT JWKS mode (RS256/ES256) requires a Pro license.\n\n\
                 The free tier supports JWT HS256 (simple mode) only.\n\n\
                 Upgrade to Pro for $12/month:\n\
                 → {}\n\n\
                 Or switch to simple mode in your config:\n\
                 [auth.jwt]\n\
                 mode = \"simple\"\n\
                 secret = \"your-secret-key\"",
                PRICING_URL
            )));
        }
    }

    // HTTP/SSE transports require Pro
    #[cfg(not(feature = "pro"))]
    match config.upstream.transport {
        TransportType::Http => {
            return Err(ConfigError::Validation(format!(
                "HTTP transport requires a Pro license.\n\n\
                 The free tier supports stdio transport only.\n\n\
                 Upgrade to Pro for $12/month:\n\
                 → {}\n\n\
                 Or use stdio transport:\n\
                 [upstream]\n\
                 transport = \"stdio\"\n\
                 command = \"npx\"\n\
                 args = [\"-y\", \"@your/mcp-server\"]",
                PRICING_URL
            )));
        }
        TransportType::Sse => {
            return Err(ConfigError::Validation(format!(
                "SSE transport requires a Pro license.\n\n\
                 The free tier supports stdio transport only.\n\n\
                 Upgrade to Pro for $12/month:\n\
                 → {}\n\n\
                 Or use stdio transport:\n\
                 [upstream]\n\
                 transport = \"stdio\"\n\
                 command = \"npx\"\n\
                 args = [\"-y\", \"@your/mcp-server\"]",
                PRICING_URL
            )));
        }
        TransportType::Stdio => {}
    }

    Ok(())
}

/// Validate Enterprise tier features
#[cfg_attr(feature = "enterprise", allow(unused_variables))]
fn validate_enterprise_features(config: &Config) -> Result<(), ConfigError> {
    // mTLS requires Enterprise
    #[cfg(not(feature = "enterprise"))]
    if let Some(ref mtls_config) = config.auth.mtls {
        if mtls_config.enabled {
            return Err(ConfigError::Validation(format!(
                "mTLS client certificate authentication requires an Enterprise license.\n\n\
                 Upgrade to Enterprise:\n\
                 → {}\n\n\
                 Enterprise features include:\n\
                 - mTLS client certificate authentication\n\
                 - Multi-server routing\n\
                 - SIEM audit log shipping\n\
                 - OpenTelemetry tracing\n\
                 - Admin guard tools (keys, audit, config)",
                PRICING_URL
            )));
        }
    }

    // Multi-server routing requires Enterprise
    #[cfg(not(feature = "enterprise"))]
    if !config.upstream.servers.is_empty() {
        return Err(ConfigError::Validation(format!(
            "Multi-server routing requires an Enterprise license.\n\n\
             You have {} servers configured. The free tier supports single-server mode only.\n\n\
             Upgrade to Enterprise:\n\
             → {}\n\n\
             Or use single-server mode:\n\
             [upstream]\n\
             transport = \"stdio\"\n\
             command = \"your-mcp-server\"",
            config.upstream.servers.len(),
            PRICING_URL
        )));
    }

    // SIEM audit log shipping requires Enterprise
    #[cfg(not(feature = "enterprise"))]
    if config.audit.export_url.is_some() {
        return Err(ConfigError::Validation(format!(
            "Audit log shipping to SIEM requires an Enterprise license.\n\n\
             The free tier supports local file and console logging only.\n\n\
             Upgrade to Enterprise:\n\
             → {}\n\n\
             Or use local logging:\n\
             [audit]\n\
             enabled = true\n\
             file = \"/var/log/mcp-guard.log\"",
            PRICING_URL
        )));
    }

    // OpenTelemetry tracing requires Enterprise
    #[cfg(not(feature = "enterprise"))]
    if config.tracing.enabled && config.tracing.otlp_endpoint.is_some() {
        return Err(ConfigError::Validation(format!(
            "OpenTelemetry tracing with OTLP export requires an Enterprise license.\n\n\
             The free tier supports local tracing only.\n\n\
             Upgrade to Enterprise:\n\
             → {}\n\n\
             Or disable OTLP export:\n\
             [tracing]\n\
             enabled = true\n\
             # Remove otlp_endpoint to use local tracing only",
            PRICING_URL
        )));
    }

    // Per-tool rate limits require Enterprise
    #[cfg(not(feature = "enterprise"))]
    if !config.rate_limit.tool_limits.is_empty() {
        return Err(ConfigError::Validation(format!(
            "Per-tool rate limiting requires an Enterprise license.\n\n\
             You have {} tool limit rules configured. The free tier supports global rate limiting only.\n\n\
             Upgrade to Enterprise:\n\
             → {}\n\n\
             Or use global rate limiting:\n\
             [rate_limit]\n\
             enabled = true\n\
             requests_per_second = 25\n\
             burst_size = 10",
            config.rate_limit.tool_limits.len(),
            PRICING_URL
        )));
    }

    Ok(())
}

/// Get the current tier name based on compiled features
pub fn current_tier() -> &'static str {
    #[cfg(feature = "enterprise")]
    {
        "Enterprise"
    }
    #[cfg(all(feature = "pro", not(feature = "enterprise")))]
    {
        "Pro"
    }
    #[cfg(not(any(feature = "pro", feature = "enterprise")))]
    {
        "Free"
    }
}

/// Check if a feature is available in the current tier
pub fn is_feature_available(feature: &str) -> bool {
    match feature {
        // Free tier features
        "api_key_auth" | "jwt_hs256" | "stdio_transport" | "global_rate_limit" | "file_audit"
        | "console_audit" | "prometheus_metrics" => true,

        // Pro tier features
        "oauth" | "jwt_jwks" | "http_transport" | "sse_transport" | "per_identity_rate_limit" => {
            cfg!(feature = "pro") || cfg!(feature = "enterprise")
        }

        // Enterprise tier features
        "mtls"
        | "multi_server_routing"
        | "siem_audit"
        | "opentelemetry"
        | "per_tool_rate_limit"
        | "admin_guard_tools" => cfg!(feature = "enterprise"),

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        AuditConfig, AuthConfig, MtlsConfig, RateLimitConfig, ServerConfig, ServerRouteConfig,
        TracingConfig, UpstreamConfig,
    };

    fn create_minimal_config() -> Config {
        Config {
            server: ServerConfig::default(),
            auth: AuthConfig::default(),
            rate_limit: RateLimitConfig::default(),
            audit: AuditConfig::default(),
            tracing: TracingConfig::default(),
            upstream: UpstreamConfig {
                transport: TransportType::Stdio,
                command: Some("echo".to_string()),
                args: vec![],
                url: None,
                servers: vec![],
            },
            database_url: None,
        }
    }

    #[test]
    fn test_current_tier() {
        let tier = current_tier();
        // Just verify it returns one of the expected values
        assert!(
            tier == "Free" || tier == "Pro" || tier == "Enterprise",
            "Unexpected tier: {}",
            tier
        );
    }

    #[test]
    fn test_free_tier_features_always_available() {
        assert!(is_feature_available("api_key_auth"));
        assert!(is_feature_available("jwt_hs256"));
        assert!(is_feature_available("stdio_transport"));
        assert!(is_feature_available("global_rate_limit"));
        assert!(is_feature_available("file_audit"));
        assert!(is_feature_available("prometheus_metrics"));
    }

    #[test]
    fn test_unknown_feature() {
        assert!(!is_feature_available("unknown_feature"));
    }

    #[test]
    fn test_validate_tier_minimal_config() {
        let config = create_minimal_config();
        // A minimal config should always pass tier validation
        assert!(validate_tier(&config).is_ok());
    }

    #[cfg(not(feature = "pro"))]
    #[test]
    fn test_oauth_requires_pro() {
        use crate::config::{OAuthConfig, OAuthProvider};

        let mut config = create_minimal_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "http://localhost:3000/callback".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: Default::default(),
            token_cache_ttl_secs: 300,
        });

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("OAuth 2.1"));
        assert!(err.contains("Pro license"));
    }

    #[cfg(not(feature = "pro"))]
    #[test]
    fn test_http_transport_requires_pro() {
        let mut config = create_minimal_config();
        config.upstream.transport = TransportType::Http;
        config.upstream.url = Some("http://localhost:8080".to_string());
        config.upstream.command = None;

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("HTTP transport"));
        assert!(err.contains("Pro license"));
    }

    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_mtls_requires_enterprise() {
        let mut config = create_minimal_config();
        config.auth.mtls = Some(MtlsConfig {
            enabled: true,
            identity_source: crate::config::MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.0/8".to_string()],
        });

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("mTLS"));
        assert!(err.contains("Enterprise license"));
    }

    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_multi_server_requires_enterprise() {
        let mut config = create_minimal_config();
        config.upstream.servers = vec![
            ServerRouteConfig {
                name: "server1".to_string(),
                path_prefix: "/s1".to_string(),
                transport: TransportType::Stdio,
                command: Some("echo".to_string()),
                args: vec![],
                url: None,
                strip_prefix: false,
            },
            ServerRouteConfig {
                name: "server2".to_string(),
                path_prefix: "/s2".to_string(),
                transport: TransportType::Stdio,
                command: Some("echo".to_string()),
                args: vec![],
                url: None,
                strip_prefix: false,
            },
        ];

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Multi-server routing"));
        assert!(err.contains("Enterprise license"));
    }

    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_siem_audit_requires_enterprise() {
        let mut config = create_minimal_config();
        config.audit.export_url = Some("https://siem.example.com/logs".to_string());

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("SIEM"));
        assert!(err.contains("Enterprise license"));
    }

    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_opentelemetry_requires_enterprise() {
        let mut config = create_minimal_config();
        config.tracing.enabled = true;
        config.tracing.otlp_endpoint = Some("http://jaeger:4317".to_string());

        let result = validate_tier(&config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("OpenTelemetry"));
        assert!(err.contains("Enterprise license"));
    }
}
