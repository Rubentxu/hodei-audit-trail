//! Hodei Audit SDK
//!
//! Middleware para captura automática de eventos de auditoría en aplicaciones Axum.
//! Proporciona integración con 1-liner y configuración flexible.
//!
//! # Ejemplo de Uso
//!
//! ```rust
//! use hodei_audit_sdk::AuditSdkConfig;
//!
//! // 1. Configurar el SDK
//! let config = AuditSdkConfig::builder()
//!     .service_name("my-service")
//!     .tenant_id("tenant-123")
//!     .audit_service_url("http://audit-service:50052")
//!     .batch_size(100)
//!     .enable_request_body(true)
//!     .enable_response_body(false)
//!     .build().unwrap();
//!
//! // 2. Crear el layer de auditoría
//! let layer = config.layer();
//! // 3. Aplicar como middleware en Axum: .layer(layer)
//! ```
//!
//! # Features Flags
//!
//! - `request-body`: Captura el cuerpo de la request
//! - `response-body`: Captura el cuerpo de la response
//! - `hrn-resolution`: Habilita resolución de HRN
//! - `custom-enricher`: Habilita enrichers personalizados

pub mod batch;
pub mod client;
pub mod config;
pub mod error;
pub mod hrn;
pub mod middleware;
pub mod models;
pub mod types;

pub use batch::{BatchQueue, BatchStats, FlushPolicy, GrpcConnectionPool, RetryConfig, RetryError};
pub use client::{AuditClient, AuditQueryResult};
pub use config::{AuditConfigBuilder, AuditSdkConfig, HrnMetadata, HrnResolver};
pub use error::AuditError;
pub use hrn::{Hrn, enrich_event_with_hrn, generate_hrn_from_path};
pub use middleware::AuditLayer;
pub use models::{AuditEvent, EventBuilder};
pub use types::AuditQuery;

/// Resultado de operaciones del SDK
pub type Result<T> = std::result::Result<T, AuditError>;

// Re-export types for convenience
pub use config::AuditSdkConfig as AuditConfig;

/// Trait para extensiones de auditoría
pub trait AuditExtensions {
    /// Obtener el nombre del servicio
    fn service_name(&self) -> &str;

    /// Obtener el tenant ID
    fn tenant_id(&self) -> Option<&str>;

    /// Verificar si está habilitado el capture de request body
    fn request_body_enabled(&self) -> bool;

    /// Verificar si está habilitado el capture de response body
    fn response_body_enabled(&self) -> bool;
}

impl AuditExtensions for AuditSdkConfig {
    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn tenant_id(&self) -> Option<&str> {
        self.tenant_id.as_deref()
    }

    fn request_body_enabled(&self) -> bool {
        self.enable_request_body
    }

    fn response_body_enabled(&self) -> bool {
        self.enable_response_body
    }
}
