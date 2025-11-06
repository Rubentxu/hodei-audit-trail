//! Configuración del SDK

/// Configuración para el cliente de auditoría
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Endpoint del servicio Hodei Audit
    pub endpoint: String,
    /// ID del tenant
    pub tenant_id: String,
    /// Timeout para requests (en segundos)
    pub timeout_seconds: u64,
    /// Buffer size para batching
    pub buffer_size: usize,
}

impl AuditConfig {
    /// Crear un builder para la configuración
    pub fn builder() -> AuditConfigBuilder {
        AuditConfigBuilder::new()
    }
}

/// Builder para AuditConfig
pub struct AuditConfigBuilder {
    endpoint: Option<String>,
    tenant_id: Option<String>,
    timeout_seconds: Option<u64>,
    buffer_size: Option<usize>,
}

impl AuditConfigBuilder {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            tenant_id: None,
            timeout_seconds: Some(30),
            buffer_size: Some(1000),
        }
    }

    /// Configurar endpoint
    pub fn endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.to_string());
        self
    }

    /// Configurar tenant ID
    pub fn tenant_id(mut self, tenant_id: &str) -> Self {
        self.tenant_id = Some(tenant_id.to_string());
        self
    }

    /// Configurar timeout
    pub fn timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Configurar buffer size
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = Some(buffer_size);
        self
    }

    /// Construir la configuración
    pub fn build(self) -> Result<AuditConfig, crate::error::AuditError> {
        let endpoint = self.endpoint.ok_or_else(|| {
            crate::error::AuditError::ConfigurationError("endpoint is required".to_string())
        })?;

        let tenant_id = self.tenant_id.ok_or_else(|| {
            crate::error::AuditError::ConfigurationError("tenant_id is required".to_string())
        })?;

        Ok(AuditConfig {
            endpoint,
            tenant_id,
            timeout_seconds: self.timeout_seconds.unwrap_or(30),
            buffer_size: self.buffer_size.unwrap_or(1000),
        })
    }
}
