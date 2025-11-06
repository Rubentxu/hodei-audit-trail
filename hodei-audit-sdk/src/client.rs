//! Cliente manual para logging de eventos de auditoría
//!
//! Este módulo proporciona el `AuditClient` para logging manual de eventos
//! que no se capturan automáticamente a través del middleware.

use crate::config::AuditSdkConfig;
use crate::error::AuditError;
use crate::hrn::Hrn;
use crate::models::AuditEvent;
use std::sync::Arc;
use tonic::transport::Channel;

/// Cliente manual para logging de auditoría
#[derive(Debug, Clone)]
pub struct AuditClient {
    /// Configuración del cliente
    config: Arc<AuditSdkConfig>,
    /// Canal gRPC (en una implementación real sería un cliente gRPC)
    _channel: Option<Channel>,
}

impl AuditClient {
    /// Crear un nuevo cliente de auditoría
    pub async fn new(url: String) -> Result<Self, AuditError> {
        // En una implementación real, aquí crearíamos el canal gRPC
        let config = AuditSdkConfig::default();
        let _channel = Channel::from_shared(url)
            .map_err(|e| AuditError::ConfigurationError(format!("Invalid URL: {}", e)))? // Change this to GrpcError when tonic is available
            .connect()
            .await
            .ok(); // En la implementación actual, no conectamos realmente

        Ok(Self {
            config: Arc::new(config),
            _channel,
        })
    }

    /// Crear un nuevo cliente con configuración personalizada
    pub async fn with_config(config: AuditSdkConfig) -> Result<Self, AuditError> {
        Ok(Self {
            config: Arc::new(config),
            _channel: None,
        })
    }

    /// Log de un evento individual
    pub async fn log(&self, event: AuditEvent) -> Result<(), AuditError> {
        // En una implementación real, aquí enviaríamos el evento vía gRPC
        // Por ahora, solo logueamos
        tracing::debug!("Logging audit event: {}", event.event_name);

        // Simular envío exitoso
        Ok(())
    }

    /// Log de múltiples eventos en batch
    pub async fn log_batch(&self, events: Vec<AuditEvent>) -> Result<(), AuditError> {
        if events.is_empty() {
            return Ok(());
        }

        // En una implementación real, aquí enviaríamos el batch vía gRPC
        tracing::debug!("Logging batch of {} audit events", events.len());

        // Simular envío exitoso
        Ok(())
    }

    /// Consultar eventos de auditoría
    pub async fn query(
        &self,
        _query: crate::types::AuditQuery,
    ) -> Result<AuditQueryResult, AuditError> {
        // En una implementación real, aquí consultaríamos el servicio
        // Por ahora, retornamos un resultado vacío
        Ok(AuditQueryResult {
            total: 0,
            events: vec![],
        })
    }

    /// Resolver metadata de un HRN
    pub async fn resolve_hrn(&self, _hrn: Hrn) -> Result<crate::config::HrnMetadata, AuditError> {
        // En una implementación real, aquí consultaríamos metadata del HRN
        // Por ahora, retornamos metadata por defecto
        Ok(crate::config::HrnMetadata {
            display_name: "Unknown Resource".to_string(),
            tags: vec![],
            resource_type: "unknown".to_string(),
            additional_data: None,
        })
    }

    /// Obtener la configuración del cliente
    pub fn config(&self) -> &AuditSdkConfig {
        &self.config
    }
}

/// Resultado de una consulta de auditoría
#[derive(Debug, Clone)]
pub struct AuditQueryResult {
    /// Total de eventos encontrados
    pub total: u64,
    /// Lista de eventos
    pub events: Vec<AuditEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AuditEvent;
    use std::time::Duration;

    #[tokio::test]
    async fn test_audit_client_creation() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        assert_eq!(client.config().service_name, "unknown-service");
    }

    #[tokio::test]
    async fn test_audit_client_with_config() {
        let config = AuditSdkConfig::builder()
            .service_name("test-service")
            .tenant_id("test-tenant")
            .batch_size(50)
            .build()
            .unwrap();

        let client = AuditClient::with_config(config).await.unwrap();

        assert_eq!(client.config().service_name, "test-service");
        assert_eq!(client.config().tenant_id, Some("test-tenant".to_string()));
        assert_eq!(client.config().batch_size, 50);
    }

    #[tokio::test]
    async fn test_log_single_event() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        let event = AuditEvent {
            event_name: "test-event".to_string(),
            event_category: 0,
            hrn: "hrn:hodei:api:tenant-123:global:user/test".to_string(),
            user_id: "user-123".to_string(),
            tenant_id: "tenant-123".to_string(),
            trace_id: "trace-456".to_string(),
            resource_path: "/api/test".to_string(),
            http_method: Some("GET".to_string()),
            http_status: Some(200),
            source_ip: None,
            user_agent: None,
            additional_data: None,
        };

        let result = client.log(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_log_batch_events() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        let events = vec![
            AuditEvent {
                event_name: "event-1".to_string(),
                event_category: 0,
                hrn: "hrn:hodei:api:tenant-123:global:user/test1".to_string(),
                user_id: "user-123".to_string(),
                tenant_id: "tenant-123".to_string(),
                trace_id: "trace-456".to_string(),
                resource_path: "/api/test1".to_string(),
                http_method: Some("GET".to_string()),
                http_status: Some(200),
                source_ip: None,
                user_agent: None,
                additional_data: None,
            },
            AuditEvent {
                event_name: "event-2".to_string(),
                event_category: 0,
                hrn: "hrn:hodei:api:tenant-123:global:user/test2".to_string(),
                user_id: "user-123".to_string(),
                tenant_id: "tenant-123".to_string(),
                trace_id: "trace-456".to_string(),
                resource_path: "/api/test2".to_string(),
                http_method: Some("POST".to_string()),
                http_status: Some(201),
                source_ip: None,
                user_agent: None,
                additional_data: None,
            },
        ];

        let result = client.log_batch(events).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_log_empty_batch() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        let result = client.log_batch(vec![]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_events() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        let query = crate::types::AuditQuery {
            hrn: Some("hrn:hodei:api:tenant-123:global:user/test".to_string()),
            tenant_id: Some("tenant-123".to_string()),
            user_id: Some("user-123".to_string()),
            start_time: None,
            end_time: None,
            limit: 100,
            offset: 0,
        };

        let result = client.query(query).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().total, 0);
    }

    #[tokio::test]
    async fn test_resolve_hrn() {
        let client = AuditClient::new("http://audit-service:50052".to_string())
            .await
            .unwrap();

        let hrn = Hrn::parse("hrn:hodei:api:tenant-123:global:user/test").unwrap();

        let result = client.resolve_hrn(hrn).await;
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.resource_type, "unknown");
    }
}
