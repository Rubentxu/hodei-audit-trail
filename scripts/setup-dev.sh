#!/bin/bash
# Script de setup de entorno de desarrollo para Hodei Audit

set -e

echo "🚀 Setting up Hodei Audit development environment..."
echo ""

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para imprimir con color
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Verificar prerequisitos
echo "🔍 Checking prerequisites..."

if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker first."
    exit 1
fi
print_success "Docker found"

if ! command -v docker-compose &> /dev/null && ! command -v docker compose &> /dev/null; then
    print_error "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi
print_success "Docker Compose found"

if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed. Please install Rust first."
    exit 1
fi
print_success "Rust/Cargo found"

if ! command -v just &> /dev/null; then
    print_warning "Just is not installed. Installing..."
    cargo install just
fi
print_success "Just found"

# Crear directorios necesarios
echo ""
echo "📁 Creating directories..."
mkdir -p config/clickhouse
mkdir -p config/vector
mkdir -p config/prometheus
mkdir -p config/grafana/dashboards
mkdir -p config/grafana/datasources
mkdir -p logs
mkdir -p data/clickhouse
mkdir -p data/vector
mkdir -p data/minio
mkdir -p data/prometheus
mkdir -p data/grafana

print_success "Directories created"

# Verificar si .env existe
if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        print_warning ".env not found, copying from .env.example"
        cp .env.example .env
        print_success ".env created from .env.example"
    else
        print_warning ".env not found, you may need to create it"
    fi
else
    print_success ".env file found"
fi

# Construir y levantar servicios
echo ""
echo "🐳 Starting Docker services..."
echo "This may take a few minutes on first run..."

if command -v docker compose &> /dev/null; then
    DOCKER_COMPOSE="docker compose"
else
    DOCKER_COMPOSE="docker-compose"
fi

# Verificar si los servicios ya están corriendo
if $DOCKER_COMPOSE -f docker-compose.dev.yml ps | grep -q "Up"; then
    print_warning "Some services are already running."
    read -p "Do you want to restart them? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        $DOCKER_COMPOSE -f docker-compose.dev.yml down
    fi
fi

$DOCKER_COMPOSE -f docker-compose.dev.yml up -d

# Esperar a que los servicios estén listos
echo ""
echo "⏳ Waiting for services to be ready..."

# ClickHouse
echo "Waiting for ClickHouse..."
retries=30
until $DOCKER_COMPOSE -f docker-compose.dev.yml exec -T clickhouse clickhouse-client --query "SELECT 1" &> /dev/null || [ $retries -eq 0 ]; do
    echo -n "."
    sleep 2
    retries=$((retries-1))
done

if [ $retries -eq 0 ]; then
    print_error "ClickHouse failed to start"
    exit 1
fi
print_success "ClickHouse is ready"

# MinIO
echo "Waiting for MinIO..."
retries=30
until curl -s http://localhost:9000/minio/health/live &> /dev/null || [ $retries -eq 0 ]; do
    echo -n "."
    sleep 2
    retries=$((retries-1))
done

if [ $retries -eq 0 ]; then
    print_error "MinIO failed to start"
    exit 1
fi
print_success "MinIO is ready"

# Vector
echo "Waiting for Vector..."
retries=30
until curl -s http://localhost:9598/health &> /dev/null || [ $retries -eq 0 ]; do
    echo -n "."
    sleep 2
    retries=$((retries-1))
done

if [ $retries -eq 0 ]; then
    print_warning "Vector health check failed (this is normal on first run)"
else
    print_success "Vector is ready"
fi

# Prometheus
echo "Waiting for Prometheus..."
retries=30
until curl -s http://localhost:9090/-/healthy &> /dev/null || [ $retries -eq 0 ]; do
    echo -n "."
    sleep 2
    retries=$((retries-1))
done

if [ $retries -eq 0 ]; then
    print_warning "Prometheus health check failed"
else
    print_success "Prometheus is ready"
fi

# Verificar estado de los servicios
echo ""
echo "📊 Service status:"
$DOCKER_COMPOSE -f docker-compose.dev.yml ps

# Instalar dependencias Rust
echo ""
echo "📦 Installing Rust dependencies..."
just install-deps

# Build del proyecto
echo ""
echo "🔨 Building Hodei Audit project..."
just build

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Services available at:"
echo "  - ClickHouse:     http://localhost:8123"
echo "  - MinIO:          http://localhost:9001 (console) / :9000 (API)"
echo "  - Prometheus:     http://localhost:9090"
echo "  - Grafana:        http://localhost:3000 (admin/admin123)"
echo "  - Jaeger:         http://localhost:16686"
echo "  - Vector:         gRPC on port 50051"
echo ""
echo "Useful commands:"
echo "  - just dev        : Start development server"
echo "  - just test       : Run tests"
echo "  - just build      : Build project"
echo "  - just validate-structure : Validate project structure"
echo "  - just validate-architecture : Validate architecture docs"
echo ""
echo "To stop services:"
echo "  docker-compose -f docker-compose.dev.yml down"
echo ""
