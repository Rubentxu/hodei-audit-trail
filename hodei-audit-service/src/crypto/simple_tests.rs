//! Test simple y directo de la implementación
//! Sin dependencias complejas de gRPC

#[cfg(test)]
mod simple_tests {
    use crate::crypto::ports::digest_chain::DigestChainService;
    use crate::crypto::ports::hashing::HashingService;
    use crate::crypto::ports::signing::{KeyPair, SigningService};
    use crate::crypto::{Ed25519Signer, InMemoryDigestChain, Sha256Hasher};

    #[tokio::test]
    async fn test_basic_crypto() {
        println!("\n=== Test Básico de Criptografía ===\n");

        // Test hashing
        let hasher = Sha256Hasher::new();
        let data = b"Hello, World!";
        let hash = hasher.hash_data(data).unwrap();
        println!("Hash SHA-256: {}...", &hash[..32]);
        assert_eq!(hash.len(), 64);

        // Test signing
        let signer = Ed25519Signer::new();
        let keypair: KeyPair = signer.generate_keypair().unwrap();
        println!("Clave pública: {} bytes", keypair.public_key.len());
        println!("Clave privada: {} bytes", keypair.private_key.len());
        assert_eq!(keypair.public_key.len(), 32);
        assert_eq!(keypair.private_key.len(), 32);

        // Test sign & verify
        let message = "Test message for signing";
        let signature = signer.sign(message, &keypair.private_key).unwrap();
        let is_valid = signer
            .verify(message, &signature, &keypair.public_key)
            .unwrap();
        assert!(is_valid);
        println!("Firma válida: ✓");

        // Test chain
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![
            ("file1.parquet", "hash_value_1".to_string()),
            ("file2.parquet", "hash_value_2".to_string()),
        ];

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let digest = chain
            .generate_digest("tenant-1", now - 3600, now, &file_hashes, None)
            .await
            .unwrap();
        println!("Digest creado: {}", digest.id);
        println!("Total archivos: {}", digest.total_files);

        let exists = chain.verify_digest(&digest.id).await.unwrap();
        assert!(exists);
        println!("Digest verificado: ✓");

        println!("\n✅ Todos los tests básicos pasaron!\n");
    }

    #[tokio::test]
    async fn test_digest_chain() {
        println!("\n=== Test de Cadena de Digests ===\n");

        let chain = InMemoryDigestChain::new();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Crear cadena de 3 digests
        let digest1 = chain
            .generate_digest("tenant-test", now - 10800, now - 7200, &[], None)
            .await
            .unwrap();
        println!("Digest 1: {} (sin previous)", digest1.id);

        let digest2 = chain
            .generate_digest(
                "tenant-test",
                now - 7200,
                now - 3600,
                &[],
                Some(&digest1.id),
            )
            .await
            .unwrap();
        println!("Digest 2: {} (previous: {})", digest2.id, digest1.id);

        let digest3 = chain
            .generate_digest("tenant-test", now - 3600, now, &[], Some(&digest2.id))
            .await
            .unwrap();
        println!("Digest 3: {} (previous: {})", digest3.id, digest2.id);

        // Verificar cadena
        let chain_valid = chain.verify_chain("tenant-test").await.unwrap();
        assert!(chain_valid);
        println!("Cadena válida: ✓");

        // Listar digests
        let digests = chain.list_digests("tenant-test", None, None).await.unwrap();
        assert_eq!(digests.len(), 3);
        println!("Digests listados: {}", digests.len());

        // Obtener último
        let latest = chain.get_latest_digest("tenant-test").await.unwrap();
        assert_eq!(latest.unwrap().id, digest3.id);
        println!("Último digest obtenido: ✓");

        println!("\n✅ Test de cadena completado!\n");
    }

    #[test]
    fn test_keypair_generation() {
        println!("\n=== Test de Generación de Claves ===\n");

        let signer = Ed25519Signer::new();

        // Generar múltiples pares de claves
        for i in 1..=5 {
            let keypair = signer.generate_keypair().unwrap();
            println!(
                "Par {} - Pública: {} bytes, Privada: {} bytes",
                i,
                keypair.public_key.len(),
                keypair.private_key.len()
            );
            assert_eq!(keypair.public_key.len(), 32);
            assert_eq!(keypair.private_key.len(), 32);
        }

        println!("\n✅ Test de generación de claves completado!\n");
    }

    #[tokio::test]
    async fn test_file_hashing() {
        println!("\n=== Test de Hashing de Archivos ===\n");

        let hasher = Sha256Hasher::new();

        // Crear archivos temporales
        let temp_dir = tempfile::tempdir().unwrap();
        let file1 = temp_dir.path().join("log1.txt");
        let file2 = temp_dir.path().join("log2.txt");

        tokio::fs::write(&file1, b"Log data 1").await.unwrap();
        tokio::fs::write(&file2, b"Log data 2").await.unwrap();

        // Hash de archivos individuales
        let hash1 = hasher.hash_file(&file1).await.unwrap();
        let hash2 = hasher.hash_file(&file2).await.unwrap();

        println!("Archivo 1 hash: {}...", &hash1[..32]);
        println!("Archivo 2 hash: {}...", &hash2[..32]);

        assert_eq!(hash1.len(), 64);
        assert_eq!(hash2.len(), 64);
        assert_ne!(hash1, hash2); // Diferentes archivos = diferentes hashes

        // Hash de múltiples archivos
        let hashes = hasher.hash_files(&[&file1, &file2]).await.unwrap();
        assert_eq!(hashes.len(), 2);

        println!("\n✅ Test de hashing de archivos completado!\n");
    }

    #[tokio::test]
    async fn test_security_features() {
        println!("\n=== Test de Características de Seguridad ===\n");

        let signer = Ed25519Signer::new();
        let keypair1 = signer.generate_keypair().unwrap();
        let keypair2 = signer.generate_keypair().unwrap();

        let message = "Secure message";
        let signature = signer.sign(message, &keypair1.private_key).unwrap();

        // Test 1: Verificación con mensaje correcto
        let is_valid = signer
            .verify(message, &signature, &keypair1.public_key)
            .unwrap();
        assert!(is_valid);
        println!("✓ Verificación con mensaje correcto");

        // Test 2: Fallo con mensaje incorrecto
        let is_valid = signer
            .verify("wrong message", &signature, &keypair1.public_key)
            .unwrap();
        assert!(!is_valid);
        println!("✓ Rechazo de mensaje incorrecto");

        // Test 3: Fallo con clave pública incorrecta
        let is_valid = signer
            .verify(message, &signature, &keypair2.public_key)
            .unwrap();
        assert!(!is_valid);
        println!("✓ Rechazo de clave pública incorrecta");

        // Test 4: Extracción de clave pública
        let extracted_pubkey = signer.get_public_key(&keypair1.private_key).unwrap();
        assert_eq!(extracted_pubkey, keypair1.public_key);
        println!("✓ Extracción de clave pública");

        println!("\n✅ Test de seguridad completado!\n");
    }
}
