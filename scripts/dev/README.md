# Scripts de Desarrollo - Hodei Audit

Este directorio contiene scripts para facilitar el desarrollo con hot reloading para el backend Rust y frontend Next.js.

## 🎯 Características

- **Hot Reloading**: Recompilación automática al detectar cambios
- **Control de PIDs**: Gestión automática de procesos
- **Control de Puertos**: Verificación y liberación de puertos
- **Logs Centralizados**: Logs de todos los servicios en `.dev/logs/`
- **Dashboard Interactivo**: Vista en tiempo real del estado
- **Manejo Inteligente**: Mata procesos anteriores automáticamente

## 🚀 Comandos Principales

### Con Just (Recomendado)

```bash
# Instalar herramientas de desarrollo
just dev-setup

# Iniciar TODO (backend + frontend)
just dev-all

# Iniciar solo backend (con hot reload)
just dev-backend

# Iniciar solo frontend (con hot reload)
just dev-frontend

# Ver estado de servicios
just dev-status

# Ver logs
just dev-logs

# Dashboard interactivo
just dev-ui

# Detener servicios
just dev-stop

# Reiniciar servicios
just dev-restart
```

### Scripts Directos

```bash
# Iniciar servicios
./scripts/dev/dev-start.sh [all|backend|frontend]

# Detener servicios
./scripts/dev/dev-stop.sh

# Ver logs
./scripts/dev/dev-logs.sh [follow] [backend|frontend]

# Ver estado
./scripts/dev/dev-status.sh

# Dashboard
./scripts/dev/dev-dashboard.sh
```

## 📊 Puertos

| Servicio | Puerto | Descripción |
|----------|--------|-------------|
| Frontend Next.js | 3000 | Interfaz web |
| Backend Rust | 8080 | API REST |
| gRPC Gateway | 9000 | gRPC Web |
| Metrics | 9090 | Métricas Prometheus |

## 🔧 Herramientas Requeridas

### Backend (Rust)
- `cargo` - Compilador y gestor de paquetes
- `cargo-watch` - Hot reloading para Rust
- `just` - Task runner

```bash
# Instalar herramientas
cargo install just
cargo install cargo-watch
cargo install cargo-expand
```

### Frontend (Next.js)
- `npm` o `pnpm` - Gestor de paquetes
- `Node.js 18+` - Runtime de JavaScript

```bash
# Instalar Next.js CLI
npm install -g @next/cli
```

## 📁 Estructura de Archivos

```
.dev/
├── pids/              # Archivos PID de procesos
│   ├── backend.pid
│   └── frontend.pid
└── logs/              # Logs de servicios
    ├── backend.log
    └── frontend.log

scripts/dev/           # Scripts de desarrollo
├── dev-start.sh       # Iniciar servicios
├── dev-stop.sh        # Detener servicios
├── dev-status.sh      # Ver estado
├── dev-logs.sh        # Ver logs
└── dev-dashboard.sh   # Dashboard interactivo
```

## 🎨 Dashboard Interactivo

El dashboard muestra en tiempo real:
- Estado de todos los servicios
- PIDs y puertos
- Logs recientes
- Uso de memoria
- Comandos rápidos

### Atajos del Dashboard
- `r` - Reiniciar servicios
- `s` - Detener servicios
- `l` - Ver logs completos
- `q` - Salir

### Usar el Dashboard

```bash
# Modo interactivo (se actualiza cada 2 segundos)
just dev-ui

# Modo one-shot (ver una vez)
./scripts/dev/dev-dashboard.sh once
```

## 🔄 Hot Reloading

### Backend (Rust)
- Utiliza `cargo watch` para detectar cambios
- Recompila automáticamente al cambiar archivos
- Reinicia el servicio
- Archivos monitoreados:
  - `hodei-audit-service/`
  - `hodei-audit-common/`
- Ignora: `target/`, `*.log`, `.git/`

### Frontend (Next.js)
- Next.js incluye hot reload por defecto
- Fast Refresh para componentes React
- Recarga automática en el navegador

