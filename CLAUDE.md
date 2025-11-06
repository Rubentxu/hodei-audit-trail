# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Hodei Audit Service** is a centralized, multi-tenant audit logging system for enterprise applications. It provides secure, compliant, and scalable audit trail capabilities with comprehensive tenant isolation, API key management, and GDPR compliance.

**Architecture**: CAP/ARP (Centralized Audit Point / Audit Reporting Point) with Vector.dev integration
**Language**: Rust
**Key Technologies**: gRPC, Tonic, ClickHouse, S3/MinIO, Axum, Vector.dev

---

## Quick Start

### Development Setup

```bash
# Install dependencies
cargo install just cargo-watch
rustup target add x86_64-unknown-linux-musl

# Setup development environment
just setup

# Run services
docker-compose -f docker-compose.dev.yml up -d
```

### Build & Test

```bash
# Build all workspace crates
just build
# or
cargo build --workspace

# Run all tests
just test
# or
cargo test --workspace --all-targets

# Run specific test
cargo test -p hodei-audit-service test_abuse_detection
cargo test -p hodei-audit-sdk hrn
```

### Common Tasks

```bash
# Format code
just fmt
# or
cargo fmt --all

# Lint
just lint
# or
cargo clippy --all-targets --all-features

# Run service
cargo run -p hodei-audit-service

# Run SDK examples
cargo run -p hodei-audit-sdk --example verified-permissions-with-audit

# Run integration tests
./hodei-audit-service/run_integration_tests.sh

# Generate documentation
just docs
# or
cargo doc --workspace --no-deps
```

---

## Workspace Structure

The project is organized as a Rust workspace with 4 main crates:

```
hodei-trail/
├── Cargo.toml (workspace root)
├── justfile (task runner)
├── .env.example (environment variables template)
├── docker-compose.dev.yml (development services)
│
├── hodei-audit-proto/          # gRPC protocol definitions
│   ├── proto/                  # .proto files
│   └── src/lib.rs
│
├── hodei-audit-types/          # Shared types
│   └── src/lib.rs, hrn.rs
│
├── hodei-audit-service/        # Main service (CAP)
│   ├── src/
│   │   ├── main.rs             # Service entry point
│   │   ├── lib.rs              # Public API exports
│   │   ├── grpc/               # gRPC service implementations
│   │   │   ├── audit_control_server.rs
│   │   │   ├── audit_query_server.rs
│   │   │   ├── audit_crypto_server.rs
│   │   │   └── vector_api_server.rs
│   │   ├── storage/            # Storage backends
│   │   │   ├── clickhouse.rs
│   │   │   └── s3_storage.rs
│   │   ├── service.rs          # Business logic
│   │   ├── crypto.rs           # Cryptographic operations
│   │   ├── hrn.rs              # HRN (Hodei Resource Names)
│   │   ├── query.rs            # Query engine
│   │   ├── tenant.rs           # Multi-tenant context management
│   │   ├── api_key.rs          # API key management & rate limiting
│   │   ├── quotas.rs           # Resource quotas & abuse detection
│   │   ├── compliance.rs       # GDPR compliance & retention
│   │   ├── row_level_security.rs # Database-level tenant isolation
│   │   ├── grpc_interceptor.rs  # gRPC interceptors for validation
│   │   └── tests/              # Integration tests
│   │       ├── tenant_isolation_test.rs
│   │       └── e2e_multitenancy_test.rs
│   │
├── hodei-audit-sdk/            # Client SDK (ARP)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── client.rs           # gRPC client
│   │   ├── batch.rs            # Batch processing
│   │   ├── middleware.rs       # Axum middleware (1-liner integration)
│   │   ├── hrn.rs              # HRN utilities
│   │   └── types.rs            # SDK types
│   └── README.md
│
└── docs/                       # Documentation
    ├── architecture/           # ADRs and architecture docs
    ├── api/                    # API documentation
    └── epic-*.md              # Epic implementation plans
```

---

## Architecture Overview

### CAP/ARP Pattern

