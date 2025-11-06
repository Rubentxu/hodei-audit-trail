//! Cliente gRPC para Hodei Audit Service

use anyhow::Context;
use tonic::transport::Channel;

use crate::{config::AuditConfig, error::AuditError, models::AuditEvent};

/// Cliente principal del SDK de auditoría
pub struct AuditClient {
    /// Configuración del cliente
    config: AuditConfig,
    /// Canal gRPC
    channel: Channel,
    // TODO: Agregar clientes para cada servicio gRPC
    // audit_control_client: audit_control_service_client::AuditControlServiceClient<Channel>,
    // audit_query_client: audit_query_service_client::AuditQueryServiceClient<Channel>,
}

impl AuditClient {
    /// Crear nuevo cliente de auditoría
    pub async fn new(config: AuditConfig) -> Result<Self, AuditError> {
        let channel = Channel::from_shared(config.endpoint.clone())
            .context("Invalid endpoint")
            .map_err(AuditError::ConfigurationError)?;

        Ok(Self {
            config,
            channel,
            // audit_control_client: AuditControlServiceClient::new(channel.clone()),
            // audit_query_client: AuditQueryServiceClient::new(channel),
        })
    }

    /// Publicar un evento de auditoría
    pub async fn publish_event(&self, event: AuditEvent) -> Result<(), AuditError> {
        // TODO: Implementar publicación de evento
        tracing::info!("Publishing event: {:?}", event.event_name);
        Ok(())
    }

    /// Publicar múltiples eventos (batching)
    pub async fn publish_batch(&self, events: Vec<AuditEvent>) -> Result<(), AuditError> {
        // TODO: Implementar batch publishing
        tracing::info!("Publishing batch of {} events", events.len());
        Ok(())
    }

    /// Consultar eventos
    pub async fn query_events(&self, _query: String) -> Result<Vec<AuditEvent>, AuditError> {
        // TODO: Implementar query
        Ok(vec![])
    }
}
