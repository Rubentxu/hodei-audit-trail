//! Hodei Audit Service - Main Service Implementation
//!
//! This module provides the main service that orchestrates all components:
//! - TieredStorage for event persistence
//! - HrnResolver for HRN metadata resolution
//! - EventEnricher for event enrichment
//! - QueryEngine for querying events
//! - ClickHouse and S3 clients for storage

use crate::clickhouse::ClickHouseClient;
use crate::enrichment::EventEnricher;
use crate::query::{AuditQuery as EngineQuery, QueryEngine as Engine, QueryResult};
use crate::s3_storage::S3Client;
use crate::storage::TieredStorage;
use anyhow::Result;
use hodei_audit_proto::{AuditEvent, EventId, Hrn, TenantId};
use hodei_audit_types::hrn::{HrnError, HrnMetadata, HrnResolver};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{error, info, warn};

/// Main service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Enable event enrichment
    pub enable_enrichment: bool,
    /// Enable HRN resolution
    pub enable_hrn_resolution: bool,
    /// Enable tiered storage
    pub enable_tiered_storage: bool,
    /// Batch size for processing
    pub batch_size: usize,
    /// Query timeout in seconds
    pub query_timeout_secs: u64,
    /// Enable metrics collection
    pub enable_metrics: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            enable_enrichment: true,
            enable_hrn_resolution: true,
            enable_tiered_storage: true,
            batch_size: 1000,
            query_timeout_secs: 30,
            enable_metrics: true,
        }
    }
}

/// Service metrics
#[derive(Debug, Clone, Default)]
pub struct ServiceMetrics {
    /// Total events ingested
    pub total_events_ingested: u64,
    /// Total events queried
    pub total_events_queried: u64,
    /// Total events enriched
    pub total_events_enriched: u64,
    /// Total events failed
    pub total_events_failed: u64,
    /// Average enrichment latency
    pub avg_enrichment_latency_ms: f64,
    /// Average query latency
    pub avg_query_latency_ms: f64,
    /// Storage tier distribution
    pub hot_events: u64,
    pub warm_events: u64,
    pub cold_events: u64,
}

/// Main Hodei Audit Service
pub struct HodeiAuditService {
    /// Service configuration
    config: ServiceConfig,
    /// Tiered storage backend
    storage: Arc<TieredStorage>,
    /// HRN resolver
    hrn_resolver: Arc<HrnResolverImpl>,
    /// Event enricher
    enricher: Arc<EventEnricher>,
    /// Query engine
    query_engine: Arc<Engine>,
    /// ClickHouse client (for hot tier)
    clickhouse: Option<Arc<ClickHouseClient>>,
    /// S3 client (for warm/cold tiers)
    s3_client: Option<Arc<S3Client>>,
    /// Service metrics
    metrics: Arc<std::sync::RwLock<ServiceMetrics>>,
}

/// HRN Resolver implementation
struct HrnResolverImpl {
    /// In-memory cache (would be a real database in production)
    cache: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, HrnMetadata>>>,
    /// Cache TTL in seconds
    cache_ttl_secs: u64,
}

impl HrnResolverImpl {
    fn new() -> Self {
        Self {
            cache: Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
            cache_ttl_secs: 3600, // 1 hour
        }
    }

    /// Simulate HRN resolution from a database
    async fn resolve_from_db(&self, hrn: &hodei_audit_types::Hrn) -> Result<HrnMetadata, HrnError> {
        // In production, this would query a real database
        let metadata = HrnMetadata {
            hrn: hrn.clone(),
            display_name: format!("Resource {}", hrn.resource_path),
            description: Some(format!("{} resource", hrn.resource_type)),
            tags: std::collections::BTreeMap::new(),
            owner: Some("system".to_string()),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };
        Ok(metadata)
    }
}

#[async_trait::async_trait]
impl HrnResolver for HrnResolverImpl {
    async fn resolve(&self, hrn: &hodei_audit_types::Hrn) -> Result<HrnMetadata, HrnError> {
        let hrn_str = format!(
            "hrn:{}:{}:{}:{}:{}/{}",
            hrn.partition,
            hrn.service,
            hrn.tenant_id,
            hrn.region.as_deref().unwrap_or("global"),
            hrn.resource_type,
            hrn.resource_path
        );

        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(metadata) = cache.get(&hrn_str) {
                info!("[HrnResolver] Cache hit for HRN: {}", hrn_str);
                return Ok(metadata.clone());
            }
        }

        // Resolve from database
        info!("[HrnResolver] Cache miss, resolving from DB: {}", hrn_str);
        let metadata = self.resolve_from_db(hrn).await?;

        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(hrn_str, metadata.clone());
        }

        Ok(metadata)
    }
}

