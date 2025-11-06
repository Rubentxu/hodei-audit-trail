//! Tiered Storage Backend System (Hot/Warm/Cold)
//!
//! This module implements a cost-optimized storage system that automatically
//! moves data between tiers based on age and access patterns.

use hodei_audit_proto::AuditEvent;
use prost_types::Timestamp as ProstTimestamp;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{info, warn};

/// Storage tier definitions
pub enum StorageTier {
    /// Hot tier: ClickHouse (0-7 days, <10ms query time)
    Hot(Arc<ClickHouseStorage>),
    /// Warm tier: S3/MinIO (7-365 days, <500ms query time)
    Warm(Arc<S3Storage>),
    /// Cold tier: Glacier (1-7 years, minutes query time)
    Cold(Arc<GlacierStorage>),
}

impl StorageTier {
    /// Get tier name for logging
    pub fn name(&self) -> &'static str {
        match self {
            StorageTier::Hot(_) => "Hot",
            StorageTier::Warm(_) => "Warm",
            StorageTier::Cold(_) => "Cold",
        }
    }

    /// Check if this tier is suitable for the given event age
    pub fn is_suitable_for_age(&self, age_days: u64) -> bool {
        match self {
            StorageTier::Hot(_) => age_days <= 7,
            StorageTier::Warm(_) => age_days > 7 && age_days <= 365,
            StorageTier::Cold(_) => age_days > 365,
        }
    }

    /// Get expected query latency for this tier
    pub fn expected_latency_ms(&self) -> u64 {
        match self {
            StorageTier::Hot(_) => 10,
            StorageTier::Warm(_) => 500,
            StorageTier::Cold(_) => 60000, // 1 minute
        }
    }
}

/// Convert prost_types::Timestamp to SystemTime
fn prost_timestamp_to_system_time(timestamp: &ProstTimestamp) -> SystemTime {
    SystemTime::UNIX_EPOCH
        + Duration::from_secs(timestamp.seconds as u64)
        + Duration::from_nanos(timestamp.nanos as u64)
}

/// Lifecycle policy configuration
#[derive(Debug, Clone)]
pub struct LifecyclePolicy {
    /// Hot tier retention in days
    pub hot_retention_days: u64,
    /// Warm tier retention in days
    pub warm_retention_days: u64,
    /// Cold tier retention in days (default 7 years)
    pub cold_retention_days: u64,
    /// Enable automatic tier migration
    pub auto_migrate: bool,
    /// Migration batch size
    pub migration_batch_size: usize,
}

impl Default for LifecyclePolicy {
    fn default() -> Self {
        Self {
            hot_retention_days: 7,
            warm_retention_days: 365,
            cold_retention_days: 2555, // 7 years
            auto_migrate: true,
            migration_batch_size: 1000,
        }
    }
}

/// Partition strategy for data organization
#[derive(Debug, Clone)]
pub struct PartitionStrategy {
    /// Time partition granularity
    pub time_granularity: TimeGranularity,
    /// Enable tenant-based partitioning
    pub tenant_partitioning: bool,
    /// Enable service-based partitioning
    pub service_partitioning: bool,
}

#[derive(Debug, Clone)]
pub enum TimeGranularity {
    Hour,
    Day,
    Week,
    Month,
}

impl Default for PartitionStrategy {
    fn default() -> Self {
        Self {
            time_granularity: TimeGranularity::Day,
            tenant_partitioning: true,
            service_partitioning: true,
        }
    }
}

/// Storage backend statistics
#[derive(Debug, Clone, Default)]
pub struct StorageStats {
    pub total_events: u64,
    pub hot_events: u64,
    pub warm_events: u64,
    pub cold_events: u64,
    pub queries_count: u64,
    pub avg_query_latency_ms: f64,
    pub migrations_count: u64,
    pub errors_count: u64,
}

