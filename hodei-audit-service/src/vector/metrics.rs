//! Vector metrics and observability
//!
//! This module provides utilities for monitoring Vector.dev metrics,
//! including Prometheus integration, health checks, and alerting.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

#[cfg(feature = "vector-metrics")]
use reqwest;

use crate::vector::{VectorError, VectorResult};

/// Vector metrics and health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetrics {
    /// Buffer size in bytes
    pub buffer_size_bytes: u64,
    /// Total events received
    pub events_in_total: u64,
    /// Total events sent
    pub events_out_total: u64,
    /// Total errors
    pub errors_total: u64,
    /// Events processed per second
    pub processing_rate: f64,
    /// Timestamp of metrics
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Per-sink metrics
    pub sinks: HashMap<String, VectorSinkMetrics>,
    /// Overall health status
    pub health_status: VectorHealthStatus,
}

/// Metrics for individual sinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSinkMetrics {
    /// Sink name
    pub name: String,
    /// Total events sent to this sink
    pub events_sent: u64,
    /// Events failed
    pub events_failed: u64,
    /// Average request duration
    pub avg_request_duration_ms: f64,
    /// Error rate
    pub error_rate: f64,
    /// Sink status
    pub status: String,
}

/// Health status of Vector
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorHealthStatus {
    /// Vector is healthy
    Healthy,
    /// Vector is degraded
    Degraded,
    /// Vector is unhealthy
    Unhealthy,
    /// Status unknown
    Unknown,
}

impl std::fmt::Display for VectorHealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorHealthStatus::Healthy => write!(f, "healthy"),
            VectorHealthStatus::Degraded => write!(f, "degraded"),
            VectorHealthStatus::Unhealthy => write!(f, "unhealthy"),
            VectorHealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Vector metrics collector
#[derive(Debug, Clone)]
pub struct VectorMetricsCollector {
    /// Vector metrics endpoint
    endpoint: String,
    /// HTTP client
    client: reqwest::Client,
    /// Health check interval
    health_check_interval: Duration,
    /// Last metrics snapshot
    last_metrics: Option<VectorMetrics>,
    /// Metrics collection start time
    start_time: Instant,
}

