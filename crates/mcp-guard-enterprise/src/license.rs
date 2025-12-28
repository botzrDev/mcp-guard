//! License validation for mcp-guard Enterprise
//!
//! Enterprise licenses use online validation with offline caching:
//! 1. First, check local cache for a valid, unexpired license
//! 2. If cache miss or expired, validate online via keygen.sh API
//! 3. Cache successful validation for 30 days (offline grace period)
//!
//! This allows Enterprise deployments to continue working during
//! network outages while still enforcing license compliance.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Keygen.sh API endpoint for license validation
const KEYGEN_API_URL: &str = "https://api.keygen.sh/v1/accounts";

/// Account ID for mcp-guard on keygen.sh
const KEYGEN_ACCOUNT_ID: &str = "YOUR_KEYGEN_ACCOUNT_ID";

/// Offline grace period in days
const OFFLINE_GRACE_DAYS: i64 = 30;

/// Error type for license operations
#[derive(Debug, thiserror::Error)]
pub enum LicenseError {
    #[error("License key not found. Set MCP_GUARD_LICENSE_KEY environment variable.")]
    NotFound,

    #[error("Invalid license key format")]
    InvalidFormat,

    #[error("License validation failed: {0}")]
    ValidationFailed(String),

    #[error("License has expired (expired at {0})")]
    Expired(DateTime<Utc>),

    #[error("License tier mismatch: expected 'enterprise', got '{0}'")]
    TierMismatch(String),

    #[error("Missing required feature: {0}")]
    MissingFeature(String),

    #[error("Network error during validation: {0}")]
    NetworkError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Offline grace period expired. Please connect to the internet to revalidate.")]
    OfflineGraceExpired,
}

/// Enterprise license payload from keygen.sh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseData {
    /// License ID
    pub id: String,

    /// License key
    pub key: String,

    /// License tier (should be "enterprise")
    pub tier: String,

    /// Licensee name/email
    pub licensee: String,

    /// Maximum seats (users) allowed
    pub max_seats: Option<u32>,

    /// When the license expires
    pub expires_at: Option<DateTime<Utc>>,

    /// Enabled features
    #[serde(default)]
    pub features: Vec<String>,

    /// When this validation was performed
    pub validated_at: DateTime<Utc>,
}

/// Cached license data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedLicense {
    data: LicenseData,
    cached_at: DateTime<Utc>,
}

/// Validated Enterprise license
#[derive(Debug, Clone)]
pub struct EnterpriseLicense {
    /// The validated license data
    pub data: LicenseData,

    /// Whether this came from cache
    pub from_cache: bool,
}

impl EnterpriseLicense {
    /// Validate an Enterprise license
    ///
    /// This performs online validation with offline caching:
    /// 1. Check cache first for unexpired validation
    /// 2. If cache miss, validate online
    /// 3. Cache successful validation
    ///
    /// # Returns
    /// * `Ok(EnterpriseLicense)` if the license is valid
    /// * `Err(LicenseError)` if validation fails
    pub async fn validate() -> Result<Self, LicenseError> {
        let key = std::env::var("MCP_GUARD_LICENSE_KEY")
            .map_err(|_| LicenseError::NotFound)?;

        Self::validate_key(&key).await
    }

    /// Validate a specific license key
    pub async fn validate_key(key: &str) -> Result<Self, LicenseError> {
        // Check prefix
        if !key.starts_with("ent_") {
            return Err(LicenseError::InvalidFormat);
        }

        // Try cache first
        if let Some(cached) = Self::load_from_cache(key) {
            let age = Utc::now() - cached.cached_at;
            if age < Duration::days(OFFLINE_GRACE_DAYS) {
                info!(
                    "Using cached license validation (age: {} days)",
                    age.num_days()
                );
                return Ok(Self {
                    data: cached.data,
                    from_cache: true,
                });
            } else {
                warn!("Cached license validation expired, attempting online validation");
            }
        }

        // Online validation
        match Self::validate_online(key).await {
            Ok(data) => {
                // Cache successful validation
                Self::save_to_cache(key, &data);
                Ok(Self {
                    data,
                    from_cache: false,
                })
            }
            Err(LicenseError::NetworkError(e)) => {
                // Network error - check if we have a valid cache within grace period
                if let Some(cached) = Self::load_from_cache(key) {
                    let age = Utc::now() - cached.cached_at;
                    if age < Duration::days(OFFLINE_GRACE_DAYS) {
                        warn!(
                            "Network error during validation ({}), using cached license (age: {} days)",
                            e,
                            age.num_days()
                        );
                        return Ok(Self {
                            data: cached.data,
                            from_cache: true,
                        });
                    }
                }
                Err(LicenseError::OfflineGraceExpired)
            }
            Err(e) => Err(e),
        }
    }