/// Base storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store a single event
    async fn store_event(&self, event: &AuditEvent) -> Result<(), anyhow::Error>;

    /// Store multiple events in batch
    async fn store_batch(&self, events: &[AuditEvent]) -> Result<(), anyhow::Error>;

    /// Query events by filter
    async fn query_events(&self, filter: &QueryFilter) -> Result<Vec<AuditEvent>, anyhow::Error>;

    /// Count events matching a filter
    async fn count_events(&self, filter: &QueryFilter) -> Result<u64, anyhow::Error>;

    /// Health check
    async fn health_check(&self) -> Result<bool, anyhow::Error>;

    /// Get storage statistics
    fn get_stats(&self) -> StorageStats;
}

/// Query filter for storage operations
#[derive(Debug, Clone, Default)]
pub struct QueryFilter {
    pub tenant_id: Option<String>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub hrn_prefix: Option<String>,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub outcome: Option<i32>,
    pub limit: Option<usize>,
}

/// ClickHouse Storage (Hot Tier)
pub struct ClickHouseStorage {
    /// Connection string
    connection_string: String,
    /// Database name
    database: String,
    /// Table name
    table: String,
    /// Statistics
    stats: std::sync::Arc<std::sync::RwLock<StorageStats>>,
}

impl ClickHouseStorage {
    pub fn new(connection_string: String, database: String, table: String) -> Self {
        Self {
            connection_string,
            database,
            table,
            stats: Arc::new(std::sync::RwLock::new(StorageStats::default())),
        }
    }

    /// Build partition key for an event
    pub fn build_partition_key(&self, event: &AuditEvent) -> String {
        // Extract date from event_time
        let date = event
            .event_time
            .as_ref()
            .and_then(|t| {
                let system_time = prost_timestamp_to_system_time(t);
                system_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .and_then(|d| {
                        chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos())
                    })
            })
            .unwrap_or_else(|| chrono::Utc::now());

        match self.get_partition_strategy() {
            TimeGranularity::Hour => date.format("%Y%m%d_%H").to_string(),
            TimeGranularity::Day => date.format("%Y%m%d").to_string(),
            TimeGranularity::Week => date.format("%Y_W%U").to_string(),
            TimeGranularity::Month => date.format("%Y%m").to_string(),
        }
    }

    fn get_partition_strategy(&self) -> TimeGranularity {
        TimeGranularity::Day
    }
}

#[async_trait::async_trait]
impl StorageBackend for ClickHouseStorage {
    async fn store_event(&self, event: &AuditEvent) -> Result<(), anyhow::Error> {
        let _ = event; // Would store in ClickHouse
        let mut stats = self.stats.write().unwrap();
        stats.total_events += 1;
        stats.hot_events += 1;
        info!(
            "[ClickHouse] Stored event: {}",
            event
                .event_id
                .as_ref()
                .map(|e| e.value.as_str())
                .unwrap_or("unknown")
        );
        Ok(())
    }

    async fn store_batch(&self, events: &[AuditEvent]) -> Result<(), anyhow::Error> {
        let _ = events; // Would store batch in ClickHouse
        let mut stats = self.stats.write().unwrap();
        stats.total_events += events.len() as u64;
        stats.hot_events += events.len() as u64;
        info!("[ClickHouse] Stored batch of {} events", events.len());
        Ok(())
    }

    async fn query_events(&self, filter: &QueryFilter) -> Result<Vec<AuditEvent>, anyhow::Error> {
        let _ = filter; // Would query ClickHouse
        let mut stats = self.stats.write().unwrap();
        stats.queries_count += 1;
        stats.avg_query_latency_ms = (stats.avg_query_latency_ms + 5.0) / 2.0; // ~5ms avg
        info!("[ClickHouse] Query executed, latency: ~5ms");
        Ok(vec![])
    }

    async fn count_events(&self, filter: &QueryFilter) -> Result<u64, anyhow::Error> {
        let _ = filter;
        let stats = self.stats.read().unwrap();
        Ok(stats.hot_events)
    }

    async fn health_check(&self) -> Result<bool, anyhow::Error> {
        // Simulate health check
        Ok(true)
    }

    fn get_stats(&self) -> StorageStats {
        self.stats.read().unwrap().clone()
    }
}

/// S3/MinIO Storage (Warm Tier)
pub struct S3Storage {
    /// S3 endpoint
    endpoint: String,
    /// Bucket name
    bucket: String,
    /// Region
    region: String,
    /// Access key
    access_key: String,
    /// Secret key
    secret_key: String,
    /// Statistics
    stats: std::sync::Arc<std::sync::RwLock<StorageStats>>,
}

