//! Port para hashing criptográfico
//!
//! Abstracción para generar hashes criptográficos de datos.

use async_trait::async_trait;
use std::path::Path;
use thiserror::Error;

/// Errores del servicio de hashing
#[derive(Debug, Error)]
pub enum HashingError {
    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de hash: {0}")]
    Hash(String),
}

/// Resultado de hashing
pub type HashResult = String;

/// Port para hashing de datos
#[async_trait]
pub trait HashingService: Send + Sync + 'static {
    /// Genera un hash SHA-256 de los datos en memoria
    fn hash_data(&self, data: &[u8]) -> Result<HashResult, HashingError>;

    /// Genera un hash SHA-256 de un archivo en disco
    async fn hash_file(&self, path: &Path) -> Result<HashResult, HashingError>;

    /// Genera un hash SHA-256 de múltiples archivos
    async fn hash_files(&self, paths: &[&Path]) -> Result<Vec<(String, HashResult)>, HashingError>;

    /// Verifica que un hash coincida con los datos
    fn verify_hash(&self, data: &[u8], expected_hash: &str) -> Result<bool, HashingError>;
}
