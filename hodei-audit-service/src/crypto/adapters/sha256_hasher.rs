//! Adapter SHA-256 para hashing
//!
//! Implementación高性能 para hashing SHA-256.

use crate::crypto::ports::hashing::{HashResult, HashingError, HashingService};
use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Servicio de hashing SHA-256
#[derive(Debug, Clone)]
pub struct Sha256Hasher;

impl Sha256Hasher {
    /// Crear nuevo hasher
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HashingService for Sha256Hasher {
    fn hash_data(&self, data: &[u8]) -> Result<HashResult, HashingError> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    async fn hash_file(&self, path: &Path) -> Result<HashResult, HashingError> {
        let mut file = File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192]; // Buffer de 8KB para alto rendimiento

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    async fn hash_files(&self, paths: &[&Path]) -> Result<Vec<(String, HashResult)>, HashingError> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            let hash = self.hash_file(path).await?;
            results.push((path.display().to_string(), hash));
        }

        Ok(results)
    }

    fn verify_hash(&self, data: &[u8], expected_hash: &str) -> Result<bool, HashingError> {
        let computed_hash = self.hash_data(data)?;
        Ok(computed_hash.to_lowercase() == expected_hash.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_data() {
        let hasher = Sha256Hasher::new();
        let result = hasher.hash_data(b"hello world").unwrap();
        assert_eq!(result.len(), 64); // SHA-256 hex
        assert_eq!(
            result,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[tokio::test]
    async fn test_verify_hash_valid() {
        let hasher = Sha256Hasher::new();
        let data = b"test data";
        let hash = hasher.hash_data(data).unwrap();
        assert!(hasher.verify_hash(data, &hash).unwrap());
    }

    #[tokio::test]
    async fn test_verify_hash_invalid() {
        let hasher = Sha256Hasher::new();
        assert!(!hasher.verify_hash(b"data", "wronghash").unwrap());
    }
}
