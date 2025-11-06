#!/bin/bash
# Script de validación de estructura de proyecto para Historia 1.3

set -e

echo "🏗️  Validando estructura de proyecto Rust..."

# Verificar workspace Cargo.toml
if [ ! -f "Cargo.toml" ]; then
    echo "❌ ERROR: Cargo.toml del workspace no existe"
    exit 1
fi

echo "✅ Workspace Cargo.toml existe"

# Verificar crates requeridos
required_crates=("hodei-audit-proto" "hodei-audit-types" "hodei-audit-service" "hodei-audit-sdk")
for crate in "${required_crates[@]}"; do
    if [ ! -d "$crate" ]; then
        echo "❌ ERROR: Crate '$crate' no existe"
        exit 1
    fi
    if [ ! -f "$crate/Cargo.toml" ]; then
        echo "❌ ERROR: Cargo.toml de '$crate' no existe"
        exit 1
    fi
    echo "  ✅ $crate"
done

# Verificar estructura de directorios
echo ""
echo "📁 Verificando estructura de directorios..."

# hodei-audit-proto
if [ ! -d "hodei-audit-proto/proto" ]; then
    echo "❌ ERROR: hodei-audit-proto/proto no existe"
    exit 1
fi
echo "  ✅ hodei-audit-proto/proto"

# hodei-audit-types/src
if [ ! -d "hodei-audit-types/src" ]; then
    echo "❌ ERROR: hodei-audit-types/src no existe"
    exit 1
fi
echo "  ✅ hodei-audit-types/src"

# hodei-audit-service/src
if [ ! -d "hodei-audit-service/src" ]; then
    echo "❌ ERROR: hodei-audit-service/src no existe"
    exit 1
fi
echo "  ✅ hodei-audit-service/src"

# Verificar justfile
if [ ! -f "justfile" ]; then
    echo "❌ ERROR: justfile no existe"
    exit 1
fi
echo "  ✅ justfile"

# Verificar .github/workflows
if [ ! -d ".github/workflows" ]; then
    echo "❌ ERROR: .github/workflows no existe"
    exit 1
fi
echo "  ✅ .github/workflows"

# Verificar scripts
if [ ! -d "scripts" ]; then
    echo "❌ ERROR: scripts no existe"
    exit 1
fi
echo "  ✅ scripts"

# Verificar docs
if [ ! -d "docs" ]; then
    echo "❌ ERROR: docs no existe"
    exit 1
fi
echo "  ✅ docs"

echo ""
echo "✅ Estructura de proyecto validada correctamente"
