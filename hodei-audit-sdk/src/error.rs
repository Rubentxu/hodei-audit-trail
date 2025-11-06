//! Error types para el SDK

use thiserror::Error;

/// Error del SDK de auditoría
#[derive(Error, Debug)]
pub enum AuditError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::transport::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("HRN error: {0}")]
    HrnError(String),
}

impl AuditError {
    /// Crear error de configuración
    pub fn config_error(msg: &str) -> Self {
        Self::ConfigurationError(msg.to_string())
    }
}
