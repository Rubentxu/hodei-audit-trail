//! Backpressure Controller
//!
//! Manages system pressure and prevents overload by controlling
//! event ingestion based on queue size, processing rate, and system resources.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::{Mutex, RwLock, watch};
use tracing::{debug, error, info, warn};

/// Pressure levels
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PressureLevel {
    #[default]
    Normal,
    Moderate,
    High,
    Critical,
    Overloaded,
}

impl PressureLevel {
    /// Get numeric value for comparison
    pub fn as_u8(&self) -> u8 {
        match self {
            PressureLevel::Normal => 0,
            PressureLevel::Moderate => 1,
            PressureLevel::High => 2,
            PressureLevel::Critical => 3,
            PressureLevel::Overloaded => 4,
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            PressureLevel::Normal => "System operating normally",
            PressureLevel::Moderate => "Moderate load, monitoring",
            PressureLevel::High => "High load, reducing intake",
            PressureLevel::Critical => "Critical load, aggressive throttling",
            PressureLevel::Overloaded => "System overloaded, blocking intake",
        }
    }
}

/// Backpressure configuration
#[derive(Debug, Clone)]
pub struct BackpressureConfig {
    /// Queue size thresholds
    pub queue_size_warnings: (usize, usize, usize), // (moderate, high, critical)
    /// Processing rate thresholds (events per second)
    pub rate_warnings: (u64, u64, u64),
    /// Time window for rate calculation
    pub rate_window: Duration,
    /// Auto-recovery enabled
    pub auto_recovery: bool,
    /// Recovery delay
    pub recovery_delay: Duration,
    /// Enable metrics
    pub enable_metrics: bool,
}

impl Default for BackpressureConfig {
    fn default() -> Self {
        Self {
            queue_size_warnings: (10_000, 50_000, 80_000),
            rate_warnings: (50_000, 80_000, 100_000),
            rate_window: Duration::from_secs(1),
            auto_recovery: true,
            recovery_delay: Duration::from_secs(5),
            enable_metrics: true,
        }
    }
}

/// Backpressure controller
#[derive(Debug)]
pub struct BackpressureController {
    config: BackpressureConfig,
    current_pressure: Arc<RwLock<PressureLevel>>,
    queue_size: Arc<AtomicUsize>,
    events_processed: Arc<AtomicU64>,
    last_update: Arc<RwLock<Instant>>,
    metrics: Arc<Mutex<BackpressureMetrics>>,
}

/// Backpressure metrics
#[derive(Debug, Clone, Default)]
pub struct BackpressureMetrics {
    pub pressure_changes: u64,
    pub normal_periods: u64,
    pub moderate_periods: u64,
    pub high_periods: u64,
    pub critical_periods: u64,
    pub overloaded_periods: u64,
    pub total_events_processed: u64,
    pub avg_queue_size: f64,
    pub peak_queue_size: usize,
    pub last_pressure_level: PressureLevel,
}

impl BackpressureController {
    /// Create a new backpressure controller
    pub fn new(config: BackpressureConfig) -> Self {
        Self {
            config,
            current_pressure: Arc::new(RwLock::new(PressureLevel::Normal)),
            queue_size: Arc::new(AtomicUsize::new(0)),
            events_processed: Arc::new(AtomicU64::new(0)),
            last_update: Arc::new(RwLock::new(Instant::now())),
            metrics: Arc::new(Mutex::new(BackpressureMetrics::default())),
        }
    }

    /// Update queue size
    pub fn update_queue_size(&self, size: usize) {
        self.queue_size.store(size, Ordering::Relaxed);
    }

