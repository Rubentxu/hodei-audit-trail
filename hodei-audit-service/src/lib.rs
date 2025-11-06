//! Hodei Audit Service
//!
//! Servicio centralizado de auditoría con arquitectura CAP/ARP
//! Integrado con Vector.dev para ingesta y fan-out

pub mod crypto;
pub mod grpc;
pub mod storage;

// Re-exports públicos
pub use grpc::audit_control_server;
pub use grpc::audit_crypto_server;
pub use grpc::audit_query_server;
