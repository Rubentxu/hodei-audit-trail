#!/bin/bash
# Script para verificar estado de servicios de desarrollo

set -e

# Colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuración
PID_DIR=".dev/pids"
BACKEND_PORT=8080
FRONTEND_PORT=3000
GRPC_PORT=9000
METRICS_PORT=9090

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}         📊 ESTADO DE SERVICIOS           ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_service() {
    local name=$1
    local port=$2
    local pid_file=$3
    local log_file=$4
    local color=$5

    echo -e "${color}┌─ $name${NC}"
    echo -e "${color}│${NC}"

    # Verificar puerto
    if lsof -ti:$port >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} Puerto $port: OCUPADO"
        local pid=$(lsof -ti:$port)
        echo -e "  ${GREEN}✓${NC} PID: $pid"
    else
        echo -e "  ${RED}✗${NC} Puerto $port: LIBRE"
    fi

    # Verificar PID file
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 $pid 2>/dev/null; then
            echo -e "  ${GREEN}✓${NC} PID File: Válido (PID: $pid)"
        else
            echo -e "  ${RED}✗${NC} PID File: Huérfano (PID: $pid no existe)"
        fi
    else
        echo -e "  ${YELLOW}⚠${NC} PID File: No existe"
    fi

    # Verificar log
    if [ -f "$log_file" ]; then
        local lines=$(wc -l < "$log_file")
        local last_mod=$(stat -c %Y "$log_file" 2>/dev/null || stat -f %m "$log_file" 2>/dev/null)
        local now=$(date +%s)
        local age=$((now - last_mod))

        if [ $age -lt 60 ]; then
            echo -e "  ${GREEN}✓${NC} Log: $lines líneas (hace ${age}s)"
        else
            local age_min=$((age / 60))
            echo -e "  ${YELLOW}⚠${NC} Log: $lines líneas (hace ${age_min}m)"
        fi
    else
        echo -e "  ${YELLOW}⚠${NC} Log: No existe"
    fi

    echo -e "${color}└────────────────────────────────────────────${NC}"
    echo ""
}

print_urls() {
    echo -e "${BLUE}🌐 URLs de Desarrollo:${NC}"
    echo ""
    echo -e "  ${GREEN}✓${NC} Frontend:   http://localhost:$FRONTEND_PORT"
    echo -e "  ${GREEN}✓${NC} Backend:    http://localhost:$BACKEND_PORT"
    echo -e "  ${GREEN}✓${NC} gRPC:       http://localhost:$GRPC_PORT"
    echo -e "  ${GREEN}✓${NC} Metrics:    http://localhost:$METRICS_PORT/metrics"
    echo ""
}

print_commands() {
    echo -e "${BLUE}⚙️  Comandos Disponibles:${NC}"
    echo ""
    echo -e "  ${YELLOW}just dev-all${NC}          - Iniciar todos los servicios"
    echo -e "  ${YELLOW}just dev-backend${NC}     - Iniciar solo backend"
    echo -e "  ${YELLOW}just dev-frontend${NC}    - Iniciar solo frontend"
    echo -e "  ${YELLOW}just dev-stop${NC}        - Detener todos los servicios"
    echo -e "  ${YELLOW}just dev-restart${NC}     - Reiniciar todos los servicios"
    echo -e "  ${YELLOW}just dev-logs${NC}        - Ver logs"
    echo -e "  ${YELLOW}just dev-status${NC}      - Ver este estado"
    echo ""
}

# Verificar herramientas
check_tools() {
    echo -e "${BLUE}🔧 Herramientas:${NC}"
    echo ""

    # Cargo
    if command -v cargo >/dev/null 2>&1; then
        local version=$(cargo --version | cut -d' ' -f2)
        echo -e "  ${GREEN}✓${NC} cargo: $version"
    else
        echo -e "  ${RED}✗${NC} cargo: No instalado"
    fi

    # cargo-watch
    if command -v cargo-watch >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} cargo-watch: Instaldo"
    else
        echo -e "  ${YELLOW}⚠${NC} cargo-watch: No instalado (ejecuta: just dev-setup)"
    fi

    # npm
    if command -v npm >/dev/null 2>&1; then
        local version=$(npm --version)
        echo -e "  ${GREEN}✓${NC} npm: v$version"
    else
        echo -e "  ${RED}✗${NC} npm: No instalado"
    fi

    # lsof
    if command -v lsof >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} lsof: Disponible"
    else
        echo -e "  ${RED}✗${NC} lsof: No instalado (necesario para verificar puertos)"
    fi

    echo ""
}

# Verificar directorios
check_directories() {
    echo -e "${BLUE}📁 Directorios:${NC}"
    echo ""

    for dir in "hodei-audit-service" "hodei-audit-web" ".dev" ".dev/pids" ".dev/logs"; do
        if [ -d "$dir" ]; then
            echo -e "  ${GREEN}✓${NC} $dir"
        else
            echo -e "  ${RED}✗${NC} $dir (no existe)"
        fi
    done

    echo ""
}

# Main
print_header
check_tools
check_directories

echo -e "${BLUE}🔍 Estado de Servicios:${NC}"
echo ""

# Backend
print_service "Backend Rust" $BACKEND_PORT ".dev/pids/backend.pid" ".dev/logs/backend.log" "$GREEN"

# Frontend
print_service "Frontend Next.js" $FRONTEND_PORT ".dev/pids/frontend.pid" ".dev/logs/frontend.log" "$BLUE"

# URLs
if lsof -ti:$FRONTEND_PORT >/dev/null 2>&1 || lsof -ti:$BACKEND_PORT >/dev/null 2>&1; then
    print_urls
fi

# Comandos
print_commands

# Resumen
total_running=0
if lsof -ti:$BACKEND_PORT >/dev/null 2>&1; then ((total_running++)); fi
if lsof -ti:$FRONTEND_PORT >/dev/null 2>&1; then ((total_running++)); fi

echo -e "${BLUE}📊 Resumen:${NC}"
echo ""
if [ $total_running -eq 2 ]; then
    echo -e "  ${GREEN}🎉 Todos los servicios están corriendo${NC}"
elif [ $total_running -eq 1 ]; then
    echo -e "  ${YELLOW}⚠️  Solo 1 de 2 servicios está corriendo${NC}"
else
    echo -e "  ${RED}❌ Ningún servicio está corriendo${NC}"
fi
echo ""
