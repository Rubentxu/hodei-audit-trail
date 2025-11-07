//! Circuit Breaker
//!
//! Implements the circuit breaker pattern to prevent cascading failures
//! and protect the system from overload.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::{Mutex as TokioMutex, RwLock};
use tracing::{debug, error, info, warn};

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CircuitState {
    /// Circuit is closed, allowing requests
    #[default]
    Closed,
    /// Circuit is open, blocking requests
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

impl CircuitState {
    /// Check if requests are allowed
    pub fn allows_requests(&self) -> bool {
        match self {
            CircuitState::Closed => true,
            CircuitState::HalfOpen => true,
            CircuitState::Open => false,
        }
    }

    /// Get state description
    pub fn description(&self) -> &'static str {
        match self {
            CircuitState::Closed => "Circuit closed - requests allowed",
            CircuitState::Open => "Circuit open - requests blocked",
            CircuitState::HalfOpen => "Circuit half-open - testing recovery",
        }
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    /// Success threshold in half-open state
    pub success_threshold: u32,
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
    /// Expected error rate threshold
    pub error_rate_threshold: f64,
    /// Minimum number of requests before evaluating
    pub min_request_threshold: u32,
    /// Rolling window for metrics
    pub rolling_window: Duration,
    /// Enable automatic recovery
    pub auto_recovery: bool,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            error_rate_threshold: 0.5,
            min_request_threshold: 10,
            rolling_window: Duration::from_secs(60),
            auto_recovery: true,
        }
    }
}

/// Circuit breaker metrics
#[derive(Debug, Clone)]
pub struct CircuitBreakerMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub circuit_opens: u64,
    pub circuit_closes: u64,
    pub current_state: CircuitState,
    pub last_state_change: Instant,
    pub error_rate: f64,
    pub avg_response_time: Duration,
}

