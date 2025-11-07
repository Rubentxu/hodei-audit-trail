#!/bin/bash
# Script de configuración inicial del entorno de desarrollo
# Instala todas las herramientas necesarias y prepara el proyecto

set -e

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}   🚀 SETUP ENTORNO DE DESARROLLO - HODEI AUDIT   ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_step() {
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Verificar si un comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar si cargo está instalado
check_rust() {
    print_step "Verificando Rust..."

    if command_exists cargo; then
        local version=$(cargo --version | cut -d' ' -f2)
        print_success "Cargo encontrado: $version"

        # Verificar versión mínima (1.70+)
        local major=$(echo $version | cut -d'.' -f1)
        local minor=$(echo $version | cut -d'.' -f2)

        if [ "$major" -gt 1 ] || ([ "$major" -eq 1 ] && [ "$minor" -ge 70 ]); then
            print_success "Versión de Rust compatible"
        else
            print_warning "Versión de Rust muy antigua, se recomienda actualizar"
        fi
        return 0
    else
        print_error "Rust no está instalado"
        echo ""
        echo "Por favor instala Rust desde: https://rustup.rs/"
        return 1
    fi
}

# Instalar herramientas de Rust
install_rust_tools() {
    print_step "Instalando herramientas de Rust..."

    # Lista de herramientas
    local tools=(
        "just"
        "cargo-watch"
        "cargo-expand"
        "cargo-audit"
        "cargo-tarpaulin"
    )

    for tool in "${tools[@]}"; do
        print_step "Instalando $tool..."

        if cargo install --list | grep -q "^$tool "; then
            print_success "$tool ya está instalado"
        else
            if cargo install "$tool"; then
                print_success "$tool instalado"
            else
                print_error "Error instalando $tool"
                return 1
            fi
        fi
    done
}

# Verificar Node.js y npm
check_nodejs() {
    print_step "Verificando Node.js y npm..."

    if command_exists node; then
        local version=$(node --version)
        print_success "Node.js encontrado: $version"

        # Verificar que sea versión 18+
        local major=$(echo $version | sed 's/v//' | cut -d'.' -f1)
        if [ "$major" -ge 18 ]; then
            print_success "Versión de Node.js compatible"
        else
            print_warning "Se recomienda Node.js 18 o superior"
        fi
    else
        print_error "Node.js no está instalado"
        return 1
    fi

    if command_exists npm; then
        local version=$(npm --version)
        print_success "npm encontrado: v$version"
    else
        print_error "npm no está encontrado"
        return 1
    fi

    return 0
}

# Instalar dependencias del frontend
install_frontend_deps() {
    print_step "Instalando dependencias del frontend..."

    if [ -d "hodei-audit-web" ]; then
        cd hodei-audit-web

        if [ -f "package.json" ]; then
            if command_exists npm; then
                print_step "Ejecutando npm install..."
                npm install
                print_success "Dependencias instaladas"
            elif command_exists pnpm; then
                print_step "Ejecutando pnpm install..."
                pnpm install
                print_success "Dependencias instaladas"
            else
                print_error "No se encontró npm ni pnpm"
                cd ..
                return 1
            fi
        else
            print_warning "No se encontró package.json"
        fi

        cd ..
    else
        print_warning "Directorio hodei-audit-web no existe"
    fi
}

# Crear directorios necesarios
create_directories() {
    print_step "Creando directorios de desarrollo..."

    local dirs=(
        ".dev"
        ".dev/pids"
        ".dev/logs"
    )

    for dir in "${dirs[@]}"; do
        if [ ! -d "$dir" ]; then
            mkdir -p "$dir"
            print_success "Creado: $dir"
        else
            print_success "Ya existe: $dir"
        fi
    done
}

# Verificar estructura del proyecto
check_project_structure() {
    print_step "Verificando estructura del proyecto..."

    local required_dirs=(
        "hodei-audit-service"
        "hodei-audit-web"
        "scripts"
    )

    local missing=0

    for dir in "${required_dirs[@]}"; do
        if [ -d "$dir" ]; then
            print_success "✓ $dir"
        else
            print_error "✗ $dir (faltante)"
            missing=$((missing + 1))
        fi
    done

    if [ $missing -gt 0 ]; then
        print_warning "Faltan $missing directorios"
        return 1
    fi

    return 0
}

# Verificar que Rust crate compile
check_rust_build() {
    print_step "Verificando compilación del proyecto Rust..."

    if cargo check --workspace; then
        print_success "Proyecto Rust compila correctamente"
        return 0
    else
        print_error "Error al compilar el proyecto Rust"
        return 1
    fi
}

# Configurar git hooks (opcional)
setup_git_hooks() {
    print_step "Configurando git hooks (opcional)..."

    if [ -d ".git" ]; then
        local hooks_dir=".git/hooks"
        local pre_commit="scripts/pre-commit"

        if [ -f "$pre_commit" ]; then
            if [ ! -f "$hooks_dir/pre-commit" ]; then
                cp "$pre_commit" "$hooks_dir/pre-commit"
                chmod +x "$hooks_dir/pre-commit"
                print_success "Pre-commit hook instalado"
            else
                print_success "Pre-commit hook ya existe"
            fi
        else
            print_warning "No se encontró $pre_commit"
        fi
    else
        print_warning "No es un repositorio git"
    fi
}

# Mostrar resumen final
show_summary() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║${NC}                  ✅ SETUP COMPLETO                  ${GREEN}║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Herramientas instaladas:${NC}"
    echo "  • cargo (Rust)"
    echo "  • just (Task runner)"
    echo "  • cargo-watch (Hot reloading)"
    echo "  • cargo-expand (Macro expansion)"
    echo "  • cargo-audit (Security audit)"
    echo "  • npm (Node.js)"
    echo ""
    echo -e "${BLUE}Comandos disponibles:${NC}"
    echo "  just dev-all          - Iniciar todo"
    echo "  just dev-backend      - Solo backend"
    echo "  just dev-frontend     - Solo frontend"
    echo "  just dev-stop         - Detener"
    echo "  just dev-status       - Ver estado"
    echo "  just dev-logs         - Ver logs"
    echo "  just dev-ui           - Dashboard"
    echo ""
    echo -e "${BLUE}URLs:${NC}"
    echo "  Frontend: http://localhost:3000"
    echo "  Backend:  http://localhost:8080"
    echo ""
    echo -e "${GREEN}¡Listo para desarrollar! 🎉${NC}"
    echo ""
}

