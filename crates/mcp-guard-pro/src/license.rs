// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: LicenseRef-Commercial
//
// This file is part of MCP-Guard Pro, a commercial product.
//
// MCP-Guard Pro requires a valid commercial license for use.
// Unauthorized use, modification, or distribution is prohibited.
//
// For licensing information, visit: https://mcp-guard.io/pricing
// For support, contact: austin@botzr.dev
//! License validation for mcp-guard Pro
//!
//! Pro licenses use Ed25519 signature verification for offline validation.
//! License keys are base64-encoded and contain:
//! - License metadata (tier, features, expiry)
//! - Ed25519 signature from the license server
//!
//! # License Key Format
//!
//! ```text
//! pro_<base64-encoded-payload>.<base64-encoded-signature>
//! ```
//!
//! The payload is a JSON object:
//! ```json
//! {
//!   "tier": "pro",
//!   "issued_at": "2024-01-01T00:00:00Z",
//!   "expires_at": "2025-01-01T00:00:00Z",
//!   "licensee": "user@example.com",
//!   "features": ["oauth", "http_transport", "sse_transport", "per_identity_rate_limit"]
//! }
//! ```

use base64::Engine;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Public key for verifying Pro licenses (Ed25519)
/// This is the public half of the keypair used by the license server
/// Generated: 2025-12-31
/// CRITICAL: The private key MUST be stored securely and never committed to git
const PRO_LICENSE_PUBLIC_KEY: &str = "MCowBQYDK2VwAyEAmpIHTFtL64jVCS7KtGKeRTLEGaTktCOBOsr0GPJfWFU=";

/// Error type for license operations
#[derive(Debug, thiserror::Error)]
pub enum LicenseError {
    #[error("Invalid license key format")]
    InvalidFormat,

    #[error("License key decode error: {0}")]
    DecodeError(String),

    #[error("Invalid license signature")]
    InvalidSignature,

    #[error("License has expired (expired at {0})")]
    Expired(DateTime<Utc>),

    #[error("License tier mismatch: expected 'pro', got '{0}'")]
    TierMismatch(String),

    #[error("Missing required feature: {0}")]
    MissingFeature(String),

    #[error("License validation error: {0}")]
    ValidationError(String),
}

/// Pro license payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    /// License tier (should be "pro")
    pub tier: String,

    /// When the license was issued
    pub issued_at: DateTime<Utc>,

    /// When the license expires
    pub expires_at: DateTime<Utc>,

    /// Email or identifier of the licensee
    pub licensee: String,

    /// List of enabled features
    #[serde(default)]
    pub features: Vec<String>,
}

/// Validated Pro license
#[derive(Debug, Clone)]
pub struct ProLicense {
    /// The validated license payload
    pub payload: LicensePayload,

    /// The original license key
    pub key: String,
}

impl ProLicense {
    /// Validate a Pro license key
    ///
    /// This performs offline validation using Ed25519 signature verification.
    ///
    /// # Arguments
    /// * `key` - The license key in format `pro_<payload>.<signature>`
    ///
    /// # Returns
    /// * `Ok(ProLicense)` if the license is valid and not expired
    /// * `Err(LicenseError)` if validation fails
    ///
    /// # Example
    /// ```rust,ignore
    /// let license = ProLicense::validate("pro_xxx...yyy")?;
    /// println!("Licensed to: {}", license.payload.licensee);
    /// ```
    pub fn validate(key: &str) -> Result<Self, LicenseError> {
        // Check prefix
        let key_body = key
            .strip_prefix("pro_")
            .ok_or(LicenseError::InvalidFormat)?;

        // Split into payload and signature
        let parts: Vec<&str> = key_body.split('.').collect();
        if parts.len() != 2 {
            return Err(LicenseError::InvalidFormat);
        }

        let payload_b64 = parts[0];
        let signature_b64 = parts[1];

        // Decode payload
        let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|e| LicenseError::DecodeError(e.to_string()))?;

        let payload_json = String::from_utf8(payload_bytes)
            .map_err(|e| LicenseError::DecodeError(e.to_string()))?;

        let payload: LicensePayload = serde_json::from_str(&payload_json)
            .map_err(|e| LicenseError::DecodeError(e.to_string()))?;

        // Verify signature
        Self::verify_signature(payload_b64.as_bytes(), signature_b64)?;

        // Check tier
        if payload.tier != "pro" {
            return Err(LicenseError::TierMismatch(payload.tier.clone()));
        }

        // Check expiry
        if payload.expires_at < Utc::now() {
            return Err(LicenseError::Expired(payload.expires_at));
        }

