//! Batch processing para eventos de auditoría
//!
//! Este módulo implementa el batching inteligente con flush policies.

use crate::config::AuditSdkConfig;
use crate::error::AuditError;
use crate::models::AuditEvent;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, warn};

/// Queue de eventos con capacidad limitada
#[derive(Debug)]
pub struct BatchQueue {
    /// Eventos en la queue
    events: Arc<Mutex<Vec<AuditEvent>>>,
    /// Configuración
    config: AuditSdkConfig,
    /// Estado del flush timer
    last_flush: Arc<Mutex<Instant>>,
    /// Contador de flushes
    flush_count: Arc<std::sync::atomic::AtomicU64>,
    /// Contador de eventos procesados
    total_events: Arc<std::sync::atomic::AtomicU64>,
    /// Contador de errores
    error_count: Arc<std::sync::atomic::AtomicU64>,
}

impl BatchQueue {
    /// Crear nueva batch queue
    pub fn new(config: AuditSdkConfig) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::with_capacity(config.batch_size * 2))),
            config,
            last_flush: Arc::new(Mutex::new(Instant::now())),
            flush_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            total_events: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// Añadir evento al batch (non-blocking)
    pub fn add_event(&self, event: AuditEvent) -> Result<(), AuditError> {
        let mut events = self.events.lock().map_err(|_| {
            AuditError::ConfigurationError("Failed to acquire batch queue lock".to_string())
        })?;

        // Check backpressure - if queue is full, drop oldest event
        if events.len() >= self.config.batch_size * 2 {
            warn!("Batch queue full, dropping oldest event");
            events.remove(0);
        }

        events.push(event);
        self.total_events
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Check if we should flush based on size
        if events.len() >= self.config.batch_size {
            let events_to_send = events.split_off(0);
            drop(events); // Release lock before async operation

            // Spawn async task to flush
            let count = events_to_send.len();
            tokio::spawn(async move {
                // Simulate gRPC call
                tokio::time::sleep(Duration::from_millis(10)).await;
                debug!("Flushed {} events", count);
            });
        }

        Ok(())
    }

    /// Flush manual del batch
    pub async fn flush(&self) -> Result<(), AuditError> {
        let events = {
            let mut events = self.events.lock().map_err(|_| {
                AuditError::ConfigurationError("Failed to acquire batch queue lock".to_string())
            })?;

            if events.is_empty() {
                return Ok(());
            }

            events.split_off(0)
        };

        self.flush_batch(events).await;
        Ok(())
    }

    /// Flush batch de eventos
    async fn flush_batch(&self, events: Vec<AuditEvent>) {
        if events.is_empty() {
            return;
        }

        let event_count = events.len();
        self.flush_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Update last flush time
        {
            let mut last_flush = self.last_flush.lock().unwrap();
            *last_flush = Instant::now();
        }

        debug!("Flushing batch of {} events", event_count);

        // In a full implementation, this would send via gRPC
        // For now, we simulate the operation
        tokio::spawn(async move {
            // Simulate gRPC call
            tokio::time::sleep(Duration::from_millis(10)).await;

            debug!("Successfully sent batch of {} events", event_count);
        });
    }

    /// Obtener estadísticas del batch
    pub fn get_stats(&self) -> BatchStats {
        let queue_size = self.events.lock().map(|e| e.len()).unwrap_or(0);

        BatchStats {
            queue_size,
            total_events: self.total_events.load(std::sync::atomic::Ordering::Relaxed),
            flush_count: self.flush_count.load(std::sync::atomic::Ordering::Relaxed),
            error_count: self.error_count.load(std::sync::atomic::Ordering::Relaxed),
            time_since_last_flush: {
                let last_flush = self.last_flush.lock().unwrap();
                last_flush.elapsed()
            },
        }
    }

    /// Limpiar la queue
    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();

        let mut last_flush = self.last_flush.lock().unwrap();
        *last_flush = Instant::now();
    }
}

/// Estadísticas del batch
#[derive(Debug, Clone)]
pub struct BatchStats {
    pub queue_size: usize,
    pub total_events: u64,
    pub flush_count: u64,
    pub error_count: u64,
    pub time_since_last_flush: Duration,
}

/// Flush policy
#[derive(Debug, Clone)]
pub enum FlushPolicy {
    /// Flush cuando reach tamaño N
    Size(usize),
    /// Flush cada N segundos
    Time(Duration),
    /// Flush cuando cualquiera de las condiciones se cumple
    Hybrid(usize, Duration),
}

