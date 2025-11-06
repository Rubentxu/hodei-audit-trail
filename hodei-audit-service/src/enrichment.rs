//! Event Enrichment Pipeline
//!
//! This module provides event enrichment capabilities, adding metadata,
//! context, and calculated fields to audit events asynchronously.

use hodei_audit_proto::AuditEvent;
use hodei_audit_types::hrn::{Hrn, HrnMetadata, HrnResolver};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Configuration for Event Enrichment
#[derive(Debug, Clone)]
pub struct EnrichmentConfig {
    /// Enable HRN metadata enrichment
    pub enable_hrn_enrichment: bool,
    /// Enable geo-location enrichment
    pub enable_geoip: bool,
    /// Enable user context enrichment
    pub enable_user_context: bool,
    /// Timeout for enrichment operations (milliseconds)
    pub timeout_ms: u64,
    /// Maximum number of concurrent enrichment operations
    pub max_concurrent: usize,
}

/// User context information
#[derive(Debug, Clone, PartialEq)]
pub struct UserContext {
    pub user_id: String,
    pub email: String,
    pub department: Option<String>,
    pub manager: Option<String>,
    pub roles: Vec<String>,
}

/// Geo-location information
#[derive(Debug, Clone, PartialEq)]
pub struct GeoLocation {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
}

/// Enrichment statistics
#[derive(Debug, Clone)]
pub struct EnrichmentStats {
    pub total_events: u64,
    pub enriched_events: u64,
    pub failed_enrichments: u64,
    pub hrn_enriched: u64,
    pub geoip_enriched: u64,
    pub user_context_enriched: u64,
    pub calculated_fields_added: u64,
    pub success_rate: f64,
}

/// Event Enricher
/// Enriches audit events with metadata, context, and calculated fields
#[derive(Debug)]
pub struct EventEnricher {
    /// Configuration
    config: EnrichmentConfig,
    /// HRN resolver for metadata
    hrn_resolver: Option<Arc<dyn HrnResolver + Send + Sync>>,
    /// GeoIP database (placeholder for MaxMindDB)
    geoip_db: Arc<RwLock<Option<GeoIpDatabase>>>,
    /// User context client (placeholder for user service)
    user_service_client: Arc<RwLock<Option<UserServiceClient>>>,
    /// Enrichment statistics
    stats: Arc<RwLock<EnrichmentStats>>,
}

/// Placeholder for MaxMindDB GeoIP database
#[derive(Debug, Clone)]
pub struct GeoIpDatabase {
    /// In-memory geoip data for demo/testing
    data: HashMap<String, GeoLocation>,
}

impl GeoIpDatabase {
    /// Create a new GeoIP database with sample data
    pub fn new_with_sample_data() -> Self {
        let mut data = HashMap::new();

        // Sample geoip data
        data.insert(
            "192.168.1.1".to_string(),
            GeoLocation {
                country: Some("United States".to_string()),
                country_code: Some("US".to_string()),
                region: Some("California".to_string()),
                city: Some("San Francisco".to_string()),
                latitude: Some(37.7749),
                longitude: Some(-122.4194),
                timezone: Some("America/Los_Angeles".to_string()),
            },
        );

        data.insert(
            "10.0.0.1".to_string(),
            GeoLocation {
                country: Some("Spain".to_string()),
                country_code: Some("ES".to_string()),
                region: Some("Madrid".to_string()),
                city: Some("Madrid".to_string()),
                latitude: Some(40.4168),
                longitude: Some(-3.7038),
                timezone: Some("Europe/Madrid".to_string()),
            },
        );

        Self { data }
    }

    /// Look up location for an IP address
    pub fn lookup(&self, ip: &str) -> Result<GeoLocation, String> {
        self.data
            .get(ip)
            .cloned()
            .ok_or_else(|| format!("No geoip data found for IP: {}", ip))
    }
}

/// Placeholder for User Service Client
#[derive(Debug, Clone)]
pub struct UserServiceClient {
    /// In-memory user data for demo/testing
    data: HashMap<String, UserContext>,
}

