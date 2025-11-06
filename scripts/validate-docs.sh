#!/bin/bash
# Script de validación de documentación para Historia 1.1

set -e

echo "🔍 Validando documentación de arquitectura..."

# Verificar que existe el documento de arquitectura
if [ ! -f "docs/architecture/cap-arp-architecture.md" ]; then
    echo "❌ ERROR: docs/architecture/cap-arp-architecture.md no existe"
    exit 1
fi

# Verificar que contiene diagramas Mermaid
if ! grep -q "mermaid" docs/architecture/cap-arp-architecture.md; then
    echo "❌ ERROR: Documento no contiene diagramas Mermaid"
    exit 1
fi

# Verificar secciones requeridas
sections=("CAP" "ARP" "Vector.dev" "Flujo de Datos")
for section in "${sections[@]}"; do
    if ! grep -qi "$section" docs/architecture/cap-arp-architecture.md; then
        echo "❌ ERROR: Sección '$section' no encontrada en el documento"
        exit 1
    fi
done

echo "✅ Documentación validada correctamente"
echo "📄 Archivo: docs/architecture/cap-arp-architecture.md"
echo "📊 Diagramas Mermaid: $(grep -c "mermaid" docs/architecture/cap-arp-architecture.md) encontrados"
