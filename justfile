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
    @echo "📋 TESTS BACKEND (Rust):"
    @echo "  just test               - Ejecutar todos los tests backend"
    @echo "  just test-integration   - Tests de integración backend"
    @echo "  just test-watch         - Tests backend en modo watch"
    @echo ""
    @echo "⚛️  TESTS FRONTEND (Next.js/React):"
    @echo "  just test-frontend      - Ejecutar todos los tests frontend (Jest)"
    @echo "  just test-frontend-watch - Tests frontend en modo watch"
    @echo "  just test-e2e           - Tests E2E (Playwright - todos los navegadores)"
    @echo "  just test-e2e-chrome    - Tests E2E solo Chrome"
    @echo "  just test-e2e-firefox   - Tests E2E solo Firefox"
    @echo "  just test-e2e-webkit    - Tests E2E solo WebKit"
    @echo "  just test-performance   - Tests de rendimiento (Playwright)"
    @echo "  just test-security      - Tests de seguridad (Playwright)"
    @echo ""
    @echo "📊 COBERTURA:"
    @echo "  just coverage           - Generar reporte de cobertura backend"
    @echo "  just coverage-frontend  - Generar reporte de cobertura frontend"
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
    @echo "  just test-frontend      # Solo tests frontend"
    @echo "  just test-e2e          # Solo tests E2E"
    @echo "  just coverage-frontend # Cobertura frontend"
    @echo ""
    @echo "🚀 TESTS COMPLETOS (Backend + Frontend):"
    @echo "  just test-all          - Ejecutar TODOS los tests (backend + frontend)"
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

test-all:
    @echo ""
    @echo "╔════════════════════════════════════════════════════════════╗"
    @echo "║            🚀 EJECUTANDO TODOS LOS TESTS                  ║"
    @echo "╚════════════════════════════════════════════════════════════╝"
    @echo ""
    @echo "📋 1/4 - Ejecutando tests backend (Rust)..."
    just test
    @echo ""
    @echo "⚛️  2/4 - Ejecutando tests frontend (Jest)..."
    just test-frontend
    @echo ""
    @echo "🎭 3/4 - Ejecutando tests E2E (Playwright)..."
    just test-e2e
    @echo ""
    @echo "🔒 4/4 - Ejecutando tests de seguridad..."
    just test-security
    @echo ""
    @echo "✅ TODOS LOS TESTS COMPLETADOS EXITOSAMENTE!"
    @echo ""

test-integration:
    @echo "🧪 Ejecutando todos los tests de integración..."
    cargo test -p hodei-audit-service --lib

test-watch:
    cargo watch -x test

# Frontend Tests (Next.js/React)
test-frontend:
    @echo "🧪 Ejecutando tests frontend (Jest + React Testing Library)..."
    cd hodei-audit-web && npm test -- --coverage --watchAll=false
    @echo "✅ Tests frontend completados"

test-frontend-watch:
    @echo "🧪 Ejecutando tests frontend en modo watch..."
    cd hodei-audit-web && npm test -- --watch
    @echo "✅ Tests frontend en modo watch iniciados"

# E2E Tests (Playwright)
test-e2e:
    @echo "🎭 Ejecutando tests E2E (Playwright - todos los navegadores)..."
    cd hodei-audit-web && npx playwright test tests-e2e
    @echo "✅ Tests E2E completados"

test-e2e-chrome:
    @echo "🎭 Ejecutando tests E2E en Chrome..."
    cd hodei-audit-web && npx playwright test tests-e2e --project=chromium
    @echo "✅ Tests E2E Chrome completados"

test-e2e-firefox:
    @echo "🎭 Ejecutando tests E2E en Firefox..."
    cd hodei-audit-web && npx playwright test tests-e2e --project=firefox
    @echo "✅ Tests E2E Firefox completados"

test-e2e-webkit:
    @echo "🎭 Ejecutando tests E2E en WebKit..."
    cd hodei-audit-web && npx playwright test tests-e2e --project=webkit
    @echo "✅ Tests E2E WebKit completados"

# Performance Tests
test-performance:
    @echo "⚡ Ejecutando tests de rendimiento..."
    cd hodei-audit-web && npx playwright test tests-e2e/performance.spec.ts
    @echo "✅ Tests de rendimiento completados"

# Security Tests
test-security:
    @echo "🔒 Ejecutando tests de seguridad..."
    cd hodei-audit-web && npx playwright test tests-e2e/security.spec.ts
    @echo "✅ Tests de seguridad completados"

# Coverage
coverage:
    cargo tarpaulin --workspace --out html --output-dir coverage/

coverage-frontend:
    @echo "📊 Generando reporte de cobertura frontend..."
    cd hodei-audit-web && npm run test:coverage:html
    @echo "✅ Reporte de cobertura frontend generado en hodei-audit-web/coverage/coverage-report.html"

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

