//! Port para almacenamiento de claves
//!
//! Abstracción para diferentes sistemas de almacenamiento de claves.

use crate::key_management::ports::key_manager::KeyInfo;
use async_trait::async_trait;
use thiserror::Error;

/// Errores del KeyStore
#[derive(Debug, Error)]
pub enum KeyStoreError {
    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de serialización: {0}")]
    Serialization(String),

    #[error("Clave no encontrada")]
    NotFound,
}

impl From<serde_json::Error> for KeyStoreError {
    fn from(e: serde_json::Error) -> Self {
        KeyStoreError::Serialization(e.to_string())
    }
}

/// Port para almacenamiento de claves
#[async_trait]
pub trait KeyStore: Send + Sync + 'static {
    /// Guardar clave
    async fn save_key(&self, key: &KeyInfo, private_key: &[u8]) -> Result<(), KeyStoreError>;

    /// Cargar clave privada
    async fn load_private_key(&self, key_id: &str) -> Result<Vec<u8>, KeyStoreError>;

    /// Cargar información de clave
    async fn load_key_info(&self, key_id: &str) -> Result<KeyInfo, KeyStoreError>;

    /// Listar claves de un tenant
    async fn list_keys(&self, tenant_id: &str) -> Result<Vec<KeyInfo>, KeyStoreError>;

    /// Marcar clave como inactiva
    async fn deactivate_key(&self, key_id: &str) -> Result<(), KeyStoreError>;
}
