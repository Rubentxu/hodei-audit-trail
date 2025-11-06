//! gRPC Interceptor for Tenant Validation
//!
//! This module implements gRPC interceptors that validate tenant context
//! and ensure proper isolation between tenants.

use http::HeaderMap;
use std::future::Future;
use std::pin::Pin;
use tonic::service::Interceptor;
use tonic::{Request, Status};
use tracing::{error, info, warn};

use crate::tenant::{TenantContext, TenantContextManager, TenantExtractor};

/// Interceptor for tenant context validation
#[derive(Debug, Clone)]
pub struct TenantValidationInterceptor {
    /// Context extractor
    extractor: TenantExtractor,
    /// Enable strict validation
    strict_mode: bool,
}

impl TenantValidationInterceptor {
    /// Create a new interceptor
    pub fn new() -> Self {
        Self {
            extractor: TenantExtractor::new(),
            strict_mode: true,
        }
    }

    /// Create with custom extractor
    pub fn with_extractor(extractor: TenantExtractor) -> Self {
        Self {
            extractor,
            strict_mode: true,
        }
    }

    /// Enable/disable strict mode
    pub fn strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }

    /// Validate and extract tenant context from request
    pub fn validate_request(&self, request: &Request<()>) -> Result<TenantContext, Status> {
        let context = self.extractor.extract_from_metadata(request)?;

        if self.strict_mode {
            // In strict mode, API key is required
            if context.api_key_id.is_none() {
                return Err(Status::unauthenticated(
                    "API key is required in strict mode",
                ));
            }
        }

        // Log validation
        info!(
            "Validated tenant context: tenant_id={}, api_key_id={:?}, user_id={:?}, trace_id={}",
            context.tenant_id, context.api_key_id, context.user_id, context.trace_id
        );

        Ok(context)
    }
}

impl Default for TenantValidationInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Interceptor for TenantValidationInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Validate request
        let context = self.validate_request(&request)?;

        // Set context in manager for the duration of the request
        self.extractor.set_context(context);

        Ok(request)
    }
}

/// Async interceptor for more complex validation logic
#[derive(Debug, Clone)]
pub struct AsyncTenantValidationInterceptor {
    /// Inner interceptor
    inner: TenantValidationInterceptor,
    /// Enable API key validation
    validate_api_key: bool,
    /// Enable quota checks
    enable_quota_checks: bool,
}

impl AsyncTenantValidationInterceptor {
    /// Create a new async interceptor
    pub fn new() -> Self {
        Self {
            inner: TenantValidationInterceptor::new(),
            validate_api_key: true,
            enable_quota_checks: true,
        }
    }

    /// Create with custom settings
    pub fn with_settings(validate_api_key: bool, enable_quota_checks: bool) -> Self {
        Self {
            inner: TenantValidationInterceptor::new(),
            validate_api_key,
            enable_quota_checks,
        }
    }

    /// Validate API key (placeholder for real validation)
    async fn validate_api_key(&self, _api_key: &str) -> Result<(), Status> {
        // TODO: Implement real API key validation
        // This would check against a database or cache
        Ok(())
    }

    /// Check quota limits (placeholder for real quota check)
    async fn check_quota(&self, _tenant_id: &str) -> Result<(), Status> {
        // TODO: Implement real quota checking
        // This would check current usage against limits
        Ok(())
    }

    /// Full validation of tenant context
    pub async fn validate_request_full(
        &self,
        request: &Request<()>,
    ) -> Result<TenantContext, Status> {
        // First, extract and validate basic context
        let context = self.inner.validate_request(request)?;

        // Validate API key if required
        if self.validate_api_key {
            if let Some(api_key) = &context.api_key_id {
                self.validate_api_key(api_key).await?;
            }
        }

        // Check quota if enabled
        if self.enable_quota_checks {
            self.check_quota(&context.tenant_id).await?;
        }

        Ok(context)
    }
}

impl Default for AsyncTenantValidationInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to get tenant context from anywhere in the service
pub fn get_tenant_context() -> Option<TenantContext> {
    // This would use the global context manager
    // For now, it's a placeholder
    None
}

/// Helper function to require tenant context
pub fn require_tenant_context() -> Result<TenantContext, Status> {
    get_tenant_context().ok_or_else(|| Status::unauthenticated("No tenant context available"))
}

