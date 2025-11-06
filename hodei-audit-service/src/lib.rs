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

pub mod crypto;
pub mod enrichment;
pub mod grpc;
pub mod hrn;
pub mod storage;

// Re-exports públicos
pub use grpc::audit_control_server;
pub use grpc::audit_crypto_server;
pub use grpc::audit_query_server;
pub use grpc::vector_api_server;
