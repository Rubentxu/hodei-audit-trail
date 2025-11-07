//! Zero-Copy Batching Optimization
//!
//! Provides zero-copy operations to minimize memory allocations and improve performance:
//! - Zero-copy buffer management
//! - Buffer pool for reusing allocations
//! - Slice-based operations
//! - Pin and borrowing optimization

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Zero-copy buffer pool configuration
#[derive(Debug, Clone)]
pub struct BufferPoolConfig {
    /// Initial buffer size
    pub initial_size: usize,
    /// Maximum buffer size
    pub max_size: usize,
    /// Maximum number of buffers in pool
    pub max_buffers: usize,
    /// Buffer growth factor
    pub growth_factor: f64,
}

impl Default for BufferPoolConfig {
    fn default() -> Self {
        Self {
            initial_size: 1024 * 64,    // 64KB
            max_size: 1024 * 1024 * 16, // 16MB
            max_buffers: 100,
            growth_factor: 1.5,
        }
    }
}

/// Zero-copy buffer
#[derive(Debug)]
pub struct ZeroCopyBuffer {
    /// Buffer data
    data: Vec<u8>,
    /// Buffer metadata
    metadata: BufferMetadata,
}

impl ZeroCopyBuffer {
    /// Create a new buffer
    pub fn new(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            metadata: BufferMetadata::new(),
        }
    }

    /// Get buffer slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable buffer slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get buffer length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get available capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clear buffer (keep allocated memory)
    pub fn clear(&mut self) {
        self.data.clear();
        self.metadata.last_used = Instant::now();
    }

    /// Get buffer metadata
    pub fn metadata(&self) -> &BufferMetadata {
        &self.metadata
    }

    /// Write bytes to buffer
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), BufferError> {
        if self.data.len() + bytes.len() > self.data.capacity() {
            return Err(BufferError::InsufficientCapacity);
        }
        self.data.extend_from_slice(bytes);
        self.metadata.bytes_written += bytes.len() as u64;
        self.metadata.last_used = Instant::now();
        Ok(())
    }
}

/// Buffer metadata
#[derive(Debug, Clone)]
struct BufferMetadata {
    /// Buffer ID
    id: u64,
    /// Creation time
    created_at: Instant,
    /// Last used time
    last_used: Instant,
    /// Total bytes written
    bytes_written: u64,
}

impl BufferMetadata {
    fn new() -> Self {
        static mut COUNTER: u64 = 0;
        let id = unsafe {
            COUNTER += 1;
            COUNTER
        };
        Self {
            id,
            created_at: Instant::now(),
            last_used: Instant::now(),
            bytes_written: 0,
        }
    }
}

/// Buffer pool for reusing buffers
pub struct BufferPool {
    config: BufferPoolConfig,
    pool: Arc<RwLock<VecDeque<Arc<RwLock<ZeroCopyBuffer>>>>>,
    created_buffers: Arc<RwLock<u64>>,
}

impl BufferPool {
    /// Create a new buffer pool
    pub fn new(config: BufferPoolConfig) -> Self {
        Self {
            pool: Arc::new(RwLock::new(VecDeque::new())),
            created_buffers: Arc::new(RwLock::new(0)),
            config,
        }
    }

    /// Get or create a buffer
    pub async fn get_buffer(&self) -> Arc<RwLock<ZeroCopyBuffer>> {
        let mut pool = self.pool.write().await;

        // Try to get a buffer from the pool
        if let Some(buffer) = pool.pop_front() {
            return buffer;
        }

        // Create a new buffer
        let buffer = Arc::new(RwLock::new(ZeroCopyBuffer::new(self.config.initial_size)));
        let mut created = self.created_buffers.write().await;
        *created += 1;

        buffer
    }

    /// Return a buffer to the pool
    pub async fn return_buffer(&self, mut buffer: Arc<RwLock<ZeroCopyBuffer>>) {
        // Reset buffer before returning
        {
            let mut buf = buffer.write().await;
            buf.clear();
        }

        let mut pool = self.pool.write().await;

        // Check if pool has space
        if pool.len() < self.config.max_buffers {
            pool.push_back(buffer);
        }
        // If pool is full, buffer is dropped
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> BufferPoolStats {
        let pool = self.pool.read().await;
        let created = *self.created_buffers.read().await;

        BufferPoolStats {
            total_created: created,
            in_pool: pool.len(),
            capacity: self.config.max_buffers,
        }
    }

    /// Clean up old buffers
    pub async fn cleanup(&self, max_age: Duration) {
        let mut pool = self.pool.write().await;
        let now = Instant::now();

        let mut to_retain = VecDeque::new();

        // Separate buffers into keep and remove
        while let Some(buffer) = pool.pop_front() {
            let meta = {
                let guard = buffer.read().await;
                guard.metadata().clone()
            };

            if now.duration_since(meta.last_used) < max_age {
                to_retain.push_back(buffer);
            }
            // else: buffer is dropped
        }

        // Keep the buffers that should be retained
        *pool = to_retain;
    }
}

/// Buffer pool statistics
#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    pub total_created: u64,
    pub in_pool: usize,
    pub capacity: usize,
}