        Ok(Self {
            payload,
            key: key.to_string(),
        })
    }

    /// Verify the Ed25519 signature
    fn verify_signature(message: &[u8], signature_b64: &str) -> Result<(), LicenseError> {
        // Decode public key (SPKI format)
        let public_key_der = base64::engine::general_purpose::STANDARD
            .decode(PRO_LICENSE_PUBLIC_KEY)
            .map_err(|_| LicenseError::ValidationError("Invalid public key".to_string()))?;

        // Skip SPKI header (12 bytes) to get raw Ed25519 public key
        if public_key_der.len() < 44 {
            return Err(LicenseError::ValidationError(
                "Public key too short".to_string(),
            ));
        }
        let raw_key = &public_key_der[12..44];

        let verifying_key =
            VerifyingKey::from_bytes(raw_key.try_into().map_err(|_| {
                LicenseError::ValidationError("Invalid public key bytes".to_string())
            })?)
            .map_err(|_| LicenseError::ValidationError("Invalid Ed25519 public key".to_string()))?;

        // Decode signature
        let signature_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(signature_b64)
            .map_err(|e| LicenseError::DecodeError(e.to_string()))?;

        let signature =
            Signature::from_slice(&signature_bytes).map_err(|_| LicenseError::InvalidSignature)?;

        // Verify
        verifying_key
            .verify(message, &signature)
            .map_err(|_| LicenseError::InvalidSignature)?;

        Ok(())
    }

    /// Check if a specific feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.payload.features.iter().any(|f| f == feature)
    }

    /// Get days until expiry
    pub fn days_until_expiry(&self) -> i64 {
        let now = Utc::now();
        (self.payload.expires_at - now).num_days()
    }

    /// Check if the license is about to expire (within 30 days)
    pub fn is_expiring_soon(&self) -> bool {
        self.days_until_expiry() <= 30
    }
}

/// Pro features that can be enabled by license
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProFeature {
    /// OAuth 2.1 + PKCE authentication
    OAuth,
    /// JWT JWKS mode (auto-refreshing keys)
    JwtJwks,
    /// HTTP transport to upstream servers
    HttpTransport,
    /// SSE transport for streaming
    SseTransport,
    /// Per-identity rate limiting
    PerIdentityRateLimit,
}

impl ProFeature {
    /// Get the feature identifier string
    pub fn as_str(&self) -> &'static str {
        match self {
            ProFeature::OAuth => "oauth",
            ProFeature::JwtJwks => "jwt_jwks",
            ProFeature::HttpTransport => "http_transport",
            ProFeature::SseTransport => "sse_transport",
            ProFeature::PerIdentityRateLimit => "per_identity_rate_limit",
        }
    }

    /// All Pro features
    pub fn all() -> &'static [ProFeature] {
        &[
            ProFeature::OAuth,
            ProFeature::JwtJwks,
            ProFeature::HttpTransport,
            ProFeature::SseTransport,
            ProFeature::PerIdentityRateLimit,
        ]
    }
}

impl FromStr for ProFeature {
    type Err = LicenseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "oauth" => Ok(ProFeature::OAuth),
            "jwt_jwks" => Ok(ProFeature::JwtJwks),
            "http_transport" => Ok(ProFeature::HttpTransport),
            "sse_transport" => Ok(ProFeature::SseTransport),
            "per_identity_rate_limit" => Ok(ProFeature::PerIdentityRateLimit),
            _ => Err(LicenseError::MissingFeature(s.to_string())),
        }
    }
}

/// Check if Pro features are available (license validated)
///
/// This is a runtime check that can be used to gate Pro functionality.
pub fn is_pro_licensed() -> bool {
    // Check environment variable for license key
    std::env::var("MCP_GUARD_LICENSE_KEY")
        .ok()
        .and_then(|key| ProLicense::validate(&key).ok())
        .is_some()
}

/// Get the validated Pro license from environment
///
/// Returns `None` if no valid license is found.
pub fn get_pro_license() -> Option<ProLicense> {
    std::env::var("MCP_GUARD_LICENSE_KEY")
        .ok()
        .and_then(|key| ProLicense::validate(&key).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            ProLicense::validate("invalid"),
            Err(LicenseError::InvalidFormat)
        ));

        assert!(matches!(
            ProLicense::validate("pro_onlyonepart"),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn test_pro_feature_as_str() {
        assert_eq!(ProFeature::OAuth.as_str(), "oauth");
        assert_eq!(ProFeature::HttpTransport.as_str(), "http_transport");
    }

    #[test]
    fn test_pro_feature_from_str() {
        assert_eq!(ProFeature::from_str("oauth").unwrap(), ProFeature::OAuth);
        assert!(ProFeature::from_str("unknown").is_err());
    }

    #[test]
    fn test_is_pro_licensed_no_env() {
        // Without env var, should return false
        std::env::remove_var("MCP_GUARD_LICENSE_KEY");
        assert!(!is_pro_licensed());
    }
}
