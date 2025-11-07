//! Port para gestión de claves
//!
//! Abstracción principal para gestión de claves criptográficas.

use async_trait::async_trait;
use thiserror::Error;

/// Errores del KeyManager
#[derive(Debug, Error)]
pub enum KeyManagerError {
    #[error("Error de generación: {0}")]
    Generation(String),

    #[error("Error de almacenamiento: {0}")]
    Storage(String),

    #[error("Error de carga: {0}")]
    Load(String),

    #[error("Error de rotación: {0}")]
    Rotation(String),

    #[error("Clave no encontrada")]
    NotFound,
}

/// Información de una clave
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyInfo {
    pub id: String,
    pub tenant_id: String,
    pub public_key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub is_active: bool,
    pub version: u32,
}

/// Manifiesto de claves públicas
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeysManifest {
    pub version: String,
    pub keys: Vec<KeyInfo>,
    pub root_signature: Vec<u8>,
    pub manifest_hash: String,
    pub issued_at: u64,
}

/// Port para gestión de claves
#[async_trait]
pub trait KeyManager: Send + Sync + 'static {
    /// Generar nueva clave para un tenant
    async fn generate_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError>;

    /// Rotar clave de un tenant
    async fn rotate_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError>;

    /// Obtener clave activa de un tenant
    async fn get_active_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError>;

    /// Obtener manifiesto de claves públicas
    async fn get_manifest(&self, tenant_id: &str) -> Result<KeysManifest, KeyManagerError>;

    /// Verificar si una clave es válida
    async fn verify_key(&self, tenant_id: &str, key_id: &str) -> Result<bool, KeyManagerError>;
}
