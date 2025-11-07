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
pub mod async_io_optimization;
pub mod clickhouse;
pub mod clickhouse_tuning;
pub mod compliance;
pub mod crypto;
pub mod distributed_tracing;
pub mod enrichment;
pub mod graceful_shutdown;
pub mod grafana_dashboards;
pub mod grpc;
pub mod grpc_interceptor;
pub mod health;
pub mod hrn;
pub mod integration_tests_epic6;
pub mod key_management;
pub mod metrics;
pub mod performance;
pub mod query;
pub mod quotas;
pub mod row_level_security;
pub mod s3_storage;
pub mod service;
pub mod storage;
pub mod structured_logging;
pub mod tenant;
pub mod vector;
pub mod workers;
pub mod zero_copy_batching;

// #[cfg(all(test, feature = "integration-tests"))]
// mod integration_tests;  // Disabled - requires external services and testcontainers setup

// Re-exports públicos
pub use api_key::{ApiKey, ApiKeyError, ApiKeyMetadata, ApiKeyStore, ApiScope};
pub use async_io_optimization::{
    AsyncIoConfig, AsyncIoOptimizer, AsyncMemoryPool, AsyncMemoryPoolConfig, BatchedTaskExecutor,
};
pub use clickhouse::{ClickHouseClient, ClickHouseConfig, ClickHouseMetrics, ClickHouseSchema};
pub use clickhouse_tuning::{
    ClickHousePerformanceTuner, ClickHouseTuningConfig, CompressionSettings, IndexType,
    MemorySettings, MergeTreeSettings,
};
pub use compliance::{
    ComplianceError, ComplianceManager, ComplianceReport, DeletionReason, GDPRRequest,
    GDPRRequestStatus, GDPRRequestType, LegalHold, LegalHoldStatus, RetentionPolicy,
};
pub use crypto::ports::{digest_chain, hashing, signing};
pub use crypto::{Ed25519Signer, InMemoryDigestChain, Sha256Hasher};
pub use graceful_shutdown::{
    GracefulShutdown, HttpServerGracefulShutdown, ShutdownConfig, ShutdownState, ShutdownUtils,
    Shutdownable,
};
pub use grpc::audit_control_server;
pub use grpc::audit_crypto_server;
pub use grpc::audit_query_server;
pub use grpc::vector_api_server;
pub use grpc_interceptor::{AsyncTenantValidationInterceptor, TenantValidationInterceptor};
pub use health::{
    HealthCheckConfig, HealthCheckManager, HealthChecker, HealthResult, HealthStatus,
    ServiceHealthChecker,
};
pub use key_management::ports::{key_manager, key_store};
pub use key_management::{FileKeyStore, StandaloneKeyManager};
pub use quotas::{QuotaExceeded, QuotaManager, QuotaStatus, QuotaType, TenantQuota};
pub use row_level_security::{RlsManager, RlsPolicy, RlsQueryBuilder, SecureQueryExecutor};
pub use s3_storage::{
    CompressionType, LifecyclePolicy, ParquetStats, S3Client, S3Config, S3Metrics,
};
pub use service::{HodeiAuditService, ServiceConfig, ServiceMetrics};
pub use tenant::{TenantContext, TenantContextManager, TenantExtractor, TenantTier};
pub use vector::{
    VectorError, VectorForwarder, VectorForwarderConfig, VectorResult, VectorSinkConfig,
    VectorSinkManager, VectorSinkType, create_default_sinks,
};
pub use zero_copy_batching::{
    BatcherConfig as ZeroCopyBatcherConfig, BufferError as ZeroCopyError, BufferPool,
    BufferPoolConfig, BufferPoolStats, ZeroCopyBatch, ZeroCopyBatcher,
};

#[cfg(feature = "vector-metrics")]
pub use vector::{VectorHealthStatus, VectorMetrics, VectorMetricsCollector, VectorMetricsSummary};
pub use workers::digest_worker::{
    DigestWorker, DigestWorkerConfig, DigestWorkerError, DigestWorkerResult,
};

// Performance optimizations
pub use performance::{
    BackpressureConfig, BackpressureController, BackpressureMetrics, BatchResult, BatcherConfig,
    BatcherError, BatchingPolicy, CircuitBreaker, CircuitBreakerConfig, CircuitBreakerMetrics,
    CircuitState, ConnectionPool, PoolConfig, PoolError, PoolStats, PooledConnection,
    PressureLevel, SmartBatcher,
};

// Metrics and observability
pub use metrics::{
    AuditMetrics, BatchLabels, EventLabels, QueryLabels, get_metrics, register_metrics,
};

// Grafana dashboards
pub use grafana_dashboards::{
    AlertRule, Condition, ConditionData, DashboardConfig, GrafanaDashboardManager, GridPos, Panel,
    Target, TimeRange, VisualizationConfig,
};

// Structured logging
pub use structured_logging::{
    LogContext, LogEntry, LogLevel, SensitiveDataDetector, StructuredLogger,
};

// Distributed tracing
pub use distributed_tracing::{
    SamplingStrategy, Span, SpanAttribute, SpanEvent, SpanKind, SpanLink, SpanRecorder, Status,
    TraceId, TraceState, Tracer,
};
