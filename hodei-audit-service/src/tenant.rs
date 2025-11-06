//! Tenant Context Management
//!
//! This module provides tenant isolation and context propagation
//! across the entire audit service system.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::{Request, Status};
use tracing::{error, info, warn};

/// Tenant identifier type
pub type TenantId = String;

/// Tenant context for request isolation
#[derive(Debug, Clone, Default)]
pub struct TenantContext {
    /// Tenant identifier
    pub tenant_id: TenantId,
    /// API Key ID (if authenticated)
    pub api_key_id: Option<String>,
    /// User ID (if provided)
    pub user_id: Option<String>,
    /// Tenant metadata
    pub metadata: HashMap<String, String>,
    /// Request trace ID
    pub trace_id: String,
    /// Request span ID
    pub span_id: String,
    /// Tenant tier (Enterprise, SME, etc.)
    pub tier: TenantTier,
    /// Quota configuration for this tenant
    pub quota_config: QuotaConfig,
}

impl TenantContext {
    /// Create a new tenant context
    pub fn new(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            api_key_id: None,
            user_id: None,
            metadata: HashMap::new(),
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            tier: TenantTier::SME,
            quota_config: QuotaConfig::default(),
        }
    }

    /// Create with API key
    pub fn with_api_key(mut self, api_key_id: String) -> Self {
        self.api_key_id = Some(api_key_id);
        self
    }

    /// Create with user ID
    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Create with metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Create with tier
    pub fn with_tier(mut self, tier: TenantTier) -> Self {
        self.tier = tier;
        self
    }

    /// Create with quota config
    pub fn with_quota_config(mut self, config: QuotaConfig) -> Self {
        self.quota_config = config;
        self
    }

    /// Validate that context is complete
    pub fn validate(&self) -> Result<(), Status> {
        if self.tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        Ok(())
    }

    /// Check if tenant has required scope
    pub fn has_scope(&self, scope: &str) -> bool {
        // TODO: Implement scope validation based on API key
        // This is a placeholder - real implementation would check scopes
        self.api_key_id.is_some()
    }
}

/// Tenant tier types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantTier {
    Enterprise,
    SME,
    Startup,
}

impl Default for TenantTier {
    fn default() -> Self {
        TenantTier::SME
    }
}

impl std::fmt::Display for TenantTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantTier::Enterprise => write!(f, "Enterprise"),
            TenantTier::SME => write!(f, "SME"),
            TenantTier::Startup => write!(f, "Startup"),
        }
    }
}

/// Quota configuration for a tenant
#[derive(Debug, Clone)]
pub struct QuotaConfig {
    /// Events per second limit
    pub events_per_second: u64,
    /// Storage quota in bytes
    pub storage_quota_bytes: u64,
    /// Requests per minute limit
    pub requests_per_minute: u64,
    /// API key rate limit
    pub api_key_rate_limit: u64,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            events_per_second: 1000,         // Default 1000 events/sec
            storage_quota_bytes: 10_000_000, // Default 10MB
            requests_per_minute: 6000,       // Default 100 req/sec
            api_key_rate_limit: 100,         // Default 100 requests/sec
        }
    }
}

impl QuotaConfig {
    /// Create quota config for Enterprise tier
    pub fn enterprise() -> Self {
        Self {
            events_per_second: 10000,
            storage_quota_bytes: 1_000_000_000, // 1GB
            requests_per_minute: 60000,
            api_key_rate_limit: 1000,
        }
    }

    /// Create quota config for SME tier
    pub fn sme() -> Self {
        Self {
            events_per_second: 1000,
            storage_quota_bytes: 10_000_000, // 10MB
            requests_per_minute: 6000,
            api_key_rate_limit: 100,
        }
    }

    /// Create quota config for Startup tier
    pub fn startup() -> Self {
        Self {
            events_per_second: 100,
            storage_quota_bytes: 1_000_000, // 1MB
            requests_per_minute: 600,
            api_key_rate_limit: 10,
        }
    }
}

/// Thread-local storage for tenant context
thread_local! {
    static TENANT_CONTEXT: std::cell::RefCell<Option<TenantContext>> = std::cell::RefCell::new(None);
}

/// Tenant context manager
#[derive(Debug, Clone)]
pub struct TenantContextManager {
    /// Context storage
    context: Arc<std::sync::Mutex<Option<TenantContext>>>,
}