    /// Record event processing
    pub fn record_event(&self) {
        self.events_processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current pressure level
    pub fn get_current_pressure(&self) -> PressureLevel {
        // We need to evaluate pressure based on current metrics
        // This is a simplified version
        let queue_size = self.queue_size.load(Ordering::Relaxed);
        let (moderate, high, critical) = self.config.queue_size_warnings;

        if queue_size >= critical {
            PressureLevel::Critical
        } else if queue_size >= high {
            PressureLevel::High
        } else if queue_size >= moderate {
            PressureLevel::Moderate
        } else {
            PressureLevel::Normal
        }
    }

    /// Evaluate and update pressure level
    pub async fn evaluate(&self) -> PressureLevel {
        let queue_size = self.queue_size.load(Ordering::Relaxed);
        let events = self.events_processed.load(Ordering::Relaxed);
        let now = Instant::now();

        // Calculate pressure level
        let new_pressure = self.calculate_pressure(queue_size, events).await;

        // Update if changed
        let mut current = self.current_pressure.write().await;
        if *current != new_pressure {
            *current = new_pressure;
            self.update_metrics(new_pressure).await;
            info!("Pressure level changed to: {:?}", new_pressure);
        }

        *current
    }

    /// Calculate pressure level based on metrics
    async fn calculate_pressure(&self, queue_size: usize, events: u64) -> PressureLevel {
        let (moderate_q, high_q, critical_q) = self.config.queue_size_warnings;
        let (moderate_r, high_r, critical_r) = self.config.rate_warnings;

        // Determine pressure based on queue size and rate
        let pressure_from_queue = if queue_size >= critical_q {
            4 // Overloaded
        } else if queue_size >= high_q {
            3 // Critical
        } else if queue_size >= moderate_q {
            2 // High
        } else {
            0 // Normal
        };

        let pressure_from_rate = if events >= critical_r {
            4 // Overloaded
        } else if events >= high_r {
            3 // Critical
        } else if events >= moderate_r {
            2 // High
        } else {
            0 // Normal
        };

        // Take the higher pressure
        let max_pressure = pressure_from_queue.max(pressure_from_rate);

        match max_pressure {
            0 => PressureLevel::Normal,
            1 => PressureLevel::Moderate,
            2 => PressureLevel::High,
            3 => PressureLevel::Critical,
            _ => PressureLevel::Overloaded,
        }
    }

    /// Update metrics
    async fn update_metrics(&self, new_pressure: PressureLevel) {
        let mut metrics = self.metrics.lock().await;
        metrics.pressure_changes += 1;
        metrics.last_pressure_level = new_pressure;

        match new_pressure {
            PressureLevel::Normal => metrics.normal_periods += 1,
            PressureLevel::Moderate => metrics.moderate_periods += 1,
            PressureLevel::High => metrics.high_periods += 1,
            PressureLevel::Critical => metrics.critical_periods += 1,
            PressureLevel::Overloaded => metrics.overloaded_periods += 1,
        }
    }

    /// Check if should apply backpressure
    pub fn should_apply_backpressure(&self) -> bool {
        let pressure = self.get_current_pressure();
        pressure == PressureLevel::High
            || pressure == PressureLevel::Critical
            || pressure == PressureLevel::Overloaded
    }

    /// Get throttle rate (0.0 = no throttle, 1.0 = full throttle)
    pub fn get_throttle_rate(&self) -> f64 {
        let pressure = self.get_current_pressure();
        match pressure {
            PressureLevel::Normal => 0.0,
            PressureLevel::Moderate => 0.1,
            PressureLevel::High => 0.3,
            PressureLevel::Critical => 0.6,
            PressureLevel::Overloaded => 1.0,
        }
    }

    /// Get queue size limit based on pressure
    pub fn get_queue_size_limit(&self) -> usize {
        let (moderate, high, critical) = self.config.queue_size_warnings;
        let pressure = self.get_current_pressure();

        match pressure {
            PressureLevel::Normal => usize::MAX,
            PressureLevel::Moderate => high,
            PressureLevel::High => critical,
            PressureLevel::Critical => moderate,
            PressureLevel::Overloaded => moderate / 2,
        }
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> BackpressureMetrics {
        self.metrics.lock().await.clone()
    }

    /// Reset metrics
    pub async fn reset_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        *metrics = BackpressureMetrics::default();
        info!("Backpressure metrics reset");
    }

    /// Auto-recovery from high pressure
    pub async fn auto_recover(&self) {
        if !self.config.auto_recovery {
            return;
        }

        let pressure = self.get_current_pressure();
        if pressure != PressureLevel::Normal {
            info!("Auto-recovery triggered from {:?}", pressure);
            // Wait for recovery delay
            tokio::time::sleep(self.config.recovery_delay).await;

            // Re-evaluate
            let _ = self.evaluate().await;
        }
    }
}

/// Helper to get current pressure as a watch stream
pub struct PressureWatcher {
    receiver: watch::Receiver<PressureLevel>,
}

impl PressureWatcher {
    /// Create a new pressure watcher
    pub fn new(_controller: &BackpressureController) -> Self {
        // In a real implementation, this would create a watch channel
        // For now, this is a placeholder
        let (_sender, receiver) = watch::channel(PressureLevel::Normal);
        Self { receiver }
    }

    /// Wait for pressure change
    pub async fn changed(&mut self) -> Result<(), watch::error::RecvError> {
        self.receiver.changed().await
    }

    /// Get current value
    pub fn borrow(&self) -> PressureLevel {
        *self.receiver.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pressure_level_normal() {
        let config = BackpressureConfig {
            queue_size_warnings: (10, 50, 80),
            ..Default::default()
        };

        let controller = BackpressureController::new(config);
        controller.update_queue_size(5);

        let pressure = controller.get_current_pressure();
        assert_eq!(pressure, PressureLevel::Normal);
    }

    #[tokio::test]
    async fn test_pressure_level_high() {
        let config = BackpressureConfig {
            queue_size_warnings: (10, 50, 80),
            ..Default::default()
        };

        let controller = BackpressureController::new(config);
        controller.update_queue_size(60);

        let pressure = controller.get_current_pressure();
        assert_eq!(pressure, PressureLevel::High);
    }

    #[tokio::test]
    async fn test_throttle_rate() {
        let config = BackpressureConfig::default();
        let controller = BackpressureController::new(config);

        // Normal pressure should have no throttle
        let rate = controller.get_throttle_rate();
        assert_eq!(rate, 0.0);
    }

    #[tokio::test]
    async fn test_queue_size_limit() {
        let config = BackpressureConfig {
            queue_size_warnings: (10, 50, 80),
            ..Default::default()
        };

        let controller = BackpressureController::new(config);
        controller.update_queue_size(5);

        let limit = controller.get_queue_size_limit();
        assert_eq!(limit, usize::MAX);
    }
}
