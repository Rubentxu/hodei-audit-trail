#!/bin/bash
# Script de validación de CI/CD para Historia 1.7

set -e

echo "🔄 Validando CI/CD Base para Historia 1.7..."

# Verificar GitHub Actions
if [ ! -f ".github/workflows/ci.yml" ]; then
    echo "❌ ERROR: .github/workflows/ci.yml no existe"
    exit 1
fi
echo "  ✅ GitHub Actions configurado"

# Verificar jobs requeridos
jobs=("test" "build" "coverage")
for job in "${jobs[@]}"; do
    if ! grep -q "$job:" .github/workflows/ci.yml; then
        echo "❌ ERROR: Job '$job' no definido"
        exit 1
    fi
done
echo "  ✅ Jobs CI/CD (test, build, coverage)"

# Verificar matrix de testing
if ! grep -q "matrix:" .github/workflows/ci.yml; then
    echo "❌ ERROR: Matrix de testing no configurada"
    exit 1
fi
echo "  ✅ Matrix de testing (stable, nightly)"

# Verificar steps de calidad
quality_steps=("clippy" "fmt" "audit")
for step in "${quality_steps[@]}"; do
    if ! grep -q "$step" .github/workflows/ci.yml; then
        echo "❌ ERROR: Step '$step' no configurado"
        exit 1
    fi
done
echo "  ✅ Quality gates (clippy, fmt, audit)"

# Verificar artifacts
if ! grep -q "upload-artifact" .github/workflows/ci.yml; then
    echo "❌ ERROR: Upload de artifacts no configurado"
    exit 1
fi
echo "  ✅ Artifacts configurados"

# Verificar caching
if ! grep -q "actions/cache" .github/workflows/ci.yml; then
    echo "❌ ERROR: Caching de dependencias no configurado"
    exit 1
fi
echo "  ✅ Caching de dependencias"

# Verificar triggers
if ! grep -q "push:" .github/workflows/ci.yml || ! grep -q "pull_request:" .github/workflows/ci.yml; then
    echo "� ERROR: Triggers de CI/CD no configurados"
    exit 1
fi
echo "  ✅ Triggers (push, pull_request)"

# Verificar justfile commands
ci_commands=("fmt-check" "lint" "test" "build" "coverage" "audit")
for cmd in "${ci_commands[@]}"; do
    if ! grep -q "^$cmd:" justfile; then
        echo "❌ ERROR: Comando '$cmd' no definido en justfile"
        exit 1
    fi
done
echo "  ✅ Comandos justfile para CI"

echo ""
echo "✅ CI/CD Base validado correctamente"
echo "📋 Pipeline automatizado configurado"