```
┌─────────────────────────────────────────────────────────────┐
│                    Hodei Audit Service (CAP)                │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │    gRPC     │  │   Storage   │  │   Crypto    │         │
│  │  Services   │  │  Backends   │  │   Worker    │         │
│  │  :50052-54  │  │ ClickHouse  │  │ Digest/HMAC │         │
│  └─────────────┘  │      S3     │  └─────────────┘         │
│         │         └─────────────┘            │              │
│         │                                      │            │
│         ▼                                      ▼            │
│  ┌──────────────────────────────────────────────────┐      │
│  │         Multi-Tenant Security Layer              │      │
│  │  • Row-Level Security (RLS)                      │      │
│  │  • API Key Management                            │      │
│  │  • Resource Quotas & Rate Limiting               │      │
│  │  • GDPR Compliance                               │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
         │                                    │
         │                                    │ gRPC :50051
         │ gRPC :50052                        │
         ▼                                    ▼
┌─────────────────┐                    ┌──────────────┐
│  Client Apps    │                    │ Vector.dev   │
│   (Axum SDK)    │                    │   Ingestor   │
│   (ARP Layer)   │                    │   Fan-out    │
└─────────────────┘                    └──────────────┘
                                              │
                                              ▼
                              ┌─────────────────────────────┐
                              │         Storage Tiers        │
                              │  Hot: ClickHouse (recent)    │
                              │  Warm: S3 (archive)          │
                              │  Cold: Glacier (long-term)   │
                              └─────────────────────────────┘
```

### Multi-Tenancy (Epic 5)

Complete tenant isolation with 4-layer security:

1. **Tenant Context**: Thread-local storage with tenant_id, api_key, user_id
2. **gRPC Interceptors**: Request validation and context extraction
3. **API Key Management**: SHA-256 hashing with granular scopes
4. **Row-Level Security (RLS)**: Database-level tenant filtering in ClickHouse
5. **Resource Quotas**: Tier-based (Enterprise, SME, Startup)
6. **GDPR Compliance**: Data retention and right-to-be-forgotten

---

## Development Guidelines

### 1. Code Quality Standards

- **Formatting**: `cargo fmt --all` before commit
- **Linting**: `cargo clippy --all-targets --all-features` (0 warnings)
- **Documentation**: KDoc for all public APIs
- **Testing**: Tests required for all new features
- **Error Handling**: Use `anyhow::Error` for application errors

### 2. Test Requirements

**Every feature MUST have tests**:
- Unit tests for all public functions
- Integration tests for multi-component features
- Tests for multi-tenancy isolation
- Tests for error handling and edge cases

**Current Test Status**: 121 tests passing ✅

```bash
# Run all tests
cargo test --workspace

# Run specific test suites
cargo test -p hodei-audit-service tenant
cargo test -p hodei-audit-service quotas
cargo test -p hodei-audit-service api_key
cargo test -p hodei-audit-service compliance
cargo test -p hodei-audit-service row_level_security
cargo test -p hodei-audit-sdk

# Run integration tests
cargo test -p hodei-audit-service integration_tests

# Generate coverage report
cargo tarpaulin --workspace --out html --output-dir coverage/
```

### 3. Commit Standards

**All commits MUST follow Conventional Commits**:

```
<type>(<scope>): <description>

feat(epic5): complete multi-tenancy and security implementation
fix(clickhouse): resolve connection pool leak
docs(readme): update installation instructions
test(tenant): add isolation test for RLS
```

**Types**: feat, fix, docs, test, refactor, chore, perf, ci

### 4. Naming Conventions

- **HRN Format**: `hrn:partition:service:tenant:region:type/path`
  - Example: `hrn:hodei:verified-permissions:tenant-123:global:policy-store/default`
- **Module Names**: snake_case
- **Type Names**: PascalCase
- **Constants**: SCREAMING_SNAKE_CASE
- **Files**: snake_case.rs
- **Test Files**: *_test.rs
- **Integration Test Files**: integration_tests.rs

---

## Key Components Deep Dive

### 1. HRN System (Hodei Resource Names)

**Purpose**: Canonical hierarchical resource identification

**Format**: `hrn:<partition>:<service>:<tenant>:<region>:<resource-type>/<resource-path>`

**Components**:
- `hodei-audit-service/src/hrn.rs`: Core HRN implementation
- `hodei-audit-types/src/hrn.rs`: Shared HRN types
- `hodei-audit-sdk/src/hrn.rs`: SDK utilities

**Key Operations**:
- Parse HRN from string
- Generate parent/child HRNs
- Validate HRN structure
- Resolve HRN to metadata

**Usage**:
```rust
use hodei_audit_types::Hrn;

let hrn = Hrn::parse("hrn:hodei:api:tenant-123:global:user/create")?;
let parent = hrn.parent();  // hrn:hodei:api:tenant-123:global:user
let is_child = hrn.is_child_of("hrn:hodei:api:tenant-123:global");
```

