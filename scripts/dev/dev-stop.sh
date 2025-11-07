#!/bin/bash
# Script para detener servicios de desarrollo
# Mata procesos por PID y por puerto para asegurar limpieza

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Configuración
PID_DIR=".dev/pids"
PID_FILE_BACKEND="$PID_DIR/backend.pid"
PID_FILE_FRONTEND="$PID_DIR/frontend.pid"
PID_FILE_GRPC="$PID_DIR/grpc.pid"
PID_FILE_METRICS="$PID_DIR/metrics.pid"

BACKEND_PORT=8080
FRONTEND_PORT=3000
GRPC_PORT=9000
METRICS_PORT=9090

# Función para obtener PID de un puerto
get_pid_by_port() {
    local port=$1
    lsof -ti:$port 2>/dev/null || true
}

# Función para kill por PID
kill_by_pid() {
    local pid_file=$1
    local name=$2

    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 $pid 2>/dev/null; then
            print_status "Deteniendo $name (PID: $pid)..."
            kill -TERM $pid 2>/dev/null || true

            # Esperar hasta 5 segundos
            local count=0
            while kill -0 $pid 2>/dev/null && [ $count -lt 50 ]; do
                sleep 0.1
                count=$((count + 1))
            done

            # Si aún está vivo, forzar kill
            if kill -0 $pid 2>/dev/null; then
                print_warning "$name no respondió, forzando kill..."
                kill -9 $pid 2>/dev/null || true
            fi

            print_status "✅ $name detenido"
        else
            print_warning "$name no estaba corriendo"
        fi
        rm -f "$pid_file"
    else
        print_warning "No se encontró PID file para $name"
    fi
}

# Función para kill por puerto
kill_by_port() {
    local port=$1
    local name=$2

    local pid=$(get_pid_by_port $port)
    if [ -n "$pid" ]; then
        print_warning "Encontrado proceso $name en puerto $port (PID: $pid)"
        kill -TERM $pid 2>/dev/null || true

        # Esperar
        sleep 1

        # Forzar kill si es necesario
        if lsof -ti:$port >/dev/null 2>&1; then
            print_warning "Forzando kill de $name..."
            kill -9 $pid 2>/dev/null || true
            sleep 1
        fi

        if ! lsof -ti:$port >/dev/null 2>&1; then
            print_status "✅ $name en puerto $port liberado"
        else
            print_error "❌ No se pudo liberar puerto $port"
        fi
    fi
}

# Función para limpiar cargo watch
cleanup_cargo_watch() {
    print_status "Limpiando procesos cargo-watch..."

    # Buscar y matar cargo-watch
    pkill -f "cargo watch" 2>/dev/null || true
    pkill -f "cargo-run" 2>/dev/null || true

    # Limpiar procesos npm
    pkill -f "npm run dev" 2>/dev/null || true
    pkill -f "next dev" 2>/dev/null || true

    print_status "✅ Procesos cargo-watch limpiados"
}

# Función principal
main() {
    print_status "🛑 Deteniendo servicios de desarrollo..."
    print_status "=========================================="

    # Detener por PID primero
    print_status "Deteniendo por PID files..."
    kill_by_pid "$PID_FILE_BACKEND" "Backend Rust"
    kill_by_pid "$PID_FILE_FRONTEND" "Frontend Next.js"

    # También limpiar otros PID files si existen
    if [ -f "$PID_FILE_GRPC" ]; then rm -f "$PID_FILE_GRPC"; fi
    if [ -f "$PID_FILE_METRICS" ]; then rm -f "$PID_FILE_METRICS"; fi

    # Limpiar por puerto por seguridad
    print_status "Limpiando puertos..."
    kill_by_port $BACKEND_PORT "Backend"
    kill_by_port $FRONTEND_PORT "Frontend"
    kill_by_port $GRPC_PORT "gRPC"
    kill_by_port $METRICS_PORT "Metrics"

    # Limpiar cargo-watch y npm
    cleanup_cargo_watch

    # Limpiar procesos node en el directorio frontend
    if [ -d "hodei-audit-web" ]; then
        pkill -f "hodei-audit-web" 2>/dev/null || true
    fi

    # Limpiar procesos rust que usen el workspace
    pkill -f "hodei-audit-service" 2>/dev/null || true

    print_status "=========================================="
    print_status "✅ Todos los servicios detenidos"
    print_status ""
    print_status "Para reiniciar: just dev-all"
}

# Ejecutar
main