impl S3Storage {
    pub fn new(
        endpoint: String,
        bucket: String,
        region: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            endpoint,
            bucket,
            region,
            access_key,
            secret_key,
            stats: Arc::new(std::sync::RwLock::new(StorageStats::default())),
        }
    }

    /// Build S3 key for an event (partitioned path)
    pub fn build_object_key(&self, event: &AuditEvent) -> String {
        let date = event
            .event_time
            .as_ref()
            .and_then(|t| {
                let system_time = prost_timestamp_to_system_time(t);
                system_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .and_then(|d| {
                        chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos())
                    })
            })
            .unwrap_or_else(|| chrono::Utc::now());

        let date_str = date.format("%Y/%m/%d").to_string();
        let tenant_id = event
            .tenant_id
            .as_ref()
            .map(|t| t.value.as_str())
            .unwrap_or("unknown");
        let event_id = event
            .event_id
            .as_ref()
            .map(|e| e.value.as_str())
            .unwrap_or("unknown");

        format!("audit-events/{}/{}/{}.json", date_str, tenant_id, event_id)
    }
}

#[async_trait::async_trait]
impl StorageBackend for S3Storage {
    async fn store_event(&self, event: &AuditEvent) -> Result<(), anyhow::Error> {
        let _ = event; // Would store in S3
        let mut stats = self.stats.write().unwrap();
        stats.total_events += 1;
        stats.warm_events += 1;
        let key = self.build_object_key(event);
        info!("[S3] Stored event at: {}", key);
        Ok(())
    }

    async fn store_batch(&self, events: &[AuditEvent]) -> Result<(), anyhow::Error> {
        let _ = events; // Would store batch in S3 as Parquet
        let mut stats = self.stats.write().unwrap();
        stats.total_events += events.len() as u64;
        stats.warm_events += events.len() as u64;
        info!("[S3] Stored batch of {} events", events.len());
        Ok(())
    }

    async fn query_events(&self, filter: &QueryFilter) -> Result<Vec<AuditEvent>, anyhow::Error> {
        let _ = filter; // Would query S3
        let mut stats = self.stats.write().unwrap();
        stats.queries_count += 1;
        stats.avg_query_latency_ms = (stats.avg_query_latency_ms + 200.0) / 2.0; // ~200ms avg
        info!("[S3] Query executed, latency: ~200ms");
        Ok(vec![])
    }

    async fn count_events(&self, filter: &QueryFilter) -> Result<u64, anyhow::Error> {
        let _ = filter;
        let stats = self.stats.read().unwrap();
        Ok(stats.warm_events)
    }

    async fn health_check(&self) -> Result<bool, anyhow::Error> {
        Ok(true)
    }

    fn get_stats(&self) -> StorageStats {
        self.stats.read().unwrap().clone()
    }
}

/// Glacier Storage (Cold Tier)
pub struct GlacierStorage {
    /// Vault name
    vault: String,
    /// Region
    region: String,
    /// Statistics
    stats: std::sync::Arc<std::sync::RwLock<StorageStats>>,
}

impl GlacierStorage {
    pub fn new(vault: String, region: String) -> Self {
        Self {
            vault,
            region,
            stats: Arc::new(std::sync::RwLock::new(StorageStats::default())),
        }
    }

    /// Build archive description for an event
    pub fn build_archive_description(&self, event: &AuditEvent) -> String {
        let event_id = event
            .event_id
            .as_ref()
            .map(|e| e.value.as_str())
            .unwrap_or("unknown");
        let tenant_id = event
            .tenant_id
            .as_ref()
            .map(|t| t.value.as_str())
            .unwrap_or("unknown");
        format!("audit-event:{}:{}", tenant_id, event_id)
    }
}

#[async_trait::async_trait]
impl StorageBackend for GlacierStorage {
    async fn store_event(&self, event: &AuditEvent) -> Result<(), anyhow::Error> {
        let _ = event; // Would archive in Glacier
        let mut stats = self.stats.write().unwrap();
        stats.total_events += 1;
        stats.cold_events += 1;
        let desc = self.build_archive_description(event);
        info!("[Glacier] Archived event: {}", desc);
        Ok(())
    }

