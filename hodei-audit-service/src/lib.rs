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

pub mod api_key;
pub mod clickhouse;
pub mod compliance;
pub mod crypto;
pub mod enrichment;
pub mod grpc;
pub mod grpc_interceptor;
pub mod hrn;
pub mod query;
pub mod quotas;
pub mod row_level_security;
pub mod s3_storage;
pub mod service;
pub mod storage;
pub mod tenant;

#[cfg(all(test, feature = "integration-tests"))]
mod integration_tests;

// Re-exports públicos
pub use api_key::{ApiKey, ApiKeyError, ApiKeyMetadata, ApiKeyStore, ApiScope};
pub use clickhouse::{ClickHouseClient, ClickHouseConfig, ClickHouseMetrics, ClickHouseSchema};
pub use compliance::{
    ComplianceError, ComplianceManager, ComplianceReport, DeletionReason, GDPRRequest,
    GDPRRequestStatus, GDPRRequestType, LegalHold, LegalHoldStatus, RetentionPolicy,
};
pub use grpc::audit_control_server;
pub use grpc::audit_crypto_server;
pub use grpc::audit_query_server;
pub use grpc::vector_api_server;
pub use grpc_interceptor::{AsyncTenantValidationInterceptor, TenantValidationInterceptor};
pub use quotas::{QuotaExceeded, QuotaManager, QuotaStatus, QuotaType, TenantQuota};
pub use row_level_security::{RlsManager, RlsPolicy, RlsQueryBuilder, SecureQueryExecutor};
pub use s3_storage::{
    CompressionType, LifecyclePolicy, ParquetStats, S3Client, S3Config, S3Metrics,
};
pub use service::{HodeiAuditService, ServiceConfig, ServiceMetrics};
pub use tenant::{TenantContext, TenantContextManager, TenantExtractor, TenantTier};