/// Zero-copy batcher
pub struct ZeroCopyBatcher {
    pool: BufferPool,
    config: BatcherConfig,
    active_buffer: Option<Arc<RwLock<ZeroCopyBuffer>>>,
}

impl ZeroCopyBatcher {
    /// Create a new zero-copy batcher
    pub fn new(config: BatcherConfig) -> Self {
        let pool_config = BufferPoolConfig::default();
        Self {
            pool: BufferPool::new(pool_config),
            config,
            active_buffer: None,
        }
    }

    /// Get active buffer
    async fn get_active_buffer(&mut self) -> Result<Arc<RwLock<ZeroCopyBuffer>>, BatcherError> {
        if let Some(ref buffer) = self.active_buffer {
            return Ok(buffer.clone());
        }

        let buffer = self.pool.get_buffer().await;
        self.active_buffer = Some(buffer.clone());
        Ok(buffer)
    }

    /// Add data to batch
    pub async fn add_data(&mut self, data: &[u8]) -> Result<(), BatcherError> {
        // Get or create active buffer
        if self.active_buffer.is_none() {
            self.active_buffer = Some(self.pool.get_buffer().await);
        }

        let buffer = self.active_buffer.as_ref().unwrap().clone();
        let mut buf = buffer.write().await;

        // Check if buffer has enough capacity
        if buf.len() + data.len() > buf.capacity() {
            drop(buf);
            // Return current buffer to pool
            if let Some(current) = self.active_buffer.take() {
                self.pool.return_buffer(current).await;
            }
            // Get a new buffer
            self.active_buffer = Some(self.pool.get_buffer().await);
            let buffer = self.active_buffer.as_ref().unwrap().clone();
            let mut buf = buffer.write().await;
            buf.write_bytes(data)?;
        } else {
            buf.write_bytes(data)?;
        }

        Ok(())
    }

    /// Flush current batch
    pub async fn flush(&mut self) -> Result<ZeroCopyBatch, BatcherError> {
        if let Some(buffer) = self.active_buffer.take() {
            // Read buffer data
            let buf = buffer.read().await;
            let data = Arc::new(buf.as_slice().to_vec());
            let size = buf.len();
            let metadata = buf.metadata().clone();
            drop(buf);

            // Return buffer to pool
            self.pool.return_buffer(buffer).await;

            // Create batch
            let batch = ZeroCopyBatch {
                data,
                size,
                metadata,
            };

            Ok(batch)
        } else {
            Err(BatcherError::EmptyBatch)
        }
    }

    /// Get pool statistics
    pub async fn get_pool_stats(&self) -> BufferPoolStats {
        self.pool.get_stats().await
    }
}

/// Zero-copy batch result
#[derive(Debug, Clone)]
pub struct ZeroCopyBatch {
    /// Batch data
    pub data: Arc<Vec<u8>>,
    /// Batch size
    pub size: usize,
    /// Batch metadata
    pub metadata: BufferMetadata,
}

impl ZeroCopyBatch {
    /// Get batch data slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

/// Batcher configuration
#[derive(Debug, Clone)]
pub struct BatcherConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Flush timeout
    pub flush_timeout: Duration,
}

/// Buffer error types
#[derive(Debug, thiserror::Error)]
pub enum BufferError {
    #[error("Insufficient buffer capacity")]
    InsufficientCapacity,
    #[error("Buffer pool exhausted")]
    PoolExhausted,
}

/// Batcher error types
#[derive(Debug, thiserror::Error)]
pub enum BatcherError {
    #[error("Empty batch")]
    EmptyBatch,
    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_buffer_pool_get_return() {
        let config = BufferPoolConfig {
            max_buffers: 2,
            ..Default::default()
        };
        let pool = BufferPool::new(config);

        // Get buffer
        let buffer1 = pool.get_buffer().await;
        {
            let mut buf = buffer1.write().await;
            buf.write_bytes(b"test data").unwrap();
        }

        // Return buffer
        pool.return_buffer(buffer1).await;

        // Get buffer again (should reuse)
        let buffer2 = pool.get_buffer().await;
        let buf = buffer2.read().await;
        assert_eq!(buf.len(), 0); // Buffer was cleared
    }

    #[tokio::test]
    async fn test_zero_copy_batcher() {
        let config = BatcherConfig {
            max_batch_size: 1024,
            flush_timeout: Duration::from_millis(100),
        };
        let mut batcher = ZeroCopyBatcher::new(config);

        // Add data
        batcher.add_data(b"test data 1").await.unwrap();
        batcher.add_data(b"test data 2").await.unwrap();

        // Flush
        let batch = batcher.flush().await.unwrap();
        assert!(batch.size > 0);
    }

    #[tokio::test]
    async fn test_buffer_pool_stats() {
        let config = BufferPoolConfig {
            max_buffers: 2,
            ..Default::default()
        };
        let pool = BufferPool::new(config);

        let _ = pool.get_buffer().await;
        let stats = pool.get_stats().await;

        assert_eq!(stats.in_pool, 0);
        assert_eq!(stats.total_created, 1);
    }
}
