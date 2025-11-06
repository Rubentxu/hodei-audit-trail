#!/bin/bash
# Script de validación de mapeo CloudTrail → Hodei para Historia 1.2

set -e

echo "🔄 Validando mapeo de conceptos CloudTrail → Hodei..."

# Verificar que existe el documento
if [ ! -f "docs/architecture/cloudtrail-patterns.md" ]; then
    echo "❌ ERROR: Documento cloudtrail-patterns.md no existe"
    exit 1
fi

# Verificar taxonomía de eventos
event_types=("Management" "Data" "Insight")
for event_type in "${event_types[@]}"; do
    if ! grep -qi "EventCategory.*$event_type" docs/architecture/cloudtrail-patterns.md && \
       ! grep -qi "management\|data\|insight" docs/architecture/cloudtrail-patterns.md; then
        echo "⚠️  ADVERTENCIA: Event category '$event_type' no encontrado explícitamente"
    fi
done

# Verificar estructura de eventos CloudTrail-compatibles
required_fields=("EventID" "ReadOnly" "EventTime" "SourceIPAddress" "UserAgent" "ErrorCode" "ErrorMessage")
for field in "${required_fields[@]}"; do
    if ! grep -qi "$field" docs/architecture/cloudtrail-patterns.md; then
        echo "❌ ERROR: Campo requerido '$field' no encontrado"
        exit 1
    fi
done

# Verificar diseño de digest criptográfico
digest_concepts=("SHA-256" "ed25519" "digest" "hash" "chain")
for concept in "${digest_concepts[@]}"; do
    if ! grep -qi "$concept" docs/architecture/cloudtrail-patterns.md; then
        echo "❌ ERROR: Concepto de digest '$concept' no encontrado"
        exit 1
    fi
done

# Verificar AdditionalEventData
if ! grep -qi "AdditionalEventData" docs/architecture/cloudtrail-patterns.md; then
    echo "❌ ERROR: Campo AdditionalEventData no documentado"
    exit 1
fi

# Verificar que se valida con casos de uso del PRD
if ! grep -qi "PRD" docs/architecture/cloudtrail-patterns.md && \
   ! grep -qi "use case\|caso de uso" docs/architecture/cloudtrail-patterns.md; then
    echo "⚠️  ADVERTENCIA: No se encontró validación con casos de uso del PRD"
fi

echo "✅ Mapeo de conceptos CloudTrail → Hodei validado"
echo "🔑 Campos requeridos: $(echo ${#required_fields[@]}) verificados"
echo "🔐 Digest criptográfico: SHA-256 + ed25519 documentado"