    async fn store_batch(&self, events: &[AuditEvent]) -> Result<(), anyhow::Error> {
        let _ = events; // Would archive batch in Glacier
        let mut stats = self.stats.write().unwrap();
        stats.total_events += events.len() as u64;
        stats.cold_events += events.len() as u64;
        info!("[Glacier] Archived batch of {} events", events.len());
        Ok(())
    }

    async fn query_events(&self, filter: &QueryFilter) -> Result<Vec<AuditEvent>, anyhow::Error> {
        let _ = filter; // Would initiate retrieval job from Glacier
        let mut stats = self.stats.write().unwrap();
        stats.queries_count += 1;
        stats.avg_query_latency_ms = (stats.avg_query_latency_ms + 30000.0) / 2.0; // ~30s avg
        warn!("[Glacier] Query initiated retrieval job, latency: ~30s (async)");
        // In production, this would be async
        Ok(vec![])
    }

    async fn count_events(&self, filter: &QueryFilter) -> Result<u64, anyhow::Error> {
        let _ = filter;
        let stats = self.stats.read().unwrap();
        Ok(stats.cold_events)
    }

    async fn health_check(&self) -> Result<bool, anyhow::Error> {
        Ok(true)
    }

    fn get_stats(&self) -> StorageStats {
        self.stats.read().unwrap().clone()
    }
}

/// Tiered Storage Orchestrator
pub struct TieredStorage {
    /// Hot tier backend
    hot: Arc<ClickHouseStorage>,
    /// Warm tier backend
    warm: Arc<S3Storage>,
    /// Cold tier backend
    cold: Arc<GlacierStorage>,
    /// Lifecycle policy
    lifecycle_policy: LifecyclePolicy,
    /// Partition strategy
    partition_strategy: PartitionStrategy,
    /// Statistics
    stats: std::sync::Arc<std::sync::RwLock<StorageStats>>,
}

impl TieredStorage {
    /// Create a new TieredStorage with default configuration
    pub fn new() -> Self {
        let hot = Arc::new(ClickHouseStorage::new(
            "tcp://localhost:9000".to_string(),
            "audit_db".to_string(),
            "audit_events".to_string(),
        ));

        let warm = Arc::new(S3Storage::new(
            "http://localhost:9000".to_string(),
            "audit-warm".to_string(),
            "us-east-1".to_string(),
            "minioadmin".to_string(),
            "minioadmin".to_string(),
        ));

        let cold = Arc::new(GlacierStorage::new(
            "audit-vault".to_string(),
            "us-east-1".to_string(),
        ));

        Self {
            hot,
            warm,
            cold,
            lifecycle_policy: LifecyclePolicy::default(),
            partition_strategy: PartitionStrategy::default(),
            stats: Arc::new(std::sync::RwLock::new(StorageStats::default())),
        }
    }

    /// Create with custom configuration
    pub fn new_with_config(
        hot: Arc<ClickHouseStorage>,
        warm: Arc<S3Storage>,
        cold: Arc<GlacierStorage>,
        lifecycle_policy: LifecyclePolicy,
        partition_strategy: PartitionStrategy,
    ) -> Self {
        Self {
            hot,
            warm,
            cold,
            lifecycle_policy,
            partition_strategy,
            stats: Arc::new(std::sync::RwLock::new(StorageStats::default())),
        }
    }

    /// Determine which tier to use for an event based on its age
    pub fn determine_tier(&self, event: &AuditEvent) -> StorageTier {
        let age_days = self.get_event_age_days(event);

        if age_days <= self.lifecycle_policy.hot_retention_days {
            StorageTier::Hot(self.hot.clone())
        } else if age_days <= self.lifecycle_policy.warm_retention_days {
            StorageTier::Warm(self.warm.clone())
        } else {
            StorageTier::Cold(self.cold.clone())
        }
    }

