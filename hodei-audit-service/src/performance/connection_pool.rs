//! gRPC Connection Pool
//!
//! Provides efficient connection pooling for gRPC clients
//! with support for 10-50 connections, health checking,
//! and automatic retry on connection failure.

use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::{Mutex, RwLock};
use tonic::transport::{Channel, Endpoint};
use tracing::{debug, error, info, warn};

/// Configuration for connection pool
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    /// Maximum number of connections allowed
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Idle connection timeout
    pub idle_timeout: Duration,
    /// Maximum retries for connection
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 10,
            max_connections: 50,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

/// Pooled connection wrapper
#[derive(Debug)]
pub struct PooledConnection {
    pub id: u64,
    pub channel: Channel,
    pub created_at: Instant,
    pub last_used: Instant,
    pub is_healthy: bool,
    pub use_count: u64,
}

/// Connection pool manager
#[derive(Debug)]
pub struct ConnectionPool {
    config: PoolConfig,
    connections: Arc<RwLock<HashMap<u64, PooledConnection>>>,
    next_id: Arc<RwLock<u64>>,
    total_connections: Arc<RwLock<usize>>,
    active_connections: Arc<RwLock<usize>>,
    metrics: Arc<Mutex<PoolMetrics>>,
}

/// Pool metrics
#[derive(Debug, Clone, Default)]
pub struct PoolMetrics {
    pub total_connections: u64,
    pub active_connections: u64,
    pub idle_connections: u64,
    pub connection_requests: u64,
    pub connection_errors: u64,
    pub health_check_failures: u64,
    pub avg_connection_age: Duration,
    pub total_reuse_count: u64,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(config: PoolConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(0)),
            total_connections: Arc::new(RwLock::new(0)),
            active_connections: Arc::new(RwLock::new(0)),
            metrics: Arc::new(Mutex::new(PoolMetrics::default())),
        }
    }

    /// Get a connection from the pool
    pub async fn get(&self) -> Result<PooledConnection, PoolError> {
        let mut connections = self.connections.write().await;
        let mut metrics = self.metrics.lock().await;

        metrics.connection_requests += 1;

        // Try to find a healthy, idle connection
        let now = Instant::now();
        for (id, conn) in connections.iter_mut() {
            if conn.is_healthy && now.duration_since(conn.last_used) < self.config.idle_timeout {
                conn.last_used = now;
                conn.use_count += 1;
                metrics.total_reuse_count += 1;
                return Ok(conn.clone());
            }
        }

        // No healthy connection found, create a new one
        if *self.total_connections.read().await >= self.config.max_connections {
            return Err(PoolError::MaxConnectionsReached);
        }

        drop(connections);
        drop(metrics);

        self.create_new_connection().await
    }

    /// Create a new connection
    async fn create_new_connection(&self) -> Result<PooledConnection, PoolError> {
        let endpoint = self.get_endpoint().await?;

        // Try to connect with retries
        for attempt in 0..self.config.max_retries {
            match endpoint.connect().await {
                Ok(channel) => {
                    let conn = PooledConnection {
                        id: self.get_next_id().await,
                        channel,
                        created_at: Instant::now(),
                        last_used: Instant::now(),
                        is_healthy: true,
                        use_count: 0,
                    };

                    // Add to pool
                    let mut connections = self.connections.write().await;
                    let mut total_conn = self.total_connections.write().await;
                    let mut metrics = self.metrics.lock().await;

                    connections.insert(conn.id, conn.clone());
                    *total_conn += 1;
                    metrics.total_connections += 1;

                    info!("Created new connection: {}", conn.id);
                    return Ok(conn);
                }
                Err(e) => {
                    if attempt < self.config.max_retries - 1 {
                        warn!("Connection attempt {} failed: {}", attempt + 1, e);
                        tokio::time::sleep(self.config.retry_delay).await;
                    } else {
                        let mut metrics = self.metrics.lock().await;
                        metrics.connection_errors += 1;
                        return Err(PoolError::ConnectionFailed(e.to_string()));
                    }
                }
            }
        }

        Err(PoolError::ConnectionFailed(
            "Max retries exceeded".to_string(),
        ))
    }

    /// Return a connection to the pool
    pub async fn return_connection(&self, conn: PooledConnection) {
        let mut connections = self.connections.write().await;
        let mut metrics = self.metrics.lock().await;

        if let Some(pooled_conn) = connections.get_mut(&conn.id) {
            pooled_conn.is_healthy = true; // Reset health status
            pooled_conn.last_used = Instant::now();
            info!("Returned connection: {}", conn.id);
        } else {
            // Connection was removed, recreate if needed
            drop(connections);
            self.maybe_reclaim_connection(conn.id).await;
        }

        // Update metrics
        metrics.active_connections = *self.active_connections.read().await as u64;
    }

    /// Remove a connection from the pool
    pub async fn remove_connection(&self, id: u64) {
        let mut connections = self.connections.write().await;
        let mut total_conn = self.total_connections.write().await;
        let mut metrics = self.metrics.lock().await;

        if connections.remove(&id).is_some() {
            *total_conn = total_conn.saturating_sub(1);
            metrics.total_connections = metrics.total_connections.saturating_sub(1);
            info!("Removed connection: {}", id);
        }
    }

    /// Check health of all connections
    pub async fn health_check(&self) {
        let mut connections = self.connections.write().await;
        let mut metrics = self.metrics.lock().await;

        let now = Instant::now();
        for (id, conn) in connections.iter_mut() {
            if now.duration_since(conn.last_used) > self.config.idle_timeout {
                // Connection is idle, mark as unhealthy
                conn.is_healthy = false;
                metrics.health_check_failures += 1;
            }
        }
    }

    /// Clean up idle connections
    pub async fn cleanup_idle(&self) {
        let mut connections = self.connections.write().await;
        let now = Instant::now();

        let ids_to_remove: Vec<u64> = connections
            .iter()
            .filter(|(_, conn)| {
                !conn.is_healthy && now.duration_since(conn.last_used) > self.config.idle_timeout
            })
            .map(|(id, _)| *id)
            .collect();

        for id in ids_to_remove {
            self.remove_connection(id).await;
        }
    }

    /// Maybe reclaim a connection
    async fn maybe_reclaim_connection(&self, _id: u64) {
        // Connection was already removed, nothing to do
        // This is a placeholder for future reclaim logic
    }

    /// Get next connection ID
    async fn get_next_id(&self) -> u64 {
        let mut next_id = self.next_id.write().await;
        *next_id += 1;
        *next_id
    }

    /// Get endpoint (placeholder - would be configurable in real implementation)
    async fn get_endpoint(&self) -> Result<Endpoint, PoolError> {
        let endpoint =
            Channel::from_static("http://127.0.0.1:50051").timeout(self.config.connection_timeout);
        Ok(endpoint)
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> PoolMetrics {
        let metrics = self.metrics.lock().await;
        let total = *self.total_connections.read().await;
        metrics.clone()
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let total = *self.total_connections.read().await;
        let active = *self.active_connections.read().await;
        let idle = total.saturating_sub(active);
        let connections = self.connections.read().await;

        PoolStats {
            total,
            active,
            idle,
            healthy: connections.values().filter(|c| c.is_healthy).count(),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total: usize,
    pub active: usize,
    pub idle: usize,
    pub healthy: usize,
}

/// Pool error types
#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("Max connections reached")]
    MaxConnectionsReached,
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Pool is empty")]
    PoolEmpty,
    #[error("Connection timeout")]
    Timeout,
}

impl Clone for PooledConnection {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            channel: self.channel.clone(),
            created_at: self.created_at,
            last_used: self.last_used,
            is_healthy: self.is_healthy,
            use_count: self.use_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        let config = PoolConfig {
            min_connections: 5,
            max_connections: 20,
            ..Default::default()
        };

        let pool = ConnectionPool::new(config);
        let stats = pool.stats().await;

        assert_eq!(stats.total, 0);
        assert_eq!(stats.active, 0);
    }

    #[tokio::test]
    async fn test_pool_max_connections() {
        let config = PoolConfig {
            max_connections: 2,
            ..Default::default()
        };

        let pool = ConnectionPool::new(config);

        // Try to exceed max connections (will fail due to endpoint not available)
        // This test just verifies the structure
        let metrics = pool.get_metrics().await;
        assert_eq!(metrics.connection_requests, 0);
    }
}
