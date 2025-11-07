//! Metrics collection for Prometheus integration
//!
//! This module provides comprehensive metrics for the Hodei Audit Service:
//! - Event counters (received, published, failed)
//! - Batch size histograms
//! - Processing latency measurements
//! - Query duration tracking
//! - Active connections gauge

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Metric labels for event metrics
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EventLabels {
    pub event_type: String,
    pub tenant_id: String,
    pub status: String,
}

/// Metric labels for batch metrics
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BatchLabels {
    pub tenant_id: String,
    pub batch_type: String,
}

/// Metric labels for query metrics
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct QueryLabels {
    pub query_type: String,
    pub tenant_id: String,
    pub status: String,
}

/// Event counters
#[derive(Debug, Default, Clone)]
pub struct EventCounters {
    pub received: u64,
    pub published: u64,
    pub failed: u64,
}

/// Query metrics
#[derive(Debug, Default, Clone)]
pub struct QueryMetrics {
    pub total_duration_ms: u64,
    pub count: u64,
}

/// AuditMetrics provides comprehensive metrics collection
#[derive(Debug, Clone, Default)]
pub struct AuditMetrics {
    /// Event counters by type, tenant, and status
    pub events: HashMap<EventLabels, EventCounters>,
    /// Batch size metrics
    pub batch_sizes: HashMap<BatchLabels, Vec<usize>>,
    /// Processing latency samples
    pub processing_latencies: Vec<f64>,
    /// Query duration metrics
    pub query_durations: HashMap<QueryLabels, QueryMetrics>,
    /// Active connections count
    pub active_connections: u64,
    /// Total events processed
    pub total_events: u64,
    /// Total batches processed
    pub total_batches: u64,
    /// Total errors
    pub total_errors: u64,
    /// Latency samples for aggregation
    pub latency_samples: Vec<f64>,
}

impl AuditMetrics {
    /// Create a new AuditMetrics instance
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            batch_sizes: HashMap::new(),
            processing_latencies: Vec::new(),
            query_durations: HashMap::new(),
            active_connections: 0,
            total_events: 0,
            total_batches: 0,
            total_errors: 0,
            latency_samples: Vec::new(),
        }
    }

    /// Increment event counter
    pub fn increment_event(&mut self, event_type: &str, tenant_id: &str, status: &str) {
        let labels = EventLabels {
            event_type: event_type.to_string(),
            tenant_id: tenant_id.to_string(),
            status: status.to_string(),
        };

        let counters = self
            .events
            .entry(labels)
            .or_insert_with(EventCounters::default);

        match status {
            "received" => {
                counters.received += 1;
                self.total_events += 1;
            }
            "published" => {
                counters.published += 1;
            }
            "failed" => {
                counters.failed += 1;
                self.total_errors += 1;
            }
            _ => {}
        }
    }

    /// Record batch size
    pub fn record_batch_size(&mut self, size: usize, tenant_id: &str, batch_type: &str) {
        let labels = BatchLabels {
            tenant_id: tenant_id.to_string(),
            batch_type: batch_type.to_string(),
        };

        self.batch_sizes
            .entry(labels)
            .or_insert_with(Vec::new)
            .push(size);
        self.total_batches += 1;
    }

    /// Record processing latency
    pub fn record_processing_latency(&mut self, latency: std::time::Duration) {
        self.processing_latencies.push(latency.as_secs_f64());
        self.latency_samples.push(latency.as_secs_f64());
    }

    /// Record query duration
    pub fn record_query_duration(
        &mut self,
        query_type: &str,
        tenant_id: &str,
        status: &str,
        duration: std::time::Duration,
    ) {
        let labels = QueryLabels {
            query_type: query_type.to_string(),
            tenant_id: tenant_id.to_string(),
            status: status.to_string(),
        };

        let metrics = self
            .query_durations
            .entry(labels)
            .or_insert_with(QueryMetrics::default);

        metrics.count += 1;
        metrics.total_duration_ms += duration.as_millis() as u64;
    }

    /// Update active connections gauge
    pub fn set_active_connections(&mut self, count: u64) {
        self.active_connections = count;
    }

    /// Increment active connections
    pub fn increment_active_connections(&mut self) {
        self.active_connections += 1;
    }

    /// Decrement active connections
    pub fn decrement_active_connections(&mut self) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }

    /// Get average processing latency in seconds
    pub fn get_average_processing_latency(&self) -> Option<f64> {
        if self.processing_latencies.is_empty() {
            None
        } else {
            let sum: f64 = self.processing_latencies.iter().sum();
            Some(sum / self.processing_latencies.len() as f64)
        }
    }

    /// Get average query duration in milliseconds
    pub fn get_average_query_duration(&self, query_type: &str) -> Option<f64> {
        let mut total_duration = 0u64;
        let mut count = 0u64;

        for (labels, metrics) in &self.query_durations {
            if labels.query_type == query_type {
                total_duration += metrics.total_duration_ms;
                count += metrics.count;
            }
        }

        if count > 0 {
            Some(total_duration as f64 / count as f64)
        } else {
            None
        }
    }

    /// Get events per second (simplified)
    pub fn get_events_per_second(&self) -> f64 {
        if self.processing_latencies.is_empty() {
            0.0
        } else {
            let total_time = self.processing_latencies.last().unwrap_or(&0.0)
                - self.processing_latencies.first().unwrap_or(&0.0);
            if total_time > 0.0 {
                self.total_events as f64 / total_time
            } else {
                0.0
            }
        }
    }
}

