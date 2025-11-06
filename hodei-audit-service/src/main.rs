//! Hodei Audit Service
//!
//! Servicio centralizado de auditoría con arquitectura CAP/ARP

use anyhow::Result;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod grpc;
mod storage;
mod crypto;

use grpc::audit_control_server::AuditControlServer;
use grpc::audit_query_server::AuditQueryServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hodei_audit=debug,tower=debug,tonic=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr: SocketAddr = "[::1]:50052".parse()?;

    info!("🚀 Iniciando Hodei Audit Service en {}", addr);
    info!("📋 Puertos:");
    info!("   - Puerto 50052: Ingestión (AuditControlService)");
    info!("   - Puerto 50053: Query (AuditQueryService)");
    info!("   - Puerto 50054: Crypto/Digest (AuditCryptoService)");

    // TODO: Implementar servicios gRPC
    // let audit_control = grpc::AuditControlService::new();
    // let audit_query = grpc::AuditQueryService::new();
    // let audit_crypto = grpc::AuditCryptoService::new();

    warn!("⚠️  Servicios gRPC aún no implementados");

    // Server::builder()
    //     .add_service(AuditControlServer::new(audit_control))
    //     .add_service(AuditQueryServer::new(audit_query))
    //     .serve(addr)
    //     .await?;

    info!("✅ Servicio iniciado correctamente");
    Ok(())
}
