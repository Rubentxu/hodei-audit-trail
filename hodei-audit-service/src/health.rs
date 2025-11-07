//! Health Check Module
//!
//! Provides HTTP endpoints for Kubernetes health checks:
//! - /health/live - Liveness probe
//! - /health/ready - Readiness probe
//! - /health/startup - Startup probe

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Health status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    Starting,
    Healthy,
    Unhealthy,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthResult {
    pub status: HealthStatus,
    pub message: String,
    pub timestamp: SystemTime,
    pub details: HashMap<String, String>,
}

impl Default for HealthResult {
    fn default() -> Self {
        Self {
            status: HealthStatus::Healthy,
            message: "Service is healthy".to_string(),
            timestamp: SystemTime::now(),
            details: HashMap::new(),
        }
    }
}

/// Health checker for various components
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    /// Check if the component is healthy
    async fn check(&self) -> HealthResult;
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Port to listen on
    pub port: u16,
    /// Startup timeout
    pub startup_timeout: Duration,
    /// Readiness check interval
    pub readiness_check_interval: Duration,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            startup_timeout: Duration::from_secs(150), // 2.5 minutes
            readiness_check_interval: Duration::from_secs(5),
        }
    }
}

/// Health check manager
pub struct HealthCheckManager {
    /// Configuration
    config: HealthCheckConfig,
    /// Current health status
    status: Arc<RwLock<HealthResult>>,
    /// Start time
    start_time: Instant,
    /// Health checkers
    checkers: Vec<Arc<dyn HealthChecker>>,
}

impl HealthCheckManager {
    /// Create a new health check manager
    pub fn new(config: HealthCheckConfig) -> Self {
        info!(
            "[Health] Initializing health check manager on port {}",
            config.port
        );
        Self {
            config,
            status: Arc::new(RwLock::new(HealthResult::default())),
            start_time: Instant::now(),
            checkers: Vec::new(),
        }
    }

    /// Add a health checker
    pub fn add_checker(&mut self, checker: Arc<dyn HealthChecker>) {
        info!("[Health] Adding health checker");
        self.checkers.push(checker);
    }

    /// Get current health status
    pub async fn get_status(&self) -> HealthResult {
        let status = self.status.read().await;
        status.clone()
    }

    /// Update health status
    async fn update_status(&self, new_status: HealthResult) {
        let mut status = self.status.write().await;
        *status = new_status;
    }

    /// Run health checks
    pub async fn run_checks(&self) {
        info!("[Health] Running health checks...");

        let elapsed = self.start_time.elapsed();
        let is_startup = elapsed < self.config.startup_timeout;

        let mut all_healthy = true;
        let mut details = HashMap::new();

        // Check all registered checkers
        for (i, checker) in self.checkers.iter().enumerate() {
            match checker.check().await {
                HealthResult {
                    status: HealthStatus::Healthy,
                    details: checker_details,
                    ..
                } => {
                    for (key, value) in checker_details {
                        details.insert(format!("checker_{}_{}", i, key), value);
                    }
                }
                HealthResult {
                    status: HealthStatus::Unhealthy,
                    message,
                    details: checker_details,
                    ..
                } => {
                    all_healthy = false;
                    error!("[Health] Checker {} is unhealthy: {}", i, message);
                    for (key, value) in checker_details {
                        details.insert(format!("checker_{}_{}", i, key), value);
                    }
                }
                HealthResult {
                    status: HealthStatus::Starting,
                    message,
                    details: checker_details,
                    ..
                } => {
                    if !is_startup {
                        all_healthy = false;
                        warn!("[Health] Checker {} is still starting: {}", i, message);
                    }
                    for (key, value) in checker_details {
                        details.insert(format!("checker_{}_{}", i, key), value);
                    }
                }
            }
        }

        // Determine overall status
        let status = if !all_healthy {
            if is_startup {
                HealthStatus::Starting
            } else {
                HealthStatus::Unhealthy
            }
        } else {
            HealthStatus::Healthy
        };

        let message = match status {
            HealthStatus::Starting => {
                if is_startup {
                    format!(
                        "Service is starting up ({}s elapsed, {}s timeout)",
                        elapsed.as_secs(),
                        self.config.startup_timeout.as_secs()
                    )
                } else {
                    "Service is starting up".to_string()
                }
            }
            HealthStatus::Healthy => "All health checks passed".to_string(),
            HealthStatus::Unhealthy => "One or more health checks failed".to_string(),
        };

        // Update status
        let health_result = HealthResult {
            status,
            message,
            timestamp: SystemTime::now(),
            details,
        };

        self.update_status(health_result).await;
        info!("[Health] Health check completed");
    }

