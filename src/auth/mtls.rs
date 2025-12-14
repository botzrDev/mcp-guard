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

use async_trait::async_trait;
use std::collections::HashMap;

use crate::auth::{AuthError, AuthProvider, Identity};
use crate::config::{MtlsConfig, MtlsIdentitySource};

/// Header names for client certificate info (from reverse proxy)
pub const HEADER_CLIENT_CERT_CN: &str = "X-Client-Cert-CN";
pub const HEADER_CLIENT_CERT_SAN_DNS: &str = "X-Client-Cert-SAN-DNS";
pub const HEADER_CLIENT_CERT_SAN_EMAIL: &str = "X-Client-Cert-SAN-Email";
pub const HEADER_CLIENT_CERT_VERIFIED: &str = "X-Client-Cert-Verified";

/// mTLS authentication provider
///
/// Extracts identity from client certificates that have been validated
/// at the TLS layer (either by this server or a reverse proxy).
pub struct MtlsAuthProvider {
    config: MtlsConfig,
}

impl MtlsAuthProvider {
    /// Create a new mTLS auth provider
    pub fn new(config: MtlsConfig) -> Self {
        Self { config }
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
            MtlsIdentitySource::SanEmail => cert_info
                .san_email
                .first()
                .cloned()
                .ok_or_else(|| AuthError::Internal("No Email SAN in client certificate".into()))?,
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
    /// Create ClientCertInfo from HTTP headers (reverse proxy scenario)
    ///
    /// Headers expected:
    /// - X-Client-Cert-CN: Common Name from certificate
    /// - X-Client-Cert-SAN-DNS: Comma-separated DNS SANs
    /// - X-Client-Cert-SAN-Email: Comma-separated email SANs
    /// - X-Client-Cert-Verified: "SUCCESS" if verified
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Option<Self> {
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
        };

        let provider = MtlsAuthProvider::new(config);
        assert_eq!(provider.name(), "mtls");
    }

    #[test]
    fn test_extract_identity_from_cn() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
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

        let cert_info = ClientCertInfo::from_headers(&headers).unwrap();
        assert_eq!(cert_info.common_name, Some("my-service".to_string()));
        assert!(cert_info.verified);
        assert_eq!(cert_info.san_dns.len(), 2);
        assert_eq!(cert_info.san_dns[0], "service.example.com");
        assert_eq!(cert_info.san_dns[1], "api.example.com");
    }

    #[test]
    fn test_client_cert_info_from_headers_not_verified() {
        let headers = HeaderMap::new();

        let cert_info = ClientCertInfo::from_headers(&headers);
        assert!(cert_info.is_none());
    }

    #[tokio::test]
    async fn test_authenticate_with_cn_token() {
        let config = MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
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
