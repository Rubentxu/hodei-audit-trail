//! Vector API Service - Implementación
//!
//! Servicio para comunicación CAP → Vector.dev (Puerto 50051)

use tonic::{Request, Response, Status};
use tracing::{error, info, warn};

use hodei_audit_proto::{
    AuditEvent, EventBatchRequest, EventBatchResponse, EventId, HealthCheckRequest,
    HealthCheckResponse, TenantId, vector_api_server::VectorApi,
};

/// Implementación del Vector API
/// Maneja la comunicación entre el CAP y Vector.dev
#[derive(Debug, Clone)]
pub struct VectorApiServiceImpl {
    // Contador de lotes procesados (usar interior mutability)
    batch_counter: std::sync::Arc<std::sync::atomic::AtomicU64>,
    // Flag para indicar si Vector está disponible
    vector_available: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

/// Implementación por defecto
impl Default for VectorApiServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorApiServiceImpl {
    /// Crear nueva instancia del servicio
    pub fn new() -> Self {
        info!("Initializing VectorApiService");
        Self {
            batch_counter: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0)),
            vector_available: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    /// Marcar Vector como disponible/no disponible
    pub fn set_vector_available(&self, available: bool) {
        self.vector_available
            .store(available, std::sync::atomic::Ordering::SeqCst);
        if available {
            info!("Vector is now available");
        } else {
            warn!("Vector is marked as unavailable");
        }
    }

    /// Incrementar contador de lotes
    fn next_batch_id(&self) -> String {
        let count = self
            .batch_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("batch_{}", count + 1)
    }

    /// Verificar si Vector está disponible
    fn check_vector_health(&self) -> Result<(), Status> {
        if !self
            .vector_available
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            return Err(Status::unavailable("Vector is currently unavailable"));
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl VectorApi for VectorApiServiceImpl {
    /// Enviar lote de eventos a Vector
    async fn send_event_batch(
        &self,
        request: Request<EventBatchRequest>,
    ) -> Result<Response<EventBatchResponse>, Status> {
        let req = request.into_inner();
        let events = req.events.clone();
        let event_count = events.len();

        // Verificar disponibilidad de Vector
        self.check_vector_health()?;

        info!(
            event_count = event_count,
            "Received SendEventBatch request from CAP"
        );

        // Validación básica
        if events.is_empty() {
            return Err(Status::invalid_argument("events cannot be empty"));
        }

        // TODO: Implementar envío real a Vector
        // - Serializar eventos
        // - Comprimir si es necesario
        // - Enviar vía gRPC a Vector
        // - Manejar reintentos
        // - Confirmar recepción
        // - Reportar métricas

        // Simular envío a Vector
        let batch_id = self.next_batch_id();
        let received_count = event_count as u32;

        // Simular éxito (en implementación real, esperar respuesta de Vector)
        info!(
            batch_id = batch_id,
            event_count = event_count,
            "Event batch sent to Vector successfully (simulated)"
        );

        let response = EventBatchResponse {
            success: true,
            message: "Batch received by Vector".to_string(),
            batch_id,
            received_count,
        };

        Ok(Response::new(response))
    }

    /// Health check de Vector
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let req = request.into_inner();
        let component = req.component.clone();

        info!(
            component = component,
            "Received HealthCheck request for Vector"
        );

        // Verificar salud de Vector
        let healthy = self
            .vector_available
            .load(std::sync::atomic::Ordering::SeqCst);

        let health_status = if healthy {
            hodei_audit_proto::HealthStatus::StatusServing
        } else {
            hodei_audit_proto::HealthStatus::StatusNotServing
        };

        let mut metrics = std::collections::HashMap::new();
        metrics.insert(
            "batches_processed".to_string(),
            self.batch_counter
                .load(std::sync::atomic::Ordering::SeqCst)
                .to_string(),
        );
        metrics.insert(
            "status".to_string(),
            if healthy { "ok" } else { "unavailable" }.to_string(),
        );

        info!(
            component = component,
            healthy = healthy,
            "Vector health check completed"
        );

        let response = HealthCheckResponse {
            status: health_status as i32,
            message: "Vector API health check".to_string(),
            details: std::collections::HashMap::new(),
            healthy,
            status_msg: if healthy { "OK" } else { "Unavailable" }.to_string(),
            metrics,
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_event_batch_success() {
        let mut service = VectorApiServiceImpl::new();

        let events = vec![
            hodei_audit_proto::AuditEvent {
                event_id: Some(EventId {
                    value: "event-1".to_string(),
                }),
                tenant_id: Some(TenantId {
                    value: "test-tenant".to_string(),
                }),
                ..Default::default()
            },
            hodei_audit_proto::AuditEvent {
                event_id: Some(EventId {
                    value: "event-2".to_string(),
                }),
                tenant_id: Some(TenantId {
                    value: "test-tenant".to_string(),
                }),
                ..Default::default()
            },
        ];

        let request = Request::new(EventBatchRequest { events });

        let response = service.send_event_batch(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.success);
        assert_eq!(response.received_count, 2);
        assert!(!response.batch_id.is_empty());
    }

    #[tokio::test]
    async fn test_send_event_batch_empty() {
        let mut service = VectorApiServiceImpl::new();

        let request = Request::new(EventBatchRequest { events: vec![] });

        let result = service.send_event_batch(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_send_event_batch_vector_unavailable() {
        let mut service = VectorApiServiceImpl::new();
        service.set_vector_available(false);

        let events = vec![hodei_audit_proto::AuditEvent {
            event_id: Some(EventId {
                value: "event-1".to_string(),
            }),
            tenant_id: Some(TenantId {
                value: "test-tenant".to_string(),
            }),
            ..Default::default()
        }];

        let request = Request::new(EventBatchRequest { events });

        let result = service.send_event_batch(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::Unavailable);
    }

    #[tokio::test]
    async fn test_health_check_healthy() {
        let service = VectorApiServiceImpl::new();

        let request = Request::new(HealthCheckRequest {
            service_name: "vector-api".to_string(),
            component: "vector".to_string(),
        });

        let response = service.health_check(request).await.unwrap();
        let response = response.into_inner();

        assert!(response.healthy);
        assert_eq!(response.status_msg, "OK".to_string());
    }

    #[tokio::test]
    async fn test_health_check_unavailable() {
        let service = VectorApiServiceImpl::new();
        service.set_vector_available(false);

        let request = Request::new(HealthCheckRequest {
            service_name: "vector-api".to_string(),
            component: "vector".to_string(),
        });

        let response = service.health_check(request).await.unwrap();
        let response = response.into_inner();

        assert!(!response.healthy);
        assert_eq!(response.status_msg, "Unavailable".to_string());
    }

    #[tokio::test]
    async fn test_set_vector_available() {
        let service = VectorApiServiceImpl::new();

        assert!(
            service
                .vector_available
                .load(std::sync::atomic::Ordering::SeqCst)
        );
        service.set_vector_available(false);
        assert!(
            !service
                .vector_available
                .load(std::sync::atomic::Ordering::SeqCst)
        );
        service.set_vector_available(true);
        assert!(
            service
                .vector_available
                .load(std::sync::atomic::Ordering::SeqCst)
        );
    }
}
