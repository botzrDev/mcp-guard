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
//! mTLS client certificate authentication provider
//!
//! This provider extracts identity from client certificates, supporting two modes:
//! 1. Header-based: When TLS is terminated at a reverse proxy (nginx, HAProxy) that
//!    forwards client certificate info in headers (X-Client-Cert-CN, X-Client-Cert-SAN)
//! 2. Native: When using axum-server with rustls (requires additional setup)
//!
//! Common deployment pattern:
//! - Load balancer terminates mTLS and validates client certificates
//! - Load balancer forwards certificate info in HTTP headers
//! - mcp-guard extracts identity from headers
//!
//! SECURITY: When using header-based mTLS, you MUST configure `trusted_proxy_ips`
//! to prevent header spoofing attacks. Only requests from trusted proxy IPs will
//! have their mTLS headers honored.

use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;

use crate::auth::{AuthError, AuthProvider, Identity};
use crate::config::{MtlsConfig, MtlsIdentitySource};

/// Header names for client certificate info (from reverse proxy)
pub const HEADER_CLIENT_CERT_CN: &str = "X-Client-Cert-CN";
pub const HEADER_CLIENT_CERT_SAN_DNS: &str = "X-Client-Cert-SAN-DNS";
pub const HEADER_CLIENT_CERT_SAN_EMAIL: &str = "X-Client-Cert-SAN-Email";
pub const HEADER_CLIENT_CERT_VERIFIED: &str = "X-Client-Cert-Verified";

/// Trusted proxy IP validator
///
/// Validates that incoming requests with mTLS headers come from trusted proxy IPs.
/// This prevents header spoofing attacks where an attacker directly connects to
/// the server and sets fake mTLS headers.
#[derive(Debug, Clone)]
pub struct TrustedProxyValidator {
    /// List of trusted IP addresses and CIDR ranges
    trusted_ranges: Vec<TrustedRange>,
}

/// A trusted IP range (either single IP or CIDR block)
#[derive(Debug, Clone)]
enum TrustedRange {
    Single(IpAddr),
    Cidr { network: IpAddr, prefix_len: u8 },
}

impl TrustedProxyValidator {
    /// Create a new validator from a list of IP/CIDR strings
    ///
    /// Accepts formats:
    /// - Single IP: "192.168.1.1", "::1"
    /// - CIDR: "10.0.0.0/8", "fd00::/8"
    pub fn new(trusted_ips: &[String]) -> Self {
        let trusted_ranges = trusted_ips
            .iter()
            .filter_map(|s| Self::parse_range(s))
            .collect();

        Self { trusted_ranges }
    }

    /// Parse an IP or CIDR range string
    fn parse_range(s: &str) -> Option<TrustedRange> {
        let s = s.trim();

        if let Some((ip_str, prefix_str)) = s.split_once('/') {
            // CIDR format
            let network: IpAddr = ip_str.parse().ok()?;
            let prefix_len: u8 = prefix_str.parse().ok()?;

            // Validate prefix length
            let max_prefix = match network {
                IpAddr::V4(_) => 32,
                IpAddr::V6(_) => 128,
            };
            if prefix_len > max_prefix {
                return None;
            }

            Some(TrustedRange::Cidr {
                network,
                prefix_len,
            })
        } else {
            // Single IP
            let ip: IpAddr = s.parse().ok()?;
            Some(TrustedRange::Single(ip))
        }
    }