impl TenantContextManager {
    /// Create a new context manager
    pub fn new() -> Self {
        Self {
            context: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    /// Set tenant context
    pub fn set_context(&self, context: TenantContext) {
        let mut ctx = self.context.lock().unwrap();
        *ctx = Some(context);
    }

    /// Get current tenant context
    pub fn get_context(&self) -> Option<TenantContext> {
        let ctx = self.context.lock().unwrap();
        ctx.clone()
    }

    /// Clear tenant context
    pub fn clear(&self) {
        let mut ctx = self.context.lock().unwrap();
        *ctx = None;
    }

    /// Get tenant ID from context
    pub fn get_tenant_id(&self) -> Option<TenantId> {
        let ctx = self.context.lock().unwrap();
        ctx.as_ref().map(|c| c.tenant_id.clone())
    }

    /// Validate current context
    pub fn validate_current(&self) -> Result<(), Status> {
        let ctx = self.context.lock().unwrap();
        match ctx.as_ref() {
            Some(context) => context.validate(),
            None => Err(Status::unauthenticated("No tenant context set")),
        }
    }
}

impl Default for TenantContextManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware for extracting tenant context from requests
#[derive(Debug, Clone)]
pub struct TenantExtractor {
    /// Context manager
    manager: TenantContextManager,
}

impl TenantExtractor {
    /// Create a new extractor
    pub fn new() -> Self {
        Self {
            manager: TenantContextManager::new(),
        }
    }

    /// Extract tenant context from gRPC metadata
    pub fn extract_from_metadata(&self, request: &Request<()>) -> Result<TenantContext, Status> {
        let metadata = request.metadata();

        // Extract tenant ID
        let tenant_id = metadata
            .get("x-tenant-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| Status::invalid_argument("Missing x-tenant-id header"))?;

        let mut context = TenantContext::new(tenant_id);

        // Extract API key
        if let Some(api_key) = metadata.get("x-api-key").and_then(|v| v.to_str().ok()) {
            context = context.with_api_key(api_key.to_string());
        }

        // Extract user ID
        if let Some(user_id) = metadata.get("x-user-id").and_then(|v| v.to_str().ok()) {
            context = context.with_user(user_id.to_string());
        }

        // Extract trace ID
        if let Some(trace_id) = metadata.get("x-trace-id").and_then(|v| v.to_str().ok()) {
            context.trace_id = trace_id.to_string();
        }

        // Extract span ID
        if let Some(span_id) = metadata.get("x-span-id").and_then(|v| v.to_str().ok()) {
            context.span_id = span_id.to_string();
        }

        // Validate context
        context.validate()?;

        Ok(context)
    }

    /// Set context for current request
    pub fn set_context(&self, context: TenantContext) {
        self.manager.set_context(context);
    }

    /// Get current context
    pub fn get_context(&self) -> Option<TenantContext> {
        self.manager.get_context()
    }

    /// Clear context
    pub fn clear(&self) {
        self.manager.clear();
    }
}

impl Default for TenantExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_context_creation() {
        let context = TenantContext::new("test-tenant".to_string());

        assert_eq!(context.tenant_id, "test-tenant");
        assert!(context.api_key_id.is_none());
        assert!(context.user_id.is_none());
        assert!(!context.trace_id.is_empty());
        assert_eq!(context.tier, TenantTier::SME);
    }

    #[test]
    fn test_tenant_context_with_api_key() {
        let context = TenantContext::new("test-tenant".to_string())
            .with_api_key("api-key-123".to_string())
            .with_user("user-456".to_string());

        assert_eq!(context.tenant_id, "test-tenant");
        assert_eq!(context.api_key_id, Some("api-key-123".to_string()));
        assert_eq!(context.user_id, Some("user-456".to_string()));
    }

    #[test]
    fn test_tenant_context_validation() {
        // Valid context
        let valid = TenantContext::new("valid-tenant".to_string());
        assert!(valid.validate().is_ok());

        // Invalid context (empty tenant_id)
        let invalid = TenantContext::new("".to_string());
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_tenant_context_manager() {
        let manager = TenantContextManager::new();

        // Initially no context
        assert!(manager.get_context().is_none());

        // Set context
        let context = TenantContext::new("test-tenant".to_string());
        manager.set_context(context.clone());

        // Get context
        let retrieved = manager.get_context();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().tenant_id, "test-tenant");

        // Clear context
        manager.clear();
        assert!(manager.get_context().is_none());
    }

    #[test]
    fn test_tenant_tier() {
        assert_eq!(TenantTier::Enterprise.to_string(), "Enterprise");
        assert_eq!(TenantTier::SME.to_string(), "SME");
        assert_eq!(TenantTier::Startup.to_string(), "Startup");
    }

    #[test]
    fn test_quota_configs() {
        let enterprise = QuotaConfig::enterprise();
        assert_eq!(enterprise.events_per_second, 10000);

        let sme = QuotaConfig::sme();
        assert_eq!(sme.events_per_second, 1000);

        let startup = QuotaConfig::startup();
        assert_eq!(startup.events_per_second, 100);
    }

    #[test]
    fn test_tenant_extractor() {
        let extractor = TenantExtractor::new();

        // Create a mock request with metadata
        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("x-tenant-id", "test-tenant".parse().unwrap());
        request
            .metadata_mut()
            .insert("x-api-key", "api-key-123".parse().unwrap());
        request
            .metadata_mut()
            .insert("x-user-id", "user-456".parse().unwrap());

        // Extract context
        let context = extractor.extract_from_metadata(&request).unwrap();

        assert_eq!(context.tenant_id, "test-tenant");
        assert_eq!(context.api_key_id, Some("api-key-123".to_string()));
        assert_eq!(context.user_id, Some("user-456".to_string()));
    }

    #[test]
    fn test_tenant_extractor_missing_header() {
        let extractor = TenantExtractor::new();

        // Create a mock request without tenant ID
        let request = Request::new(());

        // Should fail
        let result = extractor.extract_from_metadata(&request);
        assert!(result.is_err());
    }
}
