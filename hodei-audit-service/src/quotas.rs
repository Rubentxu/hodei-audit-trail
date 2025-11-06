//! Resource Quotas and Rate Limiting
//!
//! This module provides comprehensive quota management and rate limiting
//! for tenants and API keys.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{error, info, warn};

/// Quota types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuotaType {
    /// Events per second
    EventsPerSecond,
    /// Storage quota in bytes
    StorageBytes,
    /// Requests per minute
    RequestsPerMinute,
    /// API requests per second
    ApiRequestsPerSecond,
    /// Concurrent connections
    ConcurrentConnections,
}

impl std::fmt::Display for QuotaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotaType::EventsPerSecond => write!(f, "events_per_second"),
            QuotaType::StorageBytes => write!(f, "storage_bytes"),
            QuotaType::RequestsPerMinute => write!(f, "requests_per_minute"),
            QuotaType::ApiRequestsPerSecond => write!(f, "api_requests_per_second"),
            QuotaType::ConcurrentConnections => write!(f, "concurrent_connections"),
        }
    }
}

/// Quota limit configuration
#[derive(Debug, Clone)]
pub struct QuotaLimit {
    /// Quota type
    pub quota_type: QuotaType,
    /// Maximum allowed value
    pub max_value: u64,
    /// Current usage
    pub current_usage: u64,
    /// Window duration (for sliding window limits)
    pub window_duration: Option<Duration>,
}

impl QuotaLimit {
    /// Create a new quota limit
    pub fn new(quota_type: QuotaType, max_value: u64) -> Self {
        Self {
            quota_type,
            max_value,
            current_usage: 0,
            window_duration: None,
        }
    }

    /// Create with window duration
    pub fn with_window(mut self, window: Duration) -> Self {
        self.window_duration = Some(window);
        self
    }

    /// Check if quota is exceeded
    pub fn is_exceeded(&self) -> bool {
        self.current_usage >= self.max_value
    }

    /// Get remaining quota
    pub fn remaining(&self) -> u64 {
        self.max_value.saturating_sub(self.current_usage)
    }

    /// Get usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.max_value == 0 {
            0.0
        } else {
            (self.current_usage as f64 / self.max_value as f64) * 100.0
        }
    }
}

/// Tenant quota configuration
#[derive(Debug, Clone)]
pub struct TenantQuota {
    /// Tenant ID
    pub tenant_id: String,
    /// Quota limits
    pub limits: Vec<QuotaLimit>,
    /// Billing tier
    pub tier: String,
    /// Reset time for periodic quotas
    pub next_reset: DateTime<Utc>,
}

impl TenantQuota {
    /// Create a new tenant quota
    pub fn new(tenant_id: String, tier: String) -> Self {
        let mut quota = Self {
            tenant_id,
            limits: Vec::new(),
            tier,
            next_reset: Utc::now() + chrono::Duration::hours(1),
        };

        // Apply tier-specific limits
        quota.apply_tier_limits();
        quota
    }

    /// Apply tier-specific quota limits
    fn apply_tier_limits(&mut self) {
        match self.tier.to_lowercase().as_str() {
            "enterprise" => {
                self.limits
                    .push(QuotaLimit::new(QuotaType::EventsPerSecond, 10000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::StorageBytes, 1_000_000_000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::RequestsPerMinute, 60000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ApiRequestsPerSecond, 1000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ConcurrentConnections, 1000));
            }
            "sme" => {
                self.limits
                    .push(QuotaLimit::new(QuotaType::EventsPerSecond, 1000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::StorageBytes, 10_000_000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::RequestsPerMinute, 6000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ApiRequestsPerSecond, 100));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ConcurrentConnections, 100));
            }
            "startup" => {
                self.limits
                    .push(QuotaLimit::new(QuotaType::EventsPerSecond, 100));
                self.limits
                    .push(QuotaLimit::new(QuotaType::StorageBytes, 1_000_000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::RequestsPerMinute, 600));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ApiRequestsPerSecond, 10));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ConcurrentConnections, 10));
            }
            _ => {
                // Default to SME limits
                self.limits
                    .push(QuotaLimit::new(QuotaType::EventsPerSecond, 1000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::StorageBytes, 10_000_000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::RequestsPerMinute, 6000));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ApiRequestsPerSecond, 100));
                self.limits
                    .push(QuotaLimit::new(QuotaType::ConcurrentConnections, 100));
            }
        }
    }

    /// Get quota limit for a specific type
    pub fn get_limit(&mut self, quota_type: QuotaType) -> Option<&mut QuotaLimit> {
        self.limits.iter_mut().find(|l| l.quota_type == quota_type)
    }

    /// Check if a specific quota is exceeded
    pub fn is_quota_exceeded(&mut self, quota_type: QuotaType) -> bool {
        if let Some(limit) = self.get_limit(quota_type) {
            limit.is_exceeded()
        } else {
            false
        }
    }

    /// Increment quota usage
    pub fn increment_usage(&mut self, quota_type: QuotaType, amount: u64) {
        if let Some(limit) = self.get_limit(quota_type) {
            limit.current_usage = limit.current_usage.saturating_add(amount);
        }
    }

    /// Reset usage (should be called periodically)
    pub fn reset_usage(&mut self) {
        for limit in &mut self.limits {
            limit.current_usage = 0;
        }
        self.next_reset = Utc::now() + chrono::Duration::hours(1);
    }

    /// Get all exceeded quotas
    pub fn get_exceeded_quotas(&self) -> Vec<&QuotaLimit> {
        self.limits.iter().filter(|l| l.is_exceeded()).collect()
    }
}