impl FlushPolicy {
    /// Verificar si debe hacer flush
    pub fn should_flush(&self, queue: &Vec<AuditEvent>, last_flush: &Instant) -> bool {
        let elapsed = last_flush.elapsed();

        match self {
            Self::Size(n) => queue.len() >= *n,
            Self::Time(d) => elapsed >= *d,
            Self::Hybrid(n, d) => queue.len() >= *n || elapsed >= *d,
        }
    }
}

/// Connection pool para gRPC
#[derive(Debug)]
pub struct GrpcConnectionPool {
    clients: Arc<Mutex<Vec<tonic::transport::Channel>>>,
    max_size: usize,
    min_size: usize,
    url: String,
}

impl GrpcConnectionPool {
    /// Crear nueva connection pool
    pub async fn new(url: String, max_size: usize, min_size: usize) -> Result<Self, AuditError> {
        let mut clients = Vec::new();

        // Pre-warm connections
        for i in 0..min_size {
            // In a full implementation, this would connect to the gRPC server
            // For now, we just log it
            debug!("Pre-warmed connection {}", i + 1);
        }

        Ok(Self {
            clients: Arc::new(Mutex::new(clients)),
            max_size,
            min_size,
            url,
        })
    }

    /// Obtener cliente del pool
    pub async fn get_client(&self) -> Result<tonic::transport::Channel, AuditError> {
        let mut clients = self.clients.lock().map_err(|_| {
            AuditError::ConfigurationError("Failed to acquire client pool lock".to_string())
        })?;

        if let Some(channel) = clients.pop() {
            Ok(channel)
        } else {
            // Create new connection if under max
            if clients.len() < self.max_size {
                // In a full implementation, this would create a new gRPC channel
                // For now, return an error to simulate connection
                Err(AuditError::ConfigurationError(
                    "Connection pool not fully implemented".to_string(),
                ))
            } else {
                Err(AuditError::ConfigurationError(
                    "Connection pool exhausted".to_string(),
                ))
            }
        }
    }

    /// Devolver cliente al pool
    pub fn return_client(&self, channel: tonic::transport::Channel) {
        let mut clients = self.clients.lock().unwrap();
        if clients.len() < self.max_size {
            clients.push(channel);
        }
    }
}

/// Configuración de retry
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Ejecutar con retry y exponential backoff
    pub async fn execute_with_retry<F, T, E>(&self, mut f: F) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> BoxFuture<'static, Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut delay = self.initial_delay;

        for attempt in 0..self.max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(error) if attempt < self.max_retries - 1 => {
                    warn!("Retry {} failed: {}", attempt + 1, error);
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(delay * self.multiplier as u32, self.max_delay);
                }
                Err(error) => return Err(RetryError::MaxRetriesExceeded(error)),
            }
        }

        unreachable!()
    }
}

/// Error de retry
#[derive(Debug, thiserror::Error)]
pub enum RetryError<E> {
    #[error("Max retries exceeded: {0}")]
    MaxRetriesExceeded(E),
}

// Helper for async function type
type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_batch_queue_creation() {
        let config = AuditSdkConfig::builder().batch_size(10).build().unwrap();

        let queue = BatchQueue::new(config);
        let stats = queue.get_stats();

        assert_eq!(stats.queue_size, 0);
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.flush_count, 0);
        assert_eq!(stats.error_count, 0);
    }

    #[test]
    fn test_flush_policy_size() {
        let policy = FlushPolicy::Size(5);
        let events = vec![AuditEvent::default(); 5];
        let last_flush = Instant::now() - Duration::from_millis(10);

        assert!(policy.should_flush(&events, &last_flush));

        let small_events = vec![AuditEvent::default(); 3];
        assert!(!policy.should_flush(&small_events, &last_flush));
    }

    #[test]
    fn test_flush_policy_time() {
        let policy = FlushPolicy::Time(Duration::from_millis(100));
        let events = vec![AuditEvent::default(); 2];
        let last_flush = Instant::now() - Duration::from_millis(150);

        assert!(policy.should_flush(&events, &last_flush));

        let recent_flush = Instant::now() - Duration::from_millis(50);
        assert!(!policy.should_flush(&events, &recent_flush));
    }

    #[test]
    fn test_flush_policy_hybrid() {
        let policy = FlushPolicy::Hybrid(5, Duration::from_millis(100));
        let last_flush = Instant::now() - Duration::from_millis(10);

        // Should flush because of size
        let events = vec![AuditEvent::default(); 5];
        assert!(policy.should_flush(&events, &last_flush));

        // Should flush because of time
        let last_flush_old = Instant::now() - Duration::from_millis(150);
        let small_events = vec![AuditEvent::default(); 2];
        assert!(policy.should_flush(&small_events, &last_flush_old));
    }

    #[test]
    fn test_retry_config() {
        let config = RetryConfig::default();

        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(100));
        assert!(config.max_delay > config.initial_delay);
    }
}
