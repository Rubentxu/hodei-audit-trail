//! Query Engine for Audit Events
//!
//! This module provides advanced querying capabilities for audit events
//! with filtering, sorting, pagination, and optimization.

use hodei_audit_proto::AuditEvent;
use hodei_audit_types::hrn::Hrn;
use std::time::SystemTime;
use tracing::info;

/// Query result with events and metadata
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub events: Vec<AuditEvent>,
    pub total_count: u64,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// Query filters
#[derive(Debug, Clone, Default)]
pub struct AuditQuery {
    pub tenant_id: Option<String>,
    pub hrn: Option<Hrn>,
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub outcome: Option<i32>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: usize,
    pub cursor: Option<String>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

/// Sort fields
#[derive(Debug, Clone)]
pub enum SortField {
    Timestamp,
    Hrn,
    UserId,
    Action,
}

/// Sort order
#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Query statistics
#[derive(Debug, Clone, Default)]
pub struct QueryStats {
    pub total_queries: u64,
    pub avg_latency_ms: f64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

/// Query Engine for audit events
pub struct QueryEngine {
    /// In-memory storage for demo (would be ClickHouse/S3 in production)
    events: std::sync::Arc<std::sync::RwLock<Vec<AuditEvent>>>,
    /// Query statistics
    stats: std::sync::Arc<std::sync::RwLock<QueryStats>>,
    /// Max limit for queries
    max_limit: usize,
}

impl QueryEngine {
    /// Create a new QueryEngine
    pub fn new() -> Self {
        info!("Initializing QueryEngine");
        Self {
            events: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
            stats: std::sync::Arc::new(std::sync::RwLock::new(QueryStats::default())),
            max_limit: 1000,
        }
    }

    /// Add events to the engine (for demo/testing)
    pub fn add_events(&self, new_events: Vec<AuditEvent>) {
        let count = new_events.len();
        let mut events = self.events.write().unwrap();
        events.extend(new_events);
        info!("Added {} events to QueryEngine", count);
    }

    /// Execute a query
    pub fn execute(&self, query: AuditQuery) -> Result<QueryResult, String> {
        let start_time = std::time::Instant::now();

        // Apply filters
        let filtered_events = self.apply_filters(&query);

        // Sort events
        let sorted_events = self.sort_events(filtered_events, query.sort_by, query.sort_order);

        // Apply pagination
        let paginated_events = self.apply_pagination(sorted_events, query.cursor, query.limit);

        // Calculate stats
        let latency = start_time.elapsed().as_millis() as f64;
        self.update_stats(latency);

        Ok(paginated_events)
    }

    /// Apply filters to events
    fn apply_filters(&self, query: &AuditQuery) -> Vec<AuditEvent> {
        let events = self.events.read().unwrap();
        let mut filtered: Vec<AuditEvent> = events.clone();

        // Filter by tenant_id
        if let Some(ref tenant_id) = query.tenant_id {
            filtered.retain(|event| {
                event
                    .tenant_id
                    .as_ref()
                    .map(|t| t.value == *tenant_id)
                    .unwrap_or(false)
            });
        }

        // Filter by HRN
        if let Some(ref hrn) = query.hrn {
            let hrn_str = format!(
                "hrn:{}:{}:{}:{}:{}/{}",
                hrn.partition,
                hrn.service,
                hrn.tenant_id,
                hrn.region.as_deref().unwrap_or("global"),
                hrn.resource_type,
                hrn.resource_path
            );
            filtered.retain(|event| {
                event
                    .hrn
                    .as_ref()
                    .map(|e| {
                        format!(
                            "hrn:{}:{}:{}:{}:{}/{}",
                            e.partition,
                            e.service,
                            e.tenant_id,
                            e.region,
                            e.resource_type,
                            e.resource_path
                        ) == hrn_str
                    })
                    .unwrap_or(false)
            });
        }

        // Filter by user_id
        if let Some(ref user_id) = query.user_id {
            filtered.retain(|event| {
                event
                    .user_identity
                    .as_ref()
                    .map(|u| u.user_id == *user_id)
                    .unwrap_or(false)
            });
        }

        // Filter by action
        if let Some(ref action) = query.action {
            filtered.retain(|event| event.action.contains(action));
        }

        // Filter by outcome
        if let Some(outcome) = query.outcome {
            filtered.retain(|event| event.outcome == outcome);
        }

        // Filter by time range
        if let Some(start_time) = query.start_time {
            filtered.retain(|event| {
                event
                    .event_time
                    .as_ref()
                    .and_then(|t| Self::timestamp_to_datetime(t))
                    .map(|dt| dt >= start_time)
                    .unwrap_or(false)
            });
        }

        if let Some(end_time) = query.end_time {
            filtered.retain(|event| {
                event
                    .event_time
                    .as_ref()
                    .and_then(|t| Self::timestamp_to_datetime(t))
                    .map(|dt| dt <= end_time)
                    .unwrap_or(false)
            });
        }

        filtered
    }

