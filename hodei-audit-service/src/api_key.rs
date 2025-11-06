//! API Key Management System
//!
//! This module provides secure API key generation, validation,
//! and management with granular scopes and rate limiting.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{error, info};

/// API Key scopes for granular permissions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiScope {
    /// Read audit events
    AuditRead,
    /// Write audit events
    AuditWrite,
    /// Verify cryptographic signatures
    CryptoVerify,
    /// Query audit data
    AuditQuery,
    /// Admin operations
    Admin,
    /// System monitoring
    Monitoring,
}

impl std::fmt::Display for ApiScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiScope::AuditRead => write!(f, "audit:read"),
            ApiScope::AuditWrite => write!(f, "audit:write"),
            ApiScope::CryptoVerify => write!(f, "crypto:verify"),
            ApiScope::AuditQuery => write!(f, "audit:query"),
            ApiScope::Admin => write!(f, "admin"),
            ApiScope::Monitoring => write!(f, "monitoring"),
        }
    }
}

impl ApiScope {
    /// Parse scope from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "audit:read" => Some(ApiScope::AuditRead),
            "audit:write" => Some(ApiScope::AuditWrite),
            "crypto:verify" => Some(ApiScope::CryptoVerify),
            "audit:query" => Some(ApiScope::AuditQuery),
            "admin" => Some(ApiScope::Admin),
            "monitoring" => Some(ApiScope::Monitoring),
            _ => None,
        }
    }

    /// Get all available scopes
    pub fn all() -> Vec<Self> {
        vec![
            ApiScope::AuditRead,
            ApiScope::AuditWrite,
            ApiScope::CryptoVerify,
            ApiScope::AuditQuery,
            ApiScope::Admin,
            ApiScope::Monitoring,
        ]
    }
}

/// API Key metadata (not the secret)
#[derive(Debug, Clone)]
pub struct ApiKeyMetadata {
    /// Unique key ID
    pub key_id: String,
    /// Tenant ID
    pub tenant_id: String,
    /// Human-readable name
    pub name: String,
    /// Granted scopes
    pub scopes: Vec<ApiScope>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp (if any)
    pub expires_at: Option<DateTime<Utc>>,
    /// Last used timestamp
    pub last_used_at: Option<DateTime<Utc>>,
    /// Is key enabled
    pub enabled: bool,
    /// Rate limit per second
    pub rate_limit_per_sec: u64,
    /// Key usage statistics
    pub usage_stats: ApiKeyUsageStats,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ApiKeyMetadata {
    /// Create a new API key metadata
    pub fn new(key_id: String, tenant_id: String, name: String, scopes: Vec<ApiScope>) -> Self {
        Self {
            key_id,
            tenant_id,
            name,
            scopes,
            created_at: Utc::now(),
            expires_at: None,
            last_used_at: None,
            enabled: true,
            rate_limit_per_sec: 100,
            usage_stats: ApiKeyUsageStats::default(),
            metadata: HashMap::new(),
        }
    }

    /// Create with expiration
    pub fn with_expiration(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Create with custom rate limit
    pub fn with_rate_limit(mut self, rate_limit: u64) -> Self {
        self.rate_limit_per_sec = rate_limit;
        self
    }

    /// Check if key has a specific scope
    pub fn has_scope(&self, scope: &ApiScope) -> bool {
        self.scopes.contains(scope)
    }

    /// Check if key has all required scopes
    pub fn has_scopes(&self, required_scopes: &[ApiScope]) -> bool {
        required_scopes.iter().all(|s| self.has_scope(s))
    }

    /// Check if key is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            return Utc::now() > expires_at;
        }
        false
    }

    /// Check if key is valid
    pub fn is_valid(&self) -> bool {
        self.enabled && !self.is_expired()
    }

    /// Mark key as used
    pub fn mark_used(&mut self) {
        self.last_used_at = Some(Utc::now());
        self.usage_stats.increment();
    }
}

/// API Key usage statistics
#[derive(Debug, Clone, Default)]
pub struct ApiKeyUsageStats {
    /// Total requests made
    pub total_requests: u64,
    /// Requests in the last hour
    pub requests_last_hour: u64,
    /// Requests in the last day
    pub requests_last_day: u64,
    /// First request timestamp
    pub first_used_at: Option<DateTime<Utc>>,
}

