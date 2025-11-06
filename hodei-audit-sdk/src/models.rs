//! Modelos de datos para eventos de auditoría

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Evento de auditoría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Nombre de la operación (e.g., "GET /api/users")
    pub event_name: String,
    /// Categoría del evento (Management/Data/Insight)
    pub event_category: i32,
    /// HRN del recurso
    pub hrn: String,
    /// User ID
    pub user_id: String,
    /// Tenant ID
    pub tenant_id: String,
    /// Trace ID
    pub trace_id: String,
    /// Resource path
    pub resource_path: String,
    /// HTTP method
    pub http_method: Option<String>,
    /// HTTP status
    pub http_status: Option<i32>,
    /// IP de origen
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Additional metadata
    pub additional_data: Option<serde_json::Value>,
}

impl Default for AuditEvent {
    fn default() -> Self {
        Self {
            event_name: "unknown".to_string(),
            event_category: 0,
            hrn: "".to_string(),
            user_id: "anonymous".to_string(),
            tenant_id: "unknown".to_string(),
            trace_id: "no-trace".to_string(),
            resource_path: "".to_string(),
            http_method: None,
            http_status: None,
            source_ip: None,
            user_agent: None,
            additional_data: None,
        }
    }
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
    user_id: Option<String>,
    tenant_id: Option<String>,
    trace_id: Option<String>,
    resource_path: Option<String>,
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
            user_id: None,
            tenant_id: None,
            trace_id: None,
            resource_path: None,
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

    /// Configurar user ID
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    /// Configurar tenant ID
    pub fn tenant_id(mut self, tenant_id: &str) -> Self {
        self.tenant_id = Some(tenant_id.to_string());
        self
    }

    /// Configurar trace ID
    pub fn trace_id(mut self, trace_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self
    }

    /// Configurar resource path
    pub fn resource_path(mut self, path: &str) -> Self {
        self.resource_path = Some(path.to_string());
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

        let hrn = self
            .hrn
            .ok_or_else(|| crate::error::AuditError::ParseError("hrn is required".to_string()))?;

        Ok(AuditEvent {
            event_name,
            event_category: self
                .event_category
                .map(|c| match c {
                    EventCategory::Management => 0,
                    EventCategory::Data => 1,
                    EventCategory::Insight => 2,
                })
                .unwrap_or(0),
            hrn,
            user_id: self.user_id.unwrap_or_else(|| "anonymous".to_string()),
            tenant_id: self.tenant_id.unwrap_or_else(|| "unknown".to_string()),
            trace_id: self.trace_id.unwrap_or_else(|| "no-trace".to_string()),
            resource_path: self.resource_path.unwrap_or_else(|| "".to_string()),
            source_ip: self.source_ip,
            user_agent: self.user_agent,
            http_method: None,
            http_status: None,
            additional_data: self.additional_data,
        })
    }
}