/// Middleware function for HTTP requests
pub async fn extract_tenant_from_headers(
    headers: &http::HeaderMap,
) -> Result<TenantContext, Status> {
    // Extract tenant ID
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| Status::invalid_argument("Missing x-tenant-id header"))?;

    let mut context = TenantContext::new(tenant_id);

    // Extract API key
    if let Some(api_key) = headers.get("x-api-key").and_then(|v| v.to_str().ok()) {
        context = context.with_api_key(api_key.to_string());
    }

    // Extract user ID
    if let Some(user_id) = headers.get("x-user-id").and_then(|v| v.to_str().ok()) {
        context = context.with_user(user_id.to_string());
    }

    // Extract trace ID
    if let Some(trace_id) = headers.get("x-trace-id").and_then(|v| v.to_str().ok()) {
        context.trace_id = trace_id.to_string();
    }

    // Validate context
    Ok(context)
}

/// Error types for tenant validation
#[derive(Debug, thiserror::Error)]
pub enum TenantValidationError {
    #[error("Missing tenant ID")]
    MissingTenantId,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Tenant not found")]
    TenantNotFound,
    #[error("Tenant disabled")]
    TenantDisabled,
    #[error("Quota exceeded")]
    QuotaExceeded,
    #[error("Invalid context: {0}")]
    InvalidContext(String),
}

impl From<TenantValidationError> for Status {
    fn from(error: TenantValidationError) -> Self {
        match error {
            TenantValidationError::MissingTenantId => Status::invalid_argument("Missing tenant ID"),
            TenantValidationError::InvalidApiKey => Status::unauthenticated("Invalid API key"),
            TenantValidationError::TenantNotFound => Status::not_found("Tenant not found"),
            TenantValidationError::TenantDisabled => Status::permission_denied("Tenant disabled"),
            TenantValidationError::QuotaExceeded => Status::resource_exhausted("Quota exceeded"),
            TenantValidationError::InvalidContext(msg) => {
                Status::invalid_argument(format!("Invalid context: {}", msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::metadata::MetadataValue;

    #[test]
    fn test_interceptor_creation() {
        let interceptor = TenantValidationInterceptor::new();
        assert!(interceptor.strict_mode);
    }

    #[test]
    fn test_interceptor_with_valid_request() {
        let interceptor = TenantValidationInterceptor::new();

        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("x-tenant-id", "test-tenant".parse().unwrap());
        request
            .metadata_mut()
            .insert("x-api-key", "test-api-key".parse().unwrap());

        let result = interceptor.validate_request(&request);

        assert!(result.is_ok());
        let context = result.unwrap();
        assert_eq!(context.tenant_id, "test-tenant");
        assert_eq!(context.api_key_id, Some("test-api-key".to_string()));
    }

    #[test]
    fn test_interceptor_with_missing_tenant() {
        let interceptor = TenantValidationInterceptor::new();

        let request = Request::new(());
        let result = interceptor.validate_request(&request);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn test_interceptor_strict_mode() {
        let interceptor = TenantValidationInterceptor::new().strict_mode(true);

        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("x-tenant-id", "test-tenant".parse().unwrap());
        // No API key in strict mode

        let result = interceptor.validate_request(&request);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn test_interceptor_non_strict_mode() {
        let mut interceptor = TenantValidationInterceptor::new().strict_mode(false);

        let mut request = Request::new(());
        request
            .metadata_mut()
            .insert("x-tenant-id", "test-tenant".parse().unwrap());
        // No API key in non-strict mode

        let result = interceptor.validate_request(&request);

        assert!(result.is_ok());
        let context = result.unwrap();
        assert_eq!(context.tenant_id, "test-tenant");
        assert!(context.api_key_id.is_none());
    }

    #[test]
    fn test_async_interceptor() {
        let interceptor = AsyncTenantValidationInterceptor::new();
        assert!(interceptor.validate_api_key);
        assert!(interceptor.enable_quota_checks);
    }

    #[test]
    fn test_tenant_extraction_from_headers() {
        use http::HeaderValue;

        let mut headers = http::HeaderMap::new();
        headers.insert("x-tenant-id", HeaderValue::from_static("test-tenant"));
        headers.insert("x-api-key", HeaderValue::from_static("test-key"));
        headers.insert("x-user-id", HeaderValue::from_static("test-user"));

        let result = tokio_test::block_on(extract_tenant_from_headers(&headers));

        assert!(result.is_ok());
        let context = result.unwrap();
        assert_eq!(context.tenant_id, "test-tenant");
        assert_eq!(context.api_key_id, Some("test-key".to_string()));
        assert_eq!(context.user_id, Some("test-user".to_string()));
    }

    #[test]
    fn test_error_conversion() {
        let error = TenantValidationError::MissingTenantId;
        let status: Status = error.into();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);

        let error = TenantValidationError::InvalidApiKey;
        let status: Status = error.into();
        assert_eq!(status.code(), tonic::Code::Unauthenticated);

        let error = TenantValidationError::QuotaExceeded;
        let status: Status = error.into();
        assert_eq!(status.code(), tonic::Code::ResourceExhausted);
    }
}
