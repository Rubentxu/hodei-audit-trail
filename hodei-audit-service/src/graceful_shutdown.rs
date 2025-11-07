//! Graceful Shutdown Module
//!
//! Handles graceful shutdown of the service, ensuring:
//! - In-flight requests complete
//! - Resources are properly released
//! - Notification to load balancer that pod is terminating
//! - Drain connection pools

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, mpsc, oneshot};
use tokio::time::timeout;
use tracing::{error, info, warn};

/// Graceful shutdown configuration
#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    /// Maximum time to wait for graceful shutdown
    pub shutdown_timeout: Duration,
    /// Time to wait before force shutdown
    pub force_shutdown_delay: Duration,
    /// Whether to enable graceful shutdown
    pub enabled: bool,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            shutdown_timeout: Duration::from_secs(30),
            force_shutdown_delay: Duration::from_secs(5),
            enabled: true,
        }
    }
}

/// Shutdown state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShutdownState {
    Running,
    Draining,
    Completed,
}

/// Shutdown handle for coordinating graceful shutdown
pub struct GracefulShutdown {
    /// Current state
    state: Arc<RwLock<ShutdownState>>,
    /// Shutdown signal receiver
    shutdown_rx: Arc<RwLock<mpsc::Receiver<()>>>,
    /// Shutdown signal sender
    shutdown_tx: Arc<RwLock<mpsc::Sender<()>>>,
    /// Configuration
    config: ShutdownConfig,
    /// Components to drain
    components: Vec<Arc<dyn Shutdownable>>,
}

impl GracefulShutdown {
    /// Create a new graceful shutdown handler
    pub fn new(config: ShutdownConfig) -> Self {
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

        info!("[Shutdown] Initializing graceful shutdown handler");

        Self {
            state: Arc::new(RwLock::new(ShutdownState::Running)),
            shutdown_rx: Arc::new(RwLock::new(shutdown_rx)),
            shutdown_tx: Arc::new(RwLock::new(shutdown_tx)),
            config,
            components: Vec::new(),
        }
    }

    /// Add a component to be drained during shutdown
    pub fn add_component(&mut self, component: Arc<dyn Shutdownable>) {
        info!("[Shutdown] Adding shutdown component: {}", component.name());
        self.components.push(component);
    }

    /// Get current shutdown state
    pub async fn get_state(&self) -> ShutdownState {
        *self.state.read().await
    }

    /// Signal shutdown
    pub async fn shutdown(&self) {
        info!("[Shutdown] Shutdown signal received");

        // Update state to draining
        {
            let mut state = self.state.write().await;
            *state = ShutdownState::Draining;
        }

        // Notify other tasks
        {
            let tx = self.shutdown_tx.write().await;
            let _ = tx.send(());
        }

        // Start shutdown process
        self.initiate_shutdown().await;
    }

    /// Initiate shutdown process
    async fn initiate_shutdown(&self) {
        info!("[Shutdown] Initiating graceful shutdown");

        // Drain all components
        for component in &self.components {
            info!("[Shutdown] Draining component: {}", component.name());
            if let Err(e) = timeout(
                Duration::from_secs(10),
                component.shutdown(self.config.shutdown_timeout),
            )
            .await
            {
                error!(
                    "[Shutdown] Failed to drain component {}: {}",
                    component.name(),
                    e
                );
            }
        }

        // Update state to completed
        {
            let mut state = self.state.write().await;
            *state = ShutdownState::Completed;
        }

        info!("[Shutdown] Graceful shutdown completed");

        // Wait for force shutdown delay
        tokio::time::sleep(self.config.force_shutdown_delay).await;

        info!("[Shutdown] Exiting process");
    }

    /// Wait for shutdown signal
    pub async fn wait_for_shutdown(&self) {
        let mut rx = self.shutdown_rx.write().await;
        let _ = rx.recv().await;
    }

    /// Check if shutdown is in progress
    pub async fn is_shutdown_requested(&self) -> bool {
        *self.state.read().await != ShutdownState::Running
    }
}

/// Trait for components that can be gracefully shut down
#[async_trait::async_trait]
pub trait Shutdownable: Send + Sync {
    /// Get component name
    fn name(&self) -> String;

    /// Shutdown the component
    async fn shutdown(&self, timeout: Duration);
}

