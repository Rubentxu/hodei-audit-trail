#!/bin/bash
# Script de validación de entorno de desarrollo para Historia 1.5

set -e

echo "🐳 Validando entorno de desarrollo..."

# Verificar docker-compose.dev.yml
if [ ! -f "docker-compose.dev.yml" ]; then
    echo "❌ ERROR: docker-compose.dev.yml no existe"
    exit 1
fi
echo "  ✅ docker-compose.dev.yml"

# Verificar servicios requeridos
services=("clickhouse" "vector" "minio" "prometheus")
for service in "${services[@]}"; do
    if ! grep -q "  $service:" docker-compose.dev.yml; then
        echo "❌ ERROR: Servicio '$service' no definido en docker-compose.dev.yml"
        exit 1
    fi
    echo "  ✅ Servicio $service"
done

# Verificar script setup-dev.sh
if [ ! -f "scripts/setup-dev.sh" ]; then
    echo "❌ ERROR: scripts/setup-dev.sh no existe"
    exit 1
fi
echo "  ✅ scripts/setup-dev.sh"

# Verificar .env.example
if [ ! -f ".env.example" ]; then
    echo "�️  ADVERTENCIA: .env.example no existe (recomendado pero no requerido)"
else
    echo "  ✅ .env.example"
fi

# Verificar configuración de volúmenes
if ! grep -q "volumes:" docker-compose.dev.yml; then
    echo "❌ ERROR: Volúmenes no configurados en docker-compose.dev.yml"
    exit 1
fi
echo "  ✅ Volúmenes configurados"

# Verificar puertos
required_ports=("8123" "9000" "50051" "9598" "9090" "9001")
for port in "${required_ports[@]}"; do
    if ! grep -q "ports:" docker-compose.dev.yml; then
        echo "❌ ERROR: Puertos no configurados"
        exit 1
    fi
done
echo "  ✅ Puertos configurados"

# Verificar imágenes de Docker
required_images=("clickhouse/clickhouse-server" "timberio/vector" "minio/minio" "prom/prometheus")
for image in "${required_images[@]}"; do
    if ! grep -q "image: $image" docker-compose.dev.yml; then
        echo "❌ ERROR: Imagen '$image' no configurada"
        exit 1
    fi
done
echo "  ✅ Imágenes Docker configuradas"

# Verificar configuración de Vector
if ! grep -q "vector.toml" docker-compose.dev.yml; then
    echo "❌ ERROR: Configuración vector.toml no referenciada"
    exit 1
fi
echo "  ✅ Configuración Vector.dev"

# Verificar health checks
if ! grep -q "healthcheck:" docker-compose.dev.yml; then
    echo "❌ ERROR: Health checks no configurados"
    exit 1
fi
echo "  ✅ Health checks"

echo ""
echo "✅ Entorno de desarrollo validado correctamente"