### 2. Multi-Tenant Architecture

**Implemented in**: `hodei-audit-service/src/tenant.rs`

**Components**:
- `TenantContext`: Context with tenant_id, api_key, user_id, trace_id, span_id
- `TenantContextManager`: Global context management
- `TenantExtractor`: Extracts context from gRPC metadata
- `TenantTier`: Enterprise, SME, Startup tiers

**Row-Level Security**: `hodei-audit-service/src/row_level_security.rs`
- Database-level tenant isolation in ClickHouse
- Automatic tenant filtering in all queries
- SQL injection prevention

**API Key Management**: `hodei-audit-service/src/api_key.rs`
- SHA-256 key hashing (keys never stored in plain text)
- Granular scopes: AuditRead, AuditWrite, CryptoVerify, Admin, Monitoring
- Rate limiting with token bucket algorithm
- Usage tracking and last-used timestamps

**Resource Quotas**: `hodei-audit-service/src/quotas.rs`
- Tier-based quotas:
  - Enterprise: 10k events/sec, 1GB storage, 1000 API req/sec
  - SME: 1k events/sec, 10MB storage, 100 API req/sec
  - Startup: 100 events/sec, 1MB storage, 10 API req/sec
- Abuse detection (>1000 requests/minute triggers alerts)
- Real-time usage tracking

**GDPR Compliance**: `hodei-audit-service/src/compliance.rs`
- Configurable retention policies:
  - Enterprise: 7 years (2555 days)
  - SME: 1-5 years configurable
  - Startup: 1 year (365 days)
- Legal holds for data protection during litigation
- GDPR rights support: Right to be Forgotten, Data Access, Data Portability
- Complete audit trail of all deletions

### 3. SDK Middleware

**Implemented in**: `hodei-audit-sdk/src/middleware.rs`

**Usage** (1-liner integration):
```rust
use hodei_audit_sdk::{AuditSdkConfig, AuditLayer};

let app = Router::new()
    .route("/*path", get(handler))
    .layer(AuditSdkConfig::builder()
        .service_name("my-service")
        .tenant_id("tenant-123")
        .audit_service_url("http://audit:50052")
        .build()?
        .layer());
```

**Features**:
- Automatic HTTP request capture
- HRN generation from path patterns
- Batch processing (size/time/hybrid policies)
- Connection pooling and retry logic
- < 1ms overhead

**Batch Processing**:
- Size-based: Flush when N events collected
- Time-based: Flush every N seconds
- Hybrid: Flush when either condition met
- Backpressure handling

### 4. Storage Backends

**ClickHouse** (`hodei-audit-service/src/clickhouse.rs`):
- Hot tier storage (recent events, < 30 days)
- Connection pooling
- Prepared statements
- Batch inserts
- Query optimization

**S3/MinIO** (`hodei-audit-service/src/s3_storage.rs`):
- Warm/cold tier storage
- Parquet format for analytics
- Lifecycle policies
- Compression (gzip, brotli, zstd)

### 5. gRPC Services

**Ports**:
- `:50052` - AuditControlService (ingestion from SDKs)
- `:50053` - AuditQueryService (query/analytics)
- `:50054` - AuditCryptoService (cryptographic operations)
- `:50051` - VectorApi (CAP → Vector communication)

**Services**:
- `hodei-audit-service/src/grpc/audit_control_server.rs`: Event ingestion
- `hodei-audit-service/src/grpc/audit_query_server.rs`: Query and analytics
- `hodei-audit-service/src/grpc/audit_crypto_server.rs`: Digest/verification
- `hodei-audit-service/src/grpc/vector_api_server.rs`: Vector fan-out

---

## Testing Strategy

### Test Types

1. **Unit Tests** (per crate):
   - Individual function testing
   - Edge cases and error handling
   - Mock external dependencies

2. **Integration Tests** (service):
   - Multi-component testing
   - End-to-end workflows
   - Network communication
   - Database operations

3. **Multi-Tenancy Tests** (`src/tests/tenant_isolation_test.rs`):
   - Complete tenant isolation verification
   - Cross-tenant access prevention
   - RLS policy validation
   - API key scope enforcement
   - Quota enforcement per tenant
   - GDPR compliance

4. **E2E Tests** (`src/tests/e2e_multitenancy_test.rs`):
   - Full workflow testing
   - SDK → Service → Storage
   - Vector.dev integration

### Current Test Status

