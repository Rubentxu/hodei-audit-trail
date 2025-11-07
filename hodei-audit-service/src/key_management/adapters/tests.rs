//! Tests unitarios para Key Management
//!
//! Validan la gestión segura de claves criptográficas.

#[cfg(test)]
mod file_key_store_tests {
    use super::super::super::{FileKeyStore, KeyStoreError};
    use crate::key_management::ports::key_manager::KeyInfo;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_save_and_load_key() {
        let tmp_dir = tempdir().unwrap();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());

        let key_info = KeyInfo {
            id: "key1".to_string(),
            tenant_id: "tenant1".to_string(),
            public_key: vec![1, 2, 3, 4, 5],
            created_at: 1000,
            expires_at: 2000,
            is_active: true,
            version: 1,
        };

        let private_key = vec![6, 7, 8, 9, 10];

        store.save_key(&key_info, &private_key).await.unwrap();

        let loaded_info = store.load_key_info("key1").await.unwrap();
        assert_eq!(loaded_info.id, "key1");
        assert_eq!(loaded_info.tenant_id, "tenant1");
        assert_eq!(loaded_info.public_key, vec![1, 2, 3, 4, 5]);
        assert_eq!(loaded_info.created_at, 1000);
        assert!(loaded_info.is_active);

        let loaded_private = store.load_private_key("key1").await.unwrap();
        assert_eq!(loaded_private, private_key);
    }

    #[tokio::test]
    async fn test_load_nonexistent_key() {
        let tmp_dir = tempdir().unwrap();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());

        let result = store.load_key_info("nonexistent").await;
        assert!(matches!(result, Err(KeyStoreError::NotFound)));
    }

    #[tokio::test]
    async fn test_list_keys_for_tenant() {
        let tmp_dir = tempdir().unwrap();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());

        // Guardar claves para tenant1
        let key1 = KeyInfo {
            id: "key1".to_string(),
            tenant_id: "tenant1".to_string(),
            public_key: vec![1],
            created_at: 1000,
            expires_at: 2000,
            is_active: true,
            version: 1,
        };

        let key2 = KeyInfo {
            id: "key2".to_string(),
            tenant_id: "tenant1".to_string(),
            public_key: vec![2],
            created_at: 2000,
            expires_at: 3000,
            is_active: false,
            version: 2,
        };

        let key3 = KeyInfo {
            id: "key3".to_string(),
            tenant_id: "tenant2".to_string(),
            public_key: vec![3],
            created_at: 1000,
            expires_at: 2000,
            is_active: true,
            version: 1,
        };

        store.save_key(&key1, &vec![10]).await.unwrap();
        store.save_key(&key2, &vec![20]).await.unwrap();
        store.save_key(&key3, &vec![30]).await.unwrap();

        let tenant1_keys = store.list_keys("tenant1").await.unwrap();
        assert_eq!(tenant1_keys.len(), 2);

        let tenant2_keys = store.list_keys("tenant2").await.unwrap();
        assert_eq!(tenant2_keys.len(), 1);
        assert_eq!(tenant2_keys[0].id, "key3");
    }

    #[tokio::test]
    async fn test_deactivate_key() {
        let tmp_dir = tempdir().unwrap();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());

        let key = KeyInfo {
            id: "key1".to_string(),
            tenant_id: "tenant1".to_string(),
            public_key: vec![1],
            created_at: 1000,
            expires_at: 2000,
            is_active: true,
            version: 1,
        };

        store.save_key(&key, &vec![10]).await.unwrap();

        // Verificar que está activa
        let loaded = store.load_key_info("key1").await.unwrap();
        assert!(loaded.is_active);

        // Desactivar
        store.deactivate_key("key1").await.unwrap();

        // Verificar que está inactiva
        let loaded = store.load_key_info("key1").await.unwrap();
        assert!(!loaded.is_active);
    }
}

#[cfg(test)]
mod standalone_key_manager_tests {
    use super::super::super::{Ed25519Signer, FileKeyStore, StandaloneKeyManager};
    use crate::key_management::ports::key_manager::KeyManagerError;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_generate_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        let key = manager.generate_key("tenant1").await.unwrap();

        assert_eq!(key.tenant_id, "tenant1");
        assert!(key.is_active);
        assert_eq!(key.public_key.len(), 32);
        assert!(key.id.starts_with("tenant1_"));
        assert!(key.expires_at > key.created_at);
    }

    #[tokio::test]
    async fn test_get_active_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        // Generar clave
        let key1 = manager.generate_key("tenant1").await.unwrap();

        // Obtener clave activa
        let active_key = manager.get_active_key("tenant1").await.unwrap();
        assert_eq!(active_key.id, key1.id);
        assert_eq!(active_key.tenant_id, "tenant1");
    }

    #[tokio::test]
    async fn test_get_active_key_nonexistent() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        let result = manager.get_active_key("nonexistent").await;
        assert!(matches!(result, Err(KeyManagerError::NotFound)));
    }

    #[tokio::test]
    async fn test_rotate_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        // Generar clave inicial
        let key1 = manager.generate_key("tenant1").await.unwrap();
        let key1_id = key1.id.clone();

        // Rotar clave
        let key2 = manager.rotate_key("tenant1").await.unwrap();

        // Verificar que la nueva clave es diferente
        assert_ne!(key2.id, key1_id);
        assert_eq!(key2.tenant_id, "tenant1");
        assert!(key2.is_active);

        // Verificar que la clave anterior ya no está activa
        let active_key = manager.get_active_key("tenant1").await.unwrap();
        assert_eq!(active_key.id, key2.id);
    }

    #[tokio::test]
    async fn test_get_manifest() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        // Generar dos claves
        let key1 = manager.generate_key("tenant1").await.unwrap();
        let _key2 = manager.generate_key("tenant1").await.unwrap();

        // Obtener manifiesto
        let manifest = manager.get_manifest("tenant1").await.unwrap();

        assert_eq!(manifest.version, "1.0");
        assert_eq!(manifest.keys.len(), 2);
        assert!(!manifest.manifest_hash.is_empty());
        assert!(manifest.issued_at > 0);
    }

    #[tokio::test]
    async fn test_verify_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        let key = manager.generate_key("tenant1").await.unwrap();

        // Verificar clave existente
        assert!(manager.verify_key("tenant1", &key.id).await.unwrap());

        // Verificar clave inexistente
        assert!(!manager.verify_key("tenant1", "nonexistent").await.unwrap());

        // Verificar tenant inexistente
        assert!(!manager.verify_key("nonexistent", "key1").await.unwrap());
    }

    #[tokio::test]
    async fn test_multiple_tenants_isolation() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        // Generar claves para diferentes tenants
        let key1 = manager.generate_key("tenant1").await.unwrap();
        let key2 = manager.generate_key("tenant2").await.unwrap();

        // Verificar aislamiento
        assert_eq!(key1.tenant_id, "tenant1");
        assert_eq!(key2.tenant_id, "tenant2");
        assert_ne!(key1.id, key2.id);

        // Verificar que cada tenant solo ve sus claves
        let active1 = manager.get_active_key("tenant1").await.unwrap();
        let active2 = manager.get_active_key("tenant2").await.unwrap();
        assert_eq!(active1.id, key1.id);
        assert_eq!(active2.id, key2.id);
    }
}