## 📝 Logs

### Ver Logs

```bash
# Ver últimos logs de todos los servicios
just dev-logs

# Seguir logs del backend en tiempo real
just dev-logs follow backend

# Seguir logs del frontend
just dev-logs follow frontend
```

### Ubicación de Logs
- Backend: `.dev/logs/backend.log`
- Frontend: `.dev/logs/frontend.log`

## 🛠️ Solución de Problemas

### Puerto en Uso

Si aparece "puerto en uso":
```bash
# Ver qué proceso usa el puerto
lsof -i :3000

# Forzar liberación
just dev-stop
sleep 2
just dev-all
```

### Proceso Colgado

```bash
# Ver procesos
just dev-status

# Matar todos los procesos de desarrollo
pkill -f "hodei-audit"
pkill -f "cargo"
pkill -f "next dev"

# Limpiar y reiniciar
just dev-stop
just dev-all
```

### Logs Vacíos

```bash
# Verificar que los servicios estén corriendo
just dev-status

# Limpiar logs antiguos
rm -rf .dev/logs/*

# Reiniciar
just dev-restart
```

### cargo-watch No Instaldo

```bash
# Instalar cargo-watch
cargo install cargo-watch

# Verificar instalación
cargo watch --version
```

## 🔍 Monitoreo

### Ver Estado
```bash
just dev-status
```

Salida ejemplo:
```
╔════════════════════════════════════════════════╗
║         📊 ESTADO DE SERVICIOS           ║
╚════════════════════════════════════════════════╝

┌─ Backend Rust
│
│  ✓ Puerto 8080: OCUPADO
│  ✓ PID: 12345
│  ✓ Log: 156 líneas (hace 5s)
│
└────────────────────────────────────────────

┌─ Frontend Next.js
│
│  ✓ Puerto 3000: OCUPADO
│  ✓ PID: 12346
│  ✓ Log: 89 líneas (hace 3s)
│
└────────────────────────────────────────────
```

## 🎯 Flujo de Trabajo Recomendado

1. **Setup Inicial**:
   ```bash
   just dev-setup
   ```

2. **Iniciar Desarrollo**:
   ```bash
   just dev-all
   ```

3. **Durante el Desarrollo**:
   - Edita código (se recompila automáticamente)
   - Usa `just dev-logs` para ver errores
   - Usa `just dev-status` para verificar estado
   - Abre `http://localhost:3000` para ver la app

4. **Parar**:
   ```bash
   just dev-stop
   ```

## 📈 Comandos Adicionales

```bash
# Ver todos los comandos disponibles
just --list

# Solo ejecutar tests en watch mode
cargo watch -x test

# Solo formatear código
just fmt

# Solo linting
just lint

# Ejecutar benchmarks
just bench
```

## 🐛 Debugging

### Backend

```bash
# Ver logs completos del backend
tail -f .dev/logs/backend.log

# Ejecutar backend en modo debug
RUST_LOG=debug just dev-backend

# Verificar conexión a la base de datos
curl http://localhost:8080/health
```

### Frontend

```bash
# Ver logs completos del frontend
tail -f .dev/logs/frontend.log

# Verificar que Next.js esté corriendo
curl http://localhost:3000

# Limpiar cache de Next.js
cd hodei-audit-web && rm -rf .next
```

## 💡 Tips

1. **Usa el dashboard**: `just dev-ui` es muy útil para monitoreo
2. **Revisa logs regularmente**: `just dev-logs` te ayuda a detectar problemas
3. **Mata procesos suavemente**: `just dev-stop` antes de reiniciar
4. **Monitorea memoria**: El dashboard muestra el uso de memoria
5. **Usa hot reload**: Ambos frontend y backend soportan hot reload

## 📚 Referencias

- [cargo-watch](https://github.com/passcod/cargo-watch)
- [Next.js Dev Mode](https://nextjs.org/docs/app/building-your-application/configuring/development)
- [Just](https://github.com/casey/just)
- [Rust Logging](https://docs.rs/env_logger/)
