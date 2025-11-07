#!/bin/bash
# Dashboard de desarrollo interactivo
# Muestra estado en tiempo real de los servicios

set -e

# Colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuración
REFRESH_INTERVAL=2
BACKEND_PORT=8080
FRONTEND_PORT=3000
GRPC_PORT=9000
METRICS_PORT=9090

# Función para limpiar pantalla
clear_screen() {
    clear
}

# Función para obtener estado de puerto
get_port_status() {
    local port=$1
    if lsof -ti:$port >/dev/null 2>&1; then
        local pid=$(lsof -ti:$port)
        echo -e "${GREEN}●${NC} PID:$pid"
    else
        echo -e "${RED}○${NC} Libre"
    fi
}

# Función para obtener uso de memoria
get_memory_usage() {
    if command -v ps >/dev/null 2>&1; then
        local pids=$(lsof -ti:$BACKEND_PORT,$FRONTEND_PORT 2>/dev/null || true)
        if [ -n "$pids" ]; then
            local total_mem=$(ps -p $pids -o rss= 2>/dev/null | awk '{sum+=$1} END {print sum/1024}' || echo "0")
            printf "%.1f MB" $total_mem
        else
            echo "0 MB"
        fi
    else
        echo "N/A"
    fi
}

# Función para obtener últimas líneas de log
get_last_log_lines() {
    local log_file=$1
    local lines=${2:-5}

    if [ -f "$log_file" ]; then
        tail -n $lines "$log_file" 2>/dev/null | head -3
    else
        echo "No hay logs"
    fi
}

# Función para dibujar header
draw_header() {
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}                    🖥️  DASHBOARD DE DESARROLLO                   ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}                         HODEI AUDIT                              ${BLUE}║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""
}

# Función para dibujar servicios
draw_services() {
    echo -e "${CYAN}┌──────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│${NC}                        🔧 SERVICIOS                               ${CYAN}│${NC}"
    echo -e "${CYAN}└──────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""

    echo -e "  🦀 Backend Rust (HTTP)"
    echo -e "     Puerto: $BACKEND_PORT   Estado: $(get_port_status $BACKEND_PORT)"
    echo -e "     URL:    http://localhost:$BACKEND_PORT"
    echo ""

    echo -e "  ⚛️  Frontend Next.js"
    echo -e "     Puerto: $FRONTEND_PORT  Estado: $(get_port_status $FRONTEND_PORT)"
    echo -e "     URL:    http://localhost:$FRONTEND_PORT"
    echo ""

    echo -e "  🔌 gRPC Gateway"
    echo -e "     Puerto: $GRPC_PORT      Estado: $(get_port_status $GRPC_PORT)"
    echo -e "     URL:    http://localhost:$GRPC_PORT"
    echo ""

    echo -e "  📊 Metrics"
    echo -e "     Puerto: $METRICS_PORT   Estado: $(get_port_status $METRICS_PORT)"
    echo -e "     URL:    http://localhost:$METRICS_PORT/metrics"
    echo ""
}

# Función para dibujar logs recientes
draw_logs() {
    echo -e "${CYAN}┌──────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│${NC}                        📋 LOGS RECIENTES                          ${CYAN}│${NC}"
    echo -e "${CYAN}└──────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""

    # Backend logs
    echo -e "${GREEN}Backend:${NC}"
    get_last_log_lines ".dev/logs/backend.log" 3 | sed 's/^/  /'
    echo ""

    # Frontend logs
    echo -e "${BLUE}Frontend:${NC}"
    get_last_log_lines ".dev/logs/frontend.log" 3 | sed 's/^/  /'
    echo ""
}

# Función para dibujar comandos
draw_commands() {
    echo -e "${CYAN}┌──────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│${NC}                        ⚙️  COMANDOS RÁPIDOS                       ${CYAN}│${NC}"
    echo -e "${CYAN}└──────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""

    echo -e "  ${YELLOW}r${NC} - Reiniciar servicios"
    echo -e "  ${YELLOW}s${NC} - Detener servicios"
    echo -e "  ${YELLOW}l${NC} - Ver logs completos"
    echo -e "  ${YELLOW}q${NC} - Salir"
    echo ""
}

# Función para dibujar resumen
draw_summary() {
    echo -e "${CYAN}┌──────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${CYAN}│${NC}                        📊 RESUMEN                                ${CYAN}│${NC}"
    echo -e "${CYAN}└──────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""

    local running=0
    if lsof -ti:$BACKEND_PORT >/dev/null 2>&1; then ((running++)); fi
    if lsof -ti:$FRONTEND_PORT >/dev/null 2>&1; then ((running++)); fi

    echo -e "  Servicios corriendo: $running/2"
    echo -e "  Memoria utilizada: $(get_memory_usage)"
    echo ""
}

# Función para mostrar ayuda
show_help() {
    echo ""
    echo -e "${YELLOW}Atajos de teclado:${NC}"
    echo -e "  r - Reiniciar servicios"
    echo -e "  s - Detener servicios"
    echo -e "  l - Ver logs"
    echo -e "  q - Salir del dashboard"
    echo -e "  Ctrl+C - Salir"
    echo ""
    read -p "Presiona Enter para continuar..."
}

# Función principal con bucle
main_loop() {
    local running=true
    local count=0

    while $running; do
        clear_screen
        draw_header
        draw_services
        draw_summary
        draw_logs
        draw_commands

        # Incrementar contador
        count=$((count + 1))

        # Leer input sin bloquear (solo cada 10 iteraciones)
        if [ $((count % 5)) -eq 0 ]; then
            if read -t $REFRESH_INTERVAL -n 1 key 2>/dev/null; then
                case $key in
                    'q'|'Q')
                        running=false
                        ;;
                    'r'|'R')
                        echo ""
                        echo -e "${YELLOW}Reiniciando servicios...${NC}"
                        just dev-restart
                        sleep 3
                        ;;
                    's'|'S')
                        echo ""
                        echo -e "${YELLOW}Deteniendo servicios...${NC}"
                        just dev-stop
                        sleep 1
                        ;;
                    'l'|'L')
                        show_help
                        ;;
                esac
            fi
        else
            # Solo sleep
            sleep $REFRESH_INTERVAL
        fi
    done

    clear_screen
    echo -e "${GREEN}👋 Dashboard cerrado${NC}"
    echo ""
}

# Verificar si es modo one-shot
if [ "$1" == "once" ] || [ "$1" == "-1" ]; then
    clear_screen
    draw_header
    draw_services
    draw_summary
    draw_logs
    draw_commands
else
    # Modo interactivo
    echo -e "${GREEN}Iniciando dashboard de desarrollo...${NC}"
    echo -e "${YELLOW}Presiona 'q' para salir${NC}"
    sleep 1

    # Trap para Ctrl+C
    trap 'clear_screen; echo -e "\n${GREEN}👋 Dashboard cerrado${NC}"; exit 0' INT

    # Iniciar bucle
    main_loop
fi
