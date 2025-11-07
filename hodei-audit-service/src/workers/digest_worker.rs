//! Worker para generación de digests criptográficos
//!
//! Este worker se ejecuta periódicamente para generar digests
//! de los archivos de log, creando una cadena de tamper-evidence.

use crate::crypto::ports::digest_chain::DigestChainService;
use crate::crypto::ports::hashing::HashingService;
use crate::crypto::ports::signing::SigningService;
use std::path::PathBuf;
use thiserror::Error;
use tokio::time::{Duration, Instant};

/// Errores del DigestWorker
#[derive(Debug, Error)]
pub enum DigestWorkerError {
    #[error("Error de hashing: {0}")]
    Hashing(String),

    #[error("Error de firma: {0}")]
    Signing(String),

    #[error("Error de cadena: {0}")]
    Chain(String),

    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),
}

/// Resultado de una ejecución del worker
#[derive(Debug, Clone)]
pub struct DigestWorkerResult {
    pub digest_id: String,
    pub files_processed: usize,
    pub start_time: u64,
    pub end_time: u64,
    pub duration_ms: u64,
}

/// Configuración del DigestWorker
#[derive(Debug, Clone)]
pub struct DigestWorkerConfig {
    /// Directorio de logs por tenant
    pub logs_dir: PathBuf,
    /// Intervalo de ejecución (en horas)
    pub interval_hours: u64,
    /// Timeout para procesamiento (en segundos)
    pub timeout_secs: u64,
}

/// Worker para generación de digests
pub struct DigestWorker<HS, SS, DS>
where
    HS: HashingService,
    SS: SigningService,
    DS: DigestChainService,
{
    hashing_service: HS,
    signing_service: SS,
    chain_service: DS,
    config: DigestWorkerConfig,
}

impl<HS, SS, DS> DigestWorker<HS, SS, DS>
where
    HS: HashingService,
    SS: SigningService,
    DS: DigestChainService,
{
    /// Crear nuevo worker
    pub fn new(
        hashing_service: HS,
        signing_service: SS,
        chain_service: DS,
        config: DigestWorkerConfig,
    ) -> Self {
        Self {
            hashing_service,
            signing_service,
            chain_service,
            config,
        }
    }

    /// Ejecutar una vez el worker
    pub async fn run_once(&self, tenant_id: &str) -> Result<DigestWorkerResult, DigestWorkerError> {
        let start_time = Instant::now();
        let start_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 1. Encontrar archivos de log del período
        let end_timestamp = start_timestamp;
        let start_timestamp = end_timestamp - (self.config.interval_hours * 3600);

        let log_files = self
            .find_log_files(tenant_id, start_timestamp, end_timestamp)
            .await
            .map_err(|e| DigestWorkerError::Io(e))?;

        if log_files.is_empty() {
            return Ok(DigestWorkerResult {
                digest_id: "no-files".to_string(),
                files_processed: 0,
                start_time: start_timestamp,
                end_time: end_timestamp,
                duration_ms: start_time.elapsed().as_millis() as u64,
            });
        }

        // 2. Calcular hash de cada archivo
        let file_hashes: Vec<(String, String)> = self
            .hash_log_files(&log_files)
            .await
            .map_err(|e| DigestWorkerError::Hashing(e.to_string()))?;

        // 3. Obtener digest anterior
        let previous_digest = self
            .chain_service
            .get_latest_digest(tenant_id)
            .await
            .map_err(|e| DigestWorkerError::Chain(e.to_string()))?;

        let previous_digest_id = previous_digest.as_ref().map(|d| d.id.as_str());

        // 4. Generar nuevo digest
        let file_hashes_ref: Vec<(&str, String)> = file_hashes
            .iter()
            .map(|(s, h)| (s.as_str(), h.clone()))
            .collect();
        let digest_info = self
            .chain_service
            .generate_digest(
                tenant_id,
                start_timestamp,
                end_timestamp,
                &file_hashes_ref,
                previous_digest_id,
            )
            .await
            .map_err(|e| DigestWorkerError::Chain(e.to_string()))?;

        // 5. Firmar el digest
        // TODO: Obtener clave privada del KeyManager
        let signature: Vec<u8> = vec![]; // Placeholder

        let end_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(DigestWorkerResult {
            digest_id: digest_info.id,
            files_processed: log_files.len(),
            start_time: start_timestamp,
            end_time: end_timestamp,
            duration_ms: start_time.elapsed().as_millis() as u64,
        })
    }

    /// Encontrar archivos de log del período
    async fn find_log_files(
        &self,
        tenant_id: &str,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = Vec::new();
        let tenant_dir = self.config.logs_dir.join(tenant_id);

        // TODO: Implementar búsqueda recursiva por timestamp
        // Por ahora, retorna directorio vacío
        if tenant_dir.exists() {
            // Búsqueda de archivos .parquet por rango de tiempo
            // En implementación real, usar timestamp del nombre del archivo
        }

        Ok(files)
    }

    /// Calcular hash de archivos de log
    async fn hash_log_files(
        &self,
        files: &[PathBuf],
    ) -> Result<Vec<(String, String)>, std::io::Error> {
        let mut hashes: Vec<(String, String)> = Vec::with_capacity(files.len());

        for file in files {
            let path_str = file.to_str().unwrap_or("").to_string();
            let hash = self
                .hashing_service
                .hash_file(file)
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            hashes.push((path_str, hash));
        }

        Ok(hashes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{Ed25519Signer, InMemoryDigestChain, Sha256Hasher};

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
    }
}