/// Circuit breaker
#[derive(Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitState>>,
    metrics: Arc<TokioMutex<CircuitBreakerMetrics>>,
    last_state_change: Arc<RwLock<Instant>>,
    request_times: Arc<TokioMutex<Vec<Instant>>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        let now = Instant::now();

        Self {
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            metrics: Arc::new(TokioMutex::new(CircuitBreakerMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                circuit_opens: 0,
                circuit_closes: 0,
                current_state: CircuitState::Closed,
                last_state_change: now,
                error_rate: 0.0,
                avg_response_time: Duration::from_secs(0),
            })),
            last_state_change: Arc::new(RwLock::new(now)),
            request_times: Arc::new(TokioMutex::new(Vec::new())),
        }
    }

    /// Check if request is allowed
    pub async fn can_execute(&self) -> bool {
        let state = *self.state.read().await;
        state.allows_requests()
    }

    /// Record a successful request
    pub async fn record_success(&self, response_time: Duration) {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.lock().await;
        let mut times = self.request_times.lock().await;

        // Update metrics
        metrics.total_requests += 1;
        metrics.successful_requests += 1;
        metrics.avg_response_time = Duration::from_nanos(
            ((metrics.avg_response_time.as_nanos() as u64
                * (metrics.successful_requests - 1) as u64)
                + response_time.as_nanos() as u64)
                / metrics.successful_requests as u64,
        );

        // Record request time
        times.push(Instant::now());
        self.cleanup_old_times(&mut times).await;

        // Evaluate state transition
        match *state {
            CircuitState::HalfOpen => {
                if metrics.successful_requests >= self.config.success_threshold as u64 {
                    *state = CircuitState::Closed;
                    metrics.circuit_closes += 1;
                    metrics.current_state = CircuitState::Closed;
                    *self.last_state_change.write().await = Instant::now();
                    info!("Circuit breaker closed after successful recovery test");
                }
            }
            CircuitState::Closed | CircuitState::Open => {
                // No state change on success when closed or open
            }
        }
    }

    /// Record a failed request
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.lock().await;
        let mut times = self.request_times.lock().await;

        // Update metrics
        metrics.total_requests += 1;
        metrics.failed_requests += 1;

        // Record request time
        times.push(Instant::now());
        self.cleanup_old_times(&mut times).await;

        // Calculate error rate
        let error_rate = if metrics.total_requests >= self.config.min_request_threshold as u64 {
            metrics.failed_requests as f64 / metrics.total_requests as f64
        } else {
            0.0
        };
        metrics.error_rate = error_rate;

        // Evaluate state transition
        match *state {
            CircuitState::Closed => {
                if metrics.failed_requests >= self.config.failure_threshold as u64
                    || (metrics.total_requests >= self.config.min_request_threshold as u64
                        && error_rate >= self.config.error_rate_threshold)
                {
                    *state = CircuitState::Open;
                    metrics.circuit_opens += 1;
                    metrics.current_state = CircuitState::Open;
                    *self.last_state_change.write().await = Instant::now();
                    warn!("Circuit breaker opened due to failures");
                }
            }
            CircuitState::HalfOpen => {
                // Any failure in half-open state opens the circuit
                *state = CircuitState::Open;
                metrics.circuit_opens += 1;
                metrics.current_state = CircuitState::Open;
                *self.last_state_change.write().await = Instant::now();
                warn!("Circuit breaker opened during half-open test");
            }
            CircuitState::Open => {
                // Already open, no state change
            }
        }
    }

    /// Get current state
    pub async fn get_state(&self) -> CircuitState {
        let state = *self.state.read().await;

        // Check if we should transition from Open to HalfOpen
        if state == CircuitState::Open {
            let last_change = *self.last_state_change.read().await;
            if last_change.elapsed() >= self.config.timeout {
                drop(state);
                drop(last_change);
                self.transition_to_half_open().await;
            }
        }

        *self.state.read().await
    }

    /// Transition to half-open state
    async fn transition_to_half_open(&self) {
        let mut state = self.state.write().await;
        if *state == CircuitState::Open {
            *state = CircuitState::HalfOpen;
            let mut metrics = self.metrics.lock().await;
            metrics.current_state = CircuitState::HalfOpen;
            *self.last_state_change.write().await = Instant::now();
            metrics.successful_requests = 0; // Reset for half-open testing
            info!("Circuit breaker half-open - testing service recovery");
        }
    }

    /// Manual reset to closed state
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        let mut metrics = self.metrics.lock().await;

        *state = CircuitState::Closed;
        metrics.current_state = CircuitState::Closed;
        metrics.successful_requests = 0;
        metrics.failed_requests = 0;
        metrics.total_requests = 0;
        metrics.error_rate = 0.0;
        *self.last_state_change.write().await = Instant::now();

        info!("Circuit breaker manually reset to closed");
    }

    /// Clean up old request times
    async fn cleanup_old_times(&self, times: &mut Vec<Instant>) {
        let now = Instant::now();
        let cutoff = now - self.config.rolling_window;

        // Keep only recent times
        times.retain(|&time| time > cutoff);
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> CircuitBreakerMetrics {
        self.metrics.lock().await.clone()
    }

    /// Get time since state change
    pub async fn time_since_state_change(&self) -> Duration {
        self.last_state_change.read().await.elapsed()
    }

    /// Get state description
    pub async fn get_state_description(&self) -> String {
        let state = self.get_state().await;
        let elapsed = self.time_since_state_change().await;
        format!("{} (for {:?})", state.description(), elapsed)
    }

    /// Check if circuit is healthy
    pub async fn is_healthy(&self) -> bool {
        let state = *self.state.read().await;
        state != CircuitState::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_closed_by_default() {
        let config = CircuitBreakerConfig::default();
        let cb = CircuitBreaker::new(config);

        let can_execute = cb.can_execute().await;
        assert!(can_execute);

        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_opens_on_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };

        let cb = CircuitBreaker::new(config);

        // Record 3 failures
        for _ in 0..3 {
            cb.record_failure().await;
        }

        // Circuit should be open
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Open);

        // Should not allow requests
        let can_execute = cb.can_execute().await;
        assert!(!can_execute);
    }

    #[tokio::test]
    async fn test_circuit_closes_after_successes() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            min_request_threshold: 1,
            ..Default::default()
        };

        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..3 {
            cb.record_failure().await;
        }
        assert_eq!(cb.get_state().await, CircuitState::Open);

        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should transition to half-open
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::HalfOpen);

        // Record 2 successes
        for _ in 0..2 {
            cb.record_success(Duration::from_millis(10)).await;
        }

        // Should be closed
        let state = cb.get_state().await;
        assert_eq!(state, CircuitState::Closed);
    }

    #[tokio::test]
    async fn test_manual_reset() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            min_request_threshold: 1,
            ..Default::default()
        };

        let cb = CircuitBreaker::new(config);

        // Open the circuit
        for _ in 0..3 {
            cb.record_failure().await;
        }
        assert_eq!(cb.get_state().await, CircuitState::Open);

        // Manual reset
        cb.reset().await;
        assert_eq!(cb.get_state().await, CircuitState::Closed);
    }
}
