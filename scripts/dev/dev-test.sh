#!/bin/bash
# Script para ejecutar pruebas rápidas de desarrollo
# Ejecuta tests, benchmarks y validaciones

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
    echo -e "${BLUE}║${NC}     🧪 PRUEBAS RÁPIDAS - HODEI AUDIT          ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_step() {
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Función para ejecutar tests
run_tests() {
    print_step "Ejecutando tests del proyecto..."

    if cargo test --workspace --all-targets; then
        print_success "Todos los tests pasaron"
        return 0
    else
        print_error "Algunos tests fallaron"
        return 1
    fi
}

# Función para ejecutar benchmarks
run_benchmarks() {
    print_step "Ejecutando benchmarks de rendimiento..."

    if [ -d "benchmarks" ]; then
        cd benchmarks
        if cargo bench; then
            print_success "Benchmarks completados"
            cd ..
            return 0
        else
            print_error "Error en benchmarks"
            cd ..
            return 1
        fi
    else
        print_warning "No se encontró directorio benchmarks"
        return 1
    fi
}

# Función para verificar compilación
check_build() {
    print_step "Verificando compilación..."

    if cargo check --workspace; then
        print_success "Compilación OK"
        return 0
    else
        print_error "Error de compilación"
        return 1
    fi
}

# Función para ejecutar linting
run_lint() {
    print_step "Ejecutando linting (clippy)..."

    if cargo clippy --all-targets --all-features -- -D warnings; then
        print_success "Linting OK"
        return 0
    else
        print_warning "Linting con advertencias"
        return 0
    fi
}

# Función para audit de seguridad
run_audit() {
    print_step "Ejecutando audit de seguridad..."

    if cargo audit; then
        print_success "No hay vulnerabilidades conocidas"
        return 0
    else
        print_warning "Se encontraron vulnerabilidades en dependencias"
        return 0
    fi
}

# Función para validar arquitectura
validate_architecture() {
    print_step "Validando arquitectura..."

    local scripts_dir="scripts"
    local valid=0

    if [ -f "$scripts_dir/validate-architecture-consistency.sh" ]; then
        if bash "$scripts_dir/validate-architecture-consistency.sh"; then
            print_success "Arquitectura validada"
            valid=$((valid + 1))
        else
            print_warning "Errores de arquitectura"
        fi
    fi

    if [ -f "$scripts_dir/validate-project-structure.sh" ]; then
        if bash "$scripts_dir/validate-project-structure.sh"; then
            print_success "Estructura validada"
            valid=$((valid + 1))
        else
            print_warning "Errores de estructura"
        fi
    fi

    if [ $valid -gt 0 ]; then
        return 0
    else
        print_warning "No se pudieron validar scripts de arquitectura"
        return 1
    fi
}

# Función para tests de integración
run_integration_tests() {
    print_step "Ejecutando tests de integración..."

    if cargo test -p hodei-audit-service --lib -- integration; then
        print_success "Tests de integración OK"
        return 0
    else
        print_error "Tests de integración fallaron"
        return 1
    fi
}

# Función para formateo
check_format() {
    print_step "Verificando formato de código..."

    if cargo fmt --all -- --check; then
        print_success "Formato OK"
        return 0
    else
        print_warning "Formato incorrecto, ejecuta: just fmt"
        return 1
    fi
}

# Función para pruebas del frontend
test_frontend() {
    print_step "Ejecutando tests del frontend..."

    if [ -d "hodei-audit-web" ]; then
        cd hodei-audit-web
        if npm run test --if-present; then
            print_success "Tests del frontend OK"
            cd ..
            return 0
        else
            print_warning "No se pudieron ejecutar tests del frontend"
            cd ..
            return 1
        fi
    else
        print_warning "Directorio frontend no existe"
        return 1
    fi
}

# Función para test de conectividad
test_connectivity() {
    print_step "Probando conectividad a servicios..."

    local backend_port=8080
    local frontend_port=3000

    # Test backend
    if lsof -ti:$backend_port >/dev/null 2>&1; then
        if curl -s http://localhost:$backend_port/health >/dev/null 2>&1; then
            print_success "Backend disponible en puerto $backend_port"
        else
            print_warning "Backend corriendo pero no responde en /health"
        fi
    else
        print_warning "Backend no está corriendo en puerto $backend_port"
    fi

    # Test frontend
    if lsof -ti:$frontend_port >/dev/null 2>&1; then
        if curl -s http://localhost:$frontend_port >/dev/null 2>&1; then
            print_success "Frontend disponible en puerto $frontend_port"
        else
            print_warning "Frontend corriendo pero no responde"
        fi
    else
        print_warning "Frontend no está corriendo en puerto $frontend_port"
    fi
}

# Función para test rápido
test_quick() {
    print_step "Ejecutando pruebas rápidas..."

    local errors=0

    check_format || errors=$((errors + 1))
    check_build || errors=$((errors + 1))
    run_lint || errors=$((errors + 1))
    run_audit || errors=$((errors + 1))

    if [ $errors -eq 0 ]; then
        print_success "Pruebas rápidas OK"
        return 0
    else
        print_warning "Se encontraron $errors problemas en pruebas rápidas"
        return 1
    fi
}

# Función para test completo
test_full() {
    print_step "Ejecutando suite completa de pruebas..."

    local errors=0

    print_step "Fase 1: Validaciones"
    check_format || errors=$((errors + 1))
    check_build || errors=$((errors + 1))
    run_lint || errors=$((errors + 1))
    run_audit || errors=$((errors + 1))

    echo ""
    print_step "Fase 2: Tests"
    run_tests || errors=$((errors + 1))

    echo ""
    print_step "Fase 3: Tests de integración"
    run_integration_tests || errors=$((errors + 1))

    echo ""
    print_step "Fase 4: Validación de arquitectura"
    validate_architecture || errors=$((errors + 1))

    echo ""
    print_step "Fase 5: Tests del frontend"
    test_frontend || errors=$((errors + 1))

    echo ""
    if [ $errors -eq 0 ]; then
        print_success "✅ Todas las pruebas pasaron"
        return 0
    else
        print_error "❌ $errors pruebas fallaron"
        return 1
    fi
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [quick|full|tests|bench|format|build|lint|audit|frontend|connectivity]"
    echo ""
    echo "Comandos:"
    echo "  quick         - Pruebas rápidas (formato, build, lint, audit)"
    echo "  full          - Suite completa de pruebas"
    echo "  tests         - Ejecutar todos los tests"
    echo "  bench         - Ejecutar benchmarks"
    echo "  format        - Verificar formato"
    echo "  build         - Verificar compilación"
    echo "  lint          - Ejecutar clippy"
    echo "  audit         - Audit de seguridad"
    echo "  frontend      - Tests del frontend"
    echo "  connectivity  - Probar conectividad a servicios"
    echo ""
}

# Main
case "${1:-quick}" in
    "quick")
        test_quick
        ;;
    "full")
        test_full
        ;;
    "tests")
        run_tests
        ;;
    "bench"|"benchmarks")
        run_benchmarks
        ;;
    "format")
        check_format
        ;;
    "build")
        check_build
        ;;
    "lint")
        run_lint
        ;;
    "audit")
        run_audit
        ;;
    "frontend")
        test_frontend
        ;;
    "connectivity")
        test_connectivity
        ;;
    "integration")
        run_integration_tests
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
