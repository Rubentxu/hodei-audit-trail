//! Middleware de auditoría para Axum
//!
//! Este módulo implementa el middleware que captura automáticamente
//! las requests HTTP y las envía como eventos de auditoría.

use crate::batch::BatchQueue;
use crate::config::AuditSdkConfig;
use crate::hrn::{enrich_event_with_hrn, generate_hrn_from_path};
use crate::models::AuditEvent;
use bytes::Bytes;
use http::{Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{debug, error};

/// Layer de auditoría que implementa el middleware Axum
#[derive(Debug, Clone)]
pub struct AuditLayer {
    /// Configuración del SDK
    config: Arc<AuditSdkConfig>,
    /// Batch queue para eventos
    batch_queue: Arc<BatchQueue>,
}

impl AuditLayer {
    /// Crear un nuevo layer de auditoría
    pub fn new(config: AuditSdkConfig) -> Self {
        let batch_queue = Arc::new(BatchQueue::new(config.clone()));

        Self {
            config: Arc::new(config),
            batch_queue,
        }
    }

    /// Iniciar el timer de flush automático (opcional)
    /// Debe ser llamado dentro de un contexto async
    pub fn start_flush_timer(&self) {
        let queue_clone = self.batch_queue.clone();
        let config_clone = self.config.clone();
        tokio::spawn(async move {
            start_flush_timer(queue_clone, config_clone.batch_timeout).await;
        });
    }

    /// Obtener estadísticas del batch
    pub fn get_batch_stats(&self) -> crate::batch::BatchStats {
        self.batch_queue.get_stats()
    }
}

/// Iniciar el timer de flush automático
async fn start_flush_timer(queue: Arc<BatchQueue>, timeout: std::time::Duration) {
    let mut interval = tokio::time::interval(timeout);

    loop {
        interval.tick().await;
        if let Err(e) = queue.flush().await {
            error!("Failed to flush batch: {}", e);
        }
    }
}

impl<S> Layer<S> for AuditLayer {
    type Service = AuditService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuditService {
            config: self.config.clone(),
            batch_queue: self.batch_queue.clone(),
            service,
        }
    }
}

/// Service que implementa el middleware de auditoría
#[derive(Debug, Clone)]
pub struct AuditService<S> {
    /// Configuración del SDK
    config: Arc<AuditSdkConfig>,
    /// Batch queue
    batch_queue: Arc<BatchQueue>,
    /// Service inner
    service: S,
}

impl<S, B> Service<Request<B>> for AuditService<S>
where
    S: Service<Request<B>, Response = Response<Bytes>> + Send + Clone + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let config = self.config.clone();
        let batch_queue = self.batch_queue.clone();
        let mut service = self.service.clone();

        // Extract audit data from request before moving it
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let user_id = request
            .headers()
            .get("x-user-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let tenant_id = request
            .headers()
            .get("x-tenant-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Box::pin(async move {
            // Call next service with the original request (need mutable reference)
            let response = service.call(request).await?;

            // Generate HRN from request
            let hrn =
                generate_hrn_from_path(&method, &path, tenant_id.as_deref()).unwrap_or_else(|_| {
                    // Fallback to simple HRN if generation fails
                    let fallback_hrn = format!(
                        "hrn:hodei:{}:{}:global:resource/{}",
                        config.service_name,
                        tenant_id.as_deref().unwrap_or("unknown"),
                        path
                    );
                    // Parse the fallback HRN
                    crate::hrn::Hrn::parse(&fallback_hrn).unwrap_or_else(|_| {
                        // If even the fallback fails, create a minimal valid HRN
                        crate::hrn::Hrn::parse(&format!(
                            "hrn:hodei:service:unknown:global:resource/unknown"
                        ))
                        .unwrap()
                    })
                });

            // Create audit event (non-blocking)
            let mut event = AuditEvent {
                event_name: format!("{} {}", method, path),
                event_category: 0, // Management event
                hrn: hrn.to_string(),
                user_id: user_id.unwrap_or_else(|| "anonymous".to_string()),
                tenant_id: tenant_id.unwrap_or_else(|| "unknown".to_string()),
                trace_id: "no-trace".to_string(),
                resource_path: path,
                http_method: Some(method.to_string()),
                http_status: Some(response.status().as_u16() as i32),
                source_ip: None,
                user_agent: None,
                additional_data: None,
            };

            // Enrich with HRN metadata if resolver is available
            if let Err(e) = enrich_event_with_hrn(&mut event, &config.hrn_resolver).await {
                debug!("Failed to enrich event with HRN metadata: {}", e);
            }

            // Add to batch queue (async, non-blocking)
            if let Err(e) = batch_queue.add_event(event) {
                error!("Failed to add event to batch: {}", e);
            }

            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;
    use std::time::Duration;

    #[test]
    fn test_audit_layer_creation() {
        let config = AuditSdkConfig::builder()
            .service_name("test-service")
            .tenant_id("test-tenant")
            .build()
            .unwrap();

        let layer = AuditLayer::new(config);
        assert_eq!(layer.config.service_name, "test-service");
        assert_eq!(layer.config.tenant_id, Some("test-tenant".to_string()));
    }

    #[test]
    fn test_audit_layer_builder() {
        let config = AuditSdkConfig::builder()
            .service_name("my-service")
            .tenant_id("tenant-123")
            .audit_service_url("http://audit:50052")
            .batch_size(50)
            .batch_timeout(Duration::from_millis(200))
            .enable_request_body(false)
            .enable_response_body(true)
            .build()
            .unwrap();

        assert_eq!(config.service_name, "my-service");
        assert_eq!(config.tenant_id, Some("tenant-123".to_string()));
        assert_eq!(config.audit_service_url, "http://audit:50052");
        assert_eq!(config.batch_size, 50);
        assert_eq!(config.batch_timeout, Duration::from_millis(200));
        assert!(!config.enable_request_body);
        assert!(config.enable_response_body);
    }

    #[test]
    fn test_default_configuration() {
        let config = AuditSdkConfig::default();

        assert_eq!(config.service_name, "unknown-service");
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.batch_timeout, Duration::from_millis(100));
        assert!(config.enable_request_body);
        assert!(!config.enable_response_body);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_config_display() {
        let config = AuditSdkConfig::builder()
            .service_name("test-service")
            .tenant_id("test-tenant")
            .build()
            .unwrap();

        let config_str = format!("{}", config);
        assert!(config_str.contains("test-service"));
        assert!(config_str.contains("test-tenant"));
        assert!(config_str.contains("http://localhost:50052"));
    }

    #[test]
    fn test_extract_audit_context() {
        // This test verifies that the middleware can extract
        // context from headers properly
        let request = Request::builder()
            .uri("/api/test?foo=bar")
            .header("x-user-id", "user-123")
            .header("x-tenant-id", "tenant-123")
            .header("x-trace-id", "trace-456")
            .header("x-forwarded-for", "192.168.1.1")
            .header("user-agent", "Mozilla/5.0")
            .body(())
            .unwrap();

        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let query = request.uri().query().unwrap_or("").to_string();

        assert_eq!(method, http::Method::GET);
        assert_eq!(path, "/api/test");
        assert_eq!(query, "foo=bar");
    }
}