    /// Check if an IP address is trusted
    pub fn is_trusted(&self, ip: &IpAddr) -> bool {
        if self.trusted_ranges.is_empty() {
            // No trusted ranges configured = no IPs are trusted
            return false;
        }

        for range in &self.trusted_ranges {
            match range {
                TrustedRange::Single(trusted_ip) => {
                    if ip == trusted_ip {
                        return true;
                    }
                }
                TrustedRange::Cidr {
                    network,
                    prefix_len,
                } => {
                    if Self::ip_in_cidr(ip, network, *prefix_len) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if an IP address is within a CIDR range
    fn ip_in_cidr(ip: &IpAddr, network: &IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (IpAddr::V4(ip), IpAddr::V4(net)) => {
                let ip_bits = u32::from_be_bytes(ip.octets());
                let net_bits = u32::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u32 << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (IpAddr::V6(ip), IpAddr::V6(net)) => {
                let ip_bits = u128::from_be_bytes(ip.octets());
                let net_bits = u128::from_be_bytes(net.octets());
                let mask = if prefix_len == 0 {
                    0
                } else {
                    !0u128 << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4 and IPv6 don't match
        }
    }

    /// Check if the validator has any trusted ranges configured
    pub fn has_trusted_ranges(&self) -> bool {
        !self.trusted_ranges.is_empty()
    }
}

/// mTLS authentication provider
///
/// Extracts identity from client certificates that have been validated
/// at the TLS layer (either by this server or a reverse proxy).
///
/// SECURITY: When using header-based mTLS, configure `trusted_proxy_ips` in the
/// config to prevent header spoofing. Without this, mTLS header auth is disabled.
pub struct MtlsAuthProvider {
    config: MtlsConfig,
    proxy_validator: TrustedProxyValidator,
}

impl MtlsAuthProvider {
    /// Create a new mTLS auth provider
    pub fn new(config: MtlsConfig) -> Self {
        let proxy_validator = TrustedProxyValidator::new(&config.trusted_proxy_ips);

        if config.enabled && !proxy_validator.has_trusted_ranges() {
            tracing::warn!(
                "mTLS authentication enabled but no trusted_proxy_ips configured. \
                 mTLS header authentication will be DISABLED to prevent header spoofing. \
                 Configure trusted_proxy_ips with your reverse proxy IPs."
            );
        }

        Self {
            config,
            proxy_validator,
        }
    }

    /// Check if a client IP is trusted to set mTLS headers
    pub fn is_trusted_proxy(&self, client_ip: &IpAddr) -> bool {
        self.proxy_validator.is_trusted(client_ip)
    }

    /// Check if the provider has trusted proxies configured
    pub fn has_trusted_proxies(&self) -> bool {
        self.proxy_validator.has_trusted_ranges()
    }

    /// Extract identity from client certificate info
    ///
    /// The `cert_info` contains certificate details that were extracted from
    /// either TLS connection state or HTTP headers (from reverse proxy).
    pub fn extract_identity(&self, cert_info: &ClientCertInfo) -> Result<Identity, AuthError> {
        // Extract identity based on configured source
        let id = match self.config.identity_source {
            MtlsIdentitySource::Cn => cert_info
                .common_name
                .clone()
                .ok_or_else(|| AuthError::Internal("No CN in client certificate".into()))?,
            MtlsIdentitySource::SanDns => cert_info
                .san_dns
                .first()
                .cloned()
                .ok_or_else(|| AuthError::Internal("No DNS SAN in client certificate".into()))?,
            MtlsIdentitySource::SanEmail => {
                cert_info.san_email.first().cloned().ok_or_else(|| {
                    AuthError::Internal("No Email SAN in client certificate".into())
                })?
            }
        };

        let allowed_tools = if self.config.allowed_tools.is_empty() {
            None
        } else {
            Some(self.config.allowed_tools.clone())
        };

        let mut claims = HashMap::new();
        claims.insert(
            "auth_method".to_string(),
            serde_json::Value::String("mtls".to_string()),
        );
        if let Some(ref cn) = cert_info.common_name {
            claims.insert("cn".to_string(), serde_json::Value::String(cn.clone()));
        }

        Ok(Identity {
            id,
            name: cert_info.common_name.clone(),
            allowed_tools,
            rate_limit: self.config.rate_limit,
            claims,
        })
    }
}

#[async_trait]
impl AuthProvider for MtlsAuthProvider {
    /// Authenticate using client certificate
    ///
    /// Note: For mTLS, the "token" parameter is expected to be the CN from
    /// the client certificate header (X-Client-Cert-CN). For full cert info,
    /// use `extract_identity` directly with `ClientCertInfo`.
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
        // When called via the AuthProvider trait, we only have the token
        // which should contain the CN from the header
        if token.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        // Create minimal cert info from the CN
        let cert_info = ClientCertInfo {
            common_name: Some(token.to_string()),
            san_dns: vec![],
            san_email: vec![],
            verified: true,
        };

        self.extract_identity(&cert_info)
    }

    fn name(&self) -> &str {
        "mtls"
    }
}

/// Client certificate information extracted from TLS connection or headers
#[derive(Debug, Clone, Default)]
pub struct ClientCertInfo {
    /// Common Name (CN) from certificate subject
    pub common_name: Option<String>,
    /// DNS names from Subject Alternative Name (SAN) extension
    pub san_dns: Vec<String>,
    /// Email addresses from Subject Alternative Name (SAN) extension
    pub san_email: Vec<String>,
    /// Whether the certificate was verified
    pub verified: bool,
}

impl ClientCertInfo {
    /// Create ClientCertInfo from HTTP headers with trusted proxy validation
    ///
    /// SECURITY: This validates that the client IP is from a trusted proxy before
    /// accepting the mTLS headers. If the client IP is not trusted, returns None.
    ///
    /// Headers expected:
    /// - X-Client-Cert-CN: Common Name from certificate
    /// - X-Client-Cert-SAN-DNS: Comma-separated DNS SANs
    /// - X-Client-Cert-SAN-Email: Comma-separated email SANs
    /// - X-Client-Cert-Verified: "SUCCESS" if verified
    pub fn from_headers_if_trusted(
        headers: &axum::http::HeaderMap,
        client_ip: &IpAddr,
        mtls_provider: &MtlsAuthProvider,
    ) -> Option<Self> {
        // SECURITY: Only accept mTLS headers from trusted proxy IPs
        if !mtls_provider.is_trusted_proxy(client_ip) {
            if mtls_provider.has_trusted_proxies() {
                tracing::warn!(
                    client_ip = %client_ip,
                    "Rejecting mTLS headers from untrusted IP"
                );
            }
            return None;
        }

        Self::from_headers_unchecked(headers)
    }

    /// Create ClientCertInfo from HTTP headers WITHOUT trusted proxy validation
    ///
    /// # Safety
    /// This method does NOT validate the client IP. Only use when:
    /// 1. You have already validated the client IP separately
    /// 2. In tests with trusted data
    /// 3. When TLS is terminated by the same server (no proxy headers)
    pub fn from_headers_unchecked(headers: &axum::http::HeaderMap) -> Option<Self> {
        // Check if cert was verified
        let verified = headers
            .get(HEADER_CLIENT_CERT_VERIFIED)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("success") || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        // Get CN
        let common_name = headers
            .get(HEADER_CLIENT_CERT_CN)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // If no CN and not verified, no valid cert info
        if common_name.is_none() && !verified {
            return None;
        }

        // Get SANs (comma-separated)
        let san_dns = headers
            .get(HEADER_CLIENT_CERT_SAN_DNS)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        let san_email = headers
            .get(HEADER_CLIENT_CERT_SAN_EMAIL)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        Some(ClientCertInfo {
            common_name,
            san_dns,
            san_email,
            verified,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn test_mtls_provider_creation() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(100),
            trusted_proxy_ips: vec!["127.0.0.1".to_string()],
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
        assert!(provider.has_trusted_proxies());
    }

    // --------------------------------------------------------------------------
    // Trusted Proxy Validation Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_trusted_proxy_single_ip() {
        let validator =
            TrustedProxyValidator::new(&["10.0.0.1".to_string(), "192.168.1.100".to_string()]);

        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.1.100".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.2".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn test_trusted_proxy_cidr() {
        let validator =
            TrustedProxyValidator::new(&["10.0.0.0/8".to_string(), "192.168.0.0/16".to_string()]);

        // Should match 10.x.x.x
        assert!(validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"10.255.255.255".parse().unwrap()));

        // Should match 192.168.x.x
        assert!(validator.is_trusted(&"192.168.0.1".parse().unwrap()));
        assert!(validator.is_trusted(&"192.168.255.255".parse().unwrap()));

        // Should not match others
        assert!(!validator.is_trusted(&"11.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"192.169.0.1".parse().unwrap()));
    }

    #[test]
    fn test_trusted_proxy_empty_rejects_all() {
        let validator = TrustedProxyValidator::new(&[]);

        // Empty config should reject all IPs
        assert!(!validator.is_trusted(&"127.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"10.0.0.1".parse().unwrap()));
        assert!(!validator.is_trusted(&"8.8.8.8".parse().unwrap()));
    }

    #[test]
    fn test_trusted_proxy_ipv6() {
        let validator = TrustedProxyValidator::new(&["::1".to_string(), "fd00::/8".to_string()]);

        assert!(validator.is_trusted(&"::1".parse().unwrap()));
        assert!(validator.is_trusted(&"fd00::1".parse().unwrap()));
        assert!(!validator.is_trusted(&"fe80::1".parse().unwrap()));
    }

    #[test]
    fn test_from_headers_if_trusted_accepts_trusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "trusted-client".parse().unwrap());

        let trusted_ip: IpAddr = "10.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &trusted_ip, &provider);

        assert!(cert_info.is_some());
        assert_eq!(
            cert_info.unwrap().common_name,
            Some("trusted-client".to_string())
        );
    }

    #[test]
    fn test_from_headers_if_trusted_rejects_untrusted() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.1".to_string()],
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "spoofed-client".parse().unwrap());

        // Attacker IP not in trusted list
        let attacker_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &attacker_ip, &provider);

        assert!(cert_info.is_none()); // Headers should be rejected
    }

    #[test]
    fn test_from_headers_if_trusted_rejects_when_no_trusted_configured() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // No trusted IPs!
        };
        let provider = MtlsAuthProvider::new(config);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "any-client".parse().unwrap());

        // Even localhost should be rejected
        let localhost: IpAddr = "127.0.0.1".parse().unwrap();
        let cert_info = ClientCertInfo::from_headers_if_trusted(&headers, &localhost, &provider);

        assert!(cert_info.is_none()); // No trusted proxies = reject all header auth
    }

    // --------------------------------------------------------------------------
    // Existing Tests (updated to use from_headers_unchecked)
    // --------------------------------------------------------------------------

    #[test]
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "service-client");
        assert_eq!(identity.name, Some("service-client".to_string()));
        assert!(identity.allowed_tools.is_none());
    }