    /// Sort events
    fn sort_events(
        &self,
        mut events: Vec<AuditEvent>,
        sort_by: Option<SortField>,
        sort_order: Option<SortOrder>,
    ) -> Vec<AuditEvent> {
        let order = sort_order.unwrap_or(SortOrder::Desc);

        match sort_by {
            Some(SortField::Timestamp) => {
                events.sort_by(|a, b| {
                    let a_time = a
                        .event_time
                        .as_ref()
                        .and_then(|t| Self::timestamp_to_datetime(t))
                        .unwrap_or_else(chrono::Utc::now);
                    let b_time = b
                        .event_time
                        .as_ref()
                        .and_then(|t| Self::timestamp_to_datetime(t))
                        .unwrap_or_else(chrono::Utc::now);
                    match order {
                        SortOrder::Asc => a_time.cmp(&b_time),
                        SortOrder::Desc => b_time.cmp(&a_time),
                    }
                });
            }
            Some(SortField::Hrn) => {
                events.sort_by(|a, b| {
                    let a_hrn = a
                        .hrn
                        .as_ref()
                        .map(|e| {
                            format!(
                                "hrn:{}:{}:{}:{}:{}/{}",
                                e.partition,
                                e.service,
                                e.tenant_id,
                                e.region,
                                e.resource_type,
                                e.resource_path
                            )
                        })
                        .unwrap_or_default();
                    let b_hrn = b
                        .hrn
                        .as_ref()
                        .map(|e| {
                            format!(
                                "hrn:{}:{}:{}:{}:{}/{}",
                                e.partition,
                                e.service,
                                e.tenant_id,
                                e.region,
                                e.resource_type,
                                e.resource_path
                            )
                        })
                        .unwrap_or_default();
                    match order {
                        SortOrder::Asc => a_hrn.cmp(&b_hrn),
                        SortOrder::Desc => b_hrn.cmp(&a_hrn),
                    }
                });
            }
            Some(SortField::UserId) => {
                events.sort_by(|a, b| {
                    let a_user = a
                        .user_identity
                        .as_ref()
                        .map(|u| u.user_id.clone())
                        .unwrap_or_default();
                    let b_user = b
                        .user_identity
                        .as_ref()
                        .map(|u| u.user_id.clone())
                        .unwrap_or_default();
                    match order {
                        SortOrder::Asc => a_user.cmp(&b_user),
                        SortOrder::Desc => b_user.cmp(&a_user),
                    }
                });
            }
            Some(SortField::Action) => {
                events.sort_by(|a, b| match order {
                    SortOrder::Asc => a.action.cmp(&b.action),
                    SortOrder::Desc => b.action.cmp(&a.action),
                });
            }
            None => {}
        }

        events
    }

    /// Apply pagination
    fn apply_pagination(
        &self,
        events: Vec<AuditEvent>,
        cursor: Option<String>,
        limit: usize,
    ) -> QueryResult {
        let limit = if limit > self.max_limit {
            self.max_limit
        } else {
            limit
        };

        let start_idx = if let Some(ref cursor) = cursor {
            cursor.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        let end_idx = (start_idx + limit).min(events.len());
        let paginated_events = events[start_idx..end_idx].to_vec();

        QueryResult {
            events: paginated_events,
            total_count: events.len() as u64,
            has_more: end_idx < events.len(),
            next_cursor: if end_idx < events.len() {
                Some(end_idx.to_string())
            } else {
                None
            },
        }
    }

    /// Convert prost timestamp to DateTime
    fn timestamp_to_datetime(
        timestamp: &prost_types::Timestamp,
    ) -> Option<chrono::DateTime<chrono::Utc>> {
        // Convert timestamp to SystemTime manually
        let system_time = SystemTime::UNIX_EPOCH
            + std::time::Duration::from_secs(timestamp.seconds as u64)
            + std::time::Duration::from_nanos(timestamp.nanos as u64);
        system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .and_then(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos()))
    }

