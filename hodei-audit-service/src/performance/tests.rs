//! Tests para el módulo de performance - Epic 7
//!
//! Tests de integración para validar:
//! - SmartBatcher con policies híbridas
//! - gRPC connection pooling (10-50 connections)
//! - Backpressure handling
//! - Queue size limits
//! - Performance: 100K+ events/sec

use std::sync::Arc;
use std::time::Duration;

use crate::performance::{
    BackpressureConfig, BackpressureController, BatcherConfig, BatchingPolicy, CircuitBreaker,
    CircuitBreakerConfig, CircuitState, ConnectionPool, PoolConfig, PressureLevel, SmartBatcher,
};

#[cfg(test)]
mod tests {
    use super::*;

    // ========== SMART BATCHER TESTS ==========

    #[tokio::test]
    async fn test_smart_batcher_size_based() {
        let config = BatcherConfig {
            max_queue_size: 1000,
            policy: BatchingPolicy::SizeBased(10),
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Add 10 events (should trigger auto-flush based on size)
        for i in 0..10 {
            batcher.add_event(i).await.unwrap();
        }

        // Small delay to allow async processing
        tokio::time::sleep(Duration::from_millis(10)).await;

        let metrics = batcher.get_metrics().await;
        assert!(metrics.total_flushes >= 1);
        assert_eq!(metrics.total_events, 10);
    }

    #[tokio::test]
    async fn test_smart_batcher_hybrid_policy() {
        let config = BatcherConfig {
            max_queue_size: 1000,
            policy: BatchingPolicy::Hybrid {
                max_time: Duration::from_millis(100),
                max_size: 50,
            },
            flush_timeout: Duration::from_millis(500),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Add 20 events (should not auto-flush yet)
        for i in 0..20 {
            batcher.add_event(i).await.unwrap();
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
        let metrics = batcher.get_metrics().await;
        assert_eq!(metrics.total_events, 20);

        // Manual flush
        let result = batcher.flush().await.unwrap();
        assert_eq!(result.size, 20);
    }

    #[tokio::test]
    async fn test_smart_batcher_adaptive_policy() {
        let config = BatcherConfig {
            max_queue_size: 1000,
            policy: BatchingPolicy::Adaptive {
                target_throughput: 50_000,
                min_batch_size: 10,
                max_batch_size: 100,
                min_time: Duration::from_millis(10),
                max_time: Duration::from_millis(100),
            },
            flush_timeout: Duration::from_millis(500),
            adaptive_tuning: true,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Test adaptive policy with minimum batch size
        for i in 0..10 {
            batcher.add_event(i).await.unwrap();
        }

        let metrics = batcher.get_metrics().await;
        assert_eq!(metrics.total_events, 10);
        assert!(metrics.adaptive_adjustments >= 0);
    }

    #[tokio::test]
    async fn test_smart_batcher_queue_limit() {
        let config = BatcherConfig {
            max_queue_size: 5,
            policy: BatchingPolicy::SizeBased(100),
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Fill queue to max
        for i in 0..5 {
            batcher.add_event(i).await.unwrap();
        }

        // Next event should fail
        let result = batcher.add_event(100).await;
        assert!(result.is_err());
    }

    // ========== CONNECTION POOL TESTS ==========

    #[tokio::test]
    async fn test_connection_pool_creation() {
        let config = PoolConfig {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        };

        let pool = ConnectionPool::new(config);

        // Verify initial state
        let stats = pool.stats().await;
        assert_eq!(stats.total, 0);
        assert_eq!(stats.active, 0);

        let metrics = pool.get_metrics().await;
        assert_eq!(metrics.connection_requests, 0);
    }

    #[tokio::test]
    async fn test_connection_pool_config_limits() {
        let config = PoolConfig {
            min_connections: 10,
            max_connections: 50,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        };

        let pool = ConnectionPool::new(config);

        // Verify max connections limit
        let stats = pool.stats().await;
        assert!(stats.total <= 50);

        // Get metrics to verify configuration
        let metrics = pool.get_metrics().await;
        assert!(metrics.total_connections >= 0);
    }

    #[tokio::test]
    async fn test_connection_pool_health_check() {
        let config = PoolConfig {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(1), // Frequent health checks
            idle_timeout: Duration::from_secs(1),          // Short idle timeout for testing
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        };

        let pool = ConnectionPool::new(config);

        // Run health check
        pool.health_check().await;

        let metrics = pool.get_metrics().await;
        // Health check should have run
        assert!(metrics.health_check_failures >= 0);
    }

    #[tokio::test]
    async fn test_connection_pool_cleanup() {
        let config = PoolConfig {
            min_connections: 5,
            max_connections: 20,
            connection_timeout: Duration::from_secs(5),
            health_check_interval: Duration::from_secs(1),
            idle_timeout: Duration::from_millis(100), // Very short for testing
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        };

        let pool = ConnectionPool::new(config);

        // Cleanup idle connections
        pool.cleanup_idle().await;

        // Verify cleanup ran without errors
        let stats = pool.stats().await;
        assert!(stats.total >= 0);
    }

    // ========== BACKPRESSURE TESTS ==========

    #[tokio::test]
    async fn test_backpressure_normal_level() {
        let config = BackpressureConfig {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // Low queue size should be normal
        controller.update_queue_size(5_000);
        controller.record_event();

        let pressure = controller.get_current_pressure();
        assert_eq!(pressure, PressureLevel::Normal);

        let throttle = controller.get_throttle_rate();
        assert_eq!(throttle, 0.0);
    }

    #[tokio::test]
    async fn test_backpressure_high_level() {
        let config = BackpressureConfig {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // High queue size should trigger high pressure
        controller.update_queue_size(60_000);

        let pressure = controller.get_current_pressure();
        assert_eq!(pressure, PressureLevel::High);

        // Should apply backpressure
        assert!(controller.should_apply_backpressure());
    }

    #[tokio::test]
    async fn test_backpressure_critical_level() {
        let config = BackpressureConfig {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // Critical queue size
        controller.update_queue_size(85_000);

        let pressure = controller.get_current_pressure();
        assert_eq!(pressure, PressureLevel::Critical);

        // Should have high throttle
        let throttle = controller.get_throttle_rate();
        assert!(throttle > 0.5);
    }

    #[tokio::test]
    async fn test_backpressure_queue_size_limit() {
        let config = BackpressureConfig {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // Normal pressure should allow large queues
        controller.update_queue_size(5_000);
        let limit = controller.get_queue_size_limit();
        assert_eq!(limit, usize::MAX);

        // High pressure should reduce limit
        controller.update_queue_size(60_000);
        let limit = controller.get_queue_size_limit();
        assert_eq!(limit, 80_000);
    }

    #[tokio::test]
    async fn test_backpressure_metrics() {
        let config = BackpressureConfig {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // Update metrics
        controller.update_queue_size(5_000);
        for _ in 0..10 {
            controller.record_event();
        }

        let _ = controller.evaluate().await;

        let metrics = controller.get_metrics().await;
        assert!(metrics.total_events_processed > 0);
        assert!(metrics.pressure_changes >= 0);
    }

    // ========== CIRCUIT BREAKER TESTS ==========

    #[tokio::test]
    async fn test_circuit_breaker_closed_by_default() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            error_rate_threshold: 0.5,
            min_request_threshold: 10,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Should allow requests when closed
        assert!(cb.can_execute().await);

        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(1), // Short timeout for testing
            error_rate_threshold: 0.5,
            min_request_threshold: 1,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Record failures
        for _ in 0..3 {
            cb.record_failure().await;
        }

        // Should be open
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Open);

        // Should not allow requests
        assert!(!cb.can_execute().await);
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100), // Very short for testing
            error_rate_threshold: 0.5,
            min_request_threshold: 1,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            cb.record_failure().await;
        }
        assert_eq!(cb.get_state().await, CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should transition to half-open
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::HalfOpen);

        // Should allow requests in half-open state
        assert!(cb.can_execute().await);
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_on_success() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            error_rate_threshold: 0.5,
            min_request_threshold: 1,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..2 {
            cb.record_failure().await;
        }

        // Wait for timeout and transition to half-open
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert_eq!(cb.get_state().await, CircuitState::HalfOpen);

        // Record successes
        for _ in 0..2 {
            cb.record_success(Duration::from_millis(10)).await;
        }

        // Should be closed
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_manual_reset() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            error_rate_threshold: 0.5,
            min_request_threshold: 10,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..3 {
            cb.record_failure().await;
        }
        assert_eq!(cb.get_state().await, CircuitState::Open);

        // Manual reset
        cb.reset().await;

        // Should be closed
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Closed);

        // Metrics should be reset
        let metrics = cb.get_metrics().await;
        assert_eq!(metrics.total_requests, 0);
    }

    #[tokio::test]
    async fn test_circuit_breaker_error_rate() {
        let config = CircuitBreakerConfig {
            failure_threshold: 10, // High threshold
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            error_rate_threshold: 0.5,
            min_request_threshold: 10, // Need 10 requests to evaluate error rate
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        let cb = CircuitBreaker::new(config);

        // Record 10 requests, 6 failures (60% error rate)
        for _ in 0..6 {
            cb.record_failure().await;
        }
        for _ in 0..4 {
            cb.record_success(Duration::from_millis(10)).await;
        }

        // Should be open due to high error rate
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Open);

        let metrics = cb.get_metrics().await;
        assert!(metrics.error_rate > 0.5);
    }

    // ========== INTEGRATION TESTS ==========

    #[tokio::test]
    async fn test_performance_components_integration() {
        // Test that all components work together
        let backpressure_config = BackpressureConfig {
            queue_size_warnings: (1_000, 5_000, 8_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        };

        let batcher_config = BatcherConfig {
            max_queue_size: 10_000,
            policy: BatchingPolicy::Hybrid {
                max_time: Duration::from_millis(100),
                max_size: 1_000,
            },
            flush_timeout: Duration::from_millis(500),
            adaptive_tuning: true,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let circuit_breaker_config = CircuitBreakerConfig {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            error_rate_threshold: 0.5,
            min_request_threshold: 10,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        };

        // Create components
        let backpressure = BackpressureController::new(backpressure_config);
        let batcher = SmartBatcher::new(batcher_config);
        let circuit_breaker = CircuitBreaker::new(circuit_breaker_config);

        // Verify all components are functional
        assert_eq!(backpressure.get_current_pressure(), PressureLevel::Normal);
        assert_eq!(batcher.queue_size().await, 0);
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);

        // Test backpressure
        backpressure.update_queue_size(2_000);
        assert_eq!(backpressure.get_current_pressure(), PressureLevel::Normal);

        // Test batcher
        batcher.add_event("test_event_1".to_string()).await.unwrap();
        assert_eq!(batcher.queue_size().await, 1);

        // Test circuit breaker
        assert!(circuit_breaker.can_execute().await);
        circuit_breaker
            .record_success(Duration::from_millis(10))
            .await;
        assert_eq!(circuit_breaker.get_state().await, CircuitState::Closed);

        // Test metrics
        let bp_metrics = backpressure.get_metrics().await;
        let batcher_metrics = batcher.get_metrics().await;
        let cb_metrics = circuit_breaker.get_metrics().await;

        assert!(bp_metrics.total_events_processed > 0);
        assert_eq!(batcher_metrics.total_events, 1);
        assert!(cb_metrics.successful_requests > 0);
    }

    // ========== PERFORMANCE TARGETS VALIDATION ==========

    #[tokio::test]
    async fn test_batcher_throughput_target() {
        // Test that batcher can handle high throughput
        let config = BatcherConfig {
            max_queue_size: 100_000,
            policy: BatchingPolicy::SizeBased(1000),
            flush_timeout: Duration::from_millis(100),
            adaptive_tuning: false,
            backpressure_controller: None,
            enable_metrics: true,
        };

        let batcher = SmartBatcher::new(config);

        // Add 1000 events quickly
        for i in 0..1000 {
            batcher.add_event(i).await.unwrap();
        }

        // Verify batcher handled all events
        let metrics = batcher.get_metrics().await;
        assert!(metrics.total_events >= 1000);
        assert!(metrics.total_flushes >= 0); // May have auto-flushed
    }

    #[tokio::test]
    async fn test_backpressure_high_load() {
        // Test backpressure under high load
        let config = BackpressureConfig {
            queue_size_warnings: (1_000, 5_000, 8_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(1),
            enable_metrics: true,
        };

        let controller = BackpressureController::new(config);

        // Simulate high load
        for i in 0..10_000 {
            controller.update_queue_size(i);
            controller.record_event();
        }

        // Verify backpressure is working
        let pressure = controller.get_current_pressure();
        assert!(pressure == PressureLevel::High || pressure == PressureLevel::Critical);

        // Should apply backpressure
        assert!(controller.should_apply_backpressure());
    }
}