impl ApiKeyUsageStats {
    /// Increment usage counters
    pub fn increment(&mut self) {
        self.total_requests += 1;
        self.requests_last_hour += 1;
        self.requests_last_day += 1;

        if self.first_used_at.is_none() {
            self.first_used_at = Some(Utc::now());
        }
    }

    /// Reset hourly counters (should be called hourly)
    pub fn reset_hourly(&mut self) {
        self.requests_last_hour = 0;
    }

    /// Reset daily counters (should be called daily)
    pub fn reset_daily(&mut self) {
        self.requests_last_hour = 0;
        self.requests_last_day = 0;
    }
}

/// API Key with secret (full key)
#[derive(Debug, Clone)]
pub struct ApiKey {
    /// Metadata
    pub metadata: ApiKeyMetadata,
    /// Secret key (hashed)
    pub hashed_key: String,
    /// Secret key (plaintext, only shown once)
    pub plaintext_key: String,
}

impl ApiKey {
    /// Create a new API key
    pub fn new(metadata: ApiKeyMetadata, hashed_key: String, plaintext_key: String) -> Self {
        Self {
            metadata,
            hashed_key,
            plaintext_key,
        }
    }

    /// Get the full key (for display)
    pub fn get_full_key(&self) -> &str {
        &self.plaintext_key
    }
}

/// API Key store
pub struct ApiKeyStore {
    /// In-memory storage of API key metadata
    keys: HashMap<String, ApiKeyMetadata>,
    /// Hash to metadata lookup
    hash_lookup: HashMap<String, String>, // hashed_key -> key_id
    /// Rate limit tracking
    rate_limiters: HashMap<String, RateLimiter>,
}

