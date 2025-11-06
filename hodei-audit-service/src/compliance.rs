//! Compliance and Data Retention
//!
//! This module provides comprehensive compliance features including:
//! - Data retention policies (Enterprise: 7 years, SME: 1-5 years configurable)
//! - Legal hold support
//! - GDPR compliance and right to be forgotten
//! - Audit trail for all deletions

use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use tracing::{error, info, warn};

/// Retention policy for a tenant
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    /// Tenant ID
    pub tenant_id: String,
    /// Retention period in days
    pub retention_days: i64,
    /// Policy type
    pub policy_type: RetentionPolicyType,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Is policy active
    pub is_active: bool,
}

impl RetentionPolicy {
    /// Create a new retention policy for Enterprise (7 years)
    pub fn enterprise(tenant_id: String) -> Self {
        let now = Utc::now();
        Self {
            tenant_id,
            retention_days: 2555, // 7 years
            policy_type: RetentionPolicyType::Enterprise,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    /// Create a new retention policy for SME (configurable 1-5 years)
    pub fn sme(tenant_id: String, years: i32) -> Self {
        let now = Utc::now();
        let retention_days = (years * 365) as i64;
        Self {
            tenant_id,
            retention_days,
            policy_type: RetentionPolicyType::SME { years },
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    /// Create a new retention policy for Startup (1 year)
    pub fn startup(tenant_id: String) -> Self {
        Self::sme(tenant_id, 1)
    }

    /// Update retention period
    pub fn update_retention(&mut self, retention_days: i64) {
        self.retention_days = retention_days;
        self.updated_at = Utc::now();
    }

    /// Check if an event should be deleted based on retention policy
    pub fn should_delete_event(&self, event_timestamp: DateTime<Utc>) -> bool {
        if !self.is_active {
            return false;
        }

        let cutoff_date = Utc::now() - Duration::days(self.retention_days);
        event_timestamp < cutoff_date
    }

    /// Get cutoff date for deletion
    pub fn get_cutoff_date(&self) -> DateTime<Utc> {
        Utc::now() - Duration::days(self.retention_days)
    }

    /// Enable policy
    pub fn enable(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Disable policy
    pub fn disable(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}

/// Retention policy types
#[derive(Debug, Clone)]
pub enum RetentionPolicyType {
    /// Enterprise: 7 years fixed
    Enterprise,
    /// SME: 1-5 years configurable
    SME { years: i32 },
    /// Startup: 1 year fixed
    Startup,
}

/// Legal hold status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalHoldStatus {
    /// No legal hold
    None,
    /// Legal hold active
    Active,
    /// Legal hold expired
    Expired,
}

/// Legal hold record
#[derive(Debug, Clone)]
pub struct LegalHold {
    /// Legal hold ID
    pub hold_id: String,
    /// Tenant ID
    pub tenant_id: String,
    /// Reason for legal hold
    pub reason: String,
    /// Legal case or reference
    pub legal_reference: String,
    /// User who initiated the hold
    pub initiated_by: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp (if any)
    pub expires_at: Option<DateTime<Utc>>,
    /// Status
    pub status: LegalHoldStatus,
    /// Protected data range (start)
    pub data_range_start: DateTime<Utc>,
    /// Protected data range (end)
    pub data_range_end: DateTime<Utc>,
}

impl LegalHold {
    /// Create a new legal hold
    pub fn new(
        tenant_id: String,
        reason: String,
        legal_reference: String,
        initiated_by: String,
        data_range_start: DateTime<Utc>,
        data_range_end: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            hold_id: format!("lh_{}", uuid::Uuid::new_v4()),
            tenant_id,
            reason,
            legal_reference,
            initiated_by,
            created_at: now,
            expires_at: None,
            status: LegalHoldStatus::Active,
            data_range_start,
            data_range_end,
        }
    }

    /// Create with expiration
    pub fn with_expiration(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Check if legal hold is active
    pub fn is_active(&self) -> bool {
        self.status == LegalHoldStatus::Active
    }

    /// Check if legal hold has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            return Utc::now() > expires_at;
        }
        false
    }

    /// Check if an event is protected by this legal hold
    pub fn protects_event(&self, event_timestamp: DateTime<Utc>) -> bool {
        event_timestamp >= self.data_range_start && event_timestamp <= self.data_range_end
    }

    /// Update status based on expiration
    pub fn check_expiration(&mut self) {
        if self.is_expired() {
            self.status = LegalHoldStatus::Expired;
        }
    }

    /// Release the legal hold
    pub fn release(&mut self) {
        self.status = LegalHoldStatus::None;
    }
}

/// Compliance manager
pub struct ComplianceManager {
    /// Retention policies
    retention_policies: HashMap<String, RetentionPolicy>,
    /// Legal holds
    legal_holds: HashMap<String, Vec<LegalHold>>,
    /// Deletion audit trail
    deletion_audit: Vec<DeletionAuditRecord>,
    /// GDPR requests
    gdpr_requests: Vec<GDPRRequest>,
}

impl ComplianceManager {
    /// Create a new compliance manager
    pub fn new() -> Self {
        Self {
            retention_policies: HashMap::new(),
            legal_holds: HashMap::new(),
            deletion_audit: Vec::new(),
            gdpr_requests: Vec::new(),
        }
    }

    /// Create or update retention policy
    pub fn create_retention_policy(&mut self, policy: RetentionPolicy) {
        info!(
            "[Compliance] Creating retention policy for tenant {}: {} days",
            policy.tenant_id, policy.retention_days
        );

        self.retention_policies
            .insert(policy.tenant_id.clone(), policy);
    }

    /// Get retention policy for a tenant
    pub fn get_retention_policy(&self, tenant_id: &str) -> Option<&RetentionPolicy> {
        self.retention_policies.get(tenant_id)
    }

    /// Create a legal hold
    pub fn create_legal_hold(&mut self, hold: LegalHold) {
        info!(
            "[Compliance] Creating legal hold {} for tenant {}",
            hold.hold_id, hold.tenant_id
        );

        self.legal_holds
            .entry(hold.tenant_id.clone())
            .or_insert_with(Vec::new)
            .push(hold);
    }

    /// Get legal holds for a tenant
    pub fn get_legal_holds(&self, tenant_id: &str) -> Option<&Vec<LegalHold>> {
        self.legal_holds.get(tenant_id)
    }

    /// Check if an event can be deleted (not protected by legal hold)
    pub fn can_delete_event(&self, tenant_id: &str, event_timestamp: DateTime<Utc>) -> bool {
        // Check if any legal hold protects this event
        if let Some(holds) = self.legal_holds.get(tenant_id) {
            for hold in holds {
                if hold.is_active() && hold.protects_event(event_timestamp) {
                    warn!(
                        "[Compliance] Cannot delete event: protected by legal hold {}",
                        hold.hold_id
                    );
                    return false;
                }
            }
        }
        true
    }

    /// Log a deletion for audit trail
    pub fn log_deletion(
        &mut self,
        tenant_id: String,
        event_ids: Vec<String>,
        reason: DeletionReason,
        deleted_by: String,
    ) {
        let record = DeletionAuditRecord {
            record_id: format!("del_{}", uuid::Uuid::new_v4()),
            tenant_id,
            event_ids,
            reason,
            deleted_by,
            deleted_at: Utc::now(),
        };

        info!(
            "[Compliance] Logging deletion: {} events deleted by {}",
            record.event_ids.len(),
            record.deleted_by
        );

        self.deletion_audit.push(record);
    }

    /// Get deletion audit trail for a tenant
    pub fn get_deletion_audit(&self, tenant_id: &str) -> Vec<&DeletionAuditRecord> {
        self.deletion_audit
            .iter()
            .filter(|r| r.tenant_id == tenant_id)
            .collect()
    }

    /// Create GDPR request
    pub fn create_gdpr_request(&mut self, request: GDPRRequest) {
        info!(
            "[Compliance] Creating GDPR request {} for tenant {}",
            request.request_id, request.tenant_id
        );

        self.gdpr_requests.push(request);
    }

    /// Get GDPR request by ID
    pub fn get_gdpr_request(&self, request_id: &str) -> Option<&GDPRRequest> {
        self.gdpr_requests
            .iter()
            .find(|r| r.request_id == request_id)
    }

    /// Check if an event should be deleted based on GDPR
    pub fn should_delete_for_gdpr(&self, tenant_id: &str, event_id: &str) -> bool {
        // Check if there's a pending GDPR deletion request for this tenant
        for request in &self.gdpr_requests {
            if request.tenant_id == tenant_id
                && request.request_type == GDPRRequestType::RightToBeForgotten
                && request.status == GDPRRequestStatus::Approved
            {
                // Check if the event ID is in the request
                if request.event_ids.contains(&event_id.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    /// Get events eligible for deletion based on retention policy
    pub fn get_events_for_deletion(
        &mut self,
        tenant_id: &str,
    ) -> Result<Vec<String>, ComplianceError> {
        // Get retention policy
        let policy = self
            .retention_policies
            .get(tenant_id)
            .ok_or_else(|| ComplianceError::PolicyNotFound(tenant_id.to_string()))?;

        let cutoff_date = policy.get_cutoff_date();

        // Get events from ClickHouse (mock implementation)
        let event_ids = self.get_events_before_date(tenant_id, cutoff_date)?;

        // Filter out events protected by legal hold
        let deletable_events: Vec<String> = event_ids
            .into_iter()
            .filter(|event_id| {
                // Get event timestamp (mock)
                let event_timestamp = Utc::now() - Duration::days(30);
                self.can_delete_event(tenant_id, event_timestamp)
            })
            .collect();

        info!(
            "[Compliance] Found {} events eligible for deletion for tenant {}",
            deletable_events.len(),
            tenant_id
        );

        Ok(deletable_events)
    }

    /// Get events before a specific date (mock implementation)
    fn get_events_before_date(
        &self,
        tenant_id: &str,
        _date: DateTime<Utc>,
    ) -> Result<Vec<String>, ComplianceError> {
        // In a real implementation, this would query ClickHouse
        // For now, return empty list
        info!(
            "[Compliance] Querying events before date for tenant {}",
            tenant_id
        );
        Ok(vec![])
    }

    /// Update all legal hold statuses
    pub fn update_legal_hold_statuses(&mut self) {
        for holds in self.legal_holds.values_mut() {
            for hold in holds {
                hold.check_expiration();
            }
        }
    }

    /// Generate compliance report
    pub fn generate_compliance_report(&self, tenant_id: &str) -> ComplianceReport {
        let retention_policy = self.retention_policies.get(tenant_id);
        let legal_holds = self
            .legal_holds
            .get(tenant_id)
            .map(|h| h.len())
            .unwrap_or(0);
        let deletion_audit = self.get_deletion_audit(tenant_id);
        let gdpr_requests = self
            .gdpr_requests
            .iter()
            .filter(|r| r.tenant_id == tenant_id)
            .count();

        ComplianceReport {
            tenant_id: tenant_id.to_string(),
            retention_policy: retention_policy.cloned(),
            active_legal_holds: legal_holds,
            total_deletions: deletion_audit.len(),
            gdpr_requests_pending: self
                .gdpr_requests
                .iter()
                .filter(|r| r.tenant_id == tenant_id && r.status == GDPRRequestStatus::Pending)
                .count(),
            report_generated_at: Utc::now(),
        }
    }
}

impl Default for ComplianceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Deletion reason
#[derive(Debug, Clone)]
pub enum DeletionReason {
    /// Retention policy expired
    RetentionExpired,
    /// GDPR right to be forgotten
    GDPRRightToBeForgotten,
    /// User requested deletion
    UserRequested,
    /// Legal hold released
    LegalHoldReleased,
    /// Administrative deletion
    Administrative,
}

/// Deletion audit record
#[derive(Debug, Clone)]
pub struct DeletionAuditRecord {
    pub record_id: String,
    pub tenant_id: String,
    pub event_ids: Vec<String>,
    pub reason: DeletionReason,
    pub deleted_by: String,
    pub deleted_at: DateTime<Utc>,
}

/// GDPR request types
#[derive(Debug, Clone, PartialEq)]
pub enum GDPRRequestType {
    DataAccess,         // Right of access
    DataPortability,    // Right to data portability
    RightToBeForgotten, // Right to erasure
    Rectification,      // Right to rectification
    Restriction,        // Right to restriction of processing
}

/// GDPR request status
#[derive(Debug, Clone, PartialEq)]
pub enum GDPRRequestStatus {
    Pending,
    Approved,
    InProgress,
    Completed,
    Rejected,
}

/// GDPR request
#[derive(Debug, Clone)]
pub struct GDPRRequest {
    pub request_id: String,
    pub tenant_id: String,
    pub request_type: GDPRRequestType,
    pub status: GDPRRequestStatus,
    pub subject_email: String,
    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    /// Specific event IDs (for right to be forgotten)
    pub event_ids: Vec<String>,
    /// Data export URL (for data access/portability)
    pub export_url: Option<String>,
}

impl GDPRRequest {
    /// Create a new GDPR request
    pub fn new(tenant_id: String, request_type: GDPRRequestType, subject_email: String) -> Self {
        Self {
            request_id: format!("gdpr_{}", uuid::Uuid::new_v4()),
            tenant_id,
            request_type,
            status: GDPRRequestStatus::Pending,
            subject_email,
            requested_at: Utc::now(),
            completed_at: None,
            approved_by: None,
            event_ids: Vec::new(),
            export_url: None,
        }
    }

    /// Create with specific event IDs
    pub fn with_event_ids(mut self, event_ids: Vec<String>) -> Self {
        self.event_ids = event_ids;
        self
    }

    /// Approve the request
    pub fn approve(&mut self, approved_by: String) {
        self.status = GDPRRequestStatus::Approved;
        self.approved_by = Some(approved_by);
    }

    /// Complete the request
    pub fn complete(&mut self) {
        self.status = GDPRRequestStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Reject the request
    pub fn reject(&mut self) {
        self.status = GDPRRequestStatus::Rejected;
    }
}

/// Compliance error types
#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Retention policy not found for tenant: {0}")]
    PolicyNotFound(String),

    #[error("Legal hold prevents deletion: {0}")]
    LegalHoldPreventsDeletion(String),

    #[error("Invalid retention period: {0}")]
    InvalidRetentionPeriod(String),

    #[error("GDPR request not found: {0}")]
    GDPRRequestNotFound(String),

    #[error("Other compliance error: {0}")]
    Other(String),
}

/// Compliance report
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub tenant_id: String,
    pub retention_policy: Option<RetentionPolicy>,
    pub active_legal_holds: usize,
    pub total_deletions: usize,
    pub gdpr_requests_pending: usize,
    pub report_generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enterprise_retention_policy() {
        let policy = RetentionPolicy::enterprise("tenant-123".to_string());

        assert_eq!(policy.tenant_id, "tenant-123");
        assert_eq!(policy.retention_days, 2555); // 7 years
        assert!(policy.is_active);
        assert!(matches!(
            policy.policy_type,
            RetentionPolicyType::Enterprise
        ));
    }

    #[test]
    fn test_sme_retention_policy() {
        let policy = RetentionPolicy::sme("tenant-456".to_string(), 3);

        assert_eq!(policy.tenant_id, "tenant-456");
        assert_eq!(policy.retention_days, 1095); // 3 years
        assert!(matches!(
            policy.policy_type,
            RetentionPolicyType::SME { years: 3 }
        ));
    }

    #[test]
    fn test_should_delete_event() {
        let policy = RetentionPolicy::sme("tenant-123".to_string(), 1);

        // Event from 2 years ago should be deleted
        let old_event = Utc::now() - Duration::days(730);
        assert!(policy.should_delete_event(old_event));

        // Event from 6 months ago should not be deleted
        let recent_event = Utc::now() - Duration::days(180);
        assert!(!policy.should_delete_event(recent_event));
    }

    #[test]
    fn test_cutoff_date() {
        let policy = RetentionPolicy::sme("tenant-123".to_string(), 2);
        let cutoff = policy.get_cutoff_date();

        // Cutoff should be 2 years ago
        let expected_cutoff = Utc::now() - Duration::days(730);
        let diff = (cutoff - expected_cutoff).num_seconds().abs();
        assert!(diff < 5); // Allow 5 seconds tolerance
    }

    #[test]
    fn test_legal_hold_creation() {
        let hold = LegalHold::new(
            "tenant-123".to_string(),
            "Litigation hold".to_string(),
            "Case #12345".to_string(),
            "legal@example.com".to_string(),
            Utc::now() - Duration::days(100),
            Utc::now(),
        );

        assert_eq!(hold.tenant_id, "tenant-123");
        assert_eq!(hold.reason, "Litigation hold");
        assert_eq!(hold.legal_reference, "Case #12345");
        assert!(hold.is_active());
    }

    #[test]
    fn test_legal_hold_expiration() {
        let hold = LegalHold::new(
            "tenant-123".to_string(),
            "Test".to_string(),
            "Ref".to_string(),
            "user@example.com".to_string(),
            Utc::now() - Duration::days(10),
            Utc::now(),
        )
        .with_expiration(Utc::now() - Duration::days(1));

        assert!(hold.is_active()); // Not checked yet
        let mut hold_clone = hold.clone();
        hold_clone.check_expiration();
        assert_eq!(hold_clone.status, LegalHoldStatus::Expired);
    }

    #[test]
    fn test_legal_hold_protects_event() {
        let hold = LegalHold::new(
            "tenant-123".to_string(),
            "Test".to_string(),
            "Ref".to_string(),
            "user@example.com".to_string(),
            Utc::now() - Duration::days(30),
            Utc::now(),
        );

        // Event within range should be protected
        let event_in_range = Utc::now() - Duration::days(15);
        assert!(hold.protects_event(event_in_range));

        // Event outside range should not be protected
        let event_out_of_range = Utc::now() - Duration::days(60);
        assert!(!hold.protects_event(event_out_of_range));
    }

    #[test]
    fn test_compliance_manager() {
        let mut manager = ComplianceManager::new();

        // Create retention policy
        let policy = RetentionPolicy::sme("tenant-123".to_string(), 2);
        manager.create_retention_policy(policy);

        // Get policy
        let retrieved = manager.get_retention_policy("tenant-123");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().retention_days, 730);
    }

    #[test]
    fn test_can_delete_event() {
        let mut manager = ComplianceManager::new();

        // No legal holds, should be able to delete
        let can_delete = manager.can_delete_event("tenant-123", Utc::now() - Duration::days(100));
        assert!(can_delete);

        // Create legal hold that protects the event
        let hold = LegalHold::new(
            "tenant-123".to_string(),
            "Test".to_string(),
            "Ref".to_string(),
            "user@example.com".to_string(),
            Utc::now() - Duration::days(200),
            Utc::now() - Duration::days(50),
        );
        manager.create_legal_hold(hold);

        // Event within legal hold range should not be deletable
        let can_delete = manager.can_delete_event("tenant-123", Utc::now() - Duration::days(100));
        assert!(!can_delete);
    }

    #[test]
    fn test_deletion_audit() {
        let mut manager = ComplianceManager::new();

        let event_ids = vec!["event-1".to_string(), "event-2".to_string()];
        manager.log_deletion(
            "tenant-123".to_string(),
            event_ids.clone(),
            DeletionReason::RetentionExpired,
            "admin@example.com".to_string(),
        );

        let audit = manager.get_deletion_audit("tenant-123");
        assert_eq!(audit.len(), 1);
        assert_eq!(audit[0].event_ids, event_ids);
        assert_eq!(audit[0].deleted_by, "admin@example.com");
    }

    #[test]
    fn test_gdpr_request() {
        let mut request = GDPRRequest::new(
            "tenant-123".to_string(),
            GDPRRequestType::RightToBeForgotten,
            "user@example.com".to_string(),
        )
        .with_event_ids(vec!["event-1".to_string()]);

        assert_eq!(request.request_type, GDPRRequestType::RightToBeForgotten);
        assert_eq!(request.status, GDPRRequestStatus::Pending);
        assert_eq!(request.event_ids.len(), 1);

        request.approve("privacy@example.com".to_string());
        assert_eq!(request.status, GDPRRequestStatus::Approved);

        request.complete();
        assert_eq!(request.status, GDPRRequestStatus::Completed);
    }

    #[test]
    fn test_should_delete_for_gdpr() {
        let mut manager = ComplianceManager::new();

        let mut request = GDPRRequest::new(
            "tenant-123".to_string(),
            GDPRRequestType::RightToBeForgotten,
            "user@example.com".to_string(),
        )
        .with_event_ids(vec!["event-1".to_string()]);

        // Approve the request before creating it
        request.approve("privacy@example.com".to_string());

        manager.create_gdpr_request(request);

        // Should delete event-1
        assert!(manager.should_delete_for_gdpr("tenant-123", "event-1"));

        // Should not delete event-2
        assert!(!manager.should_delete_for_gdpr("tenant-123", "event-2"));
    }

    #[test]
    fn test_compliance_report() {
        let manager = ComplianceManager::new();
        let report = manager.generate_compliance_report("tenant-123");

        assert_eq!(report.tenant_id, "tenant-123");
        assert_eq!(report.active_legal_holds, 0);
        assert_eq!(report.total_deletions, 0);
        assert_eq!(report.gdpr_requests_pending, 0);
    }
}
