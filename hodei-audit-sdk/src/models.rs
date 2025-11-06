//! Modelos de datos para eventos de auditoría

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Evento de auditoría compatible con CloudTrail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuditEvent {
    /// Identificador único del evento
    pub event_id: String,
    /// Timestamp del evento
    pub event_time: DateTime<Utc>,
    /// Fuente del evento
    pub event_source: String,
    /// Nombre de la operación
    pub event_name: String,
    /// Categoría del evento
    pub event_category: EventCategory,
    /// Solo lectura o modificación
    pub read_only: bool,
    /// HRN del recurso
    pub hrn: String,
    /// IP de origen
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Datos adicionales
    pub additional_data: Option<serde_json::Value>,
}

/// Categoría de evento (CloudTrail)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventCategory {
    Management,
    Data,
    Insight,
}

/// Builder para eventos de auditoría
pub struct EventBuilder {
    event_name: Option<String>,
    event_category: Option<EventCategory>,
    hrn: Option<String>,
    source_ip: Option<String>,
    user_agent: Option<String>,
    additional_data: Option<serde_json::Value>,
    read_only: bool,
}

impl EventBuilder {
    /// Crear nuevo builder
    pub fn new() -> Self {
        Self {
            event_name: None,
            event_category: None,
            hrn: None,
            source_ip: None,
            user_agent: None,
            additional_data: None,
            read_only: true,
        }
    }

    /// Configurar nombre del evento
    pub fn event_name(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    /// Configurar categoría del evento
    pub fn event_category(mut self, category: EventCategory) -> Self {
        self.event_category = Some(category);
        self
    }

    /// Configurar HRN
    pub fn hrn(mut self, hrn: &str) -> Self {
        self.hrn = Some(hrn.to_string());
        self
    }

    /// Configurar IP de origen
    pub fn source_ip(mut self, ip: &str) -> Self {
        self.source_ip = Some(ip.to_string());
        self
    }

    /// Configurar user agent
    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }

    /// Configurar datos adicionales
    pub fn additional_data(mut self, data: serde_json::Value) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Marcar como modificación (no solo lectura)
    pub fn read_write(mut self) -> Self {
        self.read_only = false;
        self
    }

    /// Construir el evento
    pub fn build(self) -> Result<AuditEvent, crate::error::AuditError> {
        let event_name = self.event_name.ok_or_else(|| {
            crate::error::AuditError::ParseError("event_name is required".to_string())
        })?;

        let event_category = self.event_category.ok_or_else(|| {
            crate::error::AuditError::ParseError("event_category is required".to_string())
        })?;

        let hrn = self
            .hrn
            .ok_or_else(|| crate::error::AuditError::ParseError("hrn is required".to_string()))?;

        Ok(AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            event_time: Utc::now(),
            event_source: "hodei.audit.sdk".to_string(),
            event_name,
            event_category,
            read_only: self.read_only,
            hrn,
            source_ip: self.source_ip,
            user_agent: self.user_agent,
            additional_data: self.additional_data,
        })
    }
}
