//! Port para cadena de digests
//!
//! Abstracción para manejar la cadena criptográfica de digests.

use async_trait::async_trait;
use std::path::Path;
use thiserror::Error;

/// Errores del servicio de cadena de digests
#[derive(Debug, Error)]
pub enum DigestChainError {
    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error de base de datos: {0}")]
    Database(String),
}

/// Información de un digest en la cadena
#[derive(Debug, Clone)]
pub struct DigestInfo {
    pub id: String,
    pub hash: String,
    pub signature: Vec<u8>,
    pub timestamp: u64,
    pub previous_digest_id: Option<String>,
    pub total_files: usize,
    pub total_bytes: u64,
}

/// Port para la cadena de digests
#[async_trait]
pub trait DigestChainService: Send + Sync + 'static {
    /// Genera un nuevo digest y lo añade a la cadena
    async fn generate_digest(
        &self,
        tenant_id: &str,
        start_time: u64,
        end_time: u64,
        file_hashes: &[(&str, String)],
        previous_digest_id: Option<&str>,
    ) -> Result<DigestInfo, DigestChainError>;

    /// Verifica un digest y su posición en la cadena
    async fn verify_digest(&self, digest_id: &str) -> Result<bool, DigestChainError>;

    /// Obtiene el digest más reciente de un tenant
    async fn get_latest_digest(
        &self,
        tenant_id: &str,
    ) -> Result<Option<DigestInfo>, DigestChainError>;

    /// Lista los digests de un rango de tiempo
    async fn list_digests(
        &self,
        tenant_id: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<DigestInfo>, DigestChainError>;

    /// Verifica la integridad de toda la cadena
    async fn verify_chain(&self, tenant_id: &str) -> Result<bool, DigestChainError>;
}
