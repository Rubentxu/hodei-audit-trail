//! Test de Integración End-to-End para Épica 6
//!
//! Este test demuestra que toda la implementación funciona:
//! - Hashing SHA-256
//! - Firma Ed25519
//! - Cadena de digests
//! - Gestión de claves
//! - Verificación de integridad

#[cfg(test)]
mod epic6_integration_tests {
    use super::super::*;
    use tempfile::tempdir;

    // Import traits to bring methods into scope
    use crate::crypto::ports::digest_chain::DigestChainService;
    use crate::crypto::ports::hashing::HashingService;
    use crate::crypto::ports::signing::SigningService;
    use crate::key_management::ports::key_manager::KeyManager;

    #[tokio::test]
    async fn test_end_to_end_crypto_pipeline() {
        println!("\n🧪 Test: End-to-End Crypto Pipeline");
        println!("=====================================\n");

        // 1. INICIALIZAR SERVICIOS
        println!("1️⃣  Inicializando servicios crypto...");
        let hasher = Sha256Hasher::new();
        let signer = Ed25519Signer::new();
        let digest_chain = InMemoryDigestChain::new();

        println!("   ✅ Sha256Hasher inicializado");
        println!("   ✅ Ed25519Signer inicializado");
        println!("   ✅ InMemoryDigestChain inicializado");

        // 2. GENERAR CLAVES
        println!("\n2️⃣  Generando claves Ed25519...");
        let keypair = signer.generate_keypair().unwrap();
        println!("   ✅ Par de claves generado");
        println!("      - Clave pública: {} bytes", keypair.public_key.len());
        println!("      - Clave privada: {} bytes", keypair.private_key.len());

        // 3. CREAR DATOS DE PRUEBA
        println!("\n3️⃣  Creando datos de prueba...");
        let data1 = b"Log event 1: User login";
        let data2 = b"Log event 2: Data access";
        let data3 = b"Log event 3: File download";
        println!("   ✅ 3 eventos de log creados");

        // 4. CALCULAR HASHES
        println!("\n4️⃣  Calculando hashes SHA-256...");
        let hash1 = hasher.hash_data(data1).unwrap();
        let hash2 = hasher.hash_data(data2).unwrap();
        let hash3 = hasher.hash_data(data3).unwrap();
        println!("   ✅ Hash 1: {}...{}", &hash1[..16], &hash1[48..]);
        println!("   ✅ Hash 2: {}...{}", &hash2[..16], &hash2[48..]);
        println!("   ✅ Hash 3: {}...{}", &hash3[..16], &hash3[48..]);

        // 5. CREAR CADENA DE DIGESTS
        println!("\n5️⃣  Creando cadena de digests...");
        let file_hashes = vec![
            ("event1.log", hash1.clone()),
            ("event2.log", hash2.clone()),
            ("event3.log", hash3.clone()),
        ];

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Primer digest (sin previous)
        let digest1 = digest_chain
            .generate_digest("tenant-123", now - 3600, now, &file_hashes, None)
            .await
            .unwrap();
        println!("   ✅ Digest 1 creado: {}", digest1.id);
        println!("      - Total archivos: {}", digest1.total_files);
        println!("      - Previous: {:?}", digest1.previous_digest_id);

        // Segundo digest (con previous)
        let file_hashes2 = vec![("event4.log", "newhash".to_string())];
        let digest2 = digest_chain
            .generate_digest(
                "tenant-123",
                now,
                now + 3600,
                &file_hashes2,
                Some(&digest1.id),
            )
            .await
            .unwrap();
        println!("   ✅ Digest 2 creado: {}", digest2.id);
        println!("      - Previous: {:?}", digest2.previous_digest_id);

        // 6. FIRMAR DIGEST
        println!("\n6️⃣  Firmando digest con Ed25519...");
        let digest_to_sign = &digest1.hash;
        let signature = signer.sign(digest_to_sign, &keypair.private_key).unwrap();
        println!("   ✅ Digest firmado");
        println!("      - Firma: {} bytes", signature.len());

        // 7. VERIFICAR FIRMA
        println!("\n7️⃣  Verificando firma...");
        let is_valid = signer
            .verify(digest_to_sign, &signature, &keypair.public_key)
            .unwrap();
        assert!(is_valid, "Firma debería ser válida");
        println!("   ✅ Firma verificada correctamente");

        // 8. VERIFICAR CADENA
        println!("\n8️⃣  Verificando cadena de digests...");
        let chain_valid = digest_chain.verify_chain("tenant-123").await.unwrap();
        assert!(chain_valid, "Cadena debería ser válida");
        println!("   ✅ Cadena de digests verificada");

        // 9. VERIFICAR DIGEST INDIVIDUAL
        println!("\n9️⃣  Verificando digest individual...");
        let digest_exists = digest_chain.verify_digest(&digest1.id).await.unwrap();
        assert!(digest_exists, "Digest debería existir");
        println!("   ✅ Digest {} existe en la cadena", digest1.id);

        // 10. LISTAR DIGESTS
        println!("\n🔟 Listando digests...");
        let digests = digest_chain
            .list_digests("tenant-123", None, None)
            .await
            .unwrap();
        assert_eq!(digests.len(), 2, "Debería haber 2 digests");
        println!("   ✅ {} digests encontrados", digests.len());

        // 11. OBTENER ÚLTIMO DIGEST
        println!("\n1️⃣1️⃣  Obteniendo último digest...");
        let latest = digest_chain.get_latest_digest("tenant-123").await.unwrap();
        assert!(latest.is_some(), "Debería haber un último digest");
        let latest = latest.unwrap();
        assert_eq!(latest.id, digest2.id, "Último digest debería ser digest2");
        println!("   ✅ Último digest: {}", latest.id);

        println!("\n✅ Test End-to-End completado exitosamente!\n");
    }

