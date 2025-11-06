//! Configuración del SDK de auditoría
//!
//! Este módulo proporciona la configuración del SDK usando un builder pattern
//! para facilitar la configuración flexible del middleware de auditoría.

use std::time::Duration;

/// Configuración del SDK de auditoría
#[derive(Debug, Clone)]
pub struct AuditSdkConfig {
    /// Nombre del servicio
    pub service_name: String,
    /// Tenant ID
    pub tenant_id: Option<String>,
    /// URL del servicio de auditoría
    pub audit_service_url: String,
    /// Tamaño del batch
    pub batch_size: usize,
    /// Timeout del batch
    pub batch_timeout: Duration,
    /// Habilitar captura de request body
    pub enable_request_body: bool,
    /// Habilitar captura de response body
    pub enable_response_body: bool,
    /// Timeout para requests gRPC
    pub grpc_timeout: Duration,
    /// Número de reintentos
    pub max_retries: u32,
}

impl Default for AuditSdkConfig {
    fn default() -> Self {
        Self {
            service_name: "unknown-service".to_string(),
            tenant_id: None,
            audit_service_url: "http://localhost:50052".to_string(),
            batch_size: 100,
            batch_timeout: Duration::from_millis(100),
            enable_request_body: true,
            enable_response_body: false,
            grpc_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl AuditSdkConfig {
    /// Crear un nuevo builder
    pub fn builder() -> AuditConfigBuilder {
        AuditConfigBuilder::new()
    }

    /// Crear un layer de Axum
    pub fn layer(&self) -> crate::middleware::AuditLayer {
        crate::middleware::AuditLayer::new(self.clone())
    }
}

/// Builder para AuditSdkConfig
pub struct AuditConfigBuilder {
    config: AuditSdkConfig,
}

impl AuditConfigBuilder {
    /// Crear un nuevo builder con configuración por defecto
    pub fn new() -> Self {
        Self {
            config: AuditSdkConfig::default(),
        }
    }

    /// Configurar el nombre del servicio
    pub fn service_name(mut self, service_name: &str) -> Self {
        self.config.service_name = service_name.to_string();
        self
    }

    /// Configurar el tenant ID
    pub fn tenant_id(mut self, tenant_id: &str) -> Self {
        self.config.tenant_id = Some(tenant_id.to_string());
        self
    }

    /// Configurar la URL del servicio de auditoría
    pub fn audit_service_url(mut self, url: &str) -> Self {
        self.config.audit_service_url = url.to_string();
        self
    }

    /// Configurar el tamaño del batch
    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    /// Configurar el timeout del batch
    pub fn batch_timeout(mut self, timeout: Duration) -> Self {
        self.config.batch_timeout = timeout;
        self
    }

    /// Habilitar la captura de request body
    pub fn enable_request_body(mut self, enable: bool) -> Self {
        self.config.enable_request_body = enable;
        self
    }

    /// Habilitar la captura de response body
    pub fn enable_response_body(mut self, enable: bool) -> Self {
        self.config.enable_response_body = enable;
        self
    }

    /// Configurar el timeout gRPC
    pub fn grpc_timeout(mut self, timeout: Duration) -> Self {
        self.config.grpc_timeout = timeout;
        self
    }

    /// Configurar el número máximo de reintentos
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Construir la configuración
    pub fn build(self) -> Result<AuditSdkConfig, crate::error::AuditError> {
        Ok(self.config)
    }
}

impl std::fmt::Display for AuditSdkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuditSdkConfig(service_name={}, tenant_id={:?}, audit_service_url={})",
            self.service_name, self.tenant_id, self.audit_service_url
        )
    }
}