```
✅ 121 tests passing (100% success rate)
✅ All compilation passing
✅ Zero placeholder implementations
✅ Production-ready code
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p hodei-audit-service
cargo test -p hodei-audit-sdk

# Specific test
cargo test test_abuse_detection
cargo test test_tenant_isolation

# With output
cargo test --workspace -- --nocapture

# With logging
RUST_LOG=debug cargo test -p hodei-audit-service
```

---

## Common Development Tasks

### Adding a New Feature

1. **Create feature branch**: `git checkout -b feat/my-feature`
2. **Write tests first** (TDD approach)
3. **Implement the feature**
4. **Ensure all tests pass**: `cargo test --workspace`
5. **Format code**: `cargo fmt --all`
6. **Lint**: `cargo clippy --all-targets --all-features`
7. **Commit**: `git commit -m "feat(scope): add my feature"`
8. **Create PR** for review

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run -p hodei-audit-service

# Trace specific module
RUST_LOG=hodei_audit_service::tenant=trace cargo run

# Check test output
cargo test -p hodei-audit-service -- --nocapture

# Inspect panics
RUST_BACKTRACE=full cargo test
```

### Database Operations

```bash
# Connect to ClickHouse (dev)
docker exec -it clickhouse clickhouse-client

# Query audit events
SELECT * FROM audit_events WHERE tenant_id = 'tenant-123' LIMIT 10;

# Check RLS policies
SHOW CREATE TABLE audit_events;

# View storage tiers
SELECT * FROM storage_tiers;
```

### Performance Testing

```bash
# Run load test
node scripts/load-test-sdk.js

# Run chaos test
./scripts/chaos-test.sh

# Benchmark
cargo bench --workspace
```

---

## Configuration

### Environment Variables

```bash
# Service Configuration
AUDIT_SERVICE_GRPC_PORT=50052
AUDIT_SERVICE_HTTP_PORT=8080

# ClickHouse
CLICKHOUSE_HOST=localhost
CLICKHOUSE_PORT=9000
CLICKHOUSE_DATABASE=audit
CLICKHOUSE_USER=default
CLICKHOUSE_PASSWORD=

# S3/MinIO
S3_ENDPOINT=http://localhost:9000
S3_BUCKET=audit-events
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin

# Vector.dev
VECTOR_API_URL=http://localhost:50051
VECTOR_BUFFER_PATH=/var/lib/vector

# Security
API_KEY_HASH_ALGORITHM=sha256
ENABLE_ROW_LEVEL_SECURITY=true
GDPR_COMPLIANCE_MODE=strict

