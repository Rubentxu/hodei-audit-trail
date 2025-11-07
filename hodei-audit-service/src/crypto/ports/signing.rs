//! Port para firma digital
//!
//! Abstracción para firmar y verificar datos usando Ed25519.

use async_trait::async_trait;
use thiserror::Error;

/// Errores del servicio de firma
#[derive(Debug, Error)]
pub enum SigningError {
    #[error("Error de clave: {0}")]
    Key(String),

    #[error("Error de firma: {0}")]
    Signature(String),

    #[error("Error de serialización: {0}")]
    Serialization(String),
}

impl From<ed25519_dalek::ed25519::Error> for SigningError {
    fn from(_: ed25519_dalek::ed25519::Error) -> Self {
        SigningError::Key("Invalid key data".to_string())
    }
}

/// Par de claves Ed25519
#[derive(Debug, Clone)]
pub struct KeyPair {
    /// Clave pública para verificación
    pub public_key: Vec<u8>,
    /// Clave privada para firma (almacenada de forma segura)
    pub private_key: Vec<u8>,
}

/// Port para firma digital
#[async_trait]
pub trait SigningService: Send + Sync + 'static {
    /// Genera un nuevo par de claves
    fn generate_keypair(&self) -> Result<KeyPair, SigningError>;

    /// Firma un digest con la clave privada
    fn sign(&self, digest: &str, private_key: &[u8]) -> Result<Vec<u8>, SigningError>;

    /// Verifica una firma con la clave pública
    fn verify(
        &self,
        digest: &str,
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, SigningError>;

    /// Obtiene la clave pública desde la privada
    fn get_public_key(&self, private_key: &[u8]) -> Result<Vec<u8>, SigningError>;
}
