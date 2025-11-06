#!/bin/bash
# Script de validación de ADR (Architectural Decision Records) para Historia 1.2

set -e

echo "📋 Validando ADRs de CloudTrail..."

# Verificar que existe el documento de patrones CloudTrail
if [ ! -f "docs/architecture/cloudtrail-patterns.md" ]; then
    echo "❌ ERROR: docs/architecture/cloudtrail-patterns.md no existe"
    exit 1
fi

# Verificar secciones requeridas
sections=("CloudTrail" "Event Categories" "Management" "Data" "Insight" "Digest" "EventID" "ReadOnly")
missing_sections=()

for section in "${sections[@]}"; do
    if ! grep -qi "$section" docs/architecture/cloudtrail-patterns.md; then
        missing_sections+=("$section")
    fi
done

if [ ${#missing_sections[@]} -ne 0 ]; then
    echo "❌ ERROR: Secciones faltantes en documentación:"
    printf '   - %s\n' "${missing_sections[@]}"
    exit 1
fi

# Verificar que existe el ADR específico
if [ ! -f "docs/architecture/adr-cloudtrail.md" ]; then
    echo "❌ ERROR: docs/architecture/adr-cloudtrail.md no existe"
    exit 1
fi

# Verificar campos en ADR
adr_fields=("Status" "Context" "Decision" "Consequences")
for field in "${adr_fields[@]}"; do
    if ! grep -qi "$field" docs/architecture/adr-cloudtrail.md; then
        echo "❌ ERROR: Campo '$field' no encontrado en ADR"
        exit 1
    fi
done

echo "✅ ADRs validados correctamente"
echo "📄 Documento: docs/architecture/cloudtrail-patterns.md"
echo "📄 ADR: docs/architecture/adr-cloudtrail.md"
