#!/bin/bash
# Script de validación de CI/CD para Historia 1.3

set -e

echo "🔄 Validando configuración de CI/CD..."

# Verificar .github/workflows
if [ ! -d ".github/workflows" ]; then
    echo "❌ ERROR: .github/workflows no existe"
    exit 1
fi
echo "  ✅ .github/workflows"

# Verificar workflow CI
if [ ! -f ".github/workflows/ci.yml" ]; then
    echo "❌ ERROR: .github/workflows/ci.yml no existe"
    exit 1
fi
echo "  ✅ ci.yml"

# Verificar contenido del workflow
if ! grep -q "test:" .github/workflows/ci.yml; then
    echo "❌ ERROR: Workflow no incluye job de test"
    exit 1
fi
echo "  ✅ Job de test"

if ! grep -q "build:" .github/workflows/ci.yml; then
    echo "❌ ERROR: Workflow no incluye job de build"
    exit 1
fi
echo "  ✅ Job de build"

if ! grep -q "clippy" .github/workflows/ci.yml; then
    echo "❌ ERROR: Workflow no incluye clippy (linting)"
    exit 1
fi
echo "  ✅ Linting con clippy"

if ! grep -q "cargo audit" .github/workflows/ci.yml; then
    echo "❌ ERROR: Workflow no incluye cargo audit (security)"
    exit 1
fi
echo "  ✅ Security audit"

# Verificar justfile
if [ ! -f "justfile" ]; then
    echo "❌ ERROR: justfile no existe"
    exit 1
fi
echo "  ✅ justfile"

# Verificar comandos básicos en justfile
commands=("fmt" "lint" "test" "build" "ci" "coverage" "clean")
for cmd in "${commands[@]}"; do
    if ! grep -q "^$cmd:" justfile; then
        echo "❌ ERROR: Comando '$cmd' no definido en justfile"
        exit 1
    fi
done
echo "  ✅ Comandos justfile"

echo ""
echo "✅ Configuración de CI/CD validada correctamente"
