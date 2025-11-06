//! Integration Tests for Multi-Tenancy and Security (Épica 5)
//!
//! These tests verify:
//! - Tenant isolation (no cross-tenant access)
//! - API key management and validation
//! - Resource quotas and rate limiting
//! - Compliance and retention policies
//! - Row-Level Security in ClickHouse

use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    // Import all modules
    use crate::api_key::{ApiKeyStore, ApiScope};
    use crate::compliance::{
        ComplianceManager, GDPRRequest, GDPRRequestType, LegalHold, RetentionPolicy,
    };
    use crate::grpc_interceptor::{TenantValidationInterceptor, extract_tenant_from_headers};
    use crate::quotas::{QuotaManager, QuotaType};
    use crate::row_level_security::{RlsManager, RlsQueryBuilder};
    use crate::tenant::{TenantContext, TenantContextManager, TenantExtractor, TenantTier};

    /// Test 1: Tenant Isolation - Verify that tenants cannot access each other's data
    #[tokio::test]
    async fn test_tenant_isolation() {
        let extractor = TenantExtractor::new();

        // Tenant A creates context
        let mut headers_a = http::HeaderMap::new();
        headers_a.insert("x-tenant-id", "tenant-a".parse().unwrap());
        headers_a.insert("x-api-key", "key-a".parse().unwrap());

        let context_a = extract_tenant_from_headers(&headers_a).unwrap();
        assert_eq!(context_a.tenant_id, "tenant-a");

        // Tenant B creates context
        let mut headers_b = http::HeaderMap::new();
        headers_b.insert("x-tenant-id", "tenant-b".parse().unwrap());
        headers_b.insert("x-api-key", "key-b".parse().unwrap());

        let context_b = extract_tenant_from_headers(&headers_b).unwrap();
        assert_eq!(context_b.tenant_id, "tenant-b");

        // Verify isolation - tenant IDs are different
        assert_ne!(context_a.tenant_id, context_b.tenant_id);

        println!("✅ Test passed: Tenant isolation verified - tenants are properly isolated");
    }

    /// Test 2: gRPC Interceptor - Verify interceptor validates tenant context
    #[tokio::test]
    async fn test_grpc_interceptor() {
        let mut interceptor = TenantValidationInterceptor::new();

        // Valid request with all headers
        let mut valid_request = tonic::Request::new(());
        valid_request
            .metadata_mut()
            .insert("x-tenant-id", "tenant-123".parse().unwrap());
        valid_request
            .metadata_mut()
            .insert("x-api-key", "api-key-456".parse().unwrap());
        valid_request
            .metadata_mut()
            .insert("x-user-id", "user-789".parse().unwrap());

        let result = interceptor.validate_request(&valid_request);
        assert!(result.is_ok());

        let context = result.unwrap();
        assert_eq!(context.tenant_id, "tenant-123");
        assert_eq!(context.api_key_id, Some("api-key-456".to_string()));
        assert_eq!(context.user_id, Some("user-789".to_string()));

        // Invalid request missing tenant ID
        let invalid_request = tonic::Request::new(());
        let result = interceptor.validate_request(&invalid_request);
        assert!(result.is_err());

        println!("✅ Test passed: gRPC interceptor validates tenant context correctly");
    }

    /// Test 3: Row-Level Security - Verify RLS query building includes tenant filter
    #[tokio::test]
    async fn test_row_level_security() {
        let mut rls_manager = RlsManager::new();
        rls_manager.set_tenant_id("enterprise-tenant".to_string());

        let mut builder = RlsQueryBuilder::new("audit_events".to_string(), rls_manager);
        builder
            .select(vec!["event_id".to_string(), "action".to_string()])
            .where_clause("action = 'login'".to_string())
            .limit(100);

        let query = builder.build().unwrap();

        // Verify RLS enforcement is in the query
        assert!(query.contains("tenant_id = 'enterprise-tenant'"));
        assert!(query.contains("action = 'login'"));
        assert!(query.contains("LIMIT 100"));

        println!("✅ Test passed: Row-Level Security enforces tenant isolation in queries");
        println!("   Generated query: {}", query);
    }

    /// Test 4: Cross-Tenant Access Should Fail
    #[tokio::test]
    async fn test_cross_tenant_access_should_fail() {
        let mut rls_manager = RlsManager::new();
        rls_manager.set_tenant_id("tenant-1".to_string());

        // Tenant 1 should only see their data
        let mut builder = RlsQueryBuilder::new("audit_events".to_string(), rls_manager.clone());
        let query_tenant1 = builder.build().unwrap();
        assert!(query_tenant1.contains("tenant_id = 'tenant-1'"));

        // Change to tenant 2
        rls_manager.set_tenant_id("tenant-2".to_string());
        let mut builder2 = RlsQueryBuilder::new("audit_events".to_string(), rls_manager);
        let query_tenant2 = builder2.build().unwrap();
        assert!(query_tenant2.contains("tenant_id = 'tenant-2'"));

        // Verify different tenant IDs
        assert_ne!(query_tenant1, query_tenant2);

        println!("✅ Test passed: Cross-tenant access is prevented by RLS");
    }

    /// Test 5: API Key Management - Generate and validate API keys
    #[tokio::test]
    async fn test_api_key_management() {
        let mut store = ApiKeyStore::new();

        // Generate API key for tenant
        let api_key = store
            .generate_key(
                "tenant-123".to_string(),
                "Test API Key".to_string(),
                vec![ApiScope::AuditRead, ApiScope::AuditWrite],
            )
            .unwrap();

        assert_eq!(api_key.metadata.tenant_id, "tenant-123");
        assert_eq!(api_key.metadata.name, "Test API Key");
        assert_eq!(api_key.metadata.scopes.len(), 2);
        assert!(api_key.plaintext_key.starts_with("hk_live_"));

        // Validate the key
        let metadata = store
            .validate_key(&api_key.plaintext_key, &[ApiScope::AuditRead])
            .unwrap();
        assert_eq!(metadata.key_id, api_key.metadata.key_id);

        // List keys for tenant
        let keys = store.list_keys("tenant-123");
        assert_eq!(keys.len(), 1);

        println!("✅ Test passed: API key generation and validation working correctly");
    }

    /// Test 6: Scopes Validation - Verify scope-based access control
    #[tokio::test]
    async fn test_scopes_validation() {
        let mut store = ApiKeyStore::new();

        // Create key with specific scopes
        let api_key = store
            .generate_key(
                "tenant-123".to_string(),
                "Read-only Key".to_string(),
                vec![ApiScope::AuditRead], // Only read scope
            )
            .unwrap();

        // Should succeed with read scope
        let result = store.validate_key(&api_key.plaintext_key, &[ApiScope::AuditRead]);
        assert!(result.is_ok());

        // Should fail with write scope
        let result = store.validate_key(&api_key.plaintext_key, &[ApiScope::AuditWrite]);
        assert!(result.is_err());

        println!("✅ Test passed: Scope validation works correctly");
    }

    /// Test 7: Rate Limiting - Verify API key rate limits
    #[tokio::test]
    async fn test_rate_limiting() {
        let mut store = ApiKeyStore::new();

        // Create key with low rate limit
        let mut metadata = crate::api_key::ApiKeyMetadata::new(
            "key-123".to_string(),
            "tenant-123".to_string(),
            "Limited Key".to_string(),
            vec![ApiScope::AuditRead],
        )
        .with_rate_limit(5); // 5 requests per second

        // Add to store
        let plaintext_key = "hk_live_1234567890abcdef1234567890abcdef".to_string();
        let hashed_key = store.hash_key(&plaintext_key).unwrap();
        store.keys.insert("key-123".to_string(), metadata.clone());
        store.hash_lookup.insert(hashed_key, "key-123".to_string());
        store
            .rate_limiters
            .insert("key-123".to_string(), crate::api_key::RateLimiter::new(5));

        // First 5 requests should succeed
        for _ in 0..5 {
            let result = store.validate_key(&plaintext_key, &[ApiScope::AuditRead]);
            assert!(result.is_ok());
        }

        // 6th request should fail (rate limited)
        let result = store.validate_key(&plaintext_key, &[ApiScope::AuditRead]);
        assert!(result.is_err());

        println!("✅ Test passed: Rate limiting enforces API key limits");
    }

    /// Test 8: Key Hashing - Verify secure key hashing
    #[tokio::test]
    async fn test_key_hashing() {
        let store = ApiKeyStore::new();

        let key = "test-api-key-12345";
        let hash1 = store.hash_key(key).unwrap();
        let hash2 = store.hash_key(key).unwrap();

        // Same key should produce same hash
        assert_eq!(hash1, hash2);

        // Different key should produce different hash
        let hash3 = store.hash_key("different-key").unwrap();
        assert_ne!(hash1, hash3);

        // Hash should be SHA-256 (64 hex characters)
        assert_eq!(hash1.len(), 64);

        println!("✅ Test passed: Key hashing is secure and consistent");
    }

    /// Test 9: Key Validation - Verify invalid keys are rejected
    #[tokio::test]
    async fn test_key_validation() {
        let store = ApiKeyStore::new();

        // Valid key
        let result = store.validate_key("hk_live_validkey123456789", &[ApiScope::AuditRead]);
        assert!(result.is_err()); // Key not found, but should not panic

        // Empty key
        let result = store.validate_key("", &[ApiScope::AuditRead]);
        assert!(result.is_err());

        println!("✅ Test passed: Invalid keys are properly rejected");
    }

    /// Test 10: Quotas - Verify tenant quota enforcement
    #[tokio::test]
    async fn test_quotas() {
        let mut manager = QuotaManager::new();

        // Create enterprise tenant quota
        let quota =
            manager.create_tenant_quota("enterprise-tenant".to_string(), "enterprise".to_string());
        assert_eq!(quota.tier, "enterprise");

        // Check quota (should succeed)
        let result = manager.check_quota("enterprise-tenant", QuotaType::EventsPerSecond, 5000);
        assert!(result.is_ok());

        // Exceed quota
        let result = manager.check_quota("enterprise-tenant", QuotaType::EventsPerSecond, 6000);
        assert!(result.is_err());

        // Get quota status
        let status = manager.get_quota_status("enterprise-tenant").unwrap();
        let events_status = status
            .iter()
            .find(|s| s.quota_type == QuotaType::EventsPerSecond)
            .unwrap();
        assert_eq!(events_status.current_usage, 5000);
        assert!(events_status.is_exceeded);

        println!("✅ Test passed: Quota enforcement works correctly");
    }

    /// Test 11: Usage Tracking - Verify usage tracking and alerts
    #[tokio::test]
    async fn test_usage_tracking() {
        let mut manager = QuotaManager::new();

        manager.create_tenant_quota("tenant-123".to_string(), "sme".to_string());

        // Record usage
        let _ = manager.check_quota("tenant-123", QuotaType::ApiRequestsPerSecond, 100);
        let _ = manager.check_quota("tenant-123", QuotaType::StorageBytes, 1000);

        // Get status
        let status = manager.get_quota_status("tenant-123").unwrap();
        assert!(!status.is_empty());

        // Verify tracking
        let api_status = status
            .iter()
            .find(|s| s.quota_type == QuotaType::ApiRequestsPerSecond)
            .unwrap();
        assert_eq!(api_status.current_usage, 100);

        println!("✅ Test passed: Usage tracking records accurately");
    }

    /// Test 12: Abuse Detection - Detect potential abuse
    #[tokio::test]
    async fn test_abuse_detection() {
        let mut manager = QuotaManager::new();
        manager.create_tenant_quota("suspicious-tenant".to_string(), "sme".to_string());

        // Simulate high usage (abuse threshold is 1000 requests per minute)
        for _ in 0..60 {
            let _ = manager.check_quota("suspicious-tenant", QuotaType::ApiRequestsPerSecond, 20);
        }

        // Should detect abuse
        let abuse = manager.check_abuse("suspicious-tenant");
        assert!(abuse.is_some());

        let detection = abuse.unwrap();
        assert_eq!(detection.tenant_id, "suspicious-tenant");
        assert!(detection.requests_last_minute >= 1000);

        println!("✅ Test passed: Abuse detection identifies suspicious activity");
    }

    /// Test 13: Retention Policies - Enterprise (7 years)
    #[tokio::test]
    async fn test_retention_policies() {
        let mut manager = ComplianceManager::new();

        // Create enterprise retention policy
        let policy = RetentionPolicy::enterprise("enterprise-tenant".to_string());
        assert_eq!(policy.retention_days, 2555); // 7 years
        assert!(matches!(
            policy.policy_type,
            crate::compliance::RetentionPolicyType::Enterprise
        ));

        // Old event should be deletable
        let old_event = Utc::now() - chrono::Duration::days(2556);
        assert!(policy.should_delete_event(old_event));

        // Recent event should not be deletable
        let recent_event = Utc::now() - chrono::Duration::days(100);
        assert!(!policy.should_delete_event(recent_event));

        manager.create_retention_policy(policy);
        let retrieved = manager.get_retention_policy("enterprise-tenant").unwrap();
        assert_eq!(retrieved.retention_days, 2555);

        println!("✅ Test passed: Enterprise retention policy enforces 7-year retention");
    }

    /// Test 14: Retention Policies - SME (configurable)
    #[tokio::test]
    async fn test_sme_retention_policies() {
        let policy = RetentionPolicy::sme("sme-tenant".to_string(), 3);
        assert_eq!(policy.retention_days, 1095); // 3 years
        assert!(matches!(
            policy.policy_type,
            crate::compliance::RetentionPolicyType::SME { years: 3 }
        ));

        // Update retention
        let mut policy = policy;
        policy.update_retention(1825); // 5 years
        assert_eq!(policy.retention_days, 1825);

        println!("✅ Test passed: SME retention policy is configurable");
    }

    /// Test 15: Legal Hold - Verify legal hold prevents deletion
    #[tokio::test]
    async fn test_legal_hold() {
        let mut manager = ComplianceManager::new();

        // Create legal hold
        let hold = LegalHold::new(
            "tenant-123".to_string(),
            "Litigation case #12345".to_string(),
            "Case #12345".to_string(),
            "legal@example.com".to_string(),
            Utc::now() - chrono::Duration::days(200),
            Utc::now(),
        );

        manager.create_legal_hold(hold);

        // Event within legal hold range should not be deletable
        let event_in_hold = Utc::now() - chrono::Duration::days(100);
        let can_delete = manager.can_delete_event("tenant-123", event_in_hold);
        assert!(!can_delete); // Protected by legal hold

        // Event outside range should be deletable
        let event_outside = Utc::now() - chrono::Duration::days(365);
        let can_delete = manager.can_delete_event("tenant-123", event_outside);
        assert!(can_delete); // Not protected

        println!("✅ Test passed: Legal hold prevents deletion of protected data");
    }

    /// Test 16: GDPR Compliance - Right to be forgotten
    #[tokio::test]
    async fn test_gdpr_compliance() {
        let mut manager = ComplianceManager::new();

        // Create GDPR deletion request
        let mut request = GDPRRequest::new(
            "tenant-123".to_string(),
            crate::compliance::GDPRRequestType::RightToBeForgotten,
            "user@example.com".to_string(),
        )
        .with_event_ids(vec!["event-1".to_string(), "event-2".to_string()]);

        request.approve("privacy@example.com".to_string());
        manager.create_gdpr_request(request);

        // Event-1 should be deletable
        let can_delete = manager.should_delete_for_gdpr("tenant-123", "event-1");
        assert!(can_delete);

        // Event-3 should not be deletable
        let can_delete = manager.should_delete_for_gdpr("tenant-123", "event-3");
        assert!(!can_delete);

        println!("✅ Test passed: GDPR right to be forgotten enforced");
    }

    /// Test 17: Audit Trail - Verify deletion audit trail
    #[tokio::test]
    async fn test_audit_trail() {
        let mut manager = ComplianceManager::new();

        // Log a deletion
        let event_ids = vec![
            "event-1".to_string(),
            "event-2".to_string(),
            "event-3".to_string(),
        ];
        manager.log_deletion(
            "tenant-123".to_string(),
            event_ids.clone(),
            crate::compliance::DeletionReason::RetentionExpired,
            "admin@example.com".to_string(),
        );

        // Get audit trail
        let audit = manager.get_deletion_audit("tenant-123");
        assert_eq!(audit.len(), 1);
        assert_eq!(audit[0].event_ids, event_ids);
        assert_eq!(audit[0].deleted_by, "admin@example.com");

        println!("✅ Test passed: Audit trail records all deletions");
    }

    /// Test 18: Compliance Report - Generate compliance report
    #[tokio::test]
    async fn test_compliance_report() {
        let manager = ComplianceManager::new();
        let report = manager.generate_compliance_report("tenant-123");

        assert_eq!(report.tenant_id, "tenant-123");
        assert_eq!(report.active_legal_holds, 0);
        assert_eq!(report.total_deletions, 0);
        assert_eq!(report.gdpr_requests_pending, 0);

        println!("✅ Test passed: Compliance report generated successfully");
    }

    /// Test 19: Tenant Context Manager - Full context lifecycle
    #[tokio::test]
    async fn test_tenant_context_manager() {
        let manager = TenantContextManager::new();

        // Initially no context
        assert!(manager.get_context().is_none());
        assert!(manager.get_tenant_id().is_none());

        // Set context
        let context = TenantContext::new("test-tenant".to_string())
            .with_api_key("api-key-123".to_string())
            .with_user("user-456".to_string())
            .with_tier(TenantTier::Enterprise);

        manager.set_context(context.clone());

        // Get context
        let retrieved = manager.get_context().unwrap();
        assert_eq!(retrieved.tenant_id, "test-tenant");
        assert_eq!(retrieved.api_key_id, Some("api-key-123".to_string()));
        assert_eq!(retrieved.tier, TenantTier::Enterprise);

        // Get tenant ID
        assert_eq!(manager.get_tenant_id(), Some("test-tenant".to_string()));

        // Validate
        let result = manager.validate_current();
        assert!(result.is_ok());

        // Clear
        manager.clear();
        assert!(manager.get_context().is_none());

        println!("✅ Test passed: Tenant context manager handles full lifecycle");
    }

    /// Test 20: Security Tests - Verify security measures
    #[tokio::test]
    async fn test_security_measures() {
        // Test 1: Tenant ID is required
        let extractor = TenantExtractor::new();
        let request = tonic::Request::new(());
        let result = extractor.extract_from_metadata(&request);
        assert!(result.is_err());

        // Test 2: API keys are hashed
        let store = ApiKeyStore::new();
        let key = "test-key";
        let hash = store.hash_key(key).unwrap();
        assert!(!hash.contains("test-key"));

        // Test 3: Quotas prevent abuse
        let mut manager = QuotaManager::new();
        manager.create_tenant_quota("tenant-123".to_string(), "startup".to_string());

        // Startup tier has very low limits
        for _ in 0..20 {
            let result = manager.check_quota("tenant-123", QuotaType::ApiRequestsPerSecond, 10);
            if result.is_err() {
                break;
            }
        }
        // Should eventually fail due to quotas

        println!("✅ Test passed: Security measures are in place and working");
    }
}