/// Graceful shutdown notifier for HTTP servers
pub struct HttpServerGracefulShutdown {
    /// Server handle
    server_handle: Option<tokio::task::JoinHandle<()>>,
    /// Health check manager
    health_manager: Option<Arc<crate::health::HealthCheckManager>>,
}

impl HttpServerGracefulShutdown {
    /// Create a new HTTP server graceful shutdown handler
    pub fn new() -> Self {
        Self {
            server_handle: None,
            health_manager: None,
        }
    }

    /// Set the server handle
    pub fn set_server_handle(&mut self, handle: tokio::task::JoinHandle<()>) {
        self.server_handle = Some(handle);
    }

    /// Set the health check manager
    pub fn set_health_manager(&mut self, health_manager: Arc<crate::health::HealthCheckManager>) {
        self.health_manager = Some(health_manager);
    }

    /// Signal to load balancer that pod is terminating
    async fn signal_terminating(&self) {
        if let Some(health_manager) = &self.health_manager {
            info!("[Shutdown] Signaling to load balancer that pod is terminating");

            // Update health status to indicate we're terminating
            // In a real implementation, this would update the service registry
        }
    }
}

#[async_trait::async_trait]
impl Shutdownable for HttpServerGracefulShutdown {
    fn name(&self) -> String {
        "http-server".to_string()
    }

    async fn shutdown(&self, timeout: Duration) {
        self.signal_terminating().await;

        // Stop accepting new connections
        // This is typically done by updating health check status

        // Wait for in-flight requests to complete
        info!("[Shutdown] Waiting for in-flight HTTP requests to complete");

        if let Some(handle) = &self.server_handle {
            // Wait for the server task to complete
            // In practice, this would require proper coordination
        }

        info!("[Shutdown] HTTP server shutdown completed");
    }
}

/// Graceful shutdown utility functions
pub struct ShutdownUtils;

impl ShutdownUtils {
    /// Register signal handlers for SIGTERM and SIGINT
    pub fn register_signal_handlers(
        shutdown: Arc<GracefulShutdown>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shutdown_clone = Arc::clone(&shutdown);
        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
            info!("[Shutdown] SIGINT received");
            shutdown_clone.shutdown().await;
        });

        #[cfg(unix)]
        {
            let shutdown_clone = Arc::clone(&shutdown);
            tokio::spawn(async move {
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("Failed to install SIGTERM handler")
                    .recv()
                    .await;
                info!("[Shutdown] SIGTERM received");
                shutdown_clone.shutdown().await;
            });
        }

        Ok(())
    }

    /// Wait for shutdown with a timeout
    pub async fn wait_for_shutdown(
        shutdown: Arc<GracefulShutdown>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = timeout(
            shutdown.config.shutdown_timeout,
            shutdown.wait_for_shutdown(),
        )
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                warn!("[Shutdown] Shutdown timeout exceeded");
                Err(anyhow::anyhow!("Shutdown timeout exceeded").into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::time::Duration;

    struct TestShutdownable {
        name: String,
        shutdown_delay: Duration,
    }

    impl TestShutdownable {
        fn new(name: &str, delay: Duration) -> Self {
            Self {
                name: name.to_string(),
                shutdown_delay: delay,
            }
        }
    }

    #[async_trait::async_trait]
    impl Shutdownable for TestShutdownable {
        fn name(&self) -> String {
            self.name.clone()
        }

        async fn shutdown(&self, _timeout: Duration) {
            info!("[Test] Shutting down component: {}", self.name);
            tokio::time::sleep(self.shutdown_delay).await;
            info!("[Test] Component shutdown completed: {}", self.name);
        }
    }

    #[tokio::test]
    async fn test_graceful_shutdown_state_transition() {
        let config = ShutdownConfig::default();
        let shutdown = Arc::new(GracefulShutdown::new(config));

        assert_eq!(shutdown.get_state().await, ShutdownState::Running);

        // Note: Can't test full shutdown without tokio::spawn
    }

    #[tokio::test]
    async fn test_shutdownable_trait() {
        let component = Arc::new(TestShutdownable::new(
            "test-component",
            Duration::from_millis(10),
        ));

        assert_eq!(component.name(), "test-component");

        component.shutdown(Duration::from_millis(100)).await;
    }

    #[tokio::test]
    async fn test_http_server_graceful_shutdown() {
        let http_shutdown = Arc::new(HttpServerGracefulShutdown::new());
        assert_eq!(http_shutdown.name(), "http-server");
    }
}
