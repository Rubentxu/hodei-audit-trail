use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::info;

use hodei_audit_proto::{
    AuditEvent, EventId, HealthCheckRequest, HealthCheckResponse, HealthStatus,
    PublishBatchRequest, PublishBatchResponse, PublishEventRequest, PublishEventResponse, TenantId,
    audit_control_service_server::{AuditControlService, AuditControlServiceServer},
};
use uuid::Uuid;

/// Implementación del servicio de control de auditoría
/// Maneja la ingestión de eventos desde aplicaciones cliente (ARPs)
#[derive(Debug, Clone)]
pub struct AuditControlServiceImpl {
    // Configuración interna
    config: Arc<ServiceConfig>,
    // Contador de eventos para métricas básicas
    event_counter: Arc<std::sync::atomic::AtomicU64>,
}

/// Configuración del servicio
#[derive(Debug, Clone)]
struct ServiceConfig {
    max_batch_size: usize,
    enable_metrics: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 1000,
            enable_metrics: true,
        }
    }
}

impl AuditControlServiceImpl {
    /// Crear nueva instancia del servicio
    pub fn new() -> Self {
        info!("Initializing AuditControlService");

        Self {
            config: Arc::new(ServiceConfig::default()),
            event_counter: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// Registrar evento (para testing)
    pub fn get_event_count(&self) -> u64 {
        self.event_counter.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[tonic::async_trait]
impl AuditControlService for AuditControlServiceImpl {
    /// Publicar un único evento de auditoría
    async fn publish_event(
        &self,
        request: Request<PublishEventRequest>,
    ) -> Result<Response<PublishEventResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id.clone();
        let event = req.event.clone();

        // event es Option<AuditEvent>, extraer el valor
        let event = if let Some(e) = event {
            e
        } else {
            return Err(Status::invalid_argument("event is required"));
        };
        let event_id = event.event_id.clone().unwrap_or_default().value;

        info!(
            tenant_id = tenant_id,
            event_id = event_id,
            "Received PublishEvent request"
        );

        // Validación básica
        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        if event_id.is_empty() {
            return Err(Status::invalid_argument("event_id is required"));
        }

        // TODO: Implementar lógica de persistencia
        // - Validar evento
        // - Enriquecer evento
        // - Enviar a Vector
        // - Confirmar recepción

        // Simular procesamiento exitoso
        let receipt_id = format!("receipt_{}", Uuid::new_v4());
        let receipt_time = prost_types::Timestamp::from(std::time::SystemTime::now());

        // Incrementar contador
        self.event_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        info!(
            tenant_id = tenant_id,
            event_id = event_id,
            receipt_id = receipt_id,
            "Event published successfully"
        );

        let response = PublishEventResponse {
            receipt_id,
            receipt_time: Some(receipt_time),
        };

        Ok(Response::new(response))
    }

    /// Publicar un lote de eventos de auditoría (recomendado para performance)
    async fn publish_batch(
        &self,
        request: Request<PublishBatchRequest>,
    ) -> Result<Response<PublishBatchResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id.clone();
        let events = req.events.clone();
        let batch_size = events.len();

        info!(
            tenant_id = tenant_id,
            batch_size = batch_size,
            "Received PublishBatch request"
        );

        // Validación básica
        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        if events.is_empty() {
            return Err(Status::invalid_argument("events cannot be empty"));
        }

        if batch_size > self.config.max_batch_size {
            return Err(Status::invalid_argument(format!(
                "batch size {} exceeds maximum {}",
                batch_size, self.config.max_batch_size
            )));
        }

        // Validar cada evento
        for (i, event) in events.iter().enumerate() {
            // event es &AuditEvent
            let event_id = event.event_id.clone().unwrap_or_default().value;
            if event_id.is_empty() {
                return Err(Status::invalid_argument(format!(
                    "event at index {} missing event_id",
                    i
                )));
            }
        }

        // TODO: Implementar lógica de batch
        // - Procesar en paralelo
        // - Enviar a Vector con compresión
        // - Manejar errores individuales
        // - Confirmar recepción por lote

        let batch_id = format!("batch_{}", Uuid::new_v4());
        let receipt_time = prost_types::Timestamp::from(std::time::SystemTime::now());

        // Incrementar contador
        self.event_counter
            .fetch_add(batch_size as u64, std::sync::atomic::Ordering::SeqCst);

        info!(
            tenant_id = tenant_id,
            batch_id = batch_id,
            batch_size = batch_size,
            "Batch published successfully"
        );

        let response = PublishBatchResponse {
            received_count: batch_size as i32,
            batch_id,
            receipt_time: Some(receipt_time),
            failed_events: vec![], // TODO: Implementar tracking de errores
        };

        Ok(Response::new(response))
    }

    /// Health check del servicio
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let _req = request.into_inner();

        info!("Health check requested for AuditControlService");

        let response = HealthCheckResponse {
            status: HealthStatus::StatusServing as i32,
            message: "AuditControlService is healthy".to_string(),
            details: {
                let mut details = std::collections::HashMap::new();
                details.insert("service".to_string(), "audit_control".to_string());
                details.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
                details.insert(
                    "events_received".to_string(),
                    self.get_event_count().to_string(),
                );
                details.insert("port".to_string(), "50052".to_string());
                details
            },
            healthy: true,
            status_msg: "OK".to_string(),
            metrics: std::collections::HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