impl HodeiAuditService {
    /// Create a new service instance
    pub async fn new(config: ServiceConfig) -> Result<Self> {
        info!("[Service] Initializing HodeiAuditService...");

        // Initialize tiered storage
        let storage = Arc::new(TieredStorage::new());

        // Initialize HRN resolver
        let hrn_resolver = Arc::new(HrnResolverImpl::new());

        // Initialize event enricher
        let enricher = Arc::new(EventEnricher::new());

        // Initialize query engine
        let query_engine = Arc::new(Engine::new());

        // Initialize ClickHouse client (optional)
        let clickhouse = if config.enable_tiered_storage {
            Some(Arc::new(ClickHouseClient::new_with_defaults()))
        } else {
            None
        };

        // Initialize S3 client (optional)
        let s3_client = if config.enable_tiered_storage {
            Some(Arc::new(S3Client::new_with_defaults()))
        } else {
            None
        };

        let metrics = Arc::new(std::sync::RwLock::new(ServiceMetrics::default()));

        info!("[Service] Initialized successfully");
        info!("  - Enrichment: {}", config.enable_enrichment);
        info!("  - HRN Resolution: {}", config.enable_hrn_resolution);
        info!("  - Tiered Storage: {}", config.enable_tiered_storage);
        info!("  - Batch Size: {}", config.batch_size);

        Ok(Self {
            config,
            storage,
            hrn_resolver,
            enricher,
            query_engine,
            clickhouse,
            s3_client,
            metrics,
        })
    }

    /// Create with default configuration
    pub async fn new_with_defaults() -> Result<Self> {
        Self::new(ServiceConfig::default()).await
    }

    /// Publish a single event
    pub async fn publish_event(&self, event: AuditEvent) -> Result<EventId> {
        let start_time = SystemTime::now();
        let event_id = event.event_id.clone().unwrap_or_else(|| EventId {
            value: uuid::Uuid::new_v4().to_string(),
        });

        info!("[Service] Publishing event: {}", event_id.value);

        // Step 1: Enrich event (if enabled)
        let mut enriched_event = event;
        if self.config.enable_enrichment {
            let enrich_start = SystemTime::now();
            match self.enricher.enrich(enriched_event.clone()).await {
                Ok(event) => {
                    enriched_event = event;
                    let latency = enrich_start.elapsed()?.as_millis() as f64;
                    self.update_enrichment_metrics(latency);
                    info!("[Service] Event enriched successfully");
                }
                Err(e) => {
                    warn!(
                        "[Service] Enrichment failed: {}, continuing without enrichment",
                        e
                    );
                }
            }
        }

        // Step 2: Store in tiered storage
        if self.config.enable_tiered_storage {
            if let Err(e) = self.storage.store_event(&enriched_event).await {
                error!("[Service] Failed to store event: {}", e);
                self.increment_failed_events();
                return Err(e);
            }
        }

        // Step 3: Update metrics
        let latency = start_time.elapsed()?.as_millis() as f64;
        self.update_ingestion_metrics(latency);

        // Step 4: Update storage tier metrics
        let storage_stats = self.storage.get_stats();
        self.update_storage_metrics(storage_stats);

        info!("[Service] Event published successfully in {}ms", latency);
        Ok(event_id)
    }

    /// Publish a batch of events
    pub async fn publish_batch(&self, events: Vec<AuditEvent>) -> Result<Vec<EventId>> {
        let start_time = SystemTime::now();
        let event_count = events.len();
        let mut published_ids = Vec::new();

        info!("[Service] Publishing batch of {} events", event_count);

        // Process events in parallel (in production, this would use a proper async pool)
        for event in events.into_iter() {
            match self.publish_event(event).await {
                Ok(event_id) => published_ids.push(event_id),
                Err(e) => {
                    error!("[Service] Failed to publish event in batch: {}", e);
                    self.increment_failed_events();
                }
            }
        }

        let latency = start_time.elapsed()?.as_millis() as f64;
        info!(
            "[Service] Batch published: {}/{} events in {}ms",
            published_ids.len(),
            event_count,
            latency
        );

        Ok(published_ids)
    }

    /// Query events
    pub async fn query_events(&self, query: EngineQuery) -> Result<QueryResult> {
        let start_time = SystemTime::now();

        info!("[Service] Querying events...");

        // Use query engine
        let result = self
            .query_engine
            .execute(query)
            .map_err(|e| anyhow::anyhow!("Query failed: {}", e))?;

        let latency = start_time.elapsed()?.as_millis() as f64;
        self.update_query_metrics(latency);

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();
            metrics.total_events_queried += result.events.len() as u64;
        }

