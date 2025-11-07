//! Almacenamiento de claves en archivo
//!
//! Adapter para desarrollo/testing que almacena claves en archivos encriptados.

use crate::key_management::ports::key_manager::KeyInfo;
use crate::key_management::ports::key_store::{KeyStore, KeyStoreError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Almacén de claves en archivo
#[derive(Debug)]
pub struct FileKeyStore {
    /// Directorio base para claves
    base_dir: PathBuf,
    /// Clave maestra para encriptación (en producción vendría de KMS/Vault)
    master_key: Vec<u8>,
}

impl FileKeyStore {
    /// Crear nuevo almacén de claves
    pub fn new(base_dir: PathBuf) -> Self {
        // TODO: Cargar master_key de manera segura
        let master_key = "dev-master-key-not-for-production".as_bytes().to_vec();

        Self {
            base_dir,
            master_key,
        }
    }

    /// Obtener ruta de archivo para una clave
    fn key_path(&self, key_id: &str) -> PathBuf {
        self.base_dir.join(format!("{}.key", key_id))
    }

    /// Obtener ruta de información para una clave
    fn info_path(&self, key_id: &str) -> PathBuf {
        self.base_dir.join(format!("{}.json", key_id))
    }
}

#[async_trait]
impl KeyStore for FileKeyStore {
    async fn save_key(&self, key: &KeyInfo, private_key: &[u8]) -> Result<(), KeyStoreError> {
        // Crear directorio si no existe
        fs::create_dir_all(&self.base_dir).await?;

        // Encriptar clave privada (simplificado para desarrollo)
        // En producción usar AES-GCM con master_key
        let encrypted_key = private_key.to_vec();

        // Guardar clave privada encriptada
        let key_path = self.key_path(&key.id);
        let mut file = File::create(&key_path).await?;
        file.write_all(&encrypted_key).await?;

        // Guardar información de clave en JSON
        let key_info = KeyInfoFile {
            id: key.id.clone(),
            tenant_id: key.tenant_id.clone(),
            public_key: hex::encode(&key.public_key),
            created_at: key.created_at,
            expires_at: key.expires_at,
            is_active: key.is_active,
            version: key.version,
        };

        let info_path = self.info_path(&key.id);
        let mut file = File::create(&info_path).await?;
        let json = serde_json::to_string_pretty(&key_info)?;
        file.write_all(json.as_bytes()).await?;

        Ok(())
    }

    async fn load_private_key(&self, key_id: &str) -> Result<Vec<u8>, KeyStoreError> {
        let key_path = self.key_path(key_id);

        if !key_path.exists() {
            return Err(KeyStoreError::NotFound);
        }

        let mut file = File::open(&key_path).await?;
        let mut encrypted_key = Vec::new();
        file.read_to_end(&mut encrypted_key).await?;

        // Desencriptar (simplificado)
        // En producción usar AES-GCM con master_key
        Ok(encrypted_key)
    }

    async fn load_key_info(&self, key_id: &str) -> Result<KeyInfo, KeyStoreError> {
        let info_path = self.info_path(key_id);

        if !info_path.exists() {
            return Err(KeyStoreError::NotFound);
        }

        let mut file = File::open(&info_path).await?;
        let mut json = String::new();
        file.read_to_string(&mut json).await?;

        let key_info_file: KeyInfoFile =
            serde_json::from_str(&json).map_err(|e| KeyStoreError::Serialization(e.to_string()))?;

        Ok(KeyInfo {
            id: key_info_file.id,
            tenant_id: key_info_file.tenant_id,
            public_key: hex::decode(key_info_file.public_key)
                .map_err(|e| KeyStoreError::Serialization(e.to_string()))?,
            created_at: key_info_file.created_at,
            expires_at: key_info_file.expires_at,
            is_active: key_info_file.is_active,
            version: key_info_file.version,
        })
    }

    async fn list_keys(&self, tenant_id: &str) -> Result<Vec<KeyInfo>, KeyStoreError> {
        let mut keys = Vec::new();

        let mut entries = fs::read_dir(&self.base_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let mut file = File::open(&path).await?;
                let mut json = String::new();
                file.read_to_string(&mut json).await?;

                let key_info_file: KeyInfoFile = serde_json::from_str(&json)
                    .map_err(|e| KeyStoreError::Serialization(e.to_string()))?;

                if key_info_file.tenant_id == tenant_id {
                    keys.push(KeyInfo {
                        id: key_info_file.id,
                        tenant_id: key_info_file.tenant_id,
                        public_key: hex::decode(key_info_file.public_key)
                            .map_err(|e| KeyStoreError::Serialization(e.to_string()))?,
                        created_at: key_info_file.created_at,
                        expires_at: key_info_file.expires_at,
                        is_active: key_info_file.is_active,
                        version: key_info_file.version,
                    });
                }
            }
        }

        Ok(keys)
    }

    async fn deactivate_key(&self, key_id: &str) -> Result<(), KeyStoreError> {
        let mut key_info = self.load_key_info(key_id).await?;
        key_info.is_active = false;

        let key_info_file = KeyInfoFile {
            id: key_info.id.clone(),
            tenant_id: key_info.tenant_id.clone(),
            public_key: hex::encode(&key_info.public_key),
            created_at: key_info.created_at,
            expires_at: key_info.expires_at,
            is_active: false,
            version: key_info.version,
        };

        let info_path = self.info_path(key_id);
        let mut file = File::create(&info_path).await?;
        let json = serde_json::to_string_pretty(&key_info_file)?;
        file.write_all(json.as_bytes()).await?;

        Ok(())
    }
}

/// Representación serializable de KeyInfo
#[derive(Debug, Serialize, Deserialize)]
struct KeyInfoFile {
    id: String,
    tenant_id: String,
    public_key: String,
    created_at: u64,
    expires_at: u64,
    is_active: bool,
    version: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_save_and_load_key() {
        let tmp_dir = tempdir().unwrap();
        let store = FileKeyStore::new(tmp_dir.path().to_path_buf());

        let key_info = KeyInfo {
            id: "key1".to_string(),
            tenant_id: "tenant1".to_string(),
            public_key: vec![1, 2, 3],
            created_at: 1000,
            expires_at: 2000,
            is_active: true,
            version: 1,
        };

        store.save_key(&key_info, &vec![4, 5, 6]).await.unwrap();

        let loaded_info = store.load_key_info("key1").await.unwrap();
        assert_eq!(loaded_info.id, "key1");
        assert_eq!(loaded_info.tenant_id, "tenant1");

        let loaded_private = store.load_private_key("key1").await.unwrap();
        assert_eq!(loaded_private, vec![4, 5, 6]);
    }
}