impl VectorMetricsCollector {
    /// Create a new metrics collector
    pub fn new(endpoint: String) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            endpoint,
            client,
            health_check_interval: Duration::from_secs(15),
            last_metrics: None,
            start_time: Instant::now(),
        }
    }

    /// Get Vector metrics from Prometheus endpoint
    pub async fn get_metrics(&self) -> VectorResult<VectorMetrics> {
        let url = format!("{}/metrics", self.endpoint);

        let response = self.client.get(&url).send().await.map_err(|e| {
            error!(
                error = e.to_string(),
                endpoint = url,
                "Failed to fetch Vector metrics"
            );
            VectorError::ConnectionFailed(format!("Failed to fetch metrics: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(VectorError::ConnectionFailed(format!(
                "HTTP {} when fetching metrics",
                response.status()
            )));
        }

        let text = response.text().await.map_err(|e| {
            VectorError::Deserialization(format!("Failed to read metrics response: {}", e))
        })?;

        self.parse_prometheus_metrics(&text)
    }

    /// Parse Prometheus metrics format
    fn parse_prometheus_metrics(&self, text: &str) -> VectorResult<VectorMetrics> {
        let mut buffer_size = 0u64;
        let mut events_in = 0u64;
        let mut events_out = 0u64;
        let mut errors = 0u64;
        let mut sinks = HashMap::new();

        for line in text.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse Vector-specific metrics
            if line.starts_with("vector_buffer_size_bytes") {
                if let Some(val) = self.extract_metric_value(line) {
                    buffer_size = val as u64;
                }
            } else if line.starts_with("vector_events_in_total") {
                if let Some(val) = self.extract_metric_value(line) {
                    events_in = val as u64;
                }
            } else if line.starts_with("vector_events_out_total") {
                if let Some(val) = self.extract_metric_value(line) {
                    events_out = val as u64;
                }
            } else if line.starts_with("vector_component_errors_total") {
                if let Some(val) = self.extract_metric_value(line) {
                    errors = val as u64;
                }
            } else if line.starts_with("vector_sink_sent_events_total") {
                if let Some((sink_name, value)) = self.extract_labeled_metric(line, "sink") {
                    let metrics = sinks.entry(sink_name).or_insert_with(|| VectorSinkMetrics {
                        name: sink_name.clone(),
                        events_sent: 0,
                        events_failed: 0,
                        avg_request_duration_ms: 0.0,
                        error_rate: 0.0,
                        status: "unknown".to_string(),
                    });
                    metrics.events_sent = value as u64;
                }
            } else if line.starts_with("vector_sink_request_duration_seconds") {
                if let Some((sink_name, _)) = self.extract_labeled_metric(line, "sink") {
                    // Track for avg duration calculation
                    if let Some(val) = self.extract_metric_value(line) {
                        let metrics = sinks.entry(sink_name).or_insert_with(|| VectorSinkMetrics {
                            name: sink_name.clone(),
                            events_sent: 0,
                            events_failed: 0,
                            avg_request_duration_ms: 0.0,
                            error_rate: 0.0,
                            status: "unknown".to_string(),
                        });
                        metrics.avg_request_duration_ms = val * 1000.0; // Convert to ms
                    }
                }
            }
        }

        // Calculate processing rate
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let processing_rate = if elapsed > 0.0 {
            (events_in as f64) / elapsed
        } else {
            0.0
        };

        // Determine health status
        let health_status = if events_in > 0 && errors as f64 / events_in as f64 > 0.1 {
            VectorHealthStatus::Unhealthy
        } else if errors > 0 || buffer_size > 1_000_000 {
            VectorHealthStatus::Degraded
        } else {
            VectorHealthStatus::Healthy
        };

        let metrics = VectorMetrics {
            buffer_size_bytes: buffer_size,
            events_in_total: events_in,
            events_out_total: events_out,
            errors_total: errors,
            processing_rate,
            timestamp: chrono::Utc::now(),
            sinks,
            health_status,
        };

        self.last_metrics = Some(metrics.clone());

        Ok(metrics)
    }

    /// Extract numeric value from a Prometheus metric line
    fn extract_metric_value(&self, line: &str) -> Option<f64> {
        // Format: metric_name{labels...} value timestamp
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            parts[1].parse::<f64>().ok()
        } else {
            None
        }
    }

    /// Extract labeled metric with specific label
    fn extract_labeled_metric(&self, line: &str, label_name: &str) -> Option<(String, f64)> {
        // Format: metric_name{label="value"...} value
        if let Some(brace_start) = line.find('{') {
            if let Some(brace_end) = line.find('}') {
                let labels = &line[brace_start + 1..brace_end];
                let label_pair = labels
                    .split(',')
                    .find(|p| p.starts_with(&format!("{}=", label_name)));

                if let Some(pair) = label_pair {
                    if let Some(eq_pos) = pair.find('"') {
                        let value_start = eq_pos + 1;
                        if let Some(value_end) = pair[value_start..].find('"') {
                            let value = &pair[value_start..value_start + value_end];
                            // Also extract metric value
                            if let Some(val) = self.extract_metric_value(line) {
                                return Some((value.to_string(), val));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Get the last metrics snapshot (cached)
    pub fn get_last_metrics(&self) -> Option<&VectorMetrics> {
        self.last_metrics.as_ref()
    }

    /// Check Vector health
    pub async fn check_health(&self) -> VectorResult<VectorHealthStatus> {
        let url = format!("{}/health", self.endpoint);

        let response =
            self.client.get(&url).send().await.map_err(|e| {
                VectorError::ConnectionFailed(format!("Health check failed: {}", e))
            })?;

        if !response.status().is_success() {
            return Ok(VectorHealthStatus::Unhealthy);
        }

        let health: HashMap<String, serde_json::Value> = response
            .json()
            .await
            .map_err(|e| VectorError::Deserialization(format!("Failed to parse health: {}", e)))?;

        // Check if Vector reports healthy
        if health.get("healthy").and_then(|v| v.as_bool()) == Some(true) {
            Ok(VectorHealthStatus::Healthy)
        } else {
            Ok(VectorHealthStatus::Degraded)
        }
    }

    /// Monitor Vector with periodic checks
    pub async fn monitor(&self, duration: Duration) -> VectorResult<Vec<VectorMetrics>> {
        info!(
            duration = duration.as_secs(),
            endpoint = self.endpoint,
            "Starting Vector metrics monitoring"
        );

        let mut snapshots = Vec::new();
        let start = Instant::now();

        while start.elapsed() < duration {
            match self.get_metrics().await {
                Ok(metrics) => {
                    info!(
                        events_in = metrics.events_in_total,
                        events_out = metrics.events_out_total,
                        errors = metrics.errors_total,
                        buffer_size = metrics.buffer_size_bytes,
                        health = %metrics.health_status,
                        "Vector metrics snapshot"
                    );
                    snapshots.push(metrics);
                }
                Err(e) => {
                    warn!(error = e.to_string(), "Failed to collect Vector metrics");
                }
            }

            tokio::time::sleep(self.health_check_interval).await;
        }

        info!(
            snapshots = snapshots.len(),
            "Completed Vector metrics monitoring"
        );

        Ok(snapshots)
    }

    /// Get summary of last metrics
    pub async fn get_summary(&self) -> VectorResult<VectorMetricsSummary> {
        let metrics = self.get_metrics().await?;

        Ok(VectorMetricsSummary {
            total_events: metrics.events_in_total,
            success_rate: if metrics.events_in_total > 0 {
                ((metrics.events_in_total - metrics.errors_total) as f64
                    / metrics.events_in_total as f64)
                    * 100.0
            } else {
                100.0
            },
            error_rate: if metrics.events_in_total > 0 {
                (metrics.errors_total as f64 / metrics.events_in_total as f64) * 100.0
            } else {
                0.0
            },
            buffer_usage_pct: if metrics.buffer_size_bytes > 0 {
                (metrics.buffer_size_bytes as f64 / 5_000_000_000.0) * 100.0
            } else {
                0.0
            },
            health_status: metrics.health_status,
            sink_count: metrics.sinks.len(),
            processing_rate: metrics.processing_rate,
        })
    }
}

/// Summary of Vector metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetricsSummary {
    pub total_events: u64,
    pub success_rate: f64,
    pub error_rate: f64,
    pub buffer_usage_pct: f64,
    pub health_status: VectorHealthStatus,
    pub sink_count: usize,
    pub processing_rate: f64,
}

impl std::fmt::Display for VectorMetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vector: {} events, {:.2}% success, {:.2}% errors, {:.2}% buffer, {} sinks, status: {}",
            self.total_events,
            self.success_rate,
            self.error_rate,
            self.buffer_usage_pct,
            self.sink_count,
            self.health_status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_health_status_display() {
        assert_eq!(format!("{}", VectorHealthStatus::Healthy), "healthy");
        assert_eq!(format!("{}", VectorHealthStatus::Degraded), "degraded");
        assert_eq!(format!("{}", VectorHealthStatus::Unhealthy), "unhealthy");
        assert_eq!(format!("{}", VectorHealthStatus::Unknown), "unknown");
    }

    #[test]
    fn test_vector_metrics_summary() {
        let summary = VectorMetricsSummary {
            total_events: 1000,
            success_rate: 99.0,
            error_rate: 1.0,
            buffer_usage_pct: 50.0,
            health_status: VectorHealthStatus::Healthy,
            sink_count: 3,
            processing_rate: 100.0,
        };

        let display = format!("{}", summary);
        assert!(display.contains("1000 events"));
        assert!(display.contains("99"));
        assert!(display.contains("healthy"));
    }

    #[test]
    fn test_extract_metric_value() {
        let collector = VectorMetricsCollector::new("http://localhost:9598".to_string());
        let line = "vector_events_in_total 1234.5 1638360000";
        let value = collector.extract_metric_value(line);
        assert_eq!(value, Some(1234.5));
    }
}