# Función principal
main() {
    print_header

    local errors=0

    # Verificar Rust
    if ! check_rust; then
        errors=$((errors + 1))
    fi

    # Instalar herramientas de Rust
    if ! install_rust_tools; then
        errors=$((errors + 1))
    fi

    # Verificar Node.js
    if ! check_nodejs; then
        print_warning "Node.js no encontrado, se puede instalar más tarde"
    fi

    # Instalar dependencias del frontend
    if ! install_frontend_deps; then
        print_warning "Error al instalar dependencias del frontend"
    fi

    # Crear directorios
    create_directories

    # Verificar estructura
    if ! check_project_structure; then
        errors=$((errors + 1))
    fi

    # Verificar compilación
    if ! check_rust_build; then
        errors=$((errors + 1))
    fi

    # Configurar git hooks
    setup_git_hooks

    # Mostrar resumen
    show_summary

    if [ $errors -gt 0 ]; then
        echo -e "${YELLOW}Se encontraron $errors errores durante el setup${NC}"
        echo -e "${YELLOW}Revisa los mensajes anteriores para más detalles${NC}"
        return 1
    else
        echo -e "${GREEN}¡Setup completado sin errores!${NC}"
        return 0
    fi
}

# Manejo de argumentos
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Uso: $0 [help]"
        echo ""
        echo "Este script configura el entorno de desarrollo completo:"
        echo "  • Instala herramientas de Rust"
        echo "  • Instala dependencias del frontend"
        echo "  • Crea directorios necesarios"
        echo "  • Verifica la compilación"
        echo ""
        exit 0
        ;;
    *)
        main
        ;;
esac