    #[test]
    fn test_extract_identity_from_san_dns() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::SanDns,
            allowed_tools: vec!["read_file".to_string()],
            rate_limit: Some(50),
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: Some("service-client".to_string()),
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let identity = provider.extract_identity(&cert_info).unwrap();
        assert_eq!(identity.id, "client.example.com");
        assert_eq!(identity.allowed_tools, Some(vec!["read_file".to_string()]));
        assert_eq!(identity.rate_limit, Some(50));
    }

    #[test]
    fn test_extract_identity_missing_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let cert_info = ClientCertInfo {
            common_name: None,
            san_dns: vec!["client.example.com".to_string()],
            san_email: vec![],
            verified: true,
        };

        let result = provider.extract_identity(&cert_info);
        assert!(result.is_err());
    }

    #[test]
    fn test_client_cert_info_from_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(HEADER_CLIENT_CERT_VERIFIED, "SUCCESS".parse().unwrap());
        headers.insert(HEADER_CLIENT_CERT_CN, "my-service".parse().unwrap());
        headers.insert(
            HEADER_CLIENT_CERT_SAN_DNS,
            "service.example.com, api.example.com".parse().unwrap(),
        );

        let cert_info = ClientCertInfo::from_headers_unchecked(&headers).unwrap();
        assert_eq!(cert_info.common_name, Some("my-service".to_string()));
        assert!(cert_info.verified);
        assert_eq!(cert_info.san_dns.len(), 2);
        assert_eq!(cert_info.san_dns[0], "service.example.com");
        assert_eq!(cert_info.san_dns[1], "api.example.com");
    }

    #[test]
    fn test_client_cert_info_from_headers_not_verified() {
        let headers = HeaderMap::new();

        let cert_info = ClientCertInfo::from_headers_unchecked(&headers);
        assert!(cert_info.is_none());
    }

    #[tokio::test]
    async fn test_authenticate_with_cn_token() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        };

        let provider = MtlsAuthProvider::new(config);
        let identity = provider.authenticate("my-client-cn").await.unwrap();
        assert_eq!(identity.id, "my-client-cn");
    }

    #[tokio::test]
    async fn test_authenticate_empty_token() {
        let config = MtlsConfig::default();
        let provider = MtlsAuthProvider::new(config);
        let result = provider.authenticate("").await;
        assert!(result.is_err());
    }
}
