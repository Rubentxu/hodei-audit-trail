#!/bin/bash
# Script para ver logs de desarrollo en tiempo real

set -e

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_header() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

# Función para mostrar logs
show_logs() {
    local service=$1
    local log_file=$2
    local color=$3

    if [ -f "$log_file" ]; then
        local lines=$(wc -l < "$log_file")
        echo -e "${color}📄 $service${NC}"
        echo "   Archivo: $log_file"
        echo "   Líneas: $lines"
        echo "   Últimas 20 líneas:"
        echo "   ────────────────────"
        tail -n 20 "$log_file" | sed 's/^/   /'
        echo ""
    else
        echo -e "${YELLOW}⚠️  $service${NC}"
        echo "   Archivo no encontrado: $log_file"
        echo ""
    fi
}

# Función para mostrar logs en tiempo real
tail_logs() {
    local service=$1
    local log_file=$2
    local color=$3

    if [ -f "$log_file" ]; then
        echo -e "${color}👀 $service (tiempo real)${NC}"
        echo "   Presiona Ctrl+C para salir"
        echo "   ────────────────────"
        tail -f "$log_file" | sed 's/^/   /'
    else
        echo -e "${YELLOW}⚠️  $service${NC}"
        echo "   Archivo no encontrado: $log_file"
    fi
}

# Verificar si existe tail-f
has_tail_f() {
    which tail >/dev/null 2>&1
}

# Main
print_header "📋 LOGS DE DESARROLLO - HODEI AUDIT"

if [ "$1" == "follow" ] || [ "$1" == "-f" ] || [ "$1" == "--follow" ]; then
    # Modo follow
    service="${2:-all}"

    case "$service" in
        "backend")
            tail_logs "Backend Rust" ".dev/logs/backend.log" "$GREEN"
            ;;
        "frontend")
            tail_logs "Frontend Next.js" ".dev/logs/frontend.log" "$BLUE"
            ;;
        "all")
            echo "Selecciona el servicio a seguir:"
            echo "  1) Backend"
            echo "  2) Frontend"
            read -p "Opción [1-2]: " option

            case "$option" in
                1)
                    tail_logs "Backend Rust" ".dev/logs/backend.log" "$GREEN"
                    ;;
                2)
                    tail_logs "Frontend Next.js" ".dev/logs/frontend.log" "$BLUE"
                    ;;
                *)
                    echo "Opción inválida"
                    exit 1
                    ;;
            esac
            ;;
        *)
            echo "Servicio inválido: $service"
            echo "Uso: $0 [follow] [backend|frontend|all]"
            exit 1
            ;;
    esac
else
    # Modo normal - mostrar últimos logs
    echo "Mostrando últimos logs de todos los servicios..."
    echo ""

    show_logs "Backend Rust" ".dev/logs/backend.log" "$GREEN"
    show_logs "Frontend Next.js" ".dev/logs/frontend.log" "$BLUE"

    echo ""
    echo "💡 Para seguir logs en tiempo real:"
    echo "   just dev-logs follow backend"
    echo "   just dev-logs follow frontend"
    echo ""
    echo "💡 Para limpiar logs:"
    echo "   rm -rf .dev/logs/*"
fi
