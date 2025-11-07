//! Adapter In-Memory para cadena de digests
//!
//! Implementación en memoria para desarrollo y testing.
//! En producción, se reemplazará con un adapter de base de datos.

use crate::crypto::ports::digest_chain::{DigestChainError, DigestChainService, DigestInfo};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Servicio de cadena de digests en memoria
#[derive(Debug)]
pub struct InMemoryDigestChain {
    /// Mapa de tenant_id -> lista de digests
    digests: Arc<RwLock<HashMap<String, Vec<DigestInfo>>>>,
}

impl InMemoryDigestChain {
    /// Crear nuevo servicio de cadena
    pub fn new() -> Self {
        Self {
            digests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Limpiar todos los datos (solo para testing)
    pub async fn clear(&self) {
        let mut digests = self.digests.write().await;
        digests.clear();
    }
}

#[async_trait]
impl DigestChainService for InMemoryDigestChain {
    async fn generate_digest(
        &self,
        tenant_id: &str,
        start_time: u64,
        end_time: u64,
        file_hashes: &[(&str, String)],
        previous_digest_id: Option<&str>,
    ) -> Result<DigestInfo, DigestChainError> {
        let mut digests = self.digests.write().await;

        // Verificar que el digest anterior existe si se especifica
        if let Some(prev_id) = previous_digest_id {
            let tenant_digests = digests.get(tenant_id).cloned().unwrap_or_default();
            if !tenant_digests.iter().any(|d| d.id == prev_id) {
                return Err(DigestChainError::Validation(
                    "Previous digest not found".to_string(),
                ));
            }
        }

        // Calcular hash agregado de todos los archivos
        let mut aggregated_hash = String::new();
        for (_, hash) in file_hashes {
            aggregated_hash.push_str(hash);
        }

        let digest_info = DigestInfo {
            id: format!("digest_{}_{}", tenant_id, end_time),
            hash: aggregated_hash,
            signature: vec![], // Se firmará externamente
            timestamp: end_time,
            previous_digest_id: previous_digest_id.map(|s| s.to_string()),
            total_files: file_hashes.len(),
            total_bytes: 0, // TODO: calcular bytes reales
        };

        // Añadir a la lista
        digests
            .entry(tenant_id.to_string())
            .or_insert_with(Vec::new)
            .push(digest_info.clone());

        Ok(digest_info)
    }

    async fn verify_digest(&self, digest_id: &str) -> Result<bool, DigestChainError> {
        let digests = self.digests.read().await;

        // Verificar que el digest existe
        for tenant_digests in digests.values() {
            if tenant_digests.iter().any(|d| d.id == digest_id) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn get_latest_digest(
        &self,
        tenant_id: &str,
    ) -> Result<Option<DigestInfo>, DigestChainError> {
        let digests = self.digests.read().await;
        let tenant_digests = digests.get(tenant_id);

        Ok(tenant_digests.and_then(|v| v.last().cloned()))
    }

    async fn list_digests(
        &self,
        tenant_id: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<DigestInfo>, DigestChainError> {
        let digests = self.digests.read().await;
        let tenant_digests = digests.get(tenant_id).cloned().unwrap_or_default();

        let mut filtered: Vec<DigestInfo> = tenant_digests
            .iter()
            .filter(|d| {
                let time_ok = match (start_time, end_time) {
                    (Some(start), Some(end)) => d.timestamp >= start && d.timestamp <= end,
                    (Some(start), None) => d.timestamp >= start,
                    (None, Some(end)) => d.timestamp <= end,
                    (None, None) => true,
                };
                time_ok
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(filtered)
    }

    async fn verify_chain(&self, tenant_id: &str) -> Result<bool, DigestChainError> {
        let digests = self.digests.read().await;
        let tenant_digests = digests.get(tenant_id).cloned().unwrap_or_default();

        // Verificar que la cadena es continua
        for (i, digest) in tenant_digests.iter().enumerate() {
            if let Some(ref prev_id) = digest.previous_digest_id {
                if i == 0 {
                    return Ok(false); // Primer digest no debe tener previous
                }

                let prev_digest = &tenant_digests[i - 1];
                if prev_digest.id != *prev_id {
                    return Ok(false); // Cadena rota
                }
            } else if i > 0 {
                return Ok(false); // Digest intermedio sin previous
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_digest() {
        let chain = InMemoryDigestChain::new();
        let file_hashes = vec![("file1.parquet", "hash1".to_string())];

        let digest = chain
            .generate_digest("tenant1", 1000, 2000, &file_hashes, None)
            .await
            .unwrap();

        assert_eq!(digest.timestamp, 2000);
        assert_eq!(digest.total_files, 1);
    }

    #[tokio::test]
    async fn test_verify_chain() {
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

        assert!(chain.verify_chain("tenant1").await.unwrap());
        assert_eq!(digest2.previous_digest_id, Some(digest1.id));
    }
}
