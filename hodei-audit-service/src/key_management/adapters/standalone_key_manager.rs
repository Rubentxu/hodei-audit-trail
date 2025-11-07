//! Key Manager Standalone
//!
//! Implementación de KeyManager que usa FileKeyStore.

use crate::crypto::ports::signing::{SigningError, SigningService};
use crate::key_management::ports::key_manager::{
    KeyInfo, KeyManager, KeyManagerError, KeysManifest,
};
use crate::key_management::ports::key_store::KeyStore;
use async_trait::async_trait;
use std::sync::Arc;

/// Key Manager independiente
pub struct StandaloneKeyManager<SS, KS>
where
    SS: SigningService,
    KS: KeyStore,
{
    signing_service: Arc<SS>,
    key_store: Arc<KS>,
}

impl<SS, KS> StandaloneKeyManager<SS, KS>
where
    SS: SigningService,
    KS: KeyStore,
{
    /// Crear nuevo Key Manager
    pub fn new(signing_service: SS, key_store: KS) -> Self {
        Self {
            signing_service: Arc::new(signing_service),
            key_store: Arc::new(key_store),
        }
    }
}

#[async_trait]
impl<SS, KS> KeyManager for StandaloneKeyManager<SS, KS>
where
    SS: SigningService,
    KS: KeyStore,
{
    async fn generate_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError> {
        // Generar par de claves
        let keypair = self
            .signing_service
            .generate_keypair()
            .map_err(|e| KeyManagerError::Generation(e.to_string()))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Calcular ID de clave basado en timestamp
        let key_id = format!("{}_{}_{}", tenant_id, now, keypair.public_key.len());

        let key_info = KeyInfo {
            id: key_id,
            tenant_id: tenant_id.to_string(),
            public_key: keypair.public_key,
            created_at: now,
            expires_at: now + (90 * 24 * 60 * 60), // 90 días
            is_active: true,
            version: 1,
        };

        // Guardar clave
        self.key_store
            .save_key(&key_info, &keypair.private_key)
            .await
            .map_err(|e| KeyManagerError::Storage(e.to_string()))?;

        Ok(key_info)
    }

    async fn rotate_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError> {
        // Desactivar clave actual
        if let Ok(active_key) = self.get_active_key(tenant_id).await {
            self.key_store
                .deactivate_key(&active_key.id)
                .await
                .map_err(|e| KeyManagerError::Rotation(e.to_string()))?;
        }

        // Generar nueva clave
        self.generate_key(tenant_id).await
    }

    async fn get_active_key(&self, tenant_id: &str) -> Result<KeyInfo, KeyManagerError> {
        let keys = self
            .key_store
            .list_keys(tenant_id)
            .await
            .map_err(|e| KeyManagerError::Load(e.to_string()))?;

        // Buscar clave activa
        for key in keys {
            if key.is_active {
                return Ok(key);
            }
        }

        Err(KeyManagerError::NotFound)
    }

    async fn get_manifest(&self, tenant_id: &str) -> Result<KeysManifest, KeyManagerError> {
        let keys = self
            .key_store
            .list_keys(tenant_id)
            .await
            .map_err(|e| KeyManagerError::Load(e.to_string()))?;

        // Calcular hash del manifiesto
        let manifest_json =
            serde_json::to_string(&keys).map_err(|e| KeyManagerError::Generation(e.to_string()))?;

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(manifest_json);
        let hash_result = hasher.finalize();
        let manifest_hash = hex::encode(hash_result);

        // TODO: Firmar con clave raíz
        let root_signature = vec![];

        Ok(KeysManifest {
            version: "1.0".to_string(),
            keys,
            root_signature,
            manifest_hash,
            issued_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    async fn verify_key(&self, tenant_id: &str, key_id: &str) -> Result<bool, KeyManagerError> {
        let keys = self
            .key_store
            .list_keys(tenant_id)
            .await
            .map_err(|e| KeyManagerError::Load(e.to_string()))?;

        Ok(keys.iter().any(|k| k.id == key_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::Ed25519Signer;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_generate_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = crate::key_management::FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        let key = manager.generate_key("tenant1").await.unwrap();

        assert_eq!(key.tenant_id, "tenant1");
        assert!(key.is_active);
        assert_eq!(key.public_key.len(), 32);
    }

    #[tokio::test]
    async fn test_get_active_key() {
        let tmp_dir = tempdir().unwrap();
        let signing = Ed25519Signer::new();
        let store = crate::key_management::FileKeyStore::new(tmp_dir.path().to_path_buf());
        let manager = StandaloneKeyManager::new(signing, store);

        // Generar clave
        let key1 = manager.generate_key("tenant1").await.unwrap();

        // Obtener clave activa
        let active_key = manager.get_active_key("tenant1").await.unwrap();
        assert_eq!(active_key.id, key1.id);
    }
}
