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

test-unit:
    cargo test --workspace --lib

test-integration:
    cargo test --workspace --test '*'

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
    cargo bench --workspace

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