    /// Validate license online via keygen.sh API
    async fn validate_online(key: &str) -> Result<LicenseData, LicenseError> {
        let client = reqwest::Client::new();

        let url = format!(
            "{}/{}/licenses/actions/validate-key",
            KEYGEN_API_URL, KEYGEN_ACCOUNT_ID
        );

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&serde_json::json!({
                "meta": {
                    "key": key
                }
            }))
            .send()
            .await
            .map_err(|e| LicenseError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LicenseError::ValidationFailed(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| LicenseError::ValidationFailed(e.to_string()))?;

        // Parse keygen.sh response
        let meta = body.get("meta").ok_or_else(|| {
            LicenseError::ValidationFailed("Missing meta in response".to_string())
        })?;

        let valid = meta
            .get("valid")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !valid {
            let code = meta
                .get("code")
                .and_then(|c| c.as_str())
                .unwrap_or("UNKNOWN");
            return Err(LicenseError::ValidationFailed(format!(
                "License invalid: {}",
                code
            )));
        }

        let license = body.get("data").ok_or_else(|| {
            LicenseError::ValidationFailed("Missing data in response".to_string())
        })?;

        let attributes = license.get("attributes").ok_or_else(|| {
            LicenseError::ValidationFailed("Missing attributes in response".to_string())
        })?;

        // Extract license data
        let data = LicenseData {
            id: license
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            key: key.to_string(),
            tier: attributes
                .get("metadata")
                .and_then(|m| m.get("tier"))
                .and_then(|t| t.as_str())
                .unwrap_or("enterprise")
                .to_string(),
            licensee: attributes
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string(),
            max_seats: attributes
                .get("maxMachines")
                .and_then(|m| m.as_u64())
                .map(|m| m as u32),
            expires_at: attributes
                .get("expiry")
                .and_then(|e| e.as_str())
                .and_then(|e| DateTime::parse_from_rfc3339(e).ok())
                .map(|e| e.with_timezone(&Utc)),
            features: attributes
                .get("metadata")
                .and_then(|m| m.get("features"))
                .and_then(|f| f.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            validated_at: Utc::now(),
        };

        // Verify tier
        if data.tier != "enterprise" {
            return Err(LicenseError::TierMismatch(data.tier.clone()));
        }

        // Check expiry
        if let Some(expires_at) = data.expires_at {
            if expires_at < Utc::now() {
                return Err(LicenseError::Expired(expires_at));
            }
        }

        Ok(data)
    }

    /// Get cache file path for a license key
    fn cache_path(key: &str) -> Option<PathBuf> {
        let dirs = directories::ProjectDirs::from("io", "mcp-guard", "mcp-guard")?;
        let cache_dir = dirs.cache_dir();
        std::fs::create_dir_all(cache_dir).ok()?;

        // Hash the key for filename
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        Some(cache_dir.join(format!("license_{:x}.json", hash)))
    }

    /// Load license from cache
    fn load_from_cache(key: &str) -> Option<CachedLicense> {
        let path = Self::cache_path(key)?;
        let content = std::fs::read_to_string(&path).ok()?;
        let cached: CachedLicense = serde_json::from_str(&content).ok()?;

        // Verify key matches
        if cached.data.key != key {
            return None;
        }

        debug!("Loaded license from cache: {:?}", path);
        Some(cached)
    }

    /// Save license to cache
    fn save_to_cache(key: &str, data: &LicenseData) {
        if let Some(path) = Self::cache_path(key) {
            let cached = CachedLicense {
                data: data.clone(),
                cached_at: Utc::now(),
            };
            if let Ok(content) = serde_json::to_string_pretty(&cached) {
                if std::fs::write(&path, content).is_ok() {
                    debug!("Saved license to cache: {:?}", path);
                }
            }
        }
    }

    /// Check if a specific feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.data.features.iter().any(|f| f == feature)
    }

    /// Get days until expiry (None if no expiry)
    pub fn days_until_expiry(&self) -> Option<i64> {
        self.data.expires_at.map(|e| (e - Utc::now()).num_days())
    }

    /// Check if the license is about to expire (within 30 days)
    pub fn is_expiring_soon(&self) -> bool {
        self.days_until_expiry()
            .map(|d| d <= 30)
            .unwrap_or(false)
    }
}

/// Enterprise features that can be enabled by license
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterpriseFeature {
    /// mTLS client certificate authentication
    Mtls,
    /// Multi-server path-based routing
    MultiServerRouting,
    /// SIEM audit log shipping
    SiemAudit,
    /// Per-tool rate limiting
    PerToolRateLimit,
    /// OpenTelemetry tracing
    OpenTelemetry,
    /// Admin guard tools (keys, audit, config)
    AdminGuardTools,
}

impl EnterpriseFeature {
    /// Get the feature identifier string
    pub fn as_str(&self) -> &'static str {
        match self {
            EnterpriseFeature::Mtls => "mtls",
            EnterpriseFeature::MultiServerRouting => "multi_server_routing",
            EnterpriseFeature::SiemAudit => "siem_audit",
            EnterpriseFeature::PerToolRateLimit => "per_tool_rate_limit",
            EnterpriseFeature::OpenTelemetry => "opentelemetry",
            EnterpriseFeature::AdminGuardTools => "admin_guard_tools",
        }
    }

    /// All Enterprise features
    pub fn all() -> &'static [EnterpriseFeature] {
        &[
            EnterpriseFeature::Mtls,
            EnterpriseFeature::MultiServerRouting,
            EnterpriseFeature::SiemAudit,
            EnterpriseFeature::PerToolRateLimit,
            EnterpriseFeature::OpenTelemetry,
            EnterpriseFeature::AdminGuardTools,
        ]
    }
}

/// Check if Enterprise features are available (license validated)
pub async fn is_enterprise_licensed() -> bool {
    EnterpriseLicense::validate().await.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_format() {
        // Sync test using block_on
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            std::env::set_var("MCP_GUARD_LICENSE_KEY", "invalid");
            let result = EnterpriseLicense::validate().await;
            assert!(matches!(result, Err(LicenseError::InvalidFormat)));
            std::env::remove_var("MCP_GUARD_LICENSE_KEY");
        });
    }

    #[test]
    fn test_missing_license() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            std::env::remove_var("MCP_GUARD_LICENSE_KEY");
            let result = EnterpriseLicense::validate().await;
            assert!(matches!(result, Err(LicenseError::NotFound)));
        });
    }

    #[test]
    fn test_enterprise_feature_as_str() {
        assert_eq!(EnterpriseFeature::Mtls.as_str(), "mtls");
        assert_eq!(
            EnterpriseFeature::MultiServerRouting.as_str(),
            "multi_server_routing"
        );
    }
}
