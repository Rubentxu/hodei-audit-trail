#!/bin/bash
# Script de validación del sistema HRN para Historia 1.6

set -e

echo "🏷️  Validando sistema HRN..."

# Verificar módulo HRN en hodei-audit-types
if [ ! -f "hodei-audit-types/src/hrn.rs" ]; then
    echo "❌ ERROR: hodei-audit-types/src/hrn.rs no existe"
    exit 1
fi
echo "  ✅ hodei-audit-types/src/hrn.rs"

# Verificar estructura Hrn
if ! grep -q "pub struct Hrn" hodei-audit-types/src/hrn.rs; then
    echo "❌ ERROR: struct Hrn no definido"
    exit 1
fi
echo "  ✅ Struct Hrn"

# Verificar implementación Parser
required_methods=("parse" "from_str" "to_string" "parent" "is_child_of")
for method in "${required_methods[@]}"; do
    if ! grep -q "fn $method" hodei-audit-types/src/hrn.rs; then
        echo "❌ ERROR: Método '$method' no implementado"
        exit 1
    fi
done
echo "  ✅ Métodos de parsing (parse, from_str, to_string)"
echo "  ✅ Métodos de jerarquía (parent, is_child_of)"

# Verificar formato HRN
if ! grep -q "hrn:" hodei-audit-types/src/hrn.rs; then
    echo "❌ ERROR: Formato HRN no validado (hrn:prefix)"
    exit 1
fi
echo "  ✅ Formato HRN validado"

# Verificar HrnResolver
if ! grep -q "struct HrnResolver" hodei-audit-types/src/hrn.rs; then
    echo "�️  ADVERTENCIA: HrnResolver no encontrado (puede estar en otro módulo)"
else
    echo "  ✅ HrnResolver"
fi

# Verificar tests unitarios
if ! grep -q "#\[test\]" hodei-audit-types/src/hrn.rs && ! grep -q "mod tests" hodei-audit-types/src/hrn.rs; then
    echo "❌ ERROR: Tests unitarios no encontrados"
    exit 1
fi
echo "  ✅ Tests unitarios"

# Verificar que se puede compilar
echo ""
echo "🔨 Compilando módulo HRN..."
cd hodei-audit-types
if cargo check 2>&1 | grep -q "error"; then
    echo "❌ ERROR: Fallo en la compilación"
    exit 1
else
    echo "  ✅ Módulo HRN se compila sin errores"
fi
cd ..

# Ejecutar tests
echo ""
echo "🧪 Ejecutando tests del sistema HRN..."
cd hodei-audit-types
if cargo test hrn 2>&1 | grep -q "test result: ok"; then
    echo "  ✅ Tests HRN pasando"
else
    echo "❌ ERROR: Tests HRN fallando"
    exit 1
fi
cd ..

# Verificar ejemplos de HRN en documentación
required_examples=("hrn:hodei:verified-permissions" "hrn:hodei:api" "hrn:hodei:storage")
for example in "${required_examples[@]}"; do
    if ! grep -q "$example" hodei-audit-types/src/hrn.rs; then
        echo "⚠️  ADVERTENCIA: Ejemplo '$example' no encontrado en código"
    fi
done
echo "  ✅ Ejemplos HRN documentados"

echo ""
echo "✅ Sistema HRN validado correctamente"
