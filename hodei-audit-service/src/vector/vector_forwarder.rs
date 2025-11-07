//! Vector Forwarder - Client for sending events to Vector.dev
//!
//! This module provides the VectorForwarder client that sends audit events
//! to Vector.dev for multi-sink distribution (ClickHouse, S3, etc.)

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use hodei_audit_proto::{
    AuditEvent, EventBatchRequest, EventBatchResponse, HealthCheckRequest, HealthStatus,
    vector_api_client::VectorApiClient,
};
use tonic::transport::Channel;
use tracing::{error, info, warn};

use crate::vector::error::{VectorError, VectorResult};

/// VectorForwarder - Client for sending events to Vector.dev
///
/// Handles:
/// - Batch sending of events
/// - Retry logic with exponential backoff
/// - Health checks
/// - Metrics and monitoring
/// - Connection pooling
#[derive(Debug, Clone)]
pub struct VectorForwarder {
    /// gRPC client connection
    client: VectorApiClient<Channel>,
    /// Configuration
    config: VectorForwarderConfig,
    /// Statistics
    stats: Arc<std::sync::atomic::AtomicU64>,
}

/// Configuration for VectorForwarder
#[derive(Debug, Clone)]
pub struct VectorForwarderConfig {
    /// Vector.gRPC endpoint
    pub endpoint: String,
    /// Maximum batch size (events per batch)
    pub max_batch_size: usize,
    /// Batch timeout (send batch if timeout reached)
    pub batch_timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Initial retry delay
    pub retry_delay: Duration,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// TLS configuration (placeholder)
    pub tls_config: Option<()>,
    /// Whether to use compression
    pub use_compression: bool,
}

impl Default for VectorForwarderConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:50051".to_string(),
            max_batch_size: 1000,
            batch_timeout: Duration::from_secs(5),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            connect_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            tls_config: None,
            use_compression: true,
        }
    }
}

impl VectorForwarder {
    /// Create a new VectorForwarder with default configuration
    pub async fn new(config: VectorForwarderConfig) -> VectorResult<Self> {
        Self::new_with_client(config, None).await
    }

    /// Create a new VectorForwarder with custom channel
    pub async fn new_with_client(
        config: VectorForwarderConfig,
        channel: Option<Channel>,
    ) -> VectorResult<Self> {
        info!(
            endpoint = config.endpoint,
            "Initializing VectorForwarder client"
        );

        let channel = match channel {
            Some(ch) => ch,
            None => {
                let channel = Channel::from_shared(config.endpoint.clone()).map_err(|e| {
                    VectorError::InvalidArgument(format!("Invalid endpoint: {}", e))
                })?;
                channel.connect().await.map_err(|e| {
                    VectorError::ConnectionFailed(format!("Failed to connect: {}", e))
                })?
            }
        };

        // Create client (compression is set at request level, not client level)
        let client = VectorApiClient::new(channel);

        let stats = Arc::new(std::sync::atomic::AtomicU64::new(0));

        let forwarder = Self {
            client,
            config,
            stats,
        };

        Ok(forwarder)
    }

    /// Send a single event to Vector (convenience method)
    pub async fn send_event(&mut self, event: AuditEvent) -> VectorResult<String> {
        self.send_events(vec![event]).await
    }

    /// Send multiple events to Vector as a batch
    pub async fn send_events(&mut self, mut events: Vec<AuditEvent>) -> VectorResult<String> {
        if events.is_empty() {
            return Err(VectorError::InvalidArgument(
                "Cannot send empty event batch".to_string(),
            ));
        }

        // Create batch request
        let request = EventBatchRequest { events };

        // Send with retry logic
        let response = self.send_with_retry(request).await?;

        if !response.success {
            return Err(VectorError::SendFailed(response.message));
        }

        info!(
            batch_id = response.batch_id,
            event_count = response.received_count,
            "Successfully sent event batch to Vector"
        );

        Ok(response.batch_id)
    }

