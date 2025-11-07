//! Adapter Ed25519 para firma digital
//!
//! Implementación segura para firma y verificación Ed25519.

use crate::crypto::ports::signing::{KeyPair, SigningError, SigningService};
use async_trait::async_trait;
use ed25519_dalek::{Signer as EdSigner, SigningKey, Verifier as EdVerifier, VerifyingKey};

/// Servicio de firma Ed25519
#[derive(Debug, Clone)]
pub struct Ed25519Signer;

impl Ed25519Signer {
    /// Crear nuevo signer
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SigningService for Ed25519Signer {
    fn generate_keypair(&self) -> Result<KeyPair, SigningError> {
        // Usar getrandom para obtener aleatoriedad criptográficamente segura
        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed)
            .map_err(|e| SigningError::Key(format!("Failed to generate random key: {}", e)))?;

        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        Ok(KeyPair {
            public_key: verifying_key.to_bytes().to_vec(),
            private_key: signing_key.to_bytes().to_vec(),
        })
    }

    fn sign(&self, digest: &str, private_key: &[u8]) -> Result<Vec<u8>, SigningError> {
        if private_key.len() != 32 {
            return Err(SigningError::Key("Invalid private key length".to_string()));
        }

        let signing_key = SigningKey::from_bytes(
            private_key
                .try_into()
                .map_err(|_| SigningError::Key("Failed to convert private key".to_string()))?,
        );

        let signature = signing_key.sign(digest.as_bytes());
        Ok(signature.to_bytes().to_vec())
    }

    fn verify(
        &self,
        digest: &str,
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, SigningError> {
        if public_key.len() != 32 {
            return Err(SigningError::Key("Invalid public key length".to_string()));
        }

        if signature.len() < 64 {
            return Err(SigningError::Signature(
                "Invalid signature length".to_string(),
            ));
        }

        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .map_err(|_| SigningError::Key("Failed to convert public key".to_string()))?,
        )?;

        let signature_bytes = signature[..64]
            .try_into()
            .map_err(|_| SigningError::Signature("Failed to convert signature".to_string()))?;

        let signature = ed25519_dalek::Signature::from_bytes(signature_bytes);

        match verifying_key.verify(digest.as_bytes(), &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get_public_key(&self, private_key: &[u8]) -> Result<Vec<u8>, SigningError> {
        if private_key.len() != 32 {
            return Err(SigningError::Key("Invalid private key length".to_string()));
        }

        let signing_key = SigningKey::from_bytes(
            private_key
                .try_into()
                .map_err(|_| SigningError::Key("Failed to convert private key".to_string()))?,
        );

        let verifying_key = signing_key.verifying_key();
        Ok(verifying_key.to_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();

        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.private_key.len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();
        let message = "test message";

        let signature = signer.sign(message, &keypair.private_key).unwrap();
        assert_eq!(signature.len(), 64);

        let is_valid = signer
            .verify(message, &signature, &keypair.public_key)
            .unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_wrong_message() {
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();
        let signature = signer.sign("message", &keypair.private_key).unwrap();

        // Intentar verificar un mensaje diferente
        let is_valid = signer
            .verify("different message", &signature, &keypair.public_key)
            .unwrap();
        assert!(!is_valid);
    }
}
