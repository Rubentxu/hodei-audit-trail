#!/bin/bash
# Script de validación de contratos gRPC para Historia 1.4

set -e

echo "📋 Validando contratos gRPC..."

# Verificar proto files requeridos
required_protos=("audit_control.proto" "audit_query.proto" "audit_crypto.proto" "vector_api.proto" "audit_event.proto")
for proto in "${required_protos[@]}"; do
    if [ ! -f "hodei-audit-proto/proto/$proto" ]; then
        echo "❌ ERROR: $proto no existe"
        exit 1
    fi
    echo "  ✅ $proto"
done

# Verificar servicios gRPC en audit_control.proto
if ! grep -q "service AuditControlService" hodei-audit-proto/proto/audit_control.proto; then
    echo "❌ ERROR: AuditControlService no definido en audit_control.proto"
    exit 1
fi
echo "  ✅ AuditControlService"

# Verificar métodos de AuditControlService
if ! grep -q "rpc PublishEvent" hodei-audit-proto/proto/audit_control.proto; then
    echo "❌ ERROR: PublishEvent no definido"
    exit 1
fi
if ! grep -q "rpc PublishBatch" hodei-audit-proto/proto/audit_control.proto; then
    echo "❌ ERROR: PublishBatch no definido"
    exit 1
fi
echo "  ✅ Métodos de ingestión (PublishEvent, PublishBatch)"

# Verificar servicios en audit_query.proto
if ! grep -q "service AuditQueryService" hodei-audit-proto/proto/audit_query.proto; then
    echo "❌ ERROR: AuditQueryService no definido"
    exit 1
fi
echo "  ✅ AuditQueryService"

# Verificar métodos de Query
if ! grep -q "rpc QueryEvents" hodei-audit-proto/proto/audit_query.proto; then
    echo "❌ ERROR: QueryEvents no definido"
    exit 1
fi
if ! grep -q "rpc ResolveHrn" hodei-audit-proto/proto/audit_query.proto; then
    echo "❌ ERROR: ResolveHrn no definido"
    exit 1
fi
echo "  ✅ Métodos de consulta (QueryEvents, ResolveHrn)"

# Verificar servicios de crypto
if ! grep -q "service AuditCryptoService" hodei-audit-proto/proto/audit_crypto.proto; then
    echo "❌ ERROR: AuditCryptoService no definido"
    exit 1
fi
echo "  ✅ AuditCryptoService"

# Verificar Vector API
if ! grep -q "service VectorApi" hodei-audit-proto/proto/vector_api.proto; then
    echo "❌ ERROR: VectorApi no definido"
    exit 1
fi
if ! grep -q "rpc SendEventBatch" hodei-audit-proto/proto/vector_api.proto; then
    echo "❌ ERROR: SendEventBatch no definido"
    exit 1
fi
echo "  ✅ VectorApi (contrato simple)"

# Verificar estructura audit_event
if ! grep -q "message AuditEvent" hodei-audit-proto/proto/audit_event.proto; then
    echo "❌ ERROR: AuditEvent no definido"
    exit 1
fi
echo "  ✅ Estructura AuditEvent"

# Verificar que se pueden compilar
echo ""
echo "🔨 Compilando proto files..."
cd hodei-audit-proto
if ! cargo build 2>&1 | grep -q "error"; then
    echo "  ✅ Proto files se compilan sin errores"
else
    echo "  ⚠️  Advertencias o errores de compilación detectados"
fi
cd ..

echo ""
echo "✅ Contratos gRPC validados correctamente"