# ============================================================================
# 🚀 DESARROLLO CON HOT RELOADING
# ============================================================================

# Levantar TODO en modo desarrollo (backend + frontend)
dev-all:
    @echo "🚀 Iniciando entorno de desarrollo completo..."
    @bash scripts/dev/dev-start.sh all

# Levantar solo backend Rust con hot reloading
dev-backend:
    @echo "🦀 Iniciando backend Rust con hot reloading..."
    @bash scripts/dev/dev-start.sh backend

# Levantar solo frontend Next.js con hot reloading
dev-frontend:
    @echo "⚛️  Iniciando frontend Next.js con hot reloading..."
    @bash scripts/dev/dev-start.sh frontend

# Parar todos los servicios de desarrollo
dev-stop:
    @echo "🛑 Deteniendo todos los servicios de desarrollo..."
    @bash scripts/dev/dev-stop.sh

# Reiniciar todos los servicios
dev-restart:
    @echo "🔄 Reiniciando todos los servicios..."
    just dev-stop
    sleep 2
    just dev-all

# Ver logs de desarrollo
dev-logs:
    @echo "📋 Mostrando logs de desarrollo..."
    @bash scripts/dev/dev-logs.sh

# Ver estado de los servicios
dev-status:
    @echo "📊 Estado de los servicios de desarrollo..."
    @bash scripts/dev/dev-status.sh

# Instalar herramientas de desarrollo
dev-setup:
    @echo "🔧 Instalando herramientas de desarrollo..."
    cargo install just
    cargo install cargo-watch
    cargo install cargo-expand
    cargo install cargo-audit
    npm install -g @next/cli
    @echo "✅ Herramientas instaladas"

# Desarrollo con UI dashboard
dev-ui:
    @echo "🖥️  Iniciando dashboard de desarrollo..."
    @bash scripts/dev/dev-dashboard.sh

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

# ================================================================
# 🔍 DIAGNÓSTICO Y DOCUMENTACIÓN AUTOMATIZADA (Epic 08)
# ================================================================

# Ejecutar diagnóstico completo de la aplicación web
diagnostic:
    @echo "🔍 Ejecutando diagnóstico completo de la aplicación..."
    cd hodei-audit-web && node scripts/simple-diagnostic.js
    @echo ""
    @echo "✅ Diagnóstico completado. Revisa:"
    @echo "   - Reporte: hodei-audit-web/docs/DIAGNOSTIC-REPORT.md"
    @echo "   - Screenshots: hodei-audit-web/docs/diagnostic/screenshots/"

# Generar screenshots para documentación
screenshots:
    @echo "📸 Generando screenshots para documentación..."
    cd hodei-audit-web && npx playwright test tests-e2e/screenshot.spec.ts --project=chromium --reporter=list
    @echo ""
    @echo "✅ Screenshots generados en: hodei-audit-web/docs/screenshots/"

# Generar screenshots con script standalone
screenshots-generate:
    @echo "📸 Generando screenshots (script standalone)..."
    cd hodei-audit-web && node scripts/generate-screenshots.js
    @echo ""
    @echo "✅ Screenshots generados en: hodei-audit-web/docs/screenshots/"

# Actualizar documentación completa (screenshots + diagnóstico)
docs-update:
    @echo "📚 Actualizando documentación completa..."
    @echo ""
    @echo "1️⃣ Generando screenshots..."
    just screenshots-generate
    @echo ""
    @echo "2️⃣ Ejecutando diagnóstico..."
    just diagnostic
    @echo ""
    @echo "✅ Documentación actualizada!"

# Verificar estado de la aplicación
health-check:
    @echo "🏥 Verificando estado de la aplicación..."
    @echo ""
    @echo "🔍 Frontend (Next.js):"
    @curl -s http://localhost:3000 > /dev/null && echo "   ✅ http://localhost:3000 - OK" || echo "   ❌ http://localhost:3000 - ERROR"
    @echo ""
    @echo "🔍 Backend (HTTP):"
    @curl -s http://localhost:8080/health > /dev/null 2>&1 && echo "   ✅ http://localhost:8080 - OK" || echo "   ❌ http://localhost:8080 - Not responding"
    @echo ""
    @echo "🔍 gRPC Service:"
    @curl -s http://localhost:9000 > /dev/null 2>&1 && echo "   ✅ http://localhost:9000 - OK" || echo "   ❌ http://localhost:9000 - Not responding"
    @echo ""
    @echo "🔍 Metrics:"
    @curl -s http://localhost:9090/metrics > /dev/null 2>&1 && echo "   ✅ http://localhost:9090 - OK" || echo "   ❌ http://localhost:9090 - Not responding"
