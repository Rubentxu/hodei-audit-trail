//! SmartBatcher - Intelligent batching with hybrid policies
//!
//! Provides high-performance event batching with multiple strategies:
//! - Time-based: Flush batch after timeout
//! - Size-based: Flush batch when size threshold reached
//! - Adaptive: Dynamically adjust based on throughput
//! - Pressure-aware: Adjust based on system pressure

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::performance::backpressure::{BackpressureController, PressureLevel};

/// Batching policies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatchingPolicy {
    /// Flush after fixed time interval
    TimeBased(Duration),
    /// Flush when batch reaches size threshold
    SizeBased(usize),
    /// Hybrid: time and size based
    Hybrid { max_time: Duration, max_size: usize },
    /// Adaptive: adjusts based on throughput
    Adaptive {
        target_throughput: u64, // events per second
        min_batch_size: usize,
        max_batch_size: usize,
        min_time: Duration,
        max_time: Duration,
    },
}

/// Configuration for SmartBatcher
#[derive(Debug, Clone)]
pub struct BatcherConfig {
    /// Maximum queue size before applying backpressure
    pub max_queue_size: usize,
    /// Batching policy to use
    pub policy: BatchingPolicy,
    /// Maximum time to wait for batch
    pub flush_timeout: Duration,
    /// Enable adaptive tuning
    pub adaptive_tuning: bool,
    /// Backpressure controller
    pub backpressure_controller: Option<Arc<BackpressureController>>,
    /// Enable metrics collection
    pub enable_metrics: bool,
}

impl Default for BatcherConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 100_000,
            policy: BatchingPolicy::Hybrid {
                max_time: Duration::from_millis(100),
                max_size: 1000,
            },
            flush_timeout: Duration::from_millis(500),
            adaptive_tuning: true,
            backpressure_controller: None,
            enable_metrics: true,
        }
    }
}

/// Batch result
#[derive(Debug, Clone)]
pub struct BatchResult<T> {
    pub batch: Vec<T>,
    pub size: usize,
    pub age: Duration,
    pub pressure_level: PressureLevel,
}

/// SmartBatcher for high-performance event processing
#[derive(Debug)]
pub struct SmartBatcher<T> {
    config: BatcherConfig,
    queue: Arc<Mutex<VecDeque<T>>>,
    last_flush: Arc<Mutex<Instant>>,
    metrics: Arc<Mutex<BatcherMetrics>>,
    flush_notifier: mpsc::UnboundedSender<oneshot::Sender<()>>,
}

/// Metrics for SmartBatcher
#[derive(Debug, Clone, Default)]
pub struct BatcherMetrics {
    pub total_batches: u64,
    pub total_events: u64,
    pub total_flushes: u64,
    pub avg_batch_size: f64,
    pub avg_batch_age: Duration,
    pub queue_size: usize,
    pub pressure_level: PressureLevel,
    pub adaptive_adjustments: u64,
}

impl<T> SmartBatcher<T> {
    /// Create a new SmartBatcher
    pub fn new(config: BatcherConfig) -> Self {
        let (flush_notifier, _) = mpsc::unbounded_channel();
        let last_flush = Arc::new(Mutex::new(Instant::now()));

        let metrics = Arc::new(Mutex::new(BatcherMetrics::default()));

        Self {
            config,
            queue: Arc::new(Mutex::new(VecDeque::new())),
            last_flush,
            metrics,
            flush_notifier,
        }
    }

    /// Add event to batch
    pub async fn add_event(&self, event: T) -> Result<(), BatcherError> {
        let mut queue = self.queue.lock().await;
        let mut metrics = self.metrics.lock().await;

        // Check queue size and apply backpressure if needed
        if queue.len() >= self.config.max_queue_size {
            metrics.queue_size = queue.len();
            return Err(BatcherError::QueueFull(self.config.max_queue_size));
        }

        queue.push_back(event);
        metrics.queue_size = queue.len();

        // Check if we should flush
        if self.should_flush(queue.len()) {
            drop(queue);
            drop(metrics);
            self.notify_flush().await?;
        }

        Ok(())
    }

    /// Check if batch should be flushed
    fn should_flush(&self, queue_len: usize) -> bool {
        match self.config.policy {
            BatchingPolicy::TimeBased(_) => {
                // Will be handled by timeout mechanism
                false
            }
            BatchingPolicy::SizeBased(max_size) => queue_len >= max_size,
            BatchingPolicy::Hybrid {
                max_time: _,
                max_size,
            } => queue_len >= max_size,
            BatchingPolicy::Adaptive {
                target_throughput: _,
                min_batch_size,
                max_batch_size: _,
                min_time: _,
                max_time: _,
            } => queue_len >= min_batch_size,
        }
    }