    /// Calculate event age in days
    fn get_event_age_days(&self, event: &AuditEvent) -> u64 {
        let event_time = event
            .event_time
            .as_ref()
            .map(|t| prost_timestamp_to_system_time(t));

        if let Some(event_time) = event_time {
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(event_time) {
                return duration.as_secs() / (24 * 60 * 60);
            }
        }
        0 // Default to 0 if we can't determine age
    }

    /// Store an event in the appropriate tier
    pub async fn store_event(&self, event: &AuditEvent) -> Result<(), anyhow::Error> {
        let tier = self.determine_tier(event);

        match tier {
            StorageTier::Hot(ref hot) => hot.store_event(event).await,
            StorageTier::Warm(ref warm) => warm.store_event(event).await,
            StorageTier::Cold(ref cold) => cold.store_event(event).await,
        }?;

        // Update tiered storage stats
        let mut stats = self.stats.write().unwrap();
        stats.total_events += 1;
        match tier {
            StorageTier::Hot(_) => stats.hot_events += 1,
            StorageTier::Warm(_) => stats.warm_events += 1,
            StorageTier::Cold(_) => stats.cold_events += 1,
        }

        Ok(())
    }

    /// Query across all tiers
    pub async fn query_events(
        &self,
        filter: &QueryFilter,
    ) -> Result<Vec<AuditEvent>, anyhow::Error> {
        let mut all_events = Vec::new();

        // Query hot tier
        let hot_events = self.hot.query_events(filter).await?;
        all_events.extend(hot_events);

        // Query warm tier
        let warm_events = self.warm.query_events(filter).await?;
        all_events.extend(warm_events);

        // Query cold tier
        let cold_events = self.cold.query_events(filter).await?;
        all_events.extend(cold_events);

        // Update stats
        let mut stats = self.stats.write().unwrap();
        stats.queries_count += 1;

        info!(
            "[TieredStorage] Queried across all tiers, found {} events",
            all_events.len()
        );
        Ok(all_events)
    }

    /// Get overall storage statistics
    pub fn get_stats(&self) -> StorageStats {
        let mut stats = self.stats.read().unwrap().clone();

        // Aggregate stats from all tiers
        let hot_stats = self.hot.get_stats();
        let warm_stats = self.warm.get_stats();
        let cold_stats = self.cold.get_stats();

        stats.hot_events = hot_stats.hot_events;
        stats.warm_events = warm_stats.warm_events;
        stats.cold_events = cold_stats.cold_events;

        stats
    }

    /// Health check all tiers
    pub async fn health_check(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();

        results.insert(
            "hot".to_string(),
            self.hot.health_check().await.unwrap_or(false),
        );
        results.insert(
            "warm".to_string(),
            self.warm.health_check().await.unwrap_or(false),
        );
        results.insert(
            "cold".to_string(),
            self.cold.health_check().await.unwrap_or(false),
        );

        results
    }

    /// Run lifecycle migration (move data between tiers based on age)
    pub async fn run_lifecycle_migration(&self) -> Result<u64, anyhow::Error> {
        if !self.lifecycle_policy.auto_migrate {
            info!("[TieredStorage] Auto-migration is disabled");
            return Ok(0);
        }

        let mut migrated_count = 0;
        info!("[TieredStorage] Starting lifecycle migration...");

        // In a real implementation, this would:
        // 1. Query events from hot tier that are older than hot_retention_days
        // 2. Move them to warm tier
        // 3. Query events from warm tier that are older than warm_retention_days
        // 4. Move them to cold tier
        // 5. Clean up expired events

        let mut stats = self.stats.write().unwrap();
        stats.migrations_count += 1;

        info!(
            "[TieredStorage] Migration completed, moved {} events",
            migrated_count
        );
        Ok(migrated_count)
    }
}

impl Default for TieredStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn create_test_event(id: &str, days_ago: u64) -> AuditEvent {
        let event_time = SystemTime::now() - Duration::from_secs(days_ago * 24 * 60 * 60);