/// Create a new metrics instance
pub fn create_metrics() -> Arc<RwLock<AuditMetrics>> {
    Arc::new(RwLock::new(AuditMetrics::new()))
}

/// Get global metrics instance (convenience function - creates new instance)
pub fn get_metrics() -> Arc<RwLock<AuditMetrics>> {
    create_metrics()
}

/// Register metrics with Prometheus registry (simplified stub)
pub fn register_metrics(
    _registry: &mut prometheus_client::registry::Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    // In a real implementation, this would register metrics with Prometheus
    // For now, we use internal metrics storage
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_increment_event_received() {
        let mut metrics = AuditMetrics::new();
        metrics.increment_event("test_event", "tenant_1", "received");

        let labels = EventLabels {
            event_type: "test_event".to_string(),
            tenant_id: "tenant_1".to_string(),
            status: "received".to_string(),
        };

        let counters = metrics.events.get(&labels);
        assert!(counters.is_some());
        assert_eq!(counters.unwrap().received, 1);
    }

    #[tokio::test]
    async fn test_record_batch_size() {
        let mut metrics = AuditMetrics::new();
        metrics.record_batch_size(1024, "tenant_1", "event_batch");

        let labels = BatchLabels {
            tenant_id: "tenant_1".to_string(),
            batch_type: "event_batch".to_string(),
        };

        let sizes = metrics.batch_sizes.get(&labels);
        assert!(sizes.is_some());
        assert_eq!(sizes.unwrap()[0], 1024);
    }

    #[tokio::test]
    async fn test_record_processing_latency() {
        let mut metrics = AuditMetrics::new();
        let start = Instant::now();
        // Simulate some work
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let latency = start.elapsed();

        metrics.record_processing_latency(latency);
        assert_eq!(metrics.processing_latencies.len(), 1);
    }

    #[tokio::test]
    async fn test_record_query_duration() {
        let mut metrics = AuditMetrics::new();
        let duration = std::time::Duration::from_millis(50);

        metrics.record_query_duration("select", "tenant_1", "success", duration);

        let labels = QueryLabels {
            query_type: "select".to_string(),
            tenant_id: "tenant_1".to_string(),
            status: "success".to_string(),
        };

        let query_metrics = metrics.query_durations.get(&labels);
        assert!(query_metrics.is_some());
        assert_eq!(query_metrics.unwrap().count, 1);
    }

    #[tokio::test]
    async fn test_active_connections() {
        let mut metrics = AuditMetrics::new();

        metrics.set_active_connections(10);
        assert_eq!(metrics.active_connections, 10);

        metrics.increment_active_connections();
        assert_eq!(metrics.active_connections, 11);

        metrics.decrement_active_connections();
        assert_eq!(metrics.active_connections, 10);
    }

    #[tokio::test]
    async fn test_global_metrics() {
        let metrics = get_metrics();
        let mut metrics = metrics.write().await;

        metrics.increment_event("test", "tenant_1", "received");

        let labels = EventLabels {
            event_type: "test".to_string(),
            tenant_id: "tenant_1".to_string(),
            status: "received".to_string(),
        };

        assert!(metrics.events.get(&labels).is_some());
    }

    #[tokio::test]
    async fn test_average_processing_latency() {
        let mut metrics = AuditMetrics::new();

        metrics.record_processing_latency(std::time::Duration::from_millis(10));
        metrics.record_processing_latency(std::time::Duration::from_millis(20));
        metrics.record_processing_latency(std::time::Duration::from_millis(30));

        let avg = metrics.get_average_processing_latency();
        assert!(avg.is_some());
        assert!((avg.unwrap() - 0.02).abs() < 0.001);
    }
}