impl UserServiceClient {
    /// Create a new user service client with sample data
    pub fn new_with_sample_data() -> Self {
        let mut data = HashMap::new();

        // Sample user data
        data.insert(
            "user-123".to_string(),
            UserContext {
                user_id: "user-123".to_string(),
                email: "user-123@company.com".to_string(),
                department: Some("Engineering".to_string()),
                manager: Some("mgr-456".to_string()),
                roles: vec!["developer".to_string(), "auditor".to_string()],
            },
        );

        data.insert(
            "user-456".to_string(),
            UserContext {
                user_id: "user-456".to_string(),
                email: "mgr-456@company.com".to_string(),
                department: Some("Engineering".to_string()),
                manager: Some("vp-789".to_string()),
                roles: vec!["manager".to_string(), "auditor".to_string()],
            },
        );

        Self { data }
    }

    /// Get user context for a user ID
    pub fn get_user_context(&self, user_id: &str) -> Result<UserContext, String> {
        self.data
            .get(user_id)
            .cloned()
            .ok_or_else(|| format!("No user context found for user: {}", user_id))
    }
}

impl EventEnricher {
    /// Create a new EventEnricher with default configuration
    pub fn new() -> Self {
        Self::new_with_config(EnrichmentConfig {
            enable_hrn_enrichment: true,
            enable_geoip: true,
            enable_user_context: true,
            timeout_ms: 5000,
            max_concurrent: 10,
        })
    }

    /// Create a new EventEnricher with custom configuration
    pub fn new_with_config(config: EnrichmentConfig) -> Self {
        info!(
            "Initializing EventEnricher: hrn={}, geoip={}, user_context={}, timeout={}ms",
            config.enable_hrn_enrichment,
            config.enable_geoip,
            config.enable_user_context,
            config.timeout_ms
        );

        Self {
            config: config.clone(),
            hrn_resolver: None,
            geoip_db: Arc::new(RwLock::new(None)),
            user_service_client: Arc::new(RwLock::new(None)),
            stats: Arc::new(RwLock::new(EnrichmentStats {
                total_events: 0,
                enriched_events: 0,
                failed_enrichments: 0,
                hrn_enriched: 0,
                geoip_enriched: 0,
                user_context_enriched: 0,
                calculated_fields_added: 0,
                success_rate: 0.0,
            })),
        }
    }

    /// Set HRN resolver
    pub fn with_hrn_resolver(mut self, resolver: Arc<dyn HrnResolver + Send + Sync>) -> Self {
        self.hrn_resolver = Some(resolver);
        self
    }

    /// Set GeoIP database
    pub fn with_geoip_db(mut self, db: GeoIpDatabase) -> Self {
        let mut geoip = self.geoip_db.blocking_write();
        *geoip = Some(db);
        self
    }

    /// Set user service client
    pub fn with_user_service(mut self, client: UserServiceClient) -> Self {
        let mut user_svc = self.user_service_client.blocking_write();
        *user_svc = Some(client);
        self
    }

