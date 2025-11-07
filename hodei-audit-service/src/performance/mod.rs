//! Performance optimization module
//!
//! This module provides high-performance components for handling
//! 100K+ events per second with smart batching, connection pooling,
//! and backpressure handling.

pub mod backpressure;
pub mod batcher;
pub mod circuit_breaker;
pub mod connection_pool;

pub use backpressure::{
    BackpressureConfig, BackpressureController, BackpressureMetrics, PressureLevel,
};
pub use batcher::{BatchResult, BatcherConfig, BatcherError, BatchingPolicy, SmartBatcher};
pub use circuit_breaker::{
    CircuitBreaker, CircuitBreakerConfig, CircuitBreakerMetrics, CircuitState,
};
pub use connection_pool::{ConnectionPool, PoolConfig, PoolError, PoolStats, PooledConnection};