        info!(
            "[Service] Query completed: {} events in {}ms",
            result.events.len(),
            latency
        );
        Ok(result)
    }

    /// Resolve HRN metadata
    pub async fn resolve_hrn(&self, hrn: &Hrn) -> Result<HrnMetadata, HrnError> {
        if !self.config.enable_hrn_resolution {
            return Err(HrnError::ParseError("HRN resolution disabled".to_string()));
        }

        // Convert proto Hrn to types Hrn
        let types_hrn = hodei_audit_types::Hrn::new(
            hrn.partition.clone(),
            hrn.service.clone(),
            hrn.tenant_id.clone(),
            Some(hrn.region.clone()),
            hrn.resource_type.clone(),
            hrn.resource_path.clone(),
        );

        self.hrn_resolver.resolve(&types_hrn).await
    }

    /// Get service health status
    pub async fn health_check(&self) -> Result<std::collections::HashMap<String, bool>> {
        let mut health = std::collections::HashMap::new();

        // Check storage
        if let Some(ref clickhouse) = self.clickhouse {
            health.insert(
                "clickhouse".to_string(),
                clickhouse.health_check().await.unwrap_or(false),
            );
        }

        if let Some(ref s3) = self.s3_client {
            health.insert("s3".to_string(), s3.health_check().await.unwrap_or(false));
        }

        // Always healthy if storage is disabled
        if !self.config.enable_tiered_storage {
            health.insert("storage".to_string(), true);
        }

        // Storage tier health
        let storage_health = self.storage.health_check().await;
        for (tier, is_healthy) in storage_health {
            health.insert(format!("storage-{}", tier), is_healthy);
        }

        info!("[Service] Health check: {:?}", health);
        Ok(health)
    }

    /// Get service metrics
    pub fn get_metrics(&self) -> ServiceMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Reset metrics
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().unwrap();
        *metrics = ServiceMetrics::default();
        info!("[Service] Metrics reset");
    }

    /// Update ingestion metrics
    fn update_ingestion_metrics(&self, latency_ms: f64) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.total_events_ingested += 1;
    }

    /// Update enrichment metrics
    fn update_enrichment_metrics(&self, latency_ms: f64) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.total_events_enriched += 1;
        if metrics.avg_enrichment_latency_ms == 0.0 {
            metrics.avg_enrichment_latency_ms = latency_ms;
        } else {
            metrics.avg_enrichment_latency_ms =
                (metrics.avg_enrichment_latency_ms + latency_ms) / 2.0;
        }
    }

    /// Update query metrics
    fn update_query_metrics(&self, latency_ms: f64) {
        let mut metrics = self.metrics.write().unwrap();
        if metrics.avg_query_latency_ms == 0.0 {
            metrics.avg_query_latency_ms = latency_ms;
        } else {
            metrics.avg_query_latency_ms = (metrics.avg_query_latency_ms + latency_ms) / 2.0;
        }
    }

    /// Update storage tier metrics
    fn update_storage_metrics(&self, stats: crate::storage::StorageStats) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.hot_events = stats.hot_events;
        metrics.warm_events = stats.warm_events;
        metrics.cold_events = stats.cold_events;
    }

    /// Increment failed events counter
    fn increment_failed_events(&self) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.total_events_failed += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_event(id: &str) -> AuditEvent {
        AuditEvent {
            event_id: Some(EventId {
                value: id.to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "test-tenant".to_string(),
            }),
            hrn: Some(Hrn {
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
            event_time: Some(prost_types::Timestamp::from(SystemTime::now())),
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

    #[tokio::test]
    async fn test_service_initialization() {
        let service = HodeiAuditService::new_with_defaults().await;
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_publish_event() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        let event = create_test_event("test-1");

        let result = service.publish_event(event).await;
        assert!(result.is_ok());
        assert!(result.unwrap().value.len() > 0);
    }

    #[tokio::test]
    async fn test_publish_batch() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        let events = vec![
            create_test_event("batch-1"),
            create_test_event("batch-2"),
            create_test_event("batch-3"),
        ];

        let result = service.publish_batch(events).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_query_events() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();

        let query = EngineQuery {
            tenant_id: Some("test-tenant".to_string()),
            limit: 100,
            ..Default::default()
        };

        let result = service.query_events(query).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_hrn_resolution() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        let hrn = Hrn {
            partition: "hodei".to_string(),
            service: "test".to_string(),
            tenant_id: "test-tenant".to_string(),
            region: "us-east-1".to_string(),
            resource_type: "test-resource".to_string(),
            resource_path: "test".to_string(),
        };

        let result = service.resolve_hrn(&hrn).await;
        assert!(result.is_ok());
        // Verify the resolved HRN matches the input
        let resolved = result.unwrap();
        assert_eq!(resolved.hrn.partition, hrn.partition);
        assert_eq!(resolved.hrn.service, hrn.service);
        assert_eq!(resolved.hrn.tenant_id, hrn.tenant_id);
    }

    #[tokio::test]
    async fn test_health_check() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        let health = service.health_check().await;
        assert!(health.is_ok());
        assert!(!health.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_metrics() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        let metrics = service.get_metrics();
        assert_eq!(metrics.total_events_ingested, 0);
        assert_eq!(metrics.total_events_queried, 0);
    }

    #[tokio::test]
    async fn test_metrics_reset() {
        let service = HodeiAuditService::new_with_defaults().await.unwrap();
        service.reset_metrics();
        let metrics = service.get_metrics();
        assert_eq!(metrics.total_events_ingested, 0);
    }
}