/// Quota manager
pub struct QuotaManager {
    /// Tenant quotas
    tenant_quotas: HashMap<String, TenantQuota>,
    /// Usage history (for abuse detection)
    usage_history: HashMap<String, Vec<UsageRecord>>,
    /// Abuse detection threshold
    abuse_threshold: u64,
}

impl QuotaManager {
    /// Create a new quota manager
    pub fn new() -> Self {
        Self {
            tenant_quotas: HashMap::new(),
            usage_history: HashMap::new(),
            abuse_threshold: 1000, // 1000 requests per minute threshold
        }
    }

    /// Create tenant quota
    pub fn create_tenant_quota(&mut self, tenant_id: String, tier: String) -> &TenantQuota {
        info!(
            "[Quota] Creating quota for tenant {} with tier {}",
            tenant_id, tier
        );

        let quota = TenantQuota::new(tenant_id.clone(), tier);
        self.tenant_quotas.insert(tenant_id.clone(), quota);
        self.tenant_quotas.get(&tenant_id).unwrap()
    }

    /// Get tenant quota
    pub fn get_tenant_quota(&self, tenant_id: &str) -> Option<&TenantQuota> {
        self.tenant_quotas.get(tenant_id)
    }

    /// Check if quota is exceeded
    pub fn check_quota(
        &mut self,
        tenant_id: &str,
        quota_type: QuotaType,
        amount: u64,
    ) -> Result<(), QuotaExceeded> {
        // Get or create quota
        if !self.tenant_quotas.contains_key(tenant_id) {
            self.create_tenant_quota(tenant_id.to_string(), "sme".to_string());
        }

        // First, record usage for abuse detection (even if quota exceeded)
        self.record_usage(tenant_id, quota_type, amount);

        let quota = self.tenant_quotas.get_mut(tenant_id).unwrap();

        // Check if quota would be exceeded
        let limit = quota.get_limit(quota_type);
        let (current_usage, max_value, would_exceed) = if let Some(l) = limit {
            (
                l.current_usage,
                l.max_value,
                l.current_usage + amount > l.max_value,
            )
        } else {
            (0, 0, false)
        };

        if would_exceed {
            let error = QuotaExceeded {
                tenant_id: tenant_id.to_string(),
                quota_type,
                current_usage,
                max_value,
                requested: amount,
            };

            warn!(
                "[Quota] Quota exceeded for tenant {}: {:?} - current: {}, max: {}",
                tenant_id, quota_type, current_usage, max_value
            );

            return Err(error);
        }

        // Increment usage
        quota.increment_usage(quota_type, amount);

        info!(
            "[Quota] Updated quota for tenant {}: {:?} = {}",
            tenant_id, quota_type, amount
        );

        Ok(())
    }

    /// Record usage for abuse detection
    fn record_usage(&mut self, tenant_id: &str, quota_type: QuotaType, amount: u64) {
        let now = SystemTime::now();
        let record = UsageRecord {
            timestamp: now,
            quota_type,
            amount,
        };

        self.usage_history
            .entry(tenant_id.to_string())
            .or_insert_with(Vec::new)
            .push(record);

        // Clean old records (keep last hour)
        let cutoff = now - Duration::from_secs(3600);
        if let Some(records) = self.usage_history.get_mut(tenant_id) {
            records.retain(|r| r.timestamp > cutoff);
        }
    }

