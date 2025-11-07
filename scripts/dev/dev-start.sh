#!/bin/bash
# Script para iniciar servicios de desarrollo con hot reloading
# Controla PIDs y puertos, mata procesos anteriores automáticamente

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración de puertos
BACKEND_PORT=8080
FRONTEND_PORT=3000
GRPC_PORT=9000
METRICS_PORT=9090

# Directorio de PIDs
PID_DIR=".dev/pids"
PID_FILE_BACKEND="$PID_DIR/backend.pid"
PID_FILE_FRONTEND="$PID_DIR/frontend.pid"
PID_FILE_GRPC="$PID_DIR/grpc.pid"
PID_FILE_METRICS="$PID_DIR/metrics.pid"

# Función para crear directorio de PIDs
mkdir -p "$PID_DIR"

# Función para imprimir con color
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Función para obtener PID de un puerto
get_pid_by_port() {
    local port=$1
    lsof -ti:$port 2>/dev/null || true
}

# Función para kill por puerto
kill_by_port() {
    local port=$1
    local pid=$(get_pid_by_port $port)
    if [ -n "$pid" ]; then
        print_warning "Matando proceso en puerto $port (PID: $pid)"
        kill -9 $pid 2>/dev/null || true
        sleep 1
    fi
}

# Función para limpiar PIDs huérfanos
cleanup_orphaned_pids() {
    for pid_file in "$PID_FILE_BACKEND" "$PID_FILE_FRONTEND" "$PID_FILE_GRPC" "$PID_FILE_METRICS"; do
        if [ -f "$pid_file" ]; then
            local pid=$(cat "$pid_file")
            if ! kill -0 $pid 2>/dev/null; then
                print_warning "Limpiando PID huérfano: $pid_file"
                rm -f "$pid_file"
            fi
        fi
    done
}

# Función para iniciar backend Rust
start_backend() {
    print_status "🦀 Iniciando backend Rust con hot reloading..."

    # Verificar si ya está corriendo
    if [ -f "$PID_FILE_BACKEND" ]; then
        local pid=$(cat "$PID_FILE_BACKEND")
        if kill -0 $pid 2>/dev/null; then
            print_warning "Backend ya está corriendo (PID: $pid)"
            return 0
        fi
    fi

    # Matar proceso en puerto BACKEND
    kill_by_port $BACKEND_PORT
    kill_by_port $GRPC_PORT
    kill_by_port $METRICS_PORT

    # Iniciar con cargo watch para hot reloading
    cargo watch \
        --watch hodei-audit-service \
        --watch hodei-audit-common \
        --ignore 'target/' \
        --ignore '*.log' \
        --ignore '.git/' \
        --exec "cargo run -p hodei-audit-service" \
        --delay 2 \
        > .dev/logs/backend.log 2>&1 &

    local pid=$!
    echo $pid > "$PID_FILE_BACKEND"

    # Esperar a que esté listo
    sleep 3

    # Verificar que esté corriendo
    if kill -0 $pid 2>/dev/null; then
        print_status "✅ Backend iniciado (PID: $pid)"
        print_status "   - HTTP: http://localhost:$BACKEND_PORT"
        print_status "   - gRPC: http://localhost:$GRPC_PORT"
        print_status "   - Metrics: http://localhost:$METRICS_PORT/metrics"
        return 0
    else
        print_error "❌ Error al iniciar backend"
        cat .dev/logs/backend.log
        return 1
    fi
}

# Función para iniciar frontend Next.js
start_frontend() {
    print_status "⚛️  Iniciando frontend Next.js con hot reloading..."

    # Verificar si ya está corriendo
    if [ -f "$PID_FILE_FRONTEND" ]; then
        local pid=$(cat "$PID_FILE_FRONTEND")
        if kill -0 $pid 2>/dev/null; then
            print_warning "Frontend ya está corriendo (PID: $pid)"
            return 0
        fi
    fi

    # Matar proceso en puerto FRONTEND
    kill_by_port $FRONTEND_PORT

    # Iniciar Next.js
    cd hodei-audit-web
    npm run dev > ../../.dev/logs/frontend.log 2>&1 &
    local pid=$!
    cd ../..

    echo $pid > "$PID_FILE_FRONTEND"

    # Esperar a que esté listo
    sleep 5

    # Verificar que esté corriendo
    if kill -0 $pid 2>/dev/null; then
        print_status "✅ Frontend iniciado (PID: $pid)"
        print_status "   - URL: http://localhost:$FRONTEND_PORT"
        print_status "   - Hot reload: ✅ Habilitado"
        return 0
    else
        print_error "❌ Error al iniciar frontend"
        cat .dev/logs/frontend.log
        return 1
    fi
}

# Función para iniciar todos los servicios
start_all() {
    print_status "🚀 Iniciando entorno de desarrollo completo..."
    print_status "=========================================="

    # Limpiar PIDs huérfanos
    cleanup_orphaned_pids

    # Crear logs
    mkdir -p .dev/logs

    # Iniciar backend
    if ! start_backend; then
        print_error "Fallo al iniciar backend"
        exit 1
    fi

    # Esperar un poco antes de iniciar frontend
    sleep 2

    # Iniciar frontend
    if ! start_frontend; then
        print_error "Fallo al iniciar frontend"
        exit 1
    fi

    print_status "=========================================="
    print_status "🎉 ¡Entorno de desarrollo listo!"
    print_status ""
    print_status "Servicios corriendo:"
    print_status "  • Backend Rust: http://localhost:$BACKEND_PORT"
    print_status "  • Frontend Next.js: http://localhost:$FRONTEND_PORT"
    print_status "  • gRPC Gateway: http://localhost:$GRPC_PORT"
    print_status "  • Metrics: http://localhost:$METRICS_PORT/metrics"
    print_status ""
    print_status "Logs en: .dev/logs/"
    print_status "Para parar: just dev-stop"
    print_status "Para ver logs: just dev-logs"
    print_status "Para ver estado: just dev-status"
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [all|backend|frontend]"
    echo ""
    echo "Comandos:"
    echo "  all      - Iniciar backend y frontend (por defecto)"
    echo "  backend  - Iniciar solo backend Rust"
    echo "  frontend - Iniciar solo frontend Next.js"
    echo ""
    echo "Puertos:"
    echo "  Backend: $BACKEND_PORT"
    echo "  Frontend: $FRONTEND_PORT"
    echo "  gRPC: $GRPC_PORT"
    echo "  Metrics: $METRICS_PORT"
}

# Main
case "${1:-all}" in
    "all")
        start_all
        ;;
    "backend")
        start_backend
        ;;
    "frontend")
        start_frontend
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        print_error "Comando inválido: $1"
        show_help
        exit 1
        ;;
esac
