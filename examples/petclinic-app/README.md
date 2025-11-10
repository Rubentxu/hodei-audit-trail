# Pet Clinic Application

> **Ubicado en**: `examples/petclinic-app/`

Aplicación de ejemplo que demuestra la integración completa con **hodei-audit-service** para auditoría centralizada.

Esta aplicación implementa el patrón clásico de Pet Clinic (típico de Spring Framework) migrado a **Rust/Axum** con:

- ✅ **Arquitectura limpia** (Domain, Application, Infrastructure)
- ✅ **Patrón Repository** con SQLx/PostgreSQL
- ✅ **Integración 1-liner** con hodei-audit-service
- ✅ **Auto-auditoría** de todas las operaciones HTTP
- ✅ **Multi-tenancy** nativo
- ✅ **HRN system** para recursos
- ✅ **Tests unitarios e integración**
- ✅ **Docker Compose** completo

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────┐
│               petclinic-app                 │
│  ┌───────────────────────────────────────┐  │
│  │           REST API (Axum)              │  │
│  │  - OwnersController                    │  │
│  │  - PetsController                      │  │
│  │  - VisitsController                    │  │
│  │  - VetsController                      │  │
│  └─────────────┬───────────────────────────┘  │
│                │                               │
│  ┌─────────────▼───────────────────────────┐  │
│  │        Service Layer                     │  │
│  │  - ClinicService                         │  │
│  │  - OwnerService                          │  │
│  │  - PetService                            │  │
│  └─────────────┬───────────────────────────┘  │
│                │                               │
│  ┌─────────────▼───────────────────────────┐  │
│  │       Repository Layer (SQLx)            │  │
│  │  - OwnerRepository                       │  │
│  │  - PetRepository                         │  │
│  │  - VisitRepository                       │  │
│  └─────────────┬───────────────────────────┘  │
│                │                               │
│  ┌─────────────▼───────────────────────────┐  │
│  │         Domain Models                    │  │
│  │  - Owner, Pet, Visit, Vet, Specialty    │  │
│  └─────────────────────────────────────────┘  │
│                                              │
│  ┌───────────────────────────────────────┐  │
│  │   Hodei Audit SDK (Middleware)        │  │
│  │   - Auto-capture HTTP requests         │  │
│  │   - HRN generation                     │  │
│  │   - Batch processing                   │  │
│  │   - gRPC to audit service              │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
                │
                │ gRPC
                ▼
┌─────────────────────────────────────────────┐
│        hodei-audit-service (CAP)            │
│  - Centralized audit point                  │
│  - ClickHouse + Vector.dev                  │
│  - Multi-tenant isolation                   │
│  - GDPR compliance                          │
└─────────────────────────────────────────────┘
```

## 🚀 Quick Start

### 1. Con Docker Compose (Recomendado)

```bash
# Navegar al directorio
cd examples/petclinic-app

# Levantar toda la stack
docker-compose up -d

# Ver logs
docker-compose logs -f petclinic-app

# Acceder a la API
curl http://localhost:3000/health
```

### 2. Desarrollo Local

```bash
# Navegar al directorio
cd examples/petclinic-app

# Instalar dependencias
cargo install sqlx-cli --features postgres

# Setup base de datos
export DATABASE_URL=postgresql://petclinic:petclinic@localhost:5432/petclinic
sqlx db setup

# Ejecutar
cargo run
```

## 📊 Ejemplos de API

### Owners

```bash
# Listar owners
curl http://localhost:3000/owners

# Crear owner
curl -X POST http://localhost:3000/owners \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "firstName": "John",
    "lastName": "Doe",
    "address": "123 Main St",
    "city": "Springfield",
    "telephone": "555-1234"
  }'

# Obtener owner con pets
curl http://localhost:3000/owners/1
```

### Pets

```bash
# Listar pets
curl http://localhost:3000/pets

# Añadir pet a owner
curl -X POST http://localhost:3000/owners/1/pets \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "name": "Buddy",
    "birthDate": "2020-01-01",
    "typeId": 1
  }'
```

### Visits

```bash
# Programar visita
curl -X POST http://localhost:3000/pets/1/visits \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "date": "2024-01-15",
    "description": "Annual checkup"
  }'
```

## 🔍 Auditoría

Todas las operaciones se registran automáticamente en hodei-audit-service:

```bash
# Ver eventos de auditoría (a través de hodei-audit-service)
curl -H "Authorization: Bearer YOUR_API_KEY" \
  http://hodei-audit-service:50053/v1/query/events \
  -d '{
    "tenantId": "tenant-petclinic",
    "startTime": "2024-01-01T00:00:00Z",
    "endTime": "2024-12-31T23:59:59Z"
  }'
```

### HRN Examples

Las operaciones generan HRNs automáticamente:

| Operación | HRN Generado |
|-----------|-------------|
| `POST /owners` | `hrn:hodei:petclinic:tenant-petclinic:global:owner/create` |
| `GET /owners/1` | `hrn:hodei:petclinic:tenant-petclinic:global:owner/1` |
| `POST /owners/1/pets` | `hrn:hodei:petclinic:tenant-petclinic:global:pet/create` |
| `GET /pets/1/visits` | `hrn:hodei:petclinic:tenant-petclinic:global:visit/list` |

## 🧪 Testing

```bash
# Tests unitarios
cargo test

# Tests de integración
cargo test --features testing

# Coverage
cargo tarpaulin --out html
```

## 🐳 Docker

### Build

```bash
# Desde examples/petclinic-app
docker build -t petclinic-app:latest .
```

### Run

```bash
docker run -d \
  --name petclinic-app \
  -p 3000:3000 \
  -e DATABASE_URL=postgresql://petclinic:petclinic@postgres:5432/petclinic \
  -e HODEI_AUDIT_SERVICE_URL=http://hodei-audit-service:50052 \
  petclinic-app:latest
```

## 📚 Documentación

- [Manual del Developer](docs/DEVELOPER_MANUAL.md) - Guía completa
- [API Documentation](docs/API.md) - Endpoints detallados
- [Database Schema](docs/SCHEMA.md) - Estructura de BD
- [Hodei Audit Integration](docs/AUDIT_INTEGRATION.md) - Integración con auditoría

## 🤝 Contributing

1. Fork el repo
2. Crea feature branch (`git checkout -b feature/amazing-feature`)
3. Commit tus cambios (`git commit -m 'feat: add amazing feature'`)
4. Push al branch (`git push origin feature/amazing-feature`)
5. Abre un Pull Request

## 📄 License

Apache-2.0 - Ver [LICENSE](LICENSE) para detalles.
