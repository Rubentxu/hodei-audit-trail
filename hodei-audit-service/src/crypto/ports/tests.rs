//! Tests unitarios para ports de crypto
//!
//! Validan las abstracciones y contratos.

#[cfg(test)]
mod hashing_tests {
    use super::super::hashing::{HashingError, HashingService};
    use crate::crypto::adapters::Sha256Hasher;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_hash_data_correct() {
        let hasher = Sha256Hasher::new();
        let data = b"Hello, World!";
        let hash = hasher.hash_data(data).unwrap();

        assert_eq!(hash.len(), 64);
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
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

    #[tokio::test]
    async fn test_hash_file() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, b"test content").await.unwrap();

        let hasher = Sha256Hasher::new();
        let hash = hasher.hash_file(&file_path).await.unwrap();

        assert_eq!(hash.len(), 64);
    }

    #[tokio::test]
    async fn test_hash_files_multiple() {
        let tmp_dir = tempdir().unwrap();
        let file1 = tmp_dir.path().join("file1.txt");
        let file2 = tmp_dir.path().join("file2.txt");

        tokio::fs::write(&file1, b"content1").await.unwrap();
        tokio::fs::write(&file2, b"content2").await.unwrap();

        let hasher = Sha256Hasher::new();
        let results = hasher.hash_files(&[&file1, &file2]).await.unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, file1.display().to_string());
        assert_eq!(results[1].0, file2.display().to_string());
        assert_eq!(results[0].1.len(), 64);
        assert_eq!(results[1].1.len(), 64);
    }

    #[tokio::test]
    async fn test_hash_nonexistent_file() {
        let hasher = Sha256Hasher::new();
        let path = PathBuf::from("/nonexistent/file.txt");

        let result = hasher.hash_file(&path).await;
        assert!(matches!(result, Err(HashingError::Io(_))));
    }
}

#[cfg(test)]
mod signing_tests {
    use super::super::signing::{KeyPair, SigningError, SigningService};
    use crate::crypto::adapters::Ed25519Signer;

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
        let message = "test message for signing";

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

    #[test]
    fn test_verify_wrong_key() {
        let signer = Ed25519Signer::new();
        let keypair1 = signer.generate_keypair().unwrap();
        let keypair2 = signer.generate_keypair().unwrap();
        let message = "test message";
        let signature = signer.sign(message, &keypair1.private_key).unwrap();

        // Intentar verificar con clave pública diferente
        let is_valid = signer
            .verify(message, &signature, &keypair2.public_key)
            .unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_get_public_key() {
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();

        let public_key = signer.get_public_key(&keypair.private_key).unwrap();
        assert_eq!(public_key.len(), 32);
        assert_eq!(public_key, keypair.public_key);
    }

    #[test]
    fn test_sign_with_invalid_key() {
        let signer = Ed25519Signer::new();
        let result = signer.sign("message", &[1, 2, 3]);
        assert!(matches!(result, Err(SigningError::Key(_))));
    }

    #[test]
    fn test_verify_invalid_signature() {
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();
        let result = signer.verify("message", &[1, 2, 3], &keypair.public_key);
        assert!(matches!(result, Err(SigningError::Signature(_))));
    }
}

#[cfg(test)]
mod digest_chain_tests {
    use super::super::digest_chain::{DigestChainError, DigestChainService, DigestInfo};
    use crate::crypto::adapters::InMemoryDigestChain;

    #[tokio::test]
    async fn test_generate_digest() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![
            ("file1.parquet", "hash1".to_string()),
            ("file2.parquet", "hash2".to_string()),
        ];

        let digest = chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        assert_eq!(digest.tenant_id, "tenant1");
        assert_eq!(digest.timestamp, 2000);
        assert_eq!(digest.total_files, 2);
        assert!(digest.previous_digest_id.is_none());
    }

    #[tokio::test]
    async fn test_generate_digest_with_previous() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        let digest1 = chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        let digest2 = chain
            .generate_digest("tenant1", 2000, 3000, &file_hashes, Some(&digest1.id))
            .await
            .unwrap();

        assert_eq!(digest2.previous_digest_id, Some(digest1.id));
    }

    #[tokio::test]
    async fn test_verify_digest_exists() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        let digest = chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        assert!(chain.verify_digest(&digest.id).await.unwrap());
    }

    #[tokio::test]
    async fn test_verify_digest_not_exists() {
        let chain = InMemoryDigestChain::new();
        assert!(!chain.verify_digest("nonexistent").await.unwrap());
    }

    #[tokio::test]
    async fn test_get_latest_digest() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        chain
            .generate_digest("tenant1", 2000, 3000, &file_hashes, None)
            .await
            .unwrap();

        let latest = chain.get_latest_digest("tenant1").await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().timestamp, 3000);
    }

    #[tokio::test]
    async fn test_get_latest_digest_none() {
        let chain = InMemoryDigestChain::new();
        let latest = chain.get_latest_digest("nonexistent").await.unwrap();
        assert!(latest.is_none());
    }

    #[tokio::test]
    async fn test_list_digests() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        chain
            .generate_digest("tenant1", 2000, 3000, &file_hashes, None)
            .await
            .unwrap();

        let digests = chain.list_digests("tenant1", None, None).await.unwrap();
        assert_eq!(digests.len(), 2);
    }

    #[tokio::test]
    async fn test_list_digests_with_time_filter() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        chain
            .generate_digest("tenant1", 3000, 4000, &file_hashes, None)
            .await
            .unwrap();

        let digests = chain
            .list_digests("tenant1", Some(2500), Some(3500))
            .await
            .unwrap();
        assert_eq!(digests.len(), 1);
    }

    #[tokio::test]
    async fn test_verify_chain_valid() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        let digest1 = chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        let digest2 = chain
            .generate_digest("tenant1", 2000, 3000, &file_hashes, Some(&digest1.id))
            .await
            .unwrap();

        let digest3 = chain
            .generate_digest("tenant1", 3000, 4000, &file_hashes, Some(&digest2.id))
            .await
            .unwrap();

        assert!(chain.verify_chain("tenant1").await.unwrap());
    }

    #[tokio::test]
    async fn test_verify_chain_broken() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        // Crear digest con referencia incorrecta
        let mut digests = {
            let digests_map = chain.digests.read().await;
            digests_map.get("tenant1").unwrap().clone()
        };
        digests[0].previous_digest_id = Some("wrong_id".to_string());

        let mut digests_map = chain.digests.write().await;
        digests_map.insert("tenant1".to_string(), digests);

        assert!(!chain.verify_chain("tenant1").await.unwrap());
    }
}
