//! Event Enrichment Pipeline

use hodei_audit_proto::AuditEvent;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Enrichment statistics
#[derive(Debug, Clone, Default)]
pub struct EnrichmentStats {
    pub total_events: u64,
    pub enriched_events: u64,
}

/// Event Enricher - Basic implementation
pub struct EventEnricher {
    stats: Arc<RwLock<EnrichmentStats>>,
}

impl EventEnricher {
    pub fn new() -> Self {
        info!("Initializing EventEnricher");
        Self {
            stats: Arc::new(RwLock::new(EnrichmentStats::default())),
        }
    }

    pub async fn enrich(&self, mut event: AuditEvent) -> Result<AuditEvent, String> {
        let mut stats = self.stats.write().await;
        stats.total_events += 1;

        // Add processed_at timestamp
        let processed_at = chrono::Utc::now();
        let timestamp = prost_types::Timestamp {
            seconds: processed_at.timestamp(),
            nanos: processed_at.timestamp_subsec_nanos() as i32,
        };
        event.processed_at = Some(timestamp);

        stats.enriched_events += 1;
        Ok(event)
    }

    pub async fn enrich_batch(&self, events: Vec<AuditEvent>) -> Result<Vec<AuditEvent>, String> {
        let mut results = Vec::with_capacity(events.len());
        for event in events {
            results.push(self.enrich(event).await?);
        }
        Ok(results)
    }

    pub async fn get_stats(&self) -> EnrichmentStats {
        self.stats.read().await.clone()
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
    use hodei_audit_proto::{EventId, Hrn, TenantId, UserIdentity};

    fn create_test_event() -> AuditEvent {
        AuditEvent {
            event_id: Some(EventId {
                value: "test-event".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "test-tenant".to_string(),
            }),
            hrn: Some(Hrn::default()),
            user_identity: Some(UserIdentity {
                user_id: "user-123".to_string(),
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                roles: vec![],
                tenant_id: "test-tenant".to_string(),
            }),
            http_context: Some(hodei_audit_proto::HttpContext {
                method: "GET".to_string(),
                path: "/test".to_string(),
                user_agent: "test-agent".to_string(),
                source_ip: "127.0.0.1".to_string(),
                status_code: 200,
                content_length: 0,
            }),
            action: "test".to_string(),
            event_category: 0,
            management_type: 0,
            access_type: 0,
            read_only: true,
            outcome: 0,
            error_code: "".to_string(),
            error_message: "".to_string(),
            event_time: None,
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
    async fn test_event_enricher_initialization() {
        let enricher = EventEnricher::new();
        let stats = enricher.get_stats().await;
        assert_eq!(stats.total_events, 0);
    }

    #[tokio::test]
    async fn test_enrich_basic() {
        let enricher = EventEnricher::new();
        let event = create_test_event();
        let enriched = enricher.enrich(event).await.unwrap();
        assert!(enriched.processed_at.is_some());
    }

    #[tokio::test]
    async fn test_enrich_batch() {
        let enricher = EventEnricher::new();
        let events = vec![create_test_event(), create_test_event()];
        let enriched = enricher.enrich_batch(events).await.unwrap();
        assert_eq!(enriched.len(), 2);

        let stats = enricher.get_stats().await;
        assert_eq!(stats.total_events, 2);
        assert_eq!(stats.enriched_events, 2);
    }
}