    /// Update query statistics
    fn update_stats(&self, latency_ms: f64) {
        let mut stats = self.stats.write().unwrap();
        stats.total_queries += 1;

        // Update running average
        if stats.avg_latency_ms == 0.0 {
            stats.avg_latency_ms = latency_ms;
        } else {
            stats.avg_latency_ms = (stats.avg_latency_ms + latency_ms) / 2.0;
        }
    }

    /// Get query statistics
    pub fn get_stats(&self) -> QueryStats {
        self.stats.read().unwrap().clone()
    }
}

impl Default for QueryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hodei_audit_proto::{EventId, Hrn, TenantId, UserIdentity};

    fn create_test_event(id: &str, tenant: &str, user: &str, action: &str) -> AuditEvent {
        AuditEvent {
            event_id: Some(EventId {
                value: id.to_string(),
            }),
            tenant_id: Some(TenantId {
                value: tenant.to_string(),
            }),
            hrn: Some(Hrn::default()),
            user_identity: Some(UserIdentity {
                user_id: user.to_string(),
                username: user.to_string(),
                email: format!("{}@example.com", user),
                roles: vec![],
                tenant_id: tenant.to_string(),
            }),
            http_context: Some(hodei_audit_proto::HttpContext {
                method: "GET".to_string(),
                path: "/test".to_string(),
                user_agent: "test-agent".to_string(),
                source_ip: "127.0.0.1".to_string(),
                status_code: 200,
                content_length: 0,
            }),
            action: action.to_string(),
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

    #[test]
    fn test_query_engine_initialization() {
        let engine = QueryEngine::new();
        let stats = engine.get_stats();
        assert_eq!(stats.total_queries, 0);
    }

    #[test]
    fn test_execute_query_with_filters() {
        let engine = QueryEngine::new();
        let events = vec![
            create_test_event("1", "tenant1", "user1", "GET"),
            create_test_event("2", "tenant1", "user2", "POST"),
            create_test_event("3", "tenant2", "user3", "GET"),
        ];
        engine.add_events(events);

        let query = AuditQuery {
            tenant_id: Some("tenant1".to_string()),
            limit: 100,
            ..Default::default()
        };

        let result = engine.execute(query).unwrap();
        assert_eq!(result.events.len(), 2);
        assert_eq!(result.total_count, 2);
    }

    #[test]
    fn test_execute_query_with_pagination() {
        let engine = QueryEngine::new();
        let events = (0..20)
            .map(|i| create_test_event(&i.to_string(), "tenant1", "user1", "GET"))
            .collect();
        engine.add_events(events);

        // First page
        let query1 = AuditQuery {
            limit: 5,
            ..Default::default()
        };
        let result1 = engine.execute(query1).unwrap();
        assert_eq!(result1.events.len(), 5);
        assert!(result1.has_more);
        assert!(result1.next_cursor.is_some());

        // Second page
        let query2 = AuditQuery {
            limit: 5,
            cursor: result1.next_cursor,
            ..Default::default()
        };
        let result2 = engine.execute(query2).unwrap();
        assert_eq!(result2.events.len(), 5);
        assert!(result2.has_more);

        // Verify no overlap
        assert_ne!(result1.events[0].event_id, result2.events[0].event_id);
    }

    #[test]
    fn test_execute_query_with_sorting() {
        let engine = QueryEngine::new();
        let events = vec![
            create_test_event("1", "tenant1", "user1", "Z-action"),
            create_test_event("2", "tenant1", "user1", "A-action"),
            create_test_event("3", "tenant1", "user1", "M-action"),
        ];
        engine.add_events(events);

        let query = AuditQuery {
            sort_by: Some(SortField::Action),
            sort_order: Some(SortOrder::Asc),
            limit: 100,
            ..Default::default()
        };

        let result = engine.execute(query).unwrap();
        assert_eq!(result.events.len(), 3);
        assert_eq!(result.events[0].action, "A-action");
        assert_eq!(result.events[1].action, "M-action");
        assert_eq!(result.events[2].action, "Z-action");
    }

    #[test]
    fn test_execute_query_with_limit() {
        let engine = QueryEngine::new();
        let events = (0..100)
            .map(|i| create_test_event(&i.to_string(), "tenant1", "user1", "GET"))
            .collect();
        engine.add_events(events);

        let query = AuditQuery {
            limit: 10,
            ..Default::default()
        };

        let result = engine.execute(query).unwrap();
        assert_eq!(result.events.len(), 10);
        assert_eq!(result.total_count, 100);
        assert!(result.has_more);
    }
}
