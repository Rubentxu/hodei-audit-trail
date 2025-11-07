# Just - Justfile para Hodei Audit

# Configuración
set dotenv-load := false
set export := true

# Variables
PROJECT_NAME := "hodei-audit"
DEFAULT_CARGO_ARGS := "--workspace"

# Help
default:
    @just --list

# Ayuda específica para tests
test-help:
    @echo ""
    @echo "╔════════════════════════════════════════════════════════════╗"
    @echo "║           🧪 COMANDOS DE TESTS - HODEI AUDIT             ║"
    @echo "╚════════════════════════════════════════════════════════════╝"
    @echo ""
    @echo "📋 TESTS BÁSICOS:"
    @echo "  just test               - Ejecutar todos los tests (154 tests)"
    @echo "  just test-integration   - Tests de integración (Epic 5, 6, 8)"
    @echo "  just test-watch         - Tests en modo watch"
    @echo ""
    @echo "📊 COBERTURA:"
    @echo "  just coverage           - Generar reporte de cobertura HTML"
    @echo ""
    @echo "🏁 BENCHMARKS (Epic 7):"
    @echo "  just perf-test          - ⚡ EJECUTAR TODOS los benchmarks (rápido)"
    @echo "  just bench-epic7        - TODOS los benchmarks de Epic 7"
    @echo "  just bench-batcher      - SmartBatcher policies"
    @echo "  just bench-connection-pool - gRPC connection pool"
    @echo "  just bench-backpressure - Backpressure controller"
    @echo "  just bench-circuit-breaker - Circuit breaker"
    @echo "  just bench-zero-copy    - Zero-copy batching"
    @echo "  just bench-throughput   - Throughput target (100K/sec)"
    @echo "  just bench-concurrent   - Concurrent operations"
    @echo ""
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo ""
    @echo "💡 EJEMPLOS:"
    @echo "  just test-integration"
    @echo "  just bench-epic7"
    @echo "  just coverage"
    @echo ""

# Setup inicial del proyecto
setup:
    echo "🚀 Setting up Hodei Audit development environment..."
    cargo install just
    cargo install cargo-watch
    rustup target add x86_64-unknown-linux-musl
    just check

# Formateo de código
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Linting
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Verificación de código
check:
    cargo check --workspace

# Tests
test:
    cargo test --workspace --all-targets

test-integration:
    @echo "🧪 Ejecutando todos los tests de integración..."
    cargo test -p hodei-audit-service --lib

test-watch:
    cargo watch -x test

# Coverage
coverage:
    cargo tarpaulin --workspace --out html --output-dir coverage/

# Build
build:
    cargo build --workspace

build-release:
    cargo build --workspace --release

# Docs
docs:
    cargo doc --workspace --no-deps --open

docs-serve:
    python3 -m http.server --directory target/doc 8000

# Security audit
audit:
    cargo audit

audit-fix:
    cargo audit --fix

# CI pipeline
ci:
    just fmt-check
    just lint
    just check
    just test
    just audit

# Development
dev:
    cargo watch -x run

# Benchmarks
bench:
    @echo "🏃 Ejecutando todos los benchmarks del workspace..."
    cargo bench --workspace

# Alias para ejecutar todos los benchmarks rápidamente
perf-test:
    @echo "🚀 Ejecutando TODOS los benchmarks de Epic 7..."
    just bench-epic7

# Benchmarks de Epic 7 - Performance
bench-epic7:
    @echo ""
    @echo "╔════════════════════════════════════════════════════════════╗"
    @echo "║         🏁 BENCHMARKS EPIC 7 - PERFORMANCE TEST          ║"
    @echo "╚════════════════════════════════════════════════════════════╝"
    @echo ""
    @echo "📊 Ejecutando benchmarks de Epic 7 (Alto Rendimiento)..."
    @echo ""
    cd benchmarks && cargo bench
    @echo ""
    @echo "✅ Benchmarks de Epic 7 completados!"

# Benchmarks específicos de Epic 7
bench-batcher:
    @echo "🎯 Benchmarking SmartBatcher policies..."
    cargo bench -p hodei-audit-benchmarks smart_batcher_policies

bench-connection-pool:
    @echo "🔗 Benchmarking Connection Pool..."
    cargo bench -p hodei-audit-benchmarks connection_pool

bench-backpressure:
    @echo "⬇️  Benchmarking Backpressure Controller..."
    cargo bench -p hodei-audit-benchmarks backpressure_controller

bench-circuit-breaker:
    @echo "🔄 Benchmarking Circuit Breaker..."
    cargo bench -p hodei-audit-benchmarks circuit_breaker

bench-zero-copy:
    @echo "⚡ Benchmarking Zero-Copy Batching..."
    cargo bench -p hodei-audit-benchmarks zero_copy_batching

bench-throughput:
    @echo "🚀 Benchmarking Throughput Target (100K events/sec)..."
    cargo bench -p hodei-audit-benchmarks throughput_target

bench-concurrent:
    @echo "🔀 Benchmarking Concurrent Operations..."
    cargo bench -p hodei-audit-benchmarks concurrent_operations

# Clean
clean:
    cargo clean
    rm -rf coverage/
    rm -rf target/debug/deps/*_*

# Lint de documentación
docs-lint:
    markdown-link-check docs/**/*.md --config .markdown-link-check.json

# Setup hooks
setup-hooks:
    cp scripts/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

# Validación de arquitectura
validate-architecture:
    ./scripts/validate-docs.sh
    ./scripts/check-architecture-consistency.sh
    ./scripts/validate-adr.sh
    ./scripts/validate-cloudtrail-mapping.sh

# Validación de estructura
validate-structure:
    ./scripts/validate-project-structure.sh

# Run service
run:
    cargo run -p hodei-audit-service

# Instalación
install-deps:
    cargo install just
    cargo install cargo-watch
    cargo install cargo-audit
    cargo install cargo-tarpaulin
    npm install -g markdown-link-check