impl ApiKeyStore {
    /// Create a new API key store
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            hash_lookup: HashMap::new(),
            rate_limiters: HashMap::new(),
        }
    }

    /// Generate a new API key
    pub fn generate_key(
        &mut self,
        tenant_id: String,
        name: String,
        scopes: Vec<ApiScope>,
    ) -> Result<ApiKey, ApiKeyError> {
        info!("[API Key] Generating new key for tenant {}", tenant_id);

        // Generate unique key ID
        let key_id = format!("key_{}", uuid::Uuid::new_v4());

        // Generate secret key
        let plaintext_key = self.generate_secret_key();
        let hashed_key = self.hash_key(&plaintext_key)?;

        // Create metadata
        let metadata = ApiKeyMetadata::new(key_id.clone(), tenant_id, name, scopes);

        // Store metadata
        self.keys.insert(key_id.clone(), metadata.clone());
        self.hash_lookup.insert(hashed_key.clone(), key_id.clone());

        // Create rate limiter for this key
        self.rate_limiters.insert(
            key_id.clone(),
            RateLimiter::new(metadata.rate_limit_per_sec),
        );

        info!("[API Key] Generated key with ID: {}", key_id);

        Ok(ApiKey {
            metadata,
            hashed_key,
            plaintext_key,
        })
    }

    /// Validate an API key
    pub fn validate_key(
        &mut self,
        plaintext_key: &str,
        required_scopes: &[ApiScope],
    ) -> Result<ApiKeyMetadata, ApiKeyError> {
        // Hash the provided key
        let hashed_key = self.hash_key(plaintext_key)?;

        // Look up the key
        let key_id = self
            .hash_lookup
            .get(&hashed_key)
            .ok_or_else(|| ApiKeyError::InvalidKey("Key not found".to_string()))?;

        // Get metadata
        let metadata = self
            .keys
            .get(key_id)
            .ok_or_else(|| ApiKeyError::InvalidKey("Key metadata not found".to_string()))?;

        // Check if key is valid
        if !metadata.is_valid() {
            return Err(ApiKeyError::KeyDisabled(
                "Key is disabled or expired".to_string(),
            ));
        }

        // Check scopes
        if !metadata.has_scopes(required_scopes) {
            return Err(ApiKeyError::InsufficientScope(
                "Key does not have required scopes".to_string(),
            ));
        }

        // Check rate limit
        let rate_limiter = self
            .rate_limiters
            .get_mut(key_id)
            .ok_or_else(|| ApiKeyError::RateLimited("Rate limiter not found".to_string()))?;

        if !rate_limiter.check_rate_limit() {
            return Err(ApiKeyError::RateLimited("Rate limit exceeded".to_string()));
        }

        info!("[API Key] Validated key: {}", key_id);

        Ok(metadata.clone())
    }

    /// Get API key metadata
    pub fn get_key(&self, key_id: &str) -> Option<&ApiKeyMetadata> {
        self.keys.get(key_id)
    }

    /// List all API keys for a tenant
    pub fn list_keys(&self, tenant_id: &str) -> Vec<&ApiKeyMetadata> {
        self.keys
            .values()
            .filter(|k| k.tenant_id == tenant_id)
            .collect()
    }

    /// Revoke an API key
    pub fn revoke_key(&mut self, key_id: &str) -> Result<(), ApiKeyError> {
        info!("[API Key] Revoking key: {}", key_id);

        // Get metadata
        let metadata = self
            .keys
            .get(key_id)
            .ok_or_else(|| ApiKeyError::KeyNotFound("Key not found"))?;

        // Remove from lookups
        // Note: We can't remove the hash because we don't have the hashed key here
        // In a real implementation, we'd store the hash reference

        // Remove from storage
        self.keys.remove(key_id);
        self.rate_limiters.remove(key_id);

        info!("[API Key] Revoked key: {}", key_id);

        Ok(())
    }

    /// Update key usage
    pub fn update_usage(&mut self, key_id: &str) -> Result<(), ApiKeyError> {
        // Get metadata
        let metadata = self
            .keys
            .get_mut(key_id)
            .ok_or_else(|| ApiKeyError::KeyNotFound("Key not found"))?;

        // Mark as used
        metadata.mark_used();

        // Update rate limiter
        if let Some(mut rate_limiter) = self.rate_limiters.get_mut(key_id) {
            rate_limiter.record_request();
        }

        Ok(())
    }

    /// Generate secret key
    fn generate_secret_key(&self) -> String {
        // Generate a secure random key
        // Format: hk_live_{random_32_chars}
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random: String = std::iter::repeat(())
            .map(|_| {
                const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .take(32)
            .collect();

        format!("hk_live_{}", random)
    }

    /// Hash a key using SHA-256
    fn hash_key(&self, key: &str) -> Result<String, ApiKeyError> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiter for API keys
#[derive(Clone)]
struct RateLimiter {
    /// Requests per second limit
    limit_per_sec: u64,
    /// Token bucket
    tokens: u64,
    /// Last refill timestamp
    last_refill: SystemTime,
    /// Refill duration
    refill_duration: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter
    fn new(limit_per_sec: u64) -> Self {
        Self {
            limit_per_sec,
            tokens: limit_per_sec,
            last_refill: SystemTime::now(),
            refill_duration: Duration::from_secs(1),
        }
    }

    /// Check if a request is allowed
    fn check_rate_limit(&mut self) -> bool {
        self.refill();

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Record a request
    fn record_request(&mut self) {
        let _ = self.check_rate_limit();
    }

    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = SystemTime::now();
        if let Ok(elapsed) = now.duration_since(self.last_refill) {
            let tokens_to_add = (elapsed.as_secs() * self.limit_per_sec) as u64;
            if tokens_to_add > 0 {
                self.tokens = (self.tokens + tokens_to_add).min(self.limit_per_sec);
                self.last_refill = now;
            }
        }
    }
}

/// API Key error types
#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error("Invalid API key: {0}")]
    InvalidKey(String),

    #[error("Key not found")]
    KeyNotFound(&'static str),

    #[error("Key disabled: {0}")]
    KeyDisabled(String),

    #[error("Insufficient scope: {0}")]
    InsufficientScope(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Hashing error: {0}")]
    HashingError(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for ApiKeyError {
    fn from(e: anyhow::Error) -> Self {
        ApiKeyError::Other(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_scope_all() {
        let scopes = ApiScope::all();
        assert_eq!(scopes.len(), 6);
        assert!(scopes.contains(&ApiScope::AuditRead));
        assert!(scopes.contains(&ApiScope::AuditWrite));
        assert!(scopes.contains(&ApiScope::CryptoVerify));
    }

    #[test]
    fn test_api_scope_from_str() {
        assert_eq!(ApiScope::from_str("audit:read"), Some(ApiScope::AuditRead));
        assert_eq!(
            ApiScope::from_str("audit:write"),
            Some(ApiScope::AuditWrite)
        );
        assert_eq!(ApiScope::from_str("invalid:scope"), None);
    }

    #[test]
    fn test_api_key_metadata_creation() {
        let metadata = ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-456".to_string(),
            "Test Key".to_string(),
            vec![ApiScope::AuditRead, ApiScope::AuditWrite],
        );

        assert_eq!(metadata.key_id, "key-123");
        assert_eq!(metadata.tenant_id, "tenant-456");
        assert_eq!(metadata.name, "Test Key");
        assert_eq!(metadata.scopes.len(), 2);
        assert!(metadata.enabled);
    }

    #[test]
    fn test_api_key_has_scope() {
        let metadata = ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-456".to_string(),
            "Test Key".to_string(),
            vec![ApiScope::AuditRead, ApiScope::AuditWrite],
        );

        assert!(metadata.has_scope(&ApiScope::AuditRead));
        assert!(metadata.has_scope(&ApiScope::AuditWrite));
        assert!(!metadata.has_scope(&ApiScope::CryptoVerify));
    }

    #[test]
    fn test_api_key_has_all_scopes() {
        let metadata = ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-456".to_string(),
            "Test Key".to_string(),
            vec![
                ApiScope::AuditRead,
                ApiScope::AuditWrite,
                ApiScope::CryptoVerify,
            ],
        );

        assert!(metadata.has_scopes(&[ApiScope::AuditRead, ApiScope::AuditWrite]));
        assert!(!metadata.has_scopes(&[ApiScope::AuditRead, ApiScope::Admin]));
    }

    #[test]
    fn test_api_key_expiration() {
        let mut metadata = ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-456".to_string(),
            "Test Key".to_string(),
            vec![],
        );

        // Not expired initially
        assert!(!metadata.is_expired());

        // With expiration
        let future = Utc::now() + chrono::Duration::days(1);
        metadata = metadata.with_expiration(future);
        assert!(!metadata.is_expired());

        let past = Utc::now() - chrono::Duration::days(1);
        metadata = metadata.with_expiration(past);
        assert!(metadata.is_expired());
    }

    #[test]
    fn test_api_key_valid() {
        let mut metadata = ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-456".to_string(),
            "Test Key".to_string(),
            vec![],
        );

        // Valid when enabled and not expired
        assert!(metadata.is_valid());

        // Invalid when disabled
        metadata.enabled = false;
        assert!(!metadata.is_valid());

        // Invalid when expired
        metadata.enabled = true;
        let past = Utc::now() - chrono::Duration::days(1);
        metadata = metadata.with_expiration(past);
        assert!(!metadata.is_valid());
    }

    #[test]
    fn test_api_key_store_generate() {
        let mut store = ApiKeyStore::new();

        let result = store.generate_key(
            "tenant-123".to_string(),
            "Test Key".to_string(),
            vec![ApiScope::AuditRead, ApiScope::AuditWrite],
        );

        assert!(result.is_ok());
        let api_key = result.unwrap();

        assert_eq!(api_key.metadata.tenant_id, "tenant-123");
        assert_eq!(api_key.metadata.name, "Test Key");
        assert_eq!(api_key.metadata.scopes.len(), 2);
        assert!(api_key.plaintext_key.starts_with("hk_live_"));
    }

    #[test]
    fn test_api_key_store_validate() {
        let mut store = ApiKeyStore::new();

        // Generate a key
        let api_key = store
            .generate_key(
                "tenant-123".to_string(),
                "Test Key".to_string(),
                vec![ApiScope::AuditRead],
            )
            .unwrap();

        // Validate the key
        let result = store.validate_key(&api_key.plaintext_key, &[ApiScope::AuditRead]);

        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.key_id, api_key.metadata.key_id);
    }

    #[test]
    fn test_api_key_store_list_keys() {
        let mut store = ApiKeyStore::new();

        // Generate multiple keys
        store
            .generate_key(
                "tenant-123".to_string(),
                "Key 1".to_string(),
                vec![ApiScope::AuditRead],
            )
            .unwrap();

        store
            .generate_key(
                "tenant-123".to_string(),
                "Key 2".to_string(),
                vec![ApiScope::AuditWrite],
            )
            .unwrap();

        store
            .generate_key(
                "tenant-456".to_string(),
                "Key 3".to_string(),
                vec![ApiScope::AuditRead],
            )
            .unwrap();

        // List keys for tenant-123
        let keys = store.list_keys("tenant-123");
        assert_eq!(keys.len(), 2);

        // List keys for tenant-456
        let keys = store.list_keys("tenant-456");
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(10);

        // Should allow requests up to the limit
        for _ in 0..10 {
            assert!(limiter.check_rate_limit());
        }

        // Should reject requests beyond the limit
        assert!(!limiter.check_rate_limit());
    }
}