    /// Check for potential abuse
    pub fn check_abuse(&self, tenant_id: &str) -> Option<AbuseDetection> {
        let records = match self.usage_history.get(tenant_id) {
            Some(r) => r,
            None => return None,
        };

        // Count requests in last minute
        let now = SystemTime::now();
        let one_minute_ago = now - Duration::from_secs(60);
        let requests_last_minute: u64 = records
            .iter()
            .filter(|r| r.timestamp > one_minute_ago)
            .map(|r| r.amount)
            .sum();

        if requests_last_minute > self.abuse_threshold {
            Some(AbuseDetection {
                tenant_id: tenant_id.to_string(),
                requests_last_minute,
                threshold: self.abuse_threshold,
                detected_at: now,
            })
        } else {
            None
        }
    }

    /// Reset all tenant quotas
    pub fn reset_all_quotas(&mut self) {
        info!("[Quota] Resetting all tenant quotas");
        for quota in self.tenant_quotas.values_mut() {
            quota.reset_usage();
        }
    }

    /// Get quota status for a tenant
    pub fn get_quota_status(&self, tenant_id: &str) -> Option<Vec<QuotaStatus>> {
        let quota = self.tenant_quotas.get(tenant_id)?;

        let status: Vec<QuotaStatus> = quota
            .limits
            .iter()
            .map(|l| QuotaStatus {
                quota_type: l.quota_type,
                current_usage: l.current_usage,
                max_value: l.max_value,
                remaining: l.remaining(),
                usage_percentage: l.usage_percentage(),
                is_exceeded: l.is_exceeded(),
            })
            .collect();

        Some(status)
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Usage record for tracking
#[derive(Debug, Clone)]
struct UsageRecord {
    timestamp: SystemTime,
    quota_type: QuotaType,
    amount: u64,
}

/// Quota exceeded error
#[derive(Debug)]
pub struct QuotaExceeded {
    pub tenant_id: String,
    pub quota_type: QuotaType,
    pub current_usage: u64,
    pub max_value: u64,
    pub requested: u64,
}

impl std::fmt::Display for QuotaExceeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quota exceeded for tenant '{}': {:?} - current usage: {}, max: {}, requested: {}",
            self.tenant_id, self.quota_type, self.current_usage, self.max_value, self.requested
        )
    }
}

impl std::error::Error for QuotaExceeded {}

/// Abuse detection result
#[derive(Debug)]
pub struct AbuseDetection {
    pub tenant_id: String,
    pub requests_last_minute: u64,
    pub threshold: u64,
    pub detected_at: SystemTime,
}

/// Quota status
#[derive(Debug, Clone)]
pub struct QuotaStatus {
    pub quota_type: QuotaType,
    pub current_usage: u64,
    pub max_value: u64,
    pub remaining: u64,
    pub usage_percentage: f64,
    pub is_exceeded: bool,
}

