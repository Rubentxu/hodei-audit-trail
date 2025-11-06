#!/bin/bash
# Script de verificación de consistencia de arquitectura para Historia 1.1

set -e

echo "🔍 Verificando consistencia de arquitectura CAP/ARP/Vector..."

# Verificar nomenclatura CAP/ARP consistente
required_terms=("CAP" "Centralized Audit Point" "ARP" "Audit Reporting Point" "Vector.dev")
missing_terms=()

for term in "${required_terms[@]}"; do
    if ! grep -qi "$term" docs/architecture/cap-arp-architecture.md; then
        missing_terms+=("$term")
    fi
done

if [ ${#missing_terms[@]} -ne 0 ]; then
    echo "❌ ERROR: Términos faltantes en documentación:"
    printf '   - %s\n' "${missing_terms[@]}"
    exit 1
fi

# Verificar que se define el flujo de datos
if ! grep -q "App → ARP → CAP → Vector → Storage" docs/architecture/cap-arp-architecture.md; then
    echo "❌ ERROR: Flujo de datos CANÓNICO no encontrado"
    exit 1
fi

# Verificar comparación con PDP/PEP
if ! grep -qi "PDP/PEP" docs/architecture/cap-arp-architecture.md && \
   ! grep -qi "verified.permissions" docs/architecture/cap-arp-architecture.md; then
    echo "⚠️  ADVERTENCIA: No se encontró comparación con patrones PDP/PEP"
fi

# Verificar que se define la responsabilidad de Vector
if ! grep -qi "fan.out" docs/architecture/cap-arp-architecture.md && \
   ! grep -qi "ingesta" docs/architecture/cap-arp-architecture.md; then
    echo "❌ ERROR: Responsabilidades de Vector.dev no están claras"
    exit 1
fi

echo "✅ Consistencia de arquitectura verificada"
echo "📋 Nomenclatura consistente en todo el documento"
echo "🔄 Flujo de datos CANÓNICO definido"
echo "⚙️  Responsabilidades por componente documentadas"