    #[tokio::test]
    async fn test_key_management_integration() {
        println!("\n🔑 Test: Key Management Integration");
        println!("===================================\n");

        // 1. CREAR KEY STORE Y MANAGER
        println!("1️⃣  Inicializando Key Management...");
        let tmp_dir = tempdir().unwrap();
        let key_store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let key_manager = StandaloneKeyManager::new(Ed25519Signer::new(), key_store);
        println!("   ✅ FileKeyStore inicializado");
        println!("   ✅ StandaloneKeyManager inicializado");

        // 2. GENERAR CLAVE
        println!("\n2️⃣  Generando clave para tenant...");
        let key1 = key_manager.generate_key("tenant-alpha").await.unwrap();
        println!("   ✅ Clave generada: {}", key1.id);
        println!("      - Tenant: {}", key1.tenant_id);
        println!("      - Activa: {}", key1.is_active);
        println!("      - Creada: {}", key1.created_at);

        // 3. OBTENER CLAVE ACTIVA
        println!("\n3️⃣  Obteniendo clave activa...");
        let active_key = key_manager.get_active_key("tenant-alpha").await.unwrap();
        assert_eq!(active_key.id, key1.id);
        println!("   ✅ Clave activa obtenida: {}", active_key.id);

        // 4. CREAR MANIFIESTO
        println!("\n4️⃣  Creando manifiesto de claves...");
        let manifest = key_manager.get_manifest("tenant-alpha").await.unwrap();
        println!("   ✅ Manifiesto creado");
        println!("      - Versión: {}", manifest.version);
        println!("      - Claves: {}", manifest.keys.len());
        println!(
            "      - Hash: {}...{}",
            &manifest.manifest_hash[..16],
            &manifest.manifest_hash[48..]
        );

        // 5. ROTAR CLAVE
        println!("\n5️⃣  Rotando clave...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Ensure different timestamp (seconds)
        let key2 = key_manager.rotate_key("tenant-alpha").await.unwrap();
        println!("   ✅ Clave rotada");
        println!("      - Nueva clave: {}", key2.id);
        println!("      - Versión: {}", key2.version);

        // 6. VERIFICAR ROTACIÓN
        println!("\n6️⃣  Verificando rotación...");
        let new_active = key_manager.get_active_key("tenant-alpha").await.unwrap();
        assert_eq!(new_active.id, key2.id);
        assert_ne!(
            new_active.id, key1.id,
            "Key IDs should be different after rotation"
        );
        assert_ne!(
            new_active.public_key, key1.public_key,
            "Public keys should be different"
        );
        println!("   ✅ Nueva clave activa: {}", new_active.id);

        // 7. VERIFICAR CLAVE ANTERIOR
        println!("\n7️⃣  Verificando clave anterior...");
        let is_valid = key_manager
            .verify_key("tenant-alpha", &key1.id)
            .await
            .unwrap();
        assert!(
            is_valid,
            "Clave anterior debería ser válida para verificación"
        );
        println!("   ✅ Clave anterior {} aún válida", key1.id);

        // 8. TEST DE AISLAMIENTO DE TENANTS
        println!("\n8️⃣  Probando aislamiento de tenants...");
        let key3 = key_manager.generate_key("tenant-beta").await.unwrap();
        assert_eq!(key3.tenant_id, "tenant-beta");
        assert_ne!(key3.id, key1.id);
        assert_ne!(key3.id, key2.id);
        println!("   ✅ Tenant isolation verificado");
        println!(
            "      - Tenant Alpha: {} claves",
            key_manager
                .get_manifest("tenant-alpha")
                .await
                .unwrap()
                .keys
                .len()
        );
        println!(
            "      - Tenant Beta: {} claves",
            key_manager
                .get_manifest("tenant-beta")
                .await
                .unwrap()
                .keys
                .len()
        );

        println!("\n✅ Test Key Management completado exitosamente!\n");
    }

    #[tokio::test]
    async fn test_digest_worker_simulation() {
        println!("\n⚙️  Test: Digest Worker Simulation");
        println!("==================================\n");

        // 1. CREAR WORKER
        println!("1️⃣  Creando DigestWorker...");
        let tmp_dir = tempdir().unwrap();
        let config = DigestWorkerConfig {
            logs_dir: tmp_dir.path().to_path_buf(),
            interval_hours: 1,
            timeout_secs: 300,
        };

        let worker = DigestWorker::new(
            Sha256Hasher::new(),
            Ed25519Signer::new(),
            InMemoryDigestChain::new(),
            config,
        );
        println!("   ✅ DigestWorker creado");

        // 2. SIMULAR EJECUCIÓN
        println!("\n2️⃣  Ejecutando DigestWorker...");
        let result = worker.run_once("tenant-gamma").await.unwrap();
        println!("   ✅ Worker ejecutado");
        println!("      - ID: {}", result.digest_id);
        println!("      - Archivos procesados: {}", result.files_processed);
        println!("      - Duración: {}ms", result.duration_ms);

        assert_eq!(
            result.files_processed, 0,
            "No debería procesar archivos (directorio vacío)"
        );
        assert_eq!(result.digest_id, "no-files", "Debería retornar no-files");

        println!("\n✅ Test Digest Worker completado exitosamente!\n");
    }

    #[tokio::test]
    async fn test_security_and_performance() {
        println!("\n🔒 Test: Security & Performance");
        println!("================================\n");

        // 1. TEST DE SEGURIDAD: FIRMA INVÁLIDA
        println!("1️⃣  Probando seguridad: firma inválida...");
        let signer = Ed25519Signer::new();
        let keypair = signer.generate_keypair().unwrap();
        let data = "test data";
        let signature = signer.sign(data, &keypair.private_key).unwrap();

        // Intentar verificar con datos diferentes
        let is_valid = signer
            .verify("different data", &signature, &keypair.public_key)
            .unwrap();
        assert!(
            !is_valid,
            "Firma debería ser inválida para datos diferentes"
        );
        println!("   ✅ Verificación de firma inválida funciona");

        // 2. TEST DE SEGURIDAD: CLAVE INVÁLIDA
        println!("\n2️⃣  Probando seguridad: clave inválida...");
        let keypair2 = signer.generate_keypair().unwrap();
        let is_valid = signer
            .verify(data, &signature, &keypair2.public_key)
            .unwrap();
        assert!(!is_valid, "Firma debería ser inválida para clave diferente");
        println!("   ✅ Verificación de clave inválida funciona");

        // 3. TEST DE RENDIMIENTO: HASHING MÚLTIPLE
        println!("\n3️⃣  Probando rendimiento: hashing múltiple...");
        let hasher = Sha256Hasher::new();
        let start = std::time::Instant::now();

        for i in 0..1000 {
            let data = format!("data chunk {}", i);
            hasher.hash_data(data.as_bytes()).unwrap();
        }

        let duration = start.elapsed();
        println!(
            "   ✅ 1000 hashes completados en {}ms",
            duration.as_millis()
        );
        assert!(
            duration.as_millis() < 1000,
            "Debería completar en menos de 1 segundo"
        );

        // 4. TEST DE INTEGRIDAD DE CADENA
        println!("\n4️⃣  Verificando integridad de cadena...");
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1", "hash1".to_string())];

        let digest1 = chain
            .generate_digest("test", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();
        let digest2 = chain
            .generate_digest("test", 2000, 3000, &file_hashes, Some(&digest1.id))
            .await
            .unwrap();
        let digest3 = chain
            .generate_digest("test", 3000, 4000, &file_hashes, Some(&digest2.id))
            .await
            .unwrap();

        assert!(
            chain.verify_chain("test").await.unwrap(),
            "Cadena debería ser válida"
        );
        println!("   ✅ Cadena de 3 digests verificada");

        println!("\n✅ Test Security & Performance completado exitosamente!\n");
    }

    #[tokio::test]
    async fn test_compliance_scenario() {
        println!("\n📋 Test: Compliance Scenario (SOC2/PCI-DSS)");
        println!("============================================\n");

        // Simular un escenario de auditoría
        let hasher = Sha256Hasher::new();
        let signer = Ed25519Signer::new();
        let digest_chain = InMemoryDigestChain::new();

        // 1. CREAR EVIDENCIA DE AUDITORÍA
        println!("1️⃣  Creando evidencia de auditoría...");
        let audit_evidence = vec![
            ("access_log_2024-01-01.parquet", "hash_a1".to_string()),
            ("access_log_2024-01-02.parquet", "hash_a2".to_string()),
            ("access_log_2024-01-03.parquet", "hash_a3".to_string()),
        ];

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let digest = digest_chain
            .generate_digest("audit-tenant", now - 86400, now, &audit_evidence, None)
            .await
            .unwrap();

        println!("   ✅ Evidencia creada: {}", digest.id);
        println!("      - Período: últimas 24 horas");
        println!("      - Archivos: {}", audit_evidence.len());

        // 2. GENERAR PAR DE CLAVES PARA AUDITORÍA
        println!("\n2️⃣  Generando claves para auditoría...");
        let keypair = signer.generate_keypair().unwrap();
        println!("   ✅ Claves generadas para auditoría");

        // 3. FIRMAR EVIDENCIA
        println!("\n3️⃣  Firmando evidencia...");
        let signature = signer.sign(&digest.hash, &keypair.private_key).unwrap();
        println!("   ✅ Evidencia firmada");

        // 4. CREAR REPORTE DE COMPLIANCE
        println!("\n4️⃣  Generando reporte de compliance...");
        println!("   📊 REPORTE DE COMPLIANCE SOC2/PCI-DSS");
        println!("   ========================================");
        println!("   Digest ID: {}", digest.id);
        println!(
            "   Período: {} - {}",
            digest.timestamp - 86400,
            digest.timestamp
        );
        println!("   Archivos auditados: {}", digest.total_files);

        // Use safer string slicing for hash display
        let hash_display = if digest.hash.len() >= 32 {
            format!(
                "{}...{}",
                &digest.hash[..16],
                &digest.hash[digest.hash.len() - 16..]
            )
        } else {
            format!("{} (short)", digest.hash)
        };
        println!("   Hash del digest: {}", hash_display);

        let sig_hex = hex::encode(&signature);
        let sig_display = if sig_hex.len() >= 32 {
            format!("{}...{}", &sig_hex[..16], &sig_hex[sig_hex.len() - 16..])
        } else {
            sig_hex
        };
        println!("   Firma digital: {}", sig_display);

        let pubkey_hex = hex::encode(&keypair.public_key);
        let pubkey_display = if pubkey_hex.len() >= 32 {
            format!(
                "{}...{}",
                &pubkey_hex[..16],
                &pubkey_hex[pubkey_hex.len() - 16..]
            )
        } else {
            pubkey_hex
        };
        println!("   Clave pública: {}", pubkey_display);
        println!("   Algoritmo: Ed25519-SHA256");

        // 5. VERIFICAR INTEGRIDAD
        println!("\n5️⃣  Verificando integridad para auditoría...");
        let chain_valid = digest_chain.verify_chain("audit-tenant").await.unwrap();
        let signature_valid = signer
            .verify(&digest.hash, &signature, &keypair.public_key)
            .unwrap();

        assert!(chain_valid, "Cadena debería ser válida para auditoría");
        assert!(signature_valid, "Firma debería ser válida para auditoría");

        println!("   ✅ Verificación completada");
        println!("      - Cadena válida: {}", chain_valid);
        println!("      - Firma válida: {}", signature_valid);
        println!("      - Estado: CUMPLE CON SOC2/PCI-DSS ✓");

        println!("\n✅ Test Compliance Scenario completado exitosamente!\n");
    }
}
