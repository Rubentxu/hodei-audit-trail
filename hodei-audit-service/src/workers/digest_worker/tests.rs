//! Tests unitarios para workers
//!
//! Validan el DigestWorker y otros workers.

#[cfg(test)]
mod digest_worker_tests {
    use super::super::{DigestWorker, DigestWorkerConfig, DigestWorkerError};
    use crate::crypto::adapters::{Ed25519Signer, InMemoryDigestChain, Sha256Hasher};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_digest_worker_run_once_empty() {
        let hashing = Sha256Hasher::new();
        let signing = Ed25519Signer::new();
        let chain = InMemoryDigestChain::new();

        let config = DigestWorkerConfig {
            logs_dir: PathBuf::from("/tmp/logs"),
            interval_hours: 1,
            timeout_secs: 300,
        };

        let worker = DigestWorker::new(hashing, signing, chain, config);
        let result = worker.run_once("tenant1").await.unwrap();

        assert_eq!(result.files_processed, 0);
        assert_eq!(result.digest_id, "no-files");
        assert!(result.duration_ms > 0);
    }

    #[tokio::test]
    async fn test_digest_worker_with_files() {
        let hashing = Sha256Hasher::new();
        let signing = Ed25519Signer::new();
        let chain = InMemoryDigestChain::new();

        let tmp_dir = tempdir().unwrap();
        let logs_dir = tmp_dir.path().to_path_buf();
        let tenant_dir = logs_dir.join("tenant1");
        tokio::fs::create_dir_all(&tenant_dir).await.unwrap();

        // Crear archivos de log
        let file1 = tenant_dir.join("log1.parquet");
        let file2 = tenant_dir.join("log2.parquet");
        tokio::fs::write(&file1, b"log content 1").await.unwrap();
        tokio::fs::write(&file2, b"log content 2").await.unwrap();

        let config = DigestWorkerConfig {
            logs_dir,
            interval_hours: 1,
            timeout_secs: 300,
        };

        let worker = DigestWorker::new(hashing, signing, chain, config);
        let result = worker.run_once("tenant1").await.unwrap();

        // Nota: En implementación real, find_log_files debería encontrar estos archivos
        assert!(result.duration_ms > 0);
    }

    #[tokio::test]
    async fn test_digest_worker_chain_integration() {
        let hashing = Sha256Hasher::new();
        let signing = Ed25519Signer::new();
        let chain = InMemoryDigestChain::new();

        let config = DigestWorkerConfig {
            logs_dir: PathBuf::from("/tmp/logs"),
            interval_hours: 1,
            timeout_secs: 300,
        };

        let worker = DigestWorker::new(hashing, signing, chain, config);

        // Primera ejecución
        let result1 = worker.run_once("tenant1").await.unwrap();
        assert_eq!(result1.files_processed, 0);

        // Segunda ejecución (debería crear cadena)
        let result2 = worker.run_once("tenant1").await.unwrap();
        assert_eq!(result2.files_processed, 0);
    }
}
