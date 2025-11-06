#!/bin/bash
# Script de validación de enlaces para Historia 1.1

set -e

echo "🔗 Validando enlaces en documentación..."

# Instalar markdown-link-check si no está disponible
if ! command -v markdown-link-check &> /dev/null; then
    echo "📦 Instalando markdown-link-check..."
    npm install -g markdown-link-check
fi

# Verificar enlaces en el documento de arquitectura
if [ -f "docs/architecture/cap-arp-architecture.md" ]; then
    echo "🔍 Verificando enlaces en cap-arp-architecture.md..."
    markdown-link-check docs/architecture/cap-arp-architecture.md --config .markdown-link-check.json || true
else
    echo "⚠️  Archivo docs/architecture/cap-arp-architecture.md no existe aún"
fi

# Verificar que las imágenes referenciadas existen
if [ -f "docs/architecture/cap-arp-architecture.md" ]; then
    # Buscar referencias a imágenes
    image_refs=$(grep -oP '!\[.*\]\((\K)[^)]+' docs/architecture/cap-arp-architecture.md || true)
    if [ -n "$image_refs" ]; then
        echo "🖼️  Verificando imágenes referenciadas..."
        for img in $image_refs; do
            if [[ $img == http* ]]; then
                echo "   ✓ Imagen externa: $img"
            else
                if [ -f "docs/architecture/$img" ]; then
                    echo "   ✓ Imagen local encontrada: $img"
                else
                    echo "   ⚠️  Imagen no encontrada: docs/architecture/$img"
                fi
            fi
        done
    fi
fi

echo "✅ Validación de enlaces completada"