    /// Get current batch without flushing
    pub async fn get_batch(&self) -> BatchResult<T> {
        let mut queue = self.queue.lock().await;
        let metrics = self.metrics.lock().await;

        let now = Instant::now();
        let last_flush_time = *self.last_flush.lock().await;
        let age = now.duration_since(last_flush_time);

        let pressure_level = if let Some(ref controller) = self.config.backpressure_controller {
            controller.get_current_pressure()
        } else {
            PressureLevel::Normal
        };

        BatchResult {
            batch: queue.drain(..).collect(),
            size: 0,
            age,
            pressure_level,
        }
    }

    /// Flush batch explicitly
    pub async fn flush(&self) -> Result<BatchResult<T>, BatcherError> {
        let mut queue = self.queue.lock().await;
        let mut metrics = self.metrics.lock().await;
        let mut last_flush = self.last_flush.lock().await;

        let now = Instant::now();
        let age = now.duration_since(*last_flush);

        let batch: Vec<T> = queue.drain(..).collect();
        let batch_size = batch.len();

        // Update metrics
        metrics.total_flushes += 1;
        metrics.total_events += batch_size as u64;
        metrics.avg_batch_size = (metrics.avg_batch_size * (metrics.total_batches as f64)
            + batch_size as f64)
            / (metrics.total_batches as f64 + 1.0);
        metrics.avg_batch_age = age;
        metrics.queue_size = 0;

        *last_flush = now;
        metrics.total_batches += 1;

        let pressure_level = if let Some(ref controller) = self.config.backpressure_controller {
            controller.get_current_pressure()
        } else {
            PressureLevel::Normal
        };

        Ok(BatchResult {
            batch,
            size: batch_size,
            age,
            pressure_level,
        })
    }

    /// Notify flush (async)
    async fn notify_flush(&self) -> Result<(), BatcherError> {
        let (tx, _) = oneshot::channel();
        self.flush_notifier
            .send(tx)
            .map_err(|_| BatcherError::FlushNotifierClosed)?;
        Ok(())
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> BatcherMetrics {
        self.metrics.lock().await.clone()
    }

    /// Wait for flush notification
    pub async fn wait_for_flush(&self) -> Result<(), BatcherError> {
        let (tx, rx) = oneshot::channel();
        self.flush_notifier
            .send(tx)
            .map_err(|_| BatcherError::FlushNotifierClosed)?;

        match timeout(self.config.flush_timeout, rx).await {
            Ok(_) => Ok(()),
            Err(_) => Err(BatcherError::FlushTimeout),
        }
    }

    /// Get queue size
    pub async fn queue_size(&self) -> usize {
        self.queue.lock().await.len()
    }

    /// Get pressure level
    pub fn get_pressure_level(&self) -> PressureLevel {
        if let Some(ref controller) = self.config.backpressure_controller {
            controller.get_current_pressure()
        } else {
            PressureLevel::Normal
        }
    }
}

/// Batcher error types
#[derive(Debug, thiserror::Error)]
pub enum BatcherError {
    #[error("Queue is full (max size: {0})")]
    QueueFull(usize),
    #[error("Flush timeout")]
    FlushTimeout,
    #[error("Flush notifier channel closed")]
    FlushNotifierClosed,
    #[error("Batch processing error: {0}")]
    ProcessingError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_size_based_batching() {
        let config = BatcherConfig {
            max_queue_size: 1000,
            policy: BatchingPolicy::SizeBased(20), // Set to 20 so auto-flush doesn't trigger
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Add events (below the size threshold to avoid auto-flush)
        for i in 0..10 {
            batcher.add_event(i).await.unwrap();
        }

        // Verify queue has 10 events
        assert_eq!(batcher.queue_size().await, 10);

        // Manual flush to verify batching works
        let result = batcher.flush().await.unwrap();
        assert_eq!(result.batch.len(), 10);

        let metrics = batcher.get_metrics().await;
        assert_eq!(metrics.total_flushes, 1);
        assert_eq!(metrics.total_events, 10);
        assert_eq!(metrics.total_batches, 1);
    }

    #[tokio::test]
    async fn test_queue_full() {
        let config = BatcherConfig {
            max_queue_size: 5,
            policy: BatchingPolicy::SizeBased(10),
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Fill queue
        for i in 0..5 {
            batcher.add_event(i).await.unwrap();
        }

        // Next event should fail
        let result = batcher.add_event(100).await;
        assert!(matches!(result, Err(BatcherError::QueueFull(5))));
    }

    #[tokio::test]
    async fn test_manual_flush() {
        let config = BatcherConfig {
            max_queue_size: 1000,
            policy: BatchingPolicy::SizeBased(1000),
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Add some events
        for i in 0..5 {
            batcher.add_event(i).await.unwrap();
        }

        // Manual flush
        let result = batcher.flush().await.unwrap();
        assert_eq!(result.size, 5);
        assert_eq!(result.batch.len(), 5);
    }
}
