//! Módulo de criptografía

use anyhow::Result;
use ed25519_dalek::{Signer as EdSigner, SigningKey, Verifier as EdVerifier, VerifyingKey};
use sha2::{Digest, Sha256};
use signature::{Signature, Signer, Verifier};

/// Generar digest SHA-256 para un evento
pub fn generate_digest(event_data: &str, previous_hash: Option<&str>) -> Result<String> {
    let mut hasher = Sha256::new();

    if let Some(prev_hash) = previous_hash {
        hasher.update(prev_hash);
    }

    hasher.update(event_data);

    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Firmar digest con ed25519
pub fn sign_digest(digest: &str, private_key: &SigningKey) -> Result<String> {
    let signature = private_key.sign(digest.as_bytes());
    Ok(hex::encode(signature.to_bytes()))
}

/// Verificar firma de digest
pub fn verify_digest(digest: &str, signature: &str, public_key: &VerifyingKey) -> Result<bool> {
    let signature_bytes = hex::decode(signature)?;
    let signature_bytes = signature_bytes[..64]
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid signature length"))?;

    let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes);
    Ok(public_key.verify(digest.as_bytes(), &signature).is_ok())
}

/// Generar par de claves ed25519
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::from_bytes(&rand::random::<[u8; 32]>());
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_digest() {
        let digest = generate_digest("test data", None).unwrap();
        assert_eq!(digest.len(), 64); // SHA-256 hex = 64 chars
    }

    #[test]
    fn test_keypair_generation() {
        let (signing_key, verifying_key) = generate_keypair();

        // Firmar y verificar
        let digest = "test message";
        let signature = sign_digest(digest, &signing_key).unwrap();
        let is_valid = verify_digest(digest, &signature, &verifying_key).unwrap();

        assert!(is_valid);
    }
}