# Quotas
DEFAULT_TIER=sme
ABUSE_THRESHOLD_REQUESTS_PER_MINUTE=1000
```

### Service Configuration

```rust
// In hodei-audit-service/src/service.rs
pub struct ServiceConfig {
    pub grpc_port: u16,
    pub http_port: u16,
    pub max_batch_size: usize,
    pub enable_metrics: bool,
    pub clickhouse_config: ClickHouseConfig,
    pub s3_config: S3Config,
    pub vector_config: VectorConfig,
}
```

### SDK Configuration

```rust
// In hodei-audit-sdk/src/config.rs
pub struct AuditSdkConfig {
    pub service_name: String,
    pub tenant_id: Option<String>,
    pub audit_service_url: String,
    pub batch_size: usize,
    pub batch_timeout: Duration,
    pub grpc_timeout: Duration,
    pub max_retries: u32,
    pub enable_request_body: bool,
    pub enable_response_body: bool,
}
```

---

## CI/CD Pipeline

**File**: `.github/workflows/ci.yml`

**Stages**:
1. **Code Quality**:
   - `cargo fmt --check` (format validation)
   - `cargo clippy` (linting)
   - `cargo audit` (security scan)

2. **Testing**:
   - `cargo test --workspace` (unit tests)
   - `cargo tarpaulin` (coverage report)
   - Integration tests

3. **Build**:
   - `cargo build --workspace` (debug)
   - `cargo build --release` (release)
   - Cross-compilation checks

4. **Security**:
   - Dependency vulnerability scan
   - Code security analysis
   - Supply chain security

**Coverage Requirement**: >= 80%

**Badge**: `[![CI](https://github.com/ORG/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/ORG/REPO/actions/workflows/ci.yml)`

---

## Documentation

### Key Documentation Files

- `docs/README-plan.md` - Complete epic plan (10 epics)
- `docs/epic-01-fundacion-y-arquitectura.md` - Architecture foundation
- `docs/epic-05-multi-tenancy-y-seguridad.md` - Multi-tenancy implementation
- `docs/architecture/cap-arp-architecture.md` - CAP/ARP pattern
- `docs/architecture/cloudtrail-patterns.md` - CloudTrail compatibility
- `docs/api/grpc-contracts.md` - gRPC API documentation
- `hodei-audit-sdk/README.md` - SDK usage guide
- `EPIC5_COMPLETION_REPORT.md` - Epic 5 implementation summary

### Generating Docs

```bash
# Generate Rust documentation
just docs

# Serve locally
just docs-serve

# Check documentation links
markdown-link-check docs/**/*.md
```

### ADRs (Architecture Decision Records)

Location: `docs/architecture/`

Common ADRs:
- `adr-001-cap-arp-pattern.md`
- `adr-002-cloudtrail-compatibility.md`
- `adr-003-hrn-system.md`
- `adr-004-grpc-communication.md`
- `adr-005-vector-dev-integration.md`
- `adr-006-multi-tenancy-strategy.md`
- `adr-007-row-level-security.md`

---

## Troubleshooting

### Build Issues

```bash
# Clean build
just clean
cargo build --workspace

# Update dependencies
cargo update

# Check for system deps
rustup show
```

### Test Failures

```bash
# Run with detailed output
cargo test -p hodei-audit-service -- --nocapture

# Run specific failing test
cargo test test_name -- --exact --nocapture

# Check for race conditions
cargo test -p hodei-audit-service -- --test-threads=1
```

### Database Connection

```bash
# Check ClickHouse
curl http://localhost:8123ping

# Test S3/MinIO
mc alias set local http://localhost:9000 minioadmin minioadmin
mc ls local

# Check Vector.dev
curl http://localhost:9598/health
```

### Performance Issues

```bash
# Profile the service
cargo flamegraph -p hodei-audit-service

# Check metrics
curl http://localhost:8080/metrics

# Analyze coverage
open coverage/tarpaulin-report.html
```

---

## Epic Roadmap

**Current Status**: Epic 5 (Multi-Tenancy) ✅ COMPLETED

**Next Epics**:
- Epic 6: Criptographic Digest y Compliance
- Epic 7: Alto Rendimiento y Escalabilidad
- Epic 8: Vector.dev y Persistencia Avanzada
- Epic 9: Observabilidad y Métricas
- Epic 10: DevOps y Despliegue

**Target**: 23-30 weeks total (6-7 months)

---

## Useful Resources

### Internal
- [Epic 5 Completion Report](./EPIC5_COMPLETION_REPORT.md) - Complete Epic 5 implementation details
- [SDK Integration Guide](./hodei-audit-sdk/INTEGRATION-VERIFIED-PERMISSIONS.md)
- [Test Reports](./coverage/tarpaulin-report.html)

### External
- [Tonic (gRPC)](https://docs.rs/tonic/)
- [Axum (Web)](https://docs.rs/axum/)
- [ClickHouse Docs](https://clickhouse.com/docs/)
- [Vector.dev Docs](https://vector.dev/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)

---

## Code Health

**Current Metrics**:
- ✅ 121 tests passing (100% success)
- ✅ 0 compilation errors
- ✅ 0 TODO comments or placeholders
- ✅ 100% feature complete for Epic 5
- ✅ Enterprise-grade security
- ✅ Production ready

**Quality Gates**:
- All tests must pass
- Zero clippy warnings
- 100% formatter compliance
- Documentation for all public APIs
- Security scan clean

---

## Tips for Claude Code

1. **When working on multi-tenancy**: Always consider tenant isolation first
2. **When adding tests**: Follow the TDD approach - write tests first
3. **When modifying APIs**: Update both implementation and tests
4. **When debugging**: Enable `RUST_LOG=debug` for detailed traces
5. **When performance matters**: Consider batch processing and connection pooling
6. **When security matters**: Review all tenant isolation and RLS policies
7. **When integrating with SDK**: Use the 1-liner middleware approach
8. **When adding storage**: Follow the tiered storage pattern (hot/warm/cold)

**Remember**: This is a production-grade system with strict requirements for:
- Complete tenant isolation
- GDPR compliance
- High performance (10K+ events/sec)
- Security (zero tolerance for cross-tenant access)
- Observability (comprehensive metrics and tracing)

---

**Last Updated**: 2025-11-06 (Epic 5 Completion)
**Version**: 1.0
**Status**: Production Ready