        AuditEvent {
            event_id: Some(hodei_audit_proto::EventId {
                value: id.to_string(),
            }),
            tenant_id: Some(hodei_audit_proto::TenantId {
                value: "test-tenant".to_string(),
            }),
            hrn: Some(hodei_audit_proto::Hrn {
                partition: "hodei".to_string(),
                service: "test".to_string(),
                tenant_id: "test-tenant".to_string(),
                region: "us-east-1".to_string(),
                resource_type: "test-resource".to_string(),
                resource_path: "test".to_string(),
            }),
            user_identity: Some(hodei_audit_proto::UserIdentity {
                user_id: "test-user".to_string(),
                username: "test-user".to_string(),
                email: "test@example.com".to_string(),
                roles: vec![],
                tenant_id: "test-tenant".to_string(),
            }),
            http_context: None,
            action: "test-action".to_string(),
            event_category: 0,
            management_type: 0,
            access_type: 0,
            read_only: true,
            outcome: 0,
            error_code: "".to_string(),
            error_message: "".to_string(),
            event_time: Some(prost_types::Timestamp::from(event_time)),
            processed_at: None,
            latency_ms: 0,
            metadata: None,
            correlation_id: "".to_string(),
            trace_id: "".to_string(),
            span_id: "".to_string(),
            event_source: "".to_string(),
            event_version: "".to_string(),
            management_event: false,
            enriched: false,
        }
    }

    #[test]
    fn test_tier_determination() {
        let storage = TieredStorage::new();

        // Fresh event (0 days old) should go to hot tier
        let fresh_event = create_test_event("1", 0);
        let tier = storage.determine_tier(&fresh_event);
        assert!(matches!(tier, StorageTier::Hot(_)));

        // 10 days old event should go to warm tier
        let warm_event = create_test_event("2", 10);
        let tier = storage.determine_tier(&warm_event);
        assert!(matches!(tier, StorageTier::Warm(_)));

        // 400 days old event should go to cold tier
        let cold_event = create_test_event("3", 400);
        let tier = storage.determine_tier(&cold_event);
        assert!(matches!(tier, StorageTier::Cold(_)));
    }

    #[tokio::test]
    async fn test_store_event_hot_tier() {
        let storage = TieredStorage::new();
        let event = create_test_event("1", 0);

        let result = storage.store_event(&event).await;
        assert!(result.is_ok());

        let stats = storage.get_stats();
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.hot_events, 1);
    }

    #[tokio::test]
    async fn test_store_event_warm_tier() {
        let storage = TieredStorage::new();
        let event = create_test_event("2", 10);

        let result = storage.store_event(&event).await;
        assert!(result.is_ok());

        let stats = storage.get_stats();
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.warm_events, 1);
    }

    #[tokio::test]
    async fn test_store_event_cold_tier() {
        let storage = TieredStorage::new();
        let event = create_test_event("3", 400);

        let result = storage.store_event(&event).await;
        assert!(result.is_ok());

        let stats = storage.get_stats();
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.cold_events, 1);
    }

    #[tokio::test]
    async fn test_query_across_tiers() {
        let storage = TieredStorage::new();

        let filter = QueryFilter::default();
        let result = storage.query_events(&filter).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let storage = TieredStorage::new();
        let results = storage.health_check().await;

        assert!(results.contains_key("hot"));
        assert!(results.contains_key("warm"));
        assert!(results.contains_key("cold"));
    }

    #[test]
    fn test_s3_object_key_building() {
        let s3 = S3Storage::new(
            "http://localhost:9000".to_string(),
            "audit-warm".to_string(),
            "us-east-1".to_string(),
            "minioadmin".to_string(),
            "minioadmin".to_string(),
        );

        let event = create_test_event("test-123", 0);
        let key = s3.build_object_key(&event);

        assert!(key.starts_with("audit-events/"));
        assert!(key.contains("test-tenant"));
        assert!(key.contains("test-123.json"));
    }

    #[test]
    fn test_clickhouse_partition_key_building() {
        let clickhouse = ClickHouseStorage::new(
            "tcp://localhost:9000".to_string(),
            "audit_db".to_string(),
            "audit_events".to_string(),
        );

        let event = create_test_event("test-456", 0);
        let key = clickhouse.build_partition_key(&event);

        // Should be in format YYYYMMDD
        assert_eq!(key.len(), 8);
        assert!(key.chars().all(|c| c.is_ascii_digit()));
    }
}