/// Alerting for quota violations
pub struct QuotaAlert {
    /// Alert type
    pub alert_type: String,
    /// Tenant ID
    pub tenant_id: String,
    /// Alert message
    pub message: String,
    /// Severity level
    pub severity: AlertSeverity,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for QuotaAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity_str = match self.severity {
            AlertSeverity::Info => "INFO",
            AlertSeverity::Warning => "WARNING",
            AlertSeverity::Critical => "CRITICAL",
        };
        write!(
            f,
            "[{}] {}: {} - {}",
            severity_str, self.alert_type, self.tenant_id, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_limit() {
        let mut limit = QuotaLimit::new(QuotaType::EventsPerSecond, 1000);

        assert_eq!(limit.max_value, 1000);
        assert_eq!(limit.current_usage, 0);
        assert!(!limit.is_exceeded());
        assert_eq!(limit.remaining(), 1000);

        limit.current_usage = 500;
        assert!(!limit.is_exceeded());
        assert_eq!(limit.remaining(), 500);
        assert_eq!(limit.usage_percentage(), 50.0);

        limit.current_usage = 1000;
        assert!(limit.is_exceeded());
        assert_eq!(limit.remaining(), 0);
        assert_eq!(limit.usage_percentage(), 100.0);
    }

    #[test]
    fn test_tenant_quota_enterprise() {
        let mut quota = TenantQuota::new("tenant-123".to_string(), "enterprise".to_string());

        assert_eq!(quota.tier, "enterprise");
        assert!(quota.limits.len() > 0);

        // Find events per second limit
        let max_value = quota
            .get_limit(QuotaType::EventsPerSecond)
            .unwrap()
            .max_value;
        assert_eq!(max_value, 10000);

        // Check usage
        quota.increment_usage(QuotaType::EventsPerSecond, 500);
        let current = quota
            .get_limit(QuotaType::EventsPerSecond)
            .unwrap()
            .current_usage;
        assert_eq!(current, 500);
    }

    #[test]
    fn test_tenant_quota_sme() {
        let mut quota = TenantQuota::new("tenant-456".to_string(), "sme".to_string());

        assert_eq!(quota.tier, "sme");

        let events_limit = quota.get_limit(QuotaType::EventsPerSecond).unwrap();
        assert_eq!(events_limit.max_value, 1000);
    }

    #[test]
    fn test_tenant_quota_startup() {
        let mut quota = TenantQuota::new("tenant-789".to_string(), "startup".to_string());

        assert_eq!(quota.tier, "startup");

        let events_limit = quota.get_limit(QuotaType::EventsPerSecond).unwrap();
        assert_eq!(events_limit.max_value, 100);
    }

    #[test]
    fn test_quota_manager() {
        let mut manager = QuotaManager::new();

        // Create tenant quota
        let quota = manager.create_tenant_quota("tenant-123".to_string(), "sme".to_string());
        assert_eq!(quota.tenant_id, "tenant-123");

        // Check quota (should succeed)
        let result = manager.check_quota("tenant-123", QuotaType::EventsPerSecond, 500);
        assert!(result.is_ok());

        // Check quota again (should succeed)
        let result = manager.check_quota("tenant-123", QuotaType::EventsPerSecond, 400);
        assert!(result.is_ok());

        // Check quota that exceeds limit
        let result = manager.check_quota("tenant-123", QuotaType::EventsPerSecond, 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_quota_exceeded_error() {
        let error = QuotaExceeded {
            tenant_id: "tenant-123".to_string(),
            quota_type: QuotaType::EventsPerSecond,
            current_usage: 900,
            max_value: 1000,
            requested: 200,
        };

        let error_str = format!("{}", error);
        assert!(error_str.contains("tenant-123"));
        assert!(error_str.contains("EventsPerSecond"));
        assert!(error_str.contains("Quota exceeded"));
    }

    #[test]
    fn test_quota_status() {
        let mut manager = QuotaManager::new();
        manager.create_tenant_quota("tenant-123".to_string(), "sme".to_string());

        // Check quota
        let _ = manager.check_quota("tenant-123", QuotaType::EventsPerSecond, 500);

        // Get status
        let status = manager.get_quota_status("tenant-123").unwrap();
        assert!(!status.is_empty());

        // Find events per second status
        let events_status = status
            .iter()
            .find(|s| s.quota_type == QuotaType::EventsPerSecond)
            .unwrap();
        assert_eq!(events_status.current_usage, 500);
        assert_eq!(events_status.max_value, 1000);
        assert_eq!(events_status.remaining, 500);
        assert_eq!(events_status.usage_percentage, 50.0);
        assert!(!events_status.is_exceeded);
    }

    #[test]
    fn test_quota_reset() {
        let mut manager = QuotaManager::new();
        manager.create_tenant_quota("tenant-123".to_string(), "sme".to_string());

        // Use some quota
        let _ = manager.check_quota("tenant-123", QuotaType::EventsPerSecond, 800);

        let status = manager.get_quota_status("tenant-123").unwrap();
        let events_status = status
            .iter()
            .find(|s| s.quota_type == QuotaType::EventsPerSecond)
            .unwrap();
        assert_eq!(events_status.current_usage, 800);

        // Reset quotas
        manager.reset_all_quotas();

        let status = manager.get_quota_status("tenant-123").unwrap();
        let events_status = status
            .iter()
            .find(|s| s.quota_type == QuotaType::EventsPerSecond)
            .unwrap();
        assert_eq!(events_status.current_usage, 0);
    }

    #[test]
    fn test_abuse_detection() {
        let mut manager = QuotaManager::new();
        manager.create_tenant_quota("tenant-123".to_string(), "sme".to_string());

        // Make many requests (abuse threshold is 1000)
        // Use 1 request per call to stay under the 100 quota limit
        for i in 0..1500 {
            // Ignore quota errors after we hit the limit
            let _ = manager.check_quota("tenant-123", QuotaType::ApiRequestsPerSecond, 1);
            // Add a small delay to ensure timestamps are different
            if i % 10 == 0 {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }

        // Check for abuse
        let abuse = manager.check_abuse("tenant-123");
        assert!(abuse.is_some(), "Abuse detection should trigger");

        let detection = abuse.unwrap();
        assert_eq!(detection.tenant_id, "tenant-123");
        // We made 1500 calls with 1 request each
        assert!(
            detection.requests_last_minute >= 1000,
            "Should have at least 1000 requests in last minute, got {}",
            detection.requests_last_minute
        );
    }
}