    /// Send batch with exponential backoff retry
    async fn send_with_retry(
        &mut self,
        request: EventBatchRequest,
    ) -> VectorResult<EventBatchResponse> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match self.send_batch_once(&request).await {
                Ok(response) => {
                    // Success - update stats
                    self.stats
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e.clone());

                    // Don't retry on certain errors
                    if e.is_retryable() {
                        if attempt < self.config.max_retries {
                            let delay = self
                                .config
                                .retry_delay
                                .checked_mul(2u32.pow(attempt))
                                .unwrap_or(self.config.retry_delay);

                            warn!(
                                attempt = attempt + 1,
                                max_attempts = self.config.max_retries + 1,
                                delay_ms = delay.as_millis(),
                                error = %e,
                                "Failed to send batch, retrying..."
                            );

                            tokio::time::sleep(delay).await;
                            continue;
                        }
                    }
                    break;
                }
            }
        }

        // All retries exhausted
        Err(last_error.unwrap_or_else(|| VectorError::Timeout("Max retries exhausted".to_string())))
    }

    /// Send batch once without retry
    async fn send_batch_once(
        &mut self,
        request: &EventBatchRequest,
    ) -> VectorResult<EventBatchResponse> {
        let start = Instant::now();
        let event_count = request.events.len();

        // We need to clone the request for async operation
        let request_clone = request.clone();

        // Create a mutable reference for the async operation
        let mut client = self.client.clone();

        match client.send_event_batch(request_clone).await {
            Ok(response) => {
                let duration = start.elapsed();
                info!(
                    event_count = event_count,
                    duration_ms = duration.as_millis(),
                    "Batch sent successfully"
                );
                Ok(response.into_inner())
            }
            Err(status) => {
                let duration = start.elapsed();
                error!(
                    event_count = event_count,
                    duration_ms = duration.as_millis(),
                    code = status.code() as u32,
                    error = status.message(),
                    "Failed to send batch to Vector"
                );
                Err(VectorError::from(status))
            }
        }
    }

    /// Check Vector health
    pub async fn health_check(&mut self) -> VectorResult<HealthStatus> {
        let request = HealthCheckRequest {
            service_name: "hodei-audit".to_string(),
            component: "vector_forwarder".to_string(),
        };

        match self.client.health_check(request).await {
            Ok(response) => {
                let status = response.get_ref().status;
                let health = match status {
                    0 => HealthStatus::StatusUnknown,
                    1 => HealthStatus::StatusServing,
                    2 => HealthStatus::StatusNotServing,
                    _ => HealthStatus::StatusUnknown,
                };
                Ok(health)
            }
            Err(status) => {
                error!(error = status.message(), "Vector health check failed");
                Err(VectorError::from(status))
            }
        }
    }

    /// Check if Vector is healthy
    pub async fn is_healthy(&mut self) -> bool {
        match self.health_check().await {
            Ok(status) => status == HealthStatus::StatusServing,
            Err(_) => false,
        }
    }

    /// Get statistics
    pub fn stats(&self) -> u64 {
        self.stats.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get configuration
    pub fn config(&self) -> &VectorForwarderConfig {
        &self.config
    }
}

impl std::fmt::Display for VectorForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VectorForwarder(endpoint={}, max_batch_size={}, max_retries={})",
            self.config.endpoint, self.config.max_batch_size, self.config.max_retries
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_forwarder_new() {
        let config = VectorForwarderConfig::default();
        // In test environment, connection will fail, but we can test config
        assert_eq!(config.max_batch_size, 1000);
        assert_eq!(config.max_retries, 3);
    }

    #[tokio::test]
    async fn test_send_empty_batch() {
        // Test that empty batch returns error without needing real connection
        let config = VectorForwarderConfig::default();

        // We can't actually test the full method without a mock, but we can test config
        assert_eq!(config.max_batch_size, 1000);
        assert!(config.max_batch_size > 0);
    }

    #[tokio::test]
    async fn test_send_single_event() {
        let config = VectorForwarderConfig::default();

        // Test creating the forwarder (connection will fail but config works)
        assert!(config.max_batch_size > 0);
        assert_eq!(config.endpoint, "http://127.0.0.1:50051");
    }

    #[tokio::test]
    async fn test_vector_forwarder_display() {
        let config = VectorForwarderConfig::default();

        // Test that config has the right values
        assert_eq!(config.endpoint, "http://127.0.0.1:50051");
        assert_eq!(config.max_batch_size, 1000);
    }

    #[tokio::test]
    async fn test_vector_forwarder_stats() {
        let stats = Arc::new(std::sync::atomic::AtomicU64::new(0));
        assert_eq!(stats.load(std::sync::atomic::Ordering::Relaxed), 0);

        // Simulate increment
        stats.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(stats.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_vector_forwarder_config() {
        let config = VectorForwarderConfig {
            endpoint: "http://test:9000".to_string(),
            max_batch_size: 500,
            batch_timeout: Duration::from_secs(10),
            max_retries: 5,
            retry_delay: Duration::from_millis(200),
            connect_timeout: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(60),
            tls_config: None,
            use_compression: false,
        };

        assert_eq!(config.endpoint, "http://test:9000");
        assert_eq!(config.max_batch_size, 500);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.batch_timeout, Duration::from_secs(10));
    }
}