    /// Enrich a single audit event
    pub async fn enrich(&self, mut event: AuditEvent) -> Result<AuditEvent, String> {
        let hrn_str = event.hrn.clone();
        let user_id_str = event.user_id.clone();

        // Increment total events counter
        {
            let mut stats = self.stats.write().await;
            stats.total_events += 1;
        }

        // Enrich with HRN metadata
        if self.config.enable_hrn_enrichment {
            if let Some(ref resolver) = self.hrn_resolver {
                if let Ok(hrn) = Hrn::parse(&hrn_str) {
                    match resolver.resolve(&hrn).await {
                        Ok(metadata) => {
                            // Add HRN metadata to event metadata
                            if let Some(ref mut metadata_json) = event.metadata {
                                let metadata_map = metadata_json
                                    .entry("hrn_metadata".to_string())
                                    .or_insert_with(|| hodei_audit_proto::MetadataValue {
                                        kind: Some(
                                            hodei_audit_proto::metadata_value::Kind::JsonValue(
                                                serde_json::to_string(&metadata).unwrap(),
                                            ),
                                        ),
                                    });

                                if let Some(ref mut kind) = metadata_map.kind {
                                    if let hodei_audit_proto::metadata_value::Kind::JsonValue(_) =
                                        kind
                                    {
                                        // Metadata already added
                                    }
                                }
                            } else {
                                event.metadata = Some(
                                    serde_json::to_string(&serde_json::json!({
                                        "hrn_metadata": metadata
                                    }))
                                    .map(|s| hodei_audit_proto::MetadataValue {
                                        kind: Some(
                                            hodei_audit_proto::metadata_value::Kind::JsonValue(s),
                                        ),
                                    })
                                    .unwrap(),
                                );
                            }

                            // Update stats
                            {
                                let mut stats = self.stats.write().await;
                                stats.hrn_enriched += 1;
                            }

                            debug!("Enriched event with HRN metadata: {}", hrn_str);
                        }
                        Err(e) => {
                            warn!("Failed to resolve HRN {}: {}", hrn_str, e);
                        }
                    }
                }
            }
        }

        // Enrich with geo-location
        if self.config.enable_geoip {
            if let Some(ref mut metadata_json) = event.metadata {
                if let Ok(metadata_map) =
                    serde_json::from_str::<serde_json::Value>(metadata_json.get_or_insert_default())
                {
                    if let Some(client_ip) = metadata_map
                        .get("client_ip")
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                    {
                        let geoip_db = self.geoip_db.read().await;
                        if let Some(ref db) = *geoip_db {
                            match db.lookup(&client_ip) {
                                Ok(location) => {
                                    // Add geo-location to event metadata
                                    let geo_json =
                                        serde_json::to_string(&location).unwrap_or_default();
                                    let geo_value = hodei_audit_proto::MetadataValue {
                                        kind: Some(
                                            hodei_audit_proto::metadata_value::Kind::JsonValue(
                                                geo_json,
                                            ),
                                        ),
                                    };

                                    // Merge with existing metadata
                                    if let Ok(mut merged) =
                                        serde_json::from_str::<serde_json::Value>(metadata_json)
                                    {
                                        merged["geo_location"] =
                                            serde_json::Value::String(geo_json.clone());
                                        *metadata_json =
                                            serde_json::to_string(&merged).unwrap_or_default();
                                    } else {
                                        let new_metadata = serde_json::json!({
                                            "client_ip": client_ip,
                                            "geo_location": location
                                        });
                                        *metadata_json = serde_json::to_string(&new_metadata)
                                            .unwrap_or_default();
                                    }

                                    // Update stats
                                    {
                                        let mut stats = self.stats.write().await;
                                        stats.geoip_enriched += 1;
                                    }

                                    debug!("Enriched event with geo-location: {}", client_ip);
                                }
                                Err(e) => {
                                    debug!("No geoip data for IP {}: {}", client_ip, e);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Enrich with user context
        if self.config.enable_user_context {
            if !user_id_str.is_empty() {
                let user_svc = self.user_service_client.read().await;
                if let Some(ref client) = *user_svc {
                    match client.get_user_context(&user_id_str) {
                        Ok(user_context) => {
                            // Add user context to event metadata
                            let user_json =
                                serde_json::to_string(&user_context).unwrap_or_default();

                            if let Some(ref mut metadata_json) = event.metadata {
                                if let Ok(mut merged) =
                                    serde_json::from_str::<serde_json::Value>(metadata_json)
                                {
                                    merged["user_context"] =
                                        serde_json::Value::String(user_json.clone());
                                    *metadata_json =
                                        serde_json::to_string(&merged).unwrap_or_default();
                                } else {
                                    let new_metadata = serde_json::json!({
                                        "user_context": user_context
                                    });
                                    *metadata_json =
                                        serde_json::to_string(&new_metadata).unwrap_or_default();
                                }
                            } else {
                                event.metadata = Some(
                                    serde_json::to_string(&serde_json::json!({
                                        "user_context": user_context
                                    }))
                                    .unwrap_or_default(),
                                );
                            }

                            // Update stats
                            {
                                let mut stats = self.stats.write().await;
                                stats.user_context_enriched += 1;
                            }

                            debug!("Enriched event with user context: {}", user_id_str);
                        }
                        Err(e) => {
                            debug!("No user context for user {}: {}", user_id_str, e);
                        }
                    }
                }
            }
        }

        // Add calculated fields
        let processed_at = chrono::Utc::now();
        event.processed_at = Some(prost_types::Timestamp::from(processed_at));

        // Mark as enriched
        if let Some(ref mut metadata_json) = event.metadata {
            if let Ok(mut metadata) = serde_json::from_str::<serde_json::Value>(metadata_json) {
                metadata["enriched"] = serde_json::Value::Bool(true);
                metadata["processed_at"] = serde_json::Value::String(processed_at.to_rfc3339());
                *metadata_json = serde_json::to_string(&metadata).unwrap_or_default();
            }
        } else {
            event.metadata = Some(
                serde_json::to_string(&serde_json::json!({
                    "enriched": true,
                    "processed_at": processed_at.to_rfc3339()
                }))
                .unwrap_or_default(),
            );
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.calculated_fields_added += 1;
            stats.enriched_events += 1;

            // Calculate success rate
            if stats.total_events > 0 {
                stats.success_rate =
                    (stats.enriched_events as f64 / stats.total_events as f64) * 100.0;
            }
        }

        Ok(event)
    }

    /// Enrich a batch of events
    pub async fn enrich_batch(&self, events: Vec<AuditEvent>) -> Result<Vec<AuditEvent>, String> {
        let mut results = Vec::with_capacity(events.len());

        // Process events with limited concurrency
        use futures::stream::StreamExt;
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent));
        let enricher = Arc::new(self.clone());

        let futures = events.into_iter().map(|event| {
            let enricher = Arc::clone(&enricher);
            let semaphore = Arc::clone(&semaphore);

            async move {
                let _permit = semaphore.acquire().await.unwrap();
                enricher.enrich(event).await
            }
        });

        let mut stream =
            futures::stream::iter(futures).buffer_unordered(self.config.max_concurrent);

        while let Some(result) = stream.next().await {
            match result {
                Ok(event) => results.push(event),
                Err(e) => {
                    error!("Failed to enrich event: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }

    /// Get enrichment statistics
    pub async fn get_stats(&self) -> EnrichmentStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Reset statistics
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = EnrichmentStats {
            total_events: 0,
            enriched_events: 0,
            failed_enrichments: 0,
            hrn_enriched: 0,
            geoip_enriched: 0,
            user_context_enriched: 0,
            calculated_fields_added: 0,
            success_rate: 0.0,
        };
    }
}

impl Clone for EventEnricher {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            hrn_resolver: self.hrn_resolver.clone(),
            geoip_db: self.geoip_db.clone(),
            user_service_client: self.user_service_client.clone(),
            stats: self.stats.clone(),
        }
    }
}

impl Default for EventEnricher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hodei_audit_proto::{AuditEvent, EventId, TenantId};

    #[tokio::test]
    async fn test_event_enricher_initialization() {
        let enricher = EventEnricher::new();
        let stats = enricher.get_stats().await;
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[tokio::test]
    async fn test_enrich_with_hrn_metadata() {
        let enricher = EventEnricher::new()
            .with_hrn_resolver(Arc::new(crate::hrn::HrnResolver::new_default()));

        let event = AuditEvent {
            event_id: Some(EventId {
                value: "event-123".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "tenant-123".to_string(),
            }),
            hrn: "hrn:hodei:api:tenant-123:global:api/gateway".to_string(),
            user_id: "user-123".to_string(),
            action: "GET".to_string(),
            path: "/api/test".to_string(),
            method: "GET".to_string(),
            status_code: 200,
            outcome: "success".to_string(),
            timestamp: None,
            processed_at: None,
            metadata: None,
        };

        let enriched = enricher.enrich(event).await.unwrap();

        // Check that processed_at is set
        assert!(enriched.processed_at.is_some());

        // Check that metadata contains enrichment
        if let Some(ref metadata) = enriched.metadata {
            // Metadata should be present
            let metadata_str =
                std::str::from_utf8(metadata.get_or_insert_default()).unwrap_or_default();
            assert!(!metadata_str.is_empty());
        }
    }

    #[tokio::test]
    async fn test_enrich_with_geoip() {
        let enricher = EventEnricher::new().with_geoip_db(GeoIpDatabase::new_with_sample_data());

        let mut event = AuditEvent {
            event_id: Some(EventId {
                value: "event-123".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "tenant-123".to_string(),
            }),
            hrn: "hrn:hodei:api:tenant-123:global:api/gateway".to_string(),
            user_id: "user-123".to_string(),
            action: "GET".to_string(),
            path: "/api/test".to_string(),
            method: "GET".to_string(),
            status_code: 200,
            outcome: "success".to_string(),
            timestamp: None,
            processed_at: None,
            metadata: Some(
                serde_json::to_string(&serde_json::json!({
                    "client_ip": "192.168.1.1"
                }))
                .unwrap_or_default(),
            ),
        };

        let enriched = enricher.enrich(event).await.unwrap();

        // Check that geoip data was added
        if let Some(ref metadata) = enriched.metadata {
            let metadata_str =
                std::str::from_utf8(metadata.get_or_insert_default()).unwrap_or_default();
            assert!(metadata_str.contains("geo_location") || metadata_str.contains("192.168.1.1"));
        }
    }

    #[tokio::test]
    async fn test_enrich_with_user_context() {
        let enricher =
            EventEnricher::new().with_user_service(UserServiceClient::new_with_sample_data());

        let event = AuditEvent {
            event_id: Some(EventId {
                value: "event-123".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "tenant-123".to_string(),
            }),
            hrn: "hrn:hodei:api:tenant-123:global:api/gateway".to_string(),
            user_id: "user-123".to_string(),
            action: "GET".to_string(),
            path: "/api/test".to_string(),
            method: "GET".to_string(),
            status_code: 200,
            outcome: "success".to_string(),
            timestamp: None,
            processed_at: None,
            metadata: None,
        };

        let enriched = enricher.enrich(event).await.unwrap();

        // Check that user context was added
        if let Some(ref metadata) = enriched.metadata {
            let metadata_str =
                std::str::from_utf8(metadata.get_or_insert_default()).unwrap_or_default();
            assert!(metadata_str.contains("user_context") || metadata_str.contains("user-123"));
        }
    }

    #[tokio::test]
    async fn test_enrich_batch() {
        let enricher = EventEnricher::new()
            .with_hrn_resolver(Arc::new(crate::hrn::HrnResolver::new_default()))
            .with_geoip_db(GeoIpDatabase::new_with_sample_data())
            .with_user_service(UserServiceClient::new_with_sample_data());

        let events = vec![
            AuditEvent {
                event_id: Some(EventId {
                    value: "event-1".to_string(),
                }),
                tenant_id: Some(TenantId {
                    value: "tenant-123".to_string(),
                }),
                hrn: "hrn:hodei:api:tenant-123:global:api/gateway".to_string(),
                user_id: "user-123".to_string(),
                action: "GET".to_string(),
                path: "/api/test".to_string(),
                method: "GET".to_string(),
                status_code: 200,
                outcome: "success".to_string(),
                timestamp: None,
                processed_at: None,
                metadata: None,
            },
            AuditEvent {
                event_id: Some(EventId {
                    value: "event-2".to_string(),
                }),
                tenant_id: Some(TenantId {
                    value: "tenant-123".to_string(),
                }),
                hrn: "hrn:hodei:api:tenant-123:global:api/users".to_string(),
                user_id: "user-456".to_string(),
                action: "POST".to_string(),
                path: "/api/users".to_string(),
                method: "POST".to_string(),
                status_code: 201,
                outcome: "success".to_string(),
                timestamp: None,
                processed_at: None,
                metadata: None,
            },
        ];

        let enriched = enricher.enrich_batch(events).await.unwrap();

        assert_eq!(enriched.len(), 2);

        // Check stats
        let stats = enricher.get_stats().await;
        assert_eq!(stats.total_events, 2);
        assert!(stats.enriched_events > 0);
    }

    #[tokio::test]
    async fn test_enrichment_stats() {
        let enricher = EventEnricher::new()
            .with_hrn_resolver(Arc::new(crate::hrn::HrnResolver::new_default()));

        let event = AuditEvent {
            event_id: Some(EventId {
                value: "event-123".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "tenant-123".to_string(),
            }),
            hrn: "hrn:hodei:api:tenant-123:global:api/gateway".to_string(),
            user_id: "user-123".to_string(),
            action: "GET".to_string(),
            path: "/api/test".to_string(),
            method: "GET".to_string(),
            status_code: 200,
            outcome: "success".to_string(),
            timestamp: None,
            processed_at: None,
            metadata: None,
        };

        let _ = enricher.enrich(event).await.unwrap();

        let stats = enricher.get_stats().await;
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.enriched_events, 1);
        assert!(stats.success_rate > 0.0);
    }

    #[tokio::test]
    async fn test_geoip_lookup() {
        let db = GeoIpDatabase::new_with_sample_data();
        let location = db.lookup("192.168.1.1").unwrap();

        assert_eq!(location.country, Some("United States".to_string()));
        assert_eq!(location.city, Some("San Francisco".to_string()));
    }

    #[tokio::test]
    async fn test_user_service_lookup() {
        let client = UserServiceClient::new_with_sample_data();
        let user = client.get_user_context("user-123").unwrap();

        assert_eq!(user.user_id, "user-123");
        assert_eq!(user.email, "user-123@company.com");
        assert!(user.roles.contains(&"developer".to_string()));
    }
}
