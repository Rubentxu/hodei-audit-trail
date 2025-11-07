//! gRPC Services Implementation
//!
//! Implementación de los servicios gRPC para el Hodei Audit Service
//! Incluye: AuditControl, AuditQuery, AuditCrypto y VectorApi

use std::sync::Arc;
use tonic::{Request, Response, Status, transport::Server};
use tracing::info;

use crate::crypto::{Ed25519Signer, InMemoryDigestChain, Sha256Hasher};
use crate::grpc::audit_control_server::AuditControlServiceImpl;
use crate::grpc::audit_crypto_server::AuditCryptoServiceImpl;
use crate::grpc::audit_query_server::AuditQueryServiceImpl;
use crate::grpc::vector_api_server::VectorApiServiceImpl;
use crate::key_management::{FileKeyStore, StandaloneKeyManager};

// Re-exports de los módulos
pub mod audit_control_server;
pub mod audit_crypto_server;
pub mod audit_query_server;
pub mod vector_api_server;

/// Configuración del servidor gRPC
#[derive(Debug, Clone)]
pub struct GrpcConfig {
    pub audit_control_addr: String, // Puerto 50052
    pub audit_query_addr: String,   // Puerto 50053
    pub audit_crypto_addr: String,  // Puerto 50054
    pub vector_api_addr: String,    // Puerto 50051
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            audit_control_addr: "0.0.0.0:50052".to_string(),
            audit_query_addr: "0.0.0.0:50053".to_string(),
            audit_crypto_addr: "0.0.0.0:50054".to_string(),
            vector_api_addr: "0.0.0.0:50051".to_string(),
        }
    }
}

/// Servicio de salud para monitoreo
#[derive(Debug, Default)]
pub struct HealthService {
    pub status: Arc<std::sync::atomic::AtomicU32>, // 0=Unknown, 1=Serving, 2=NotServing
}

impl HealthService {
    pub fn new() -> Self {
        Self {
            status: Arc::new(std::sync::atomic::AtomicU32::new(1)), // Default: Serving
        }
    }

    pub fn set_status(&self, status: u32) {
        self.status
            .store(status, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn get_status(&self) -> u32 {
        self.status.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Clone for HealthService {
    fn clone(&self) -> Self {
        Self {
            status: self.status.clone(),
        }
    }
}

/// Función principal para inicializar el servidor gRPC
/// Ejecuta todos los servicios en paralelo
pub async fn run_grpc_server(
    config: GrpcConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Hodei Audit Service gRPC servers...");

    // Crear instancia del health service compartido
    let health_service = HealthService::new();
    health_service.set_status(1); // SERVING

    // Inicializar servicios
    let audit_control = AuditControlServiceImpl::new();
    let audit_query = AuditQueryServiceImpl::new();

    // Inicializar servicios crypto con dependencias reales
    let hashing = Sha256Hasher::new();
    let signing = Ed25519Signer::new();
    let digest_chain = InMemoryDigestChain::new();
    let key_store = FileKeyStore::new("/tmp/keys".into());
    let key_manager = StandaloneKeyManager::new(signing.clone(), key_store);
    let audit_crypto = AuditCryptoServiceImpl::<
        Sha256Hasher,
        Ed25519Signer,
        InMemoryDigestChain,
        StandaloneKeyManager<Ed25519Signer, FileKeyStore>,
    >::new(hashing, signing, digest_chain, key_manager);

    let vector_api = VectorApiServiceImpl::new();

    // Spawner threads para cada servicio
    let handles = vec![
        // Audit Control Service (Puerto 50052)
        tokio::spawn(run_audit_control_server(
            config.audit_control_addr.clone(),
            audit_control,
        )),
        // Audit Query Service (Puerto 50053)
        tokio::spawn(run_audit_query_server(
            config.audit_query_addr.clone(),
            audit_query,
        )),
        // Audit Crypto Service (Puerto 50054)
        tokio::spawn(run_audit_crypto_server(
            config.audit_crypto_addr.clone(),
            audit_crypto,
        )),
        // Vector API Service (Puerto 50051)
        tokio::spawn(run_vector_api_server(
            config.vector_api_addr.clone(),
            vector_api,
        )),
    ];

    info!("All gRPC servers started successfully");
    info!("  - AuditControlService: {}", config.audit_control_addr);
    info!("  - AuditQueryService: {}", config.audit_query_addr);
    info!("  - AuditCryptoService: {}", config.audit_crypto_addr);
    info!("  - VectorApi: {}", config.vector_api_addr);

    // Esperar a que todos los servicios terminen
    for handle in handles {
        handle.await??;
    }

    Ok(())
}

async fn run_audit_control_server(
    addr: String,
    service: AuditControlServiceImpl,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting AuditControlService on {}", addr);

    let server = Server::builder()
        .add_service(
            hodei_audit_proto::audit_control_service_server::AuditControlServiceServer::new(
                service,
            ),
        )
        .serve(addr.parse()?)
        .await?;

    info!("AuditControlService stopped");
    Ok(server)
}

async fn run_audit_query_server(
    addr: String,
    service: AuditQueryServiceImpl,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting AuditQueryService on {}", addr);

    let server = Server::builder()
        .add_service(
            hodei_audit_proto::audit_query_service_server::AuditQueryServiceServer::new(service),
        )
        .serve(addr.parse()?)
        .await?;

    info!("AuditQueryService stopped");
    Ok(server)
}

async fn run_audit_crypto_server<HS, SS, DS, KM>(
    addr: String,
    service: AuditCryptoServiceImpl<HS, SS, DS, KM>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    HS: crate::crypto::ports::hashing::HashingService,
    SS: crate::crypto::ports::signing::SigningService,
    DS: crate::crypto::ports::digest_chain::DigestChainService,
    KM: crate::key_management::ports::key_manager::KeyManager,
{
    info!("Starting AuditCryptoService on {}", addr);

    let server = Server::builder()
        .add_service(
            hodei_audit_proto::audit_crypto_service_server::AuditCryptoServiceServer::new(service),
        )
        .serve(addr.parse()?)
        .await?;

    info!("AuditCryptoService stopped");
    Ok(server)
}

async fn run_vector_api_server(
    addr: String,
    service: VectorApiServiceImpl,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting VectorApi on {}", addr);

    let server = Server::builder()
        .add_service(hodei_audit_proto::vector_api_server::VectorApiServer::new(
            service,
        ))
        .serve(addr.parse()?)
        .await?;

    info!("VectorApi stopped");
    Ok(server)
}