    /// Start the health check HTTP server
    /// Note: This requires the `axum` feature to be enabled
    #[cfg(feature = "health-server")]
    pub async fn start_server(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let port = self.config.port;
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        info!("[Health] Starting HTTP health check server on {}", addr);

        // Use axum for HTTP server
        let app = axum::Router::new()
            .route("/health/live", axum::routing::get(live_route))
            .route("/health/ready", axum::routing::get(ready_route))
            .route("/health/startup", axum::routing::get(startup_route))
            .with_state(self);

        // Start server
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

/// Live route - checks if container is alive
#[cfg(feature = "health-server")]
async fn live_route(
    axum::extract::State(state): axum::extract::State<Arc<HealthCheckManager>>,
) -> (axum::http::StatusCode, axum::Json<HealthResult>) {
    let status = state.get_status().await;

    // Liveness is always true if we're running
    if status.status == HealthStatus::Unhealthy {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(status),
        )
    } else {
        (axum::http::StatusCode::OK, axum::Json(status))
    }
}

/// Ready route - checks if container is ready to accept traffic
#[cfg(feature = "health-server")]
async fn ready_route(
    axum::extract::State(state): axum::extract::State<Arc<HealthCheckManager>>,
) -> (axum::http::StatusCode, axum::Json<HealthResult>) {
    let status = state.get_status().await;

    match status.status {
        HealthStatus::Healthy => (axum::http::StatusCode::OK, axum::Json(status)),
        HealthStatus::Starting => (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            axum::Json(status),
        ),
        HealthStatus::Unhealthy => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(status),
        ),
    }
}

/// Startup route - checks if container has started
#[cfg(feature = "health-server")]
async fn startup_route(
    axum::extract::State(state): axum::extract::State<Arc<HealthCheckManager>>,
) -> (axum::http::StatusCode, axum::Json<HealthResult>) {
    let status = state.get_status().await;

    match status.status {
        HealthStatus::Starting | HealthStatus::Healthy => {
            (axum::http::StatusCode::OK, axum::Json(status))
        }
        HealthStatus::Unhealthy => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(status),
        ),
    }
}

/// Service health checker
pub struct ServiceHealthChecker {
    service: Arc<dyn crate::service::HodeiAuditServiceExt>,
}

impl ServiceHealthChecker {
    pub fn new(service: Arc<dyn crate::service::HodeiAuditServiceExt>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl HealthChecker for ServiceHealthChecker {
    async fn check(&self) -> HealthResult {
        match self.service.health_check().await {
            Ok(health_map) => {
                let mut details = HashMap::new();
                let mut all_healthy = true;

                for (component, is_healthy) in health_map {
                    details.insert(format!("component.{}", component), is_healthy.to_string());
                    if !is_healthy {
                        all_healthy = false;
                    }
                }

                HealthResult {
                    status: if all_healthy {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Unhealthy
                    },
                    message: if all_healthy {
                        "All service components are healthy".to_string()
                    } else {
                        "One or more service components are unhealthy".to_string()
                    },
                    timestamp: SystemTime::now(),
                    details,
                }
            }
            Err(e) => {
                error!("[Health] Health check failed: {}", e);
                HealthResult {
                    status: HealthStatus::Unhealthy,
                    message: format!("Health check failed: {}", e),
                    timestamp: SystemTime::now(),
                    details: {
                        let mut d = HashMap::new();
                        d.insert("error".to_string(), e.to_string());
                        d
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestHealthChecker {
        healthy: bool,
    }

    impl TestHealthChecker {
        fn new(healthy: bool) -> Self {
            Self { healthy }
        }
    }

    #[async_trait::async_trait]
    impl HealthChecker for TestHealthChecker {
        async fn check(&self) -> HealthResult {
            if self.healthy {
                HealthResult {
                    status: HealthStatus::Healthy,
                    message: "Test checker is healthy".to_string(),
                    timestamp: SystemTime::now(),
                    details: HashMap::from([("test".to_string(), "ok".to_string())]),
                }
            } else {
                HealthResult {
                    status: HealthStatus::Unhealthy,
                    message: "Test checker is unhealthy".to_string(),
                    timestamp: SystemTime::now(),
                    details: HashMap::new(),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_health_check_manager_healthy() {
        let mut manager = HealthCheckManager::new(HealthCheckConfig::default());
        manager.add_checker(Arc::new(TestHealthChecker::new(true)));

        manager.run_checks().await;

        let status = manager.get_status().await;
        assert_eq!(status.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_health_check_manager_unhealthy() {
        let mut config = HealthCheckConfig::default();
        config.startup_timeout = Duration::from_millis(10); // Short timeout for test
        let mut manager = HealthCheckManager::new(config);
        manager.add_checker(Arc::new(TestHealthChecker::new(false)));

        // Wait for startup timeout to elapse
        tokio::time::sleep(Duration::from_millis(20)).await;

        manager.run_checks().await;

        let status = manager.get_status().await;
        assert_eq!(status.status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_health_check_manager_multiple_checkers() {
        let mut manager = HealthCheckManager::new(HealthCheckConfig::default());
        manager.add_checker(Arc::new(TestHealthChecker::new(true)));
        manager.add_checker(Arc::new(TestHealthChecker::new(true)));

        manager.run_checks().await;

        let status = manager.get_status().await;
        assert_eq!(status.status, HealthStatus::Healthy);
        assert!(!status.details.is_empty());
    }
}
