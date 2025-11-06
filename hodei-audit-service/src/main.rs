//! Hodei Audit Service
//!
//! Servicio centralizado de auditoría con arquitectura CAP/ARP
//! Integrado con Vector.dev para ingesta y fan-out
//!
//! Servicios gRPC:
//! - Puerto 50052: AuditControlService (Ingestión)
//! - Puerto 50053: AuditQueryService (Query/Analytics)
//! - Puerto 50054: AuditCryptoService (Criptografía/Compliance)
//! - Puerto 50051: VectorApi (CAP → Vector communication)

use anyhow::Result;
use std::env;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod crypto;
mod grpc;
mod storage;

use grpc::{GrpcConfig, run_grpc_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_level(true),
        )
        .init();

    info!("🚀 Starting Hodei Audit Service");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Cargar configuración desde variables de entorno
    let config = GrpcConfig {
        audit_control_addr: env::var("AUDIT_CONTROL_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50052".to_string()),
        audit_query_addr: env::var("AUDIT_QUERY_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50053".to_string()),
        audit_crypto_addr: env::var("AUDIT_CRYPTO_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50054".to_string()),
        vector_api_addr: env::var("VECTOR_API_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:50051".to_string()),
    };

    info!("📡 gRPC Configuration:");
    info!("  - AuditControl: {}", config.audit_control_addr);
    info!("  - AuditQuery: {}", config.audit_query_addr);
    info!("  - AuditCrypto: {}", config.audit_crypto_addr);
    info!("  - VectorApi: {}", config.vector_api_addr);

    // Setup graceful shutdown
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        info!("🛑 Received Ctrl+C, starting graceful shutdown...");
    };

    // Ejecutar servidor gRPC con graceful shutdown
    tokio::select! {
        result = run_grpc_server(config) => {
            match result {
                Ok(_) => {
                    info!("✅ gRPC server shutdown completed");
                }
                Err(e) => {
                    error!("❌ gRPC server error: {}", e);
                    anyhow::bail!("gRPC server error: {}", e);
                }
            }
        }
        _ = shutdown_signal => {
            info!("🛑 Graceful shutdown initiated");
        }
    }

    Ok(())
}
