# Manual del Developer - Pet Clinic Application

## Índice

1. [Introducción](#introducción)
2. [Arquitectura del Sistema](#arquitectura-del-sistema)
3. [Tecnologías Utilizadas](#tecnologías-utilizadas)
4. [Estructura del Proyecto](#estructura-del-proyecto)
5. [Configuración del Entorno de Desarrollo](#configuración-del-entorno-de-desarrollo)
6. [Ejecución de la Aplicación](#ejecución-de-la-aplicación)
7. [Base de Datos](#base-de-datos)
8. [API REST](#api-rest)
9. [Integración con Hodei Audit Service](#integración-con-hodei-audit-service)
10. [Testing](#testing)
11. [Docker y Contenedores](#docker-y-contenedores)
12. [Monitoreo y Observabilidad](#monitoreo-y-observabilidad)
13. [Contribución](#contribución)
14. [FAQ](#faq)
15. [Recursos Adicionales](#recursos-adicionales)

---

## Introducción

La **Pet Clinic Application** es un ejemplo completo de una aplicación empresarial construida en **Rust/Axum** que demuestra:

- ✅ **Arquitectura Limpia** (Clean Architecture)
- ✅ **Patrón Repository** con SQLx
- ✅ **Integración completa** con hodei-audit-service
- ✅ **Auditoría automática** de todas las operaciones
- ✅ **Sistema HRN** (Hodei Resource Names)
- ✅ **Multi-tenancy**
- ✅ **Docker Compose** para desarrollo y producción
- ✅ **Monitoreo completo** (Prometheus + Grafana)

Esta aplicación implementa el patrón clásico de Pet Clinic (típico de Spring Framework) migrado a Rust, sirviendo como ejemplo de best practices para el desarrollo de aplicaciones empresariales.

### Propósito

- **Demostrar** la integración de hodei-audit-service en aplicaciones reales
- **Proporcionar** un ejemplo completo de arquitectura en Rust
- **Servir** como template para nuevas aplicaciones
- **Mostrar** patrones de diseño y arquitectura empresarial
- **Facilitar** el aprendizaje de Rust en contexto empresarial

---

## Arquitectura del Sistema

### Visión General

La aplicación sigue una arquitectura por capas con separación clara de responsabilidades:

```
┌─────────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  REST API       │  │  Controllers    │  │  DTOs           │ │
│  │  (Axum)         │  │  (HTTP Routes)  │  │  (Responses)    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Services       │  │  Use Cases      │  │  DTO Mapping    │ │
│  │  (Business Flow)│  │  (Orchestration)│  │  (Validation)   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                      DOMAIN LAYER                               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Entities       │  │  Domain         │  │  Value Objects  │ │
│  │  (Business      │  │  Services       │  │  (Validation)   │ │
│  │   Objects)      │  │  (Logic)        │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Repository     │  │  Repository     │  │  Domain         │ │
│  │  Interfaces     │  │  Contracts      │  │  Events         │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                  INFRASTRUCTURE LAYER                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Repositories   │  │  Database       │  │  External       │ │
│  │  (SQLx)         │  │  (PostgreSQL)   │  │  APIs           │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Patrones Aplicados

1. **Clean Architecture**
   - Separación clara de responsabilidades
   - Dependencias hacia el centro (domain)
   - Independiente de frameworks, DB, UI

2. **Repository Pattern**
   - Abstracción del acceso a datos
   - Facilita testing con mocks
   - Centraliza queries

3. **Service Layer**
   - Orquestación de casos de uso
   - Transacciones
   - Validación de negocio

4. **DTO Pattern**
   - Separación entre API y domain
   - Serialización/deserialización
   - Evitar acoplamiento

5. **HRN System**
   - Identificación única de recursos
   - Auditoría trazable
   - Integración con hodei-audit

---

## Tecnologías Utilizadas

### Core Stack

| Tecnología | Versión | Propósito |
|------------|---------|-----------|
| **Rust** | 1.75+ | Lenguaje de programación |
| **Axum** | 0.8 | Framework web HTTP |
| **SQLx** | 0.7 | ORM y SQL database |
| **PostgreSQL** | 15 | Base de datos primaria |
| **Tokio** | 1.0 | Runtime async |
| **Serde** | 1.0 | Serialización JSON |
| **Tracing** | 0.1 | Logging estructurado |

### Integración y Observabilidad

| Tecnología | Propósito |
|------------|-----------|
| **hodei-audit-sdk** | Auditoría centralizada |
| **hodei-audit-service** | Sistema de auditoría |
| **ClickHouse** | Almacenamiento audit (hot) |
| **Vector.dev** | Pipeline de datos |
| **MinIO** | Almacenamiento S3-compatible |
| **Prometheus** | Métricas |
| **Grafana** | Dashboards |

### Herramientas de Desarrollo

| Herramienta | Propósito |
|-------------|-----------|
| **Docker** | Containerización |
| **Docker Compose** | Orquestación multi-container |
| **sqlx-cli** | Migraciones de BD |
| **cargo** | Build y package manager |
| **just** | Task runner (opcional) |

---

## Estructura del Proyecto

```
petclinic-app/
├── src/
│   ├── main.rs                    # Punto de entrada
│   ├── config.rs                  # Configuración
│   │
│   ├── domain/                    # DOMINIO
│   │   ├── mod.rs
│   │   ├── entities.rs            # Entidades de negocio
│   │   ├── repositories.rs        # Contratos de repositorio
│   │   └── services.rs            # Servicios de dominio
│   │
│   ├── application/               # APLICACIÓN
│   │   └── services.rs            # Servicios de aplicación
│   │
│   ├── infrastructure/            # INFRAESTRUCTURA
│   │   └── repositories.rs        # Implementaciones SQLx
│   │
│   └── presentation/              # PRESENTACIÓN
│       ├── controllers.rs         # Controladores HTTP
│       └── mod.rs
│
├── db/
│   └── init/
│       ├── 01-init.sql            # Esquema de BD
│       └── 02-data.sql            # Datos de ejemplo
│
├── config/
│   ├── postgres/
│   │   ├── postgresql.conf
│   │   └── pg_hba.conf
│   ├── redis/
│   │   └── redis.conf
│   ├── clickhouse/
│   │   ├── config.xml
│   │   └── users.xml
│   ├── vector/
│   │   └── vector.toml
│   ├── prometheus/
│   │   └── prometheus.yml
│   └── grafana/
│       ├── datasources/
│       └── dashboards/
│
├── docs/
│   ├── DEVELOPER_MANUAL.md        # Este documento
│   ├── API.md                     # Documentación API
│   └── SCHEMA.md                  # Esquema de BD
│
├── Dockerfile                     # Imagen de la aplicación
├── docker-compose.yml             # Orquestación completa
├── docker-compose.dev.yml         # Desarrollo
├── .env.example                   # Variables de entorno
├── Cargo.toml                     # Dependencias
├── Cargo.lock                     # Versiones locked
└── README.md                      # Documentación general
```

### Capas Explicadas

#### Domain Layer (src/domain/)
Contiene la lógica de negocio pura, independiente de cualquier tecnología.

**entities.rs**
- Definición de entidades: Owner, Pet, Visit, Vet, Specialty, PetType
- Validación de negocio
- Invariantes y reglas

**repositories.rs**
- Contratos abstractos para acceso a datos
- Repository interfaces
- Generic Result types

**services.rs**
- Lógica de dominio que no encaja en entidades
- Business rules
- Cross-entity operations

#### Application Layer (src/application/)
Orquesta casos de uso, coordina repositorios y servicios de dominio.

**services.rs**
- OwnerApplicationService
- PetApplicationService
- VisitApplicationService
- VetApplicationService
- Transaccional
- DTO mapping

#### Infrastructure Layer (src/infrastructure/)
Implementaciones concretas de interfaces, acceso a DB, APIs externas.

**repositories.rs**
- SqlxOwnerRepository
- SqlxPetRepository
- SqlxVisitRepository
- SqlxVetRepository
- SqlxPetTypeRepository
- Connection pooling
- Query optimization

#### Presentation Layer (src/presentation/)
HTTP controllers, routing, DTOs, serialización.

**controllers.rs**
- REST endpoints
- Request/Response handling
- Error handling
- DTO conversion
- Validación

---

## Configuración del Entorno de Desarrollo

### Prerrequisitos

1. **Rust 1.75+**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup update
```

2. **Docker y Docker Compose**
```bash
# Ubuntu/Debian
sudo apt-get install docker.io docker-compose

# macOS
brew install docker docker-compose

# Windows
# Instalar Docker Desktop
```

3. **sqlx-cli (para migraciones)**
```bash
cargo install sqlx-cli --features postgres
```

4. **just (opcional, task runner)**
```bash
cargo install just
```

### Configuración Inicial

1. **Clonar el repositorio**
```bash
git clone <repository-url>
cd hodei-trail/petclinic-app
```

2. **Copiar variables de entorno**
```bash
cp .env.example .env
# Editar .env con tus configuraciones
```

3. **Verificar configuración**
```bash
# Verificar que Rust está instalado
rustc --version
cargo --version

# Verificar que Docker está funcionando
docker --version
docker-compose --version
```

---

## Ejecución de la Aplicación

### Opción 1: Docker Compose (Recomendado)

**Levantar toda la stack**

```bash
# Construir y levantar todos los servicios
docker-compose up -d

# Ver logs en tiempo real
docker-compose logs -f petclinic-app

# Ver logs de un servicio específico
docker-compose logs -f postgres
docker-compose logs -f hodei-audit-service
```

**Servicios disponibles**

- Pet Clinic API: http://localhost:3000
- Adminer (DB UI): http://localhost:8080
- ClickHouse: http://localhost:8123
- MinIO Console: http://localhost:9001
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3001

**Comandos útiles**

```bash
# Ver estado de contenedores
docker-compose ps

# Reiniciar servicio específico
docker-compose restart petclinic-app

# Parar todos los servicios
docker-compose down

# Parar y eliminar volúmenes
docker-compose down -v

# Reconstruir aplicación
docker-compose up -d --build petclinic-app
```

### Opción 2: Desarrollo Local

**1. Configurar PostgreSQL**

```bash
# Instalar PostgreSQL
# Ubuntu/Debian
sudo apt-get install postgresql postgresql-contrib

# macOS
brew install postgresql

# Iniciar servicio
sudo service postgresql start  # Linux
brew services start postgresql  # macOS
```

**2. Crear base de datos**

```bash
# Conectar a PostgreSQL
psql -U postgres

# Crear usuario y BD
CREATE USER petclinic WITH PASSWORD 'petclinic';
CREATE DATABASE petclinic OWNER petclinic;
GRANT ALL PRIVILEGES ON DATABASE petclinic TO petclinic;
\q
```

**3. Configurar variables**

```bash
export DATABASE_URL=postgresql://petclinic:petclinic@localhost:5432/petclinic
export HODEI_AUDIT_SERVICE_URL=http://localhost:50052
export RUST_LOG=debug
```

**4. Ejecutar migraciones**

```bash
sqlx db setup
```

**5. Ejecutar la aplicación**

```bash
# Modo debug
cargo run

# Modo release
cargo run --release

# Con logging
RUST_LOG=debug cargo run
```

### Verificación de Instalación

**1. Health Check**

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "ok"
}
```

**2. Probar endpoints básicos**

```bash
# Listar tipos de mascota
curl http://localhost:3000/pet-types

# Listar veterinarios
curl http://localhost:3000/vets

# Listar owners
curl http://localhost:3000/owners
```

---

## Base de Datos

### Esquema

La base de datos sigue un modelo relacional clásico:

```
┌──────────────┐       ┌──────────┐       ┌─────────────┐
│    Owners    │◄──────│  Pets    │◄──────│   Visits    │
│              │       │          │       │             │
│ • id (PK)    │       │ • id     │       │ • id (PK)   │
│ • first_name │       │ • name   │       │ • pet_id    │
│ • last_name  │       │ • birth  │       │ • date      │
│ • address    │       │ • type   │       │ • desc      │
│ • city       │       │ • owner  │       └─────────────┘
│ • phone      │       └──────────┘                 ▲
└──────────────┘                                  │
                                                 │
┌──────────────┐       ┌──────────────┐          │
│  Pet Types   │       │    Vets      │          │
│              │       │              │          │
│ • id (PK)    │       │ • id (PK)    │          │
│ • name       │       │ • first_name │          │
└──────────────┘       │ • last_name  │          │
                       └──────────────┘          │
                                ▲                  │
                                │                  │
                       ┌───────────────┐          │
                       │ Vet-Specialty │          │
                       │  (Join Table) │──────────┘
                       │               │
                       │ • vet_id      │
                       │ • specialty   │
                       └───────────────┘
                                ▲
                                │
                       ┌──────────────┐
                       │ Specialties  │
                       │              │
                       │ • id (PK)    │
                       │ • name       │
                       └──────────────┘
```

### Tablas Principales

**owners**
- Información de propietarios de mascotas
- Relación 1:N con pets
- Campos: id, first_name, last_name, address, city, telephone, timestamps

**pets**
- Mascotas registradas
- Pertenecen a un owner (FK)
- Tienen un tipo (FK a types)
- Relación 1:N con visits
- Campos: id, name, birth_date, type_id, owner_id, timestamps

**visits**
- Visitas médicas
- Asociadas a una mascota
- Campos: id, pet_id, visit_date, description, created_at

**types**
- Tipos de mascotas (Dog, Cat, Bird, etc.)
- Campos: id, name

**vets**
- Veterinarios
- Relación N:M con specialties via vet_specialties
- Campos: id, first_name, last_name

**specialties**
- Especialidades veterinarias
- Campos: id, name

**vet_specialties**
- Tabla junction para relación N:M
- Campos: vet_id, specialty_id (PK compuesta)

### Migraciones

**Crear nueva migración**

```bash
sqlx migrate add -r "add_new_feature"
```

Esto crea archivos:
- `migrations/YYYYMMDDHHMMSS_add_new_feature.sql`

**Ejecutar migraciones**

```bash
# En desarrollo
sqlx migrate run

# Verificar estado
sqlx migrate status

# Revertir última migración
sqlx migrate revert
```

**Ejemplo de migración**

```sql
-- migrations/20240101000000_add_feature.sql

ALTER TABLE owners ADD COLUMN email VARCHAR(255);

-- Reversible
ALTER TABLE owners DROP COLUMN email;
```

### Consultas Principales

**Find owner by ID with pets**

```sql
SELECT o.*, p.*, pt.name as type_name
FROM owners o
LEFT JOIN pets p ON o.id = p.owner_id
LEFT JOIN types pt ON p.type_id = pt.id
WHERE o.id = $1;
```

**Find pet with visits**

```sql
SELECT p.*, v.*, t.name as type_name
FROM pets p
LEFT JOIN visits v ON p.id = v.pet_id
LEFT JOIN types t ON p.type_id = t.id
WHERE p.id = $1
ORDER BY v.visit_date DESC;
```

**Statistics query**

```sql
SELECT 
    COUNT(*) as total_pets,
    t.name as type,
    COUNT(*) as count
FROM pets p
JOIN types t ON p.type_id = t.id
GROUP BY t.id, t.name
ORDER BY count DESC;
```

---

## API REST

### OpenAPI Specification

La API sigue principios RESTful con:

- **CRUD completo** para recursos principales
- **Códigos de estado HTTP** apropiados
- **JSON** para request/response
- **Validación** de entrada
- **Auditoría automática** via hodei-audit

### Endpoints

#### 1. Health Check

**GET /health**

Verificar estado de la aplicación.

**Response 200 OK**
```json
{
  "status": "ok"
}
```

#### 2. Owners

**GET /owners**

Listar todos los owners o buscar por lastName.

**Query Parameters**
- `lastName` (opcional): Filtro por apellido

**Response 200 OK**
```json
[
  {
    "id": 1,
    "first_name": "George",
    "last_name": "Franklin",
    "full_name": "George Franklin",
    "address": "110 W. Liberty St.",
    "city": "Madison",
    "telephone": "6085551023",
    "pets": []
  }
]
```

**GET /owners/{id}**

Obtener owner por ID con sus mascotas.

**Response 200 OK**
```json
{
  "id": 1,
  "first_name": "George",
  "last_name": "Franklin",
  "full_name": "George Franklin",
  "address": "110 W. Liberty St.",
  "city": "Madison",
  "telephone": "6085551023",
  "pets": [
    {
      "id": 1,
      "name": "Leo",
      "birth_date": "2010-09-07",
      "type_id": 1,
      "owner_id": 1,
      "age": 13,
      "visits": []
    }
  ]
}
```

**Response 404 Not Found**
```json
"Owner not found"
```

**POST /owners**

Crear nuevo owner.

**Request**
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "address": "123 Main St",
  "city": "Springfield",
  "telephone": "555-1234"
}
```

**Response 201 Created**
```json
1
```

**PUT /owners/{id}**

Actualizar owner.

**Request**
```json
{
  "first_name": "John",
  "last_name": "Smith",
  "address": "456 Oak Ave",
  "city": "Springfield",
  "telephone": "555-5678"
}
```

**Response 200 OK**
```json
"Owner updated successfully"
```

**DELETE /owners/{id}**

Eliminar owner y todas sus mascotas.

**Response 200 OK**
```json
"Owner deleted successfully"
```

**GET /owners/{id}/pets**

Listar mascotas de un owner.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "name": "Leo",
    "birth_date": "2010-09-07",
    "type_id": 1,
    "owner_id": 1,
    "age": 13,
    "visits": []
  }
]
```

**POST /owners/{id}/pets**

Añadir mascota a owner.

**Request**
```json
{
  "name": "Buddy",
  "birth_date": "2020-01-01",
  "type_id": 1
}
```

**Response 201 Created**
```json
2
```

#### 3. Pets

**GET /pets**

Listar todas las mascotas.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "name": "Leo",
    "birth_date": "2010-09-07",
    "type_id": 1,
    "owner_id": 1,
    "age": 13,
    "visits": []
  }
]
```

**GET /pets/{id}**

Obtener mascota por ID.

**Response 200 OK**
```json
{
  "id": 1,
  "name": "Leo",
  "birth_date": "2010-09-07",
  "type_id": 1,
  "owner_id": 1,
  "age": 13,
  "visits": [
    {
      "id": 1,
      "pet_id": 1,
      "date": "2013-01-01",
      "description": "rabies vaccination"
    }
  ]
}
```

**PUT /pets/{id}**

Actualizar mascota.

**Request**
```json
{
  "name": "Leo Updated",
  "birth_date": "2010-09-07",
  "type_id": 1,
  "owner_id": 1
}
```

**DELETE /pets/{id}**

Eliminar mascota.

**GET /pets/{id}/visits**

Listar visitas de una mascota.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "pet_id": 1,
    "date": "2013-01-01",
    "description": "rabies vaccination"
  }
]
```

**POST /pets/{id}/visits**

Añadir visita a mascota.

**Request**
```json
{
  "date": "2024-01-15",
  "description": "Annual checkup"
}
```

**Response 201 Created**
```json
5
```

#### 4. Visits

**GET /visits**

Listar todas las visitas.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "pet_id": 1,
    "date": "2013-01-01",
    "description": "rabies vaccination"
  }
]
```

**GET /visits/{id}**

Obtener visita por ID.

**Response 200 OK**
```json
{
  "id": 1,
  "pet_id": 1,
  "date": "2013-01-01",
  "description": "rabies vaccination"
}
```

**POST /visits**

Crear nueva visita.

**Request**
```json
{
  "pet_id": 1,
  "date": "2024-01-15",
  "description": "Annual checkup"
}
```

**Response 201 Created**
```json
6
```

**PUT /visits/{id}**

Actualizar visita.

**DELETE /visits/{id}**

Eliminar visita.

#### 5. Vets

**GET /vets**

Listar todos los veterinarios.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "first_name": "James",
    "last_name": "Carter",
    "full_name": "James Carter",
    "specialties": ["Radiology"]
  }
]
```

**GET /vets/{id}**

Obtener veterinario por ID.

**Response 200 OK**
```json
{
  "id": 1,
  "first_name": "James",
  "last_name": "Carter",
  "full_name": "James Carter",
  "specialties": ["Radiology"]
}
```

#### 6. Pet Types

**GET /pet-types**

Listar todos los tipos de mascota.

**Response 200 OK**
```json
[
  {
    "id": 1,
    "name": "Dog"
  },
  {
    "id": 2,
    "name": "Cat"
  }
]
```

### Códigos de Estado

- **200 OK** - Operación exitosa
- **201 Created** - Recurso creado
- **400 Bad Request** - Datos inválidos
- **404 Not Found** - Recurso no encontrado
- **500 Internal Server Error** - Error del servidor

### Ejemplos de Uso

**Crear owner con mascota**

```bash
# 1. Crear owner
curl -X POST http://localhost:3000/owners \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "firstName": "Alice",
    "lastName": "Johnson",
    "address": "789 Pine St",
    "city": "Madison",
    "telephone": "555-9999"
  }'

# Response: 1

# 2. Añadir mascota
curl -X POST http://localhost:3000/owners/1/pets \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "name": "Fluffy",
    "birthDate": "2022-05-15",
    "typeId": 2
  }'

# Response: 2

# 3. Programar visita
curl -X POST http://localhost:3000/pets/2/visits \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-123" \
  -H "x-tenant-id: tenant-petclinic" \
  -d '{
    "date": "2024-02-01",
    "description": "First checkup"
  }'

# Response: 3
```

**Buscar owner**

```bash
# Por apellido
curl "http://localhost:3000/owners?lastName=Davis"

# Todos los owners
curl http://localhost:3000/owners
```

---

## Integración con Hodei Audit Service

### Overview

La Pet Clinic application se integra automáticamente con **hodei-audit-service** para auditoría centralizada de todas las operaciones HTTP.

### Cómo Funciona

1. **Middleware automático** intercepta todas las requests HTTP
2. **Genera HRN** (Hodei Resource Name) para cada endpoint
3. **Extrae contexto** (user_id, tenant_id, trace_id) de headers
4. **Crea evento de auditoría** con metadata completa
5. **Envía via gRPC** a hodei-audit-service
6. **Batch processing** para optimización de red

### Configuración

**En main.rs**

```rust
use hodei_audit_sdk::{AuditSdkConfig, AuditLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar SDK de auditoría
    let audit_config = AuditSdkConfig::builder()
        .service_name("petclinic")
        .tenant_id("tenant-petclinic")
        .audit_service_url("http://hodei-audit-service:50052")
        .batch_size(100)
        .batch_timeout(std::time::Duration::from_millis(100))
        .enable_request_body(false)  // Para producción
        .enable_response_body(false)
        .build()?;

    // Crear layer de auditoría
    let audit_layer = AuditLayer::new(audit_config);

    // Aplicar como middleware
    let app = Router::new()
        .nest("/owners", owners_routes())
        .nest("/pets", pets_routes())
        // ... más rutas
        .layer(audit_layer);  // <- Middleware de auditoría

    // ... resto del código
}
```

**Headers requeridos**

La aplicación espera los siguientes headers para contexto de auditoría:

- `x-user-id` - ID del usuario (ej: "user-123")
- `x-tenant-id` - ID del tenant (ej: "tenant-petclinic")
- `x-trace-id` - ID de trace (opcional)

**Ejemplo de request con headers**

```bash
curl -X POST http://localhost:3000/owners \
  -H "Content-Type: application/json" \
  -H "x-user-id: user-456" \
  -H "x-tenant-id: tenant-petclinic" \
  -H "x-trace-id: trace-789" \
  -d '{"firstName": "John", "lastName": "Doe"}'
```

### HRN System

**Formato de HRN**

```
hrn:hodei:{service}:{tenant}:{scope}:{resource_type}/{resource_id}
```

**Ejemplos generados automáticamente**

| Operación | Endpoint | HRN |
|-----------|----------|-----|
| Crear owner | POST /owners | `hrn:hodei:petclinic:tenant-petclinic:global:owner/create` |
| Obtener owner | GET /owners/1 | `hrn:hodei:petclinic:tenant-petclinic:global:owner/1` |
| Actualizar owner | PUT /owners/1 | `hrn:hodei:petclinic:tenant-petclinic:global:owner/1` |
| Eliminar owner | DELETE /owners/1 | `hrn:hodei:petclinic:tenant-petclinic:global:owner/1` |
| Añadir mascota | POST /owners/1/pets | `hrn:hodei:petclinic:tenant-petclinic:global:pet/create` |
| Obtener mascota | GET /pets/1 | `hrn:hodei:petclinic:tenant-petclinic:global:pet/1` |
| Listar visitas | GET /pets/1/visits | `hrn:hodei:petclinic:tenant-petclinic:global:visit/list` |
| Crear visita | POST /pets/1/visits | `hrn:hodei:petclinic:tenant-petclinic:global:visit/create` |
| Listar vets | GET /vets | `hrn:hodei:petclinic:tenant-petclinic:global:vet/list` |
| Health check | GET /health | `hrn:hodei:petclinic:tenant-petclinic:global:service/health` |

### Eventos de Auditoría

**Estructura del evento**

```json
{
  "event_name": "POST /owners",
  "event_category": 0,  // 0=Management, 1=Data, 2=Insight
  "hrn": "hrn:hodei:petclinic:tenant-petclinic:global:owner/create",
  "user_id": "user-456",
  "tenant_id": "tenant-petclinic",
  "trace_id": "trace-789",
  "resource_path": "/owners",
  "http_method": "POST",
  "http_status": 201,
  "source_ip": "192.168.1.100",
  "user_agent": "curl/7.68.0",
  "additional_data": {
    "hrn_display_name": "Create Owner",
    "hrn_resource_type": "owner"
  }
}
```

**Campos adicionales**

El SDK añade automáticamente:
- `source_ip` - Extraída de headers X-Forwarded-For o connection
- `user_agent` - Header User-Agent
- `timestamp` - Momento de la request
- `response_time` - Latencia de la request

### Batch Processing

El SDK usa batch processing para optimizar el rendimiento:

**Configuración**

```rust
.batch_size(100)                    // Flush después de 100 eventos
.batch_timeout(Duration::from_millis(100))  // Flush cada 100ms
```

**Flush triggers**
- Tamaño del batch alcanza límite
- Timeout expirado
- Shutdown de la aplicación

**Beneficios**
- **99% reducción** en llamadas de red
- **Throughput** de 10,000+ eventos/segundo
- **Latencia** de < 1ms overhead
- **Backpressure** automático

### Query de Eventos de Auditoría

**Desde hodei-audit-service**

```bash
curl -X POST http://hodei-audit-service:50053/v1/query/events \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "tenantId": "tenant-petclinic",
    "startTime": "2024-01-01T00:00:00Z",
    "endTime": "2024-12-31T23:59:59Z",
    "eventCategory": 0,
    "limit": 100
  }'
```

**Desde Grafana**

Acceder a http://localhost:3001 y usar dashboards pre-configurados.

### Multi-Tenancy

**Aislamiento de datos**

- Cada evento incluye `tenant_id`
- Queries están filtradas por tenant
- Row-Level Security en ClickHouse
- Zero cross-tenant access

**Headers de contexto**

```bash
# Todos los requests deben incluir estos headers
-x-tenant-id: tenant-petclinic
-x-user-id: user-{id}
```

**Sin headers**

- `tenant_id` se establece a "unknown"
- `user_id` se establece a "anonymous"
- Evento aún se registra pero sin contexto completo

### Troubleshooting

**Eventos no se están enviando**

1. Verificar que hodei-audit-service esté corriendo
```bash
curl http://hodei-audit-service:50052/health
```

2. Verificar configuración
```bash
# Habilitar debug logging
RUST_LOG=debug cargo run
```

3. Verificar conexión gRPC
```bash
# Logs de conexión
# Buscar: "Connected to audit service"
```

**Alta latencia**

- Aumentar `batch_size` (100-1000)
- Aumentar `batch_timeout` (1-5 segundos)
- Verificar métricas del audit service

**Eventos duplicados**

- Verificar que no hay múltiples middlewares
- Check de double-send
- Monitor de flush automático

---

## Testing

### Tipos de Tests

1. **Unit Tests** - Testing de funciones y métodos individuales
2. **Integration Tests** - Testing de repositorios con DB real
3. **API Tests** - Testing de endpoints HTTP
4. **Contract Tests** - Testing de integración con hodei-audit

### Ejecutar Tests

```bash
# Todos los tests
cargo test

# Tests unitarios solamente
cargo test --lib

# Tests de integración
cargo test --test integration

# Con coverage
cargo tarpaulin --out html --output-dir coverage/
open coverage/tarpaulin-report.html

# Tests específicos
cargo test domain::entities
cargo test infrastructure
cargo test presentation

# Tests con logging
RUST_LOG=cargo test
```

### Estructura de Tests

**Unit Test Example**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_validation() {
        let owner = Owner::new(
            "John".to_string(),
            "Doe".to_string(),
            Some("123 Main St".to_string()),
            Some("Springfield".to_string()),
            Some("555-1234".to_string()),
        );

        assert!(owner.validate().is_ok());
    }

    #[test]
    fn test_owner_validation_fails_empty_name() {
        let owner = Owner::new(
            "".to_string(),  // Empty name should fail
            "Doe".to_string(),
            None,
            None,
            None,
        );

        assert!(owner.validate().is_err());
    }
}
```

**Integration Test Example**

```rust
#[cfg(test)]
mod integration_tests {
    use sqlx::PgPool;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_owner_repository() {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        let repo = SqlxOwnerRepository::new(pool);

        // Create owner
        let owner = Owner::new(
            "Test".to_string(),
            "Owner".to_string(),
            None,
            None,
            None,
        );

        let saved = repo.save(&owner).await.unwrap();
        assert!(saved.id.is_some());

        // Find by ID
        let found = repo.find_by_id(saved.id.unwrap()).await.unwrap();
        assert!(found.is_some());

        // Clean up
        repo.delete(saved.id.unwrap()).await.unwrap();
    }
}
```

### Test Database

**Usar PostgreSQL en memoria para tests**

```rust
// En tests/Cargo.toml
[dev-dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "sqlite", "chrono", "uuid", "json"] }
```

**Fixture pattern**

```rust
struct TestFixture {
    owner_repo: SqlxOwnerRepository,
    pet_repo: SqlxPetRepository,
    owner_id: i32,
    pet_id: i32,
}

impl TestFixture {
    async fn new(pool: &PgPool) -> Self {
        let owner_repo = SqlxOwnerRepository::new(pool.clone());
        let pet_repo = SqlxPetRepository::new(pool.clone());

        // Create test data
        let owner = Owner::new("Test".to_string(), "Owner".to_string(), None, None, None);
        let saved_owner = owner_repo.save(&owner).await.unwrap();

        let pet = Pet::new("Test Pet".to_string(), None, 1, saved_owner.id.unwrap());
        let saved_pet = pet_repo.save(&pet).await.unwrap();

        Self {
            owner_repo,
            pet_repo,
            owner_id: saved_owner.id.unwrap(),
            pet_id: saved_pet.id.unwrap(),
        }
    }

    async fn cleanup(self, pool: &PgPool) {
        // Clean up test data
        sqlx::query!("DELETE FROM pets WHERE id = $1", self.pet_id)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query!("DELETE FROM owners WHERE id = $1", self.owner_id)
            .execute(pool)
            .await
            .unwrap();
    }
}
```

### Mocking

**Mock hodei-audit-service para tests**

```rust
#[cfg(test)]
mod audit_mock {
    use async_trait::async_trait;

    pub struct MockAuditService {
        pub events: Arc<Mutex<Vec<AuditEvent>>>,
    }

    impl MockAuditService {
        pub fn new() -> Self {
            Self {
                events: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl AuditTransport for MockAuditService {
        async fn publish(&self, event: AuditEvent) -> Result<(), AuditError> {
            self.events.lock().unwrap().push(event);
            Ok(())
        }
    }
}
```

### Continuous Integration

**.github/workflows/ci.yml**

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: petclinic
          POSTGRES_USER: petclinic
          POSTGRES_DB: petclinic_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Install sqlx-cli
      run: cargo install sqlx-cli --features postgres

    - name: Run migrations
      run: |
        export DATABASE_URL=postgresql://petclinic:petclinic@localhost:5432/petclinic_test
        sqlx migrate run

    - name: Run tests
      run: cargo test

    - name: Generate coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out xml --output-dir coverage/

    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        file: coverage/cobertura.xml
```

---

## Docker y Contenedores

### Dockerfile

**Multi-stage build**

```dockerfile
# Build stage
FROM rust:1.75-slim AS builder
# ... compile application

# Runtime stage
FROM debian:bookworm-slim AS runtime
# ... run application
```

**Beneficios**
- Imagen final optimizada (~50MB)
- No toolchain en runtime
- Mejor seguridad
- Build cacheable layers

### Docker Compose

**Servicios incluidos**

1. **petclinic-app** - La aplicación Rust/Axum
2. **postgres** - Base de datos PostgreSQL
3. **redis** - Cache (reservado para futuro)
4. **adminer** - UI para PostgreSQL
5. **hodei-audit-service** - Servicio de auditoría
6. **clickhouse** - Almacenamiento hot para audit
7. **minio** - Almacenamiento S3-compatible
8. **vector** - Pipeline de datos
9. **prometheus** - Métricas
10. **grafana** - Dashboards

**Configuración de volúmenes**

```yaml
volumes:
  postgres_data:    # Persistencia de datos
  clickhouse_data:  # Datos de auditoría
  minio_data:       # Archive de auditoría
  # ...
```

**Redes**

```yaml
networks:
  petclinic-network:  # Para aplicación y DB
    subnet: 172.20.0.0/16
  audit-network:      # Para servicios de auditoría
    subnet: 172.21.0.0/16
```

### Desarrollo con Docker

**Usar volúmenes para hot-reload**

```yaml
services:
  petclinic-app:
    volumes:
      - ./src:/app/src:ro  # Solo lectura
      - target:/app/target
```

**No se recomienda hot-reload para Rust**
- Compile time es rápido con `cargo build --release`
- Mejor usar `docker-compose up -d --build` para cambios
- O usar `cargo watch` en local

### Comandos Útiles

```bash
# Build específica
docker-compose build petclinic-app

# Ver logs
docker-compose logs -f petclinic-app
docker-compose logs -f postgres
docker-compose logs -f hodei-audit-service

# Ejecutar comando en contenedor
docker-compose exec petclinic-app bash
docker-compose exec postgres psql -U petclinic -d petclinic

# Backup BD
docker-compose exec postgres pg_dump -U petclinic petclinic > backup.sql

# Restore BD
cat backup.sql | docker-compose exec -T postgres psql -U petclinic -d petclinic

# Inspección
docker-compose exec petclinic-app sh -c "ls -la /app"
docker inspect petclinic-petclinic-app-1

# Stats
docker stats

# Networking
docker network ls
docker network inspect petclinic_petclinic-network
```

### Producción

**Consideraciones para producción**

1. **Multi-stage build** - Ya implementado
2. **Usuario no-root** - Ya implementado
3. **Health checks** - Ya configurado
4. **Resource limits** - Añadir:
```yaml
deploy:
  resources:
    limits:
      cpus: '1'
      memory: 512M
    reservations:
      cpus: '0.5'
      memory: 256M
```

5. **Secrets management** - Usar Docker secrets
6. **Logging driver** - Configurar driver apropiado
7. **Security scanning** - Usar `docker scan` o `trivy`

**Ejemplo de deploy con secretos**

```bash
# Crear secreto
echo "my-secret-key" | docker secret create api_key -

# Usar en compose
secrets:
  - api_key
```

---

## Monitoreo y Observabilidad

### Métricas

**Prometheus** colecta métricas de:
- Pet Clinic application
- hodei-audit-service
- ClickHouse
- Vector
- PostgreSQL

**Métricas disponibles**

- Request count por endpoint
- Request latency (p50, p95, p99)
- Error rate
- Database connection pool
- Audit event throughput
- Audit event latency

**Acceso a métricas**

```bash
# Métricas de la aplicación
curl http://localhost:3000/metrics

# Métricas de hodei-audit-service
curl http://hodei-audit-service:50052/metrics

# Métricas de Prometheus
curl http://localhost:9090/api/v1/query?query=request_count
```

### Dashboards (Grafana)

**Pre-configurados en http://localhost:3001**

1. **Application Overview**
   - Request rate
   - Latency
   - Error rate
   - Active connections

2. **Database Dashboard**
   - Connection count
   - Query performance
   - Slow queries
   - Database size

3. **Audit Dashboard**
   - Events per second
   - Audit latency
   - Storage usage (ClickHouse, MinIO)
   - Top endpoints

4. **Infrastructure Dashboard**
   - Container CPU/Memory
   - Network I/O
   - Disk usage

**Crear dashboard custom**

1. Acceder a http://localhost:3001
2. Login (admin/admin123)
3. Create → Dashboard
4. Add panel con query Prometheus

**Ejemplo de query**

```
# Requests per second
rate(http_requests_total[5m])

# Error rate
rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m])

# P95 latency
histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))
```

### Logging

**Stack de logging**

- **Application** - tracing (estructurado)
- **Vector** - Centralización
- **Storage** - ClickHouse/MinIO
- **Visualization** - Grafana Loki

**Configuración de logging**

```toml
# vector.toml
[sources.application_logs]
type = "file"
include = ["/var/log/petclinic/*.log"]
read_from = "beginning"

[sinks.clickhouse]
type = "clickhouse"
inputs = ["application_logs"]
```

**Ver logs en tiempo real**

```bash
# Logs de aplicación
docker-compose logs -f petclinic-app

# Todos los logs
docker-compose logs -f

# Filtrar por servicio
docker-compose logs --tail=100 petclinic-app

# Logs con timestamps
docker-compose logs -f -t petclinic-app
```

**Logs estructurados**

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "petclinic_app",
  "message": "Owner created successfully",
  "owner_id": 1,
  "user_id": "user-123",
  "trace_id": "trace-456"
}
```

### Tracing

**Contexto distribuido**

- Trace ID propagado via headers
- Integrado con hodei-audit
- Visualización en Grafana Tempo (opcional)

**Headers de tracing**

```
x-trace-id: abc-123-def-456
x-span-id: span-789
```

### Alerting

**Configurar alertas en Prometheus**

```yaml
# prometheus/alerts.yml
groups:
- name: petclinic
  rules:
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 2m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value }} errors per second"

  - alert: HighLatency
    expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High latency detected"
      description: "95th percentile latency is {{ $value }}s"
```

### Health Checks

**Application health**

```bash
curl http://localhost:3000/health
```

**Database health**

```bash
docker-compose exec postgres pg_isready -U petclinic
```

**hodei-audit-service health**

```bash
curl http://hodei-audit-service:50052/health
```

**ClickHouse health**

```bash
curl http://localhost:8123/ping
```

### Performance Tuning

**Database**

- Connection pooling
- Query optimization
- Indexes
- Read replicas

**Application**

- Async runtime tuning
- Batch size optimization
- Cache layer (Redis)
- Connection limits

**Audit service**

- Batch size tuning
- Flush timeout
- Retry configuration
- Storage tiering

---

## Contribución

### Workflow

1. **Fork** el repositorio
2. **Crear** feature branch: `git checkout -b feature/nueva-funcionalidad`
3. **Commit** cambios: `git commit -m "feat: add nueva funcionalidad"`
4. **Push** al branch: `git push origin feature/nueva-funcionalidad`
5. **Crear** Pull Request

### Convenciones de Código

**Rust Style**

```bash
# Formatear código
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Verificar
cargo check
```

**Naming Conventions**

- **Snake_case** para variables, funciones, módulos
- **PascalCase** para tipos, traits
- **SCREAMING_SNAKE_CASE** para constantes

**Ejemplo**

```rust
pub struct OwnerService {
    owner_repository: Box<dyn OwnerRepository>,
}

impl OwnerService {
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Owner>> {
        self.owner_repository.find_by_id(id).await
    }
}
```

### Commits

**Formato Conventional Commits**

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types**

- `feat` - Nueva funcionalidad
- `fix` - Bug fix
- `docs` - Documentación
- `style` - Formato (no lógica)
- `refactor` - Refactorización
- `test` - Tests
- `chore` - Tareas de build/dependency

**Ejemplos**

```bash
feat(domain): add Owner validation logic
fix(api): resolve null pointer in owner controller
docs(readme): update installation instructions
test(repository): add integration test for owner repo
refactor(service): simplify pet creation flow
chore(deps): update SQLx to version 0.7
```

### Pull Request Process

1. **Tests** - Todos los tests deben pasar
2. **Coverage** - Mantener o mejorar coverage
3. **Linting** - Sin warnings
4. **Documentation** - Actualizar si es necesario
5. **Code Review** - Al menos 1 approval

**Checklist PR**

- [ ] Tests added/updated
- [ ] All tests pass
- [ ] Code formatted (cargo fmt)
- [ ] Linting clean (cargo clippy)
- [ ] Documentation updated
- [ ] Commits follow conventional format
- [ ] PR description explains changes

### Development Guidelines

**Arquitectura**

1. **Seguir Clean Architecture**
   - Domain independiente
   - Dependency inversion
   - Separación de responsabilidades

2. **Repository Pattern**
   - Siempre usar interfaces
   - No exponer SQLx en application layer
   - Tests con mocks

3. **Error Handling**
   - Usar custom error types
   - No propagar sqlx::Error directamente
   - Logging en cada capa

4. **Testing**
   - Unit tests para domain
   - Integration tests para repositories
   - API tests para controllers

**Performance**

1. **Database**
   - Usar connection pooling
   - Índices apropiados
   - Evitar N+1 queries
   - Batch operations cuando sea posible

2. **Memory**
   - Evitar clones innecesarios
   - Usar references
   - Streaming para datasets grandes

3. **Concurrency**
   - Tokio async
   - Evitar blocking operations
   - Backpressure en batch processing

### Issue Reporting

**Bug Report Template**

```markdown
**Descripción**
Descripción clara del bug

**Pasos para Reproducir**
1. Ir a '...'
2. Click en '....'
3. Scroll hasta '....'
4. Error

**Comportamiento Esperado**
Descripción de lo que esperaba

**Screenshots**
Si aplica, añadir screenshots

**Ambiente**
- OS: [e.g. macOS 12.0]
- Rust: [e.g. 1.75]
- Pet Clinic: [e.g. 0.1.0]

**Información Adicional**
Contexto adicional
```

**Feature Request Template**

```markdown
**¿Tu feature request está relacionado con un problema?**
Descripción del problema

**Descripción de la Solución Deseada**
Descripción de lo que quieres que suceda

**Alternativas Consideradas**
Descripción de soluciones alternativas

**Información Adicional**
Mockups, contexto, etc.
```

---

## FAQ

### Generales

**P: ¿Por qué Rust para Pet Clinic?**
R: Rust ofrece:
- Performance excelente
- Memory safety sin GC
- Concurrencia sin data races
- Ecosystem maduro para web
- Type safety en compile time
- Ideal para sistemas empresariales

**P: ¿Puedo usar MySQL en lugar de PostgreSQL?**
R: Sí, pero requeriría:
- Cambiar SQL queries para MySQL syntax
- Actualizar Cargo.toml dependencies
- Modificar SQLx features
- Testear con MySQL
- Recomendamos PostgreSQL por funcionalidades avanzadas

**P: ¿Cómo escalo esta aplicación?**
R: Opciones:
1. **Vertical scaling** - Más CPU/memoria
2. **Horizontal scaling** - Múltiples instances con load balancer
3. **Database scaling** - Read replicas, sharding
4. **Cache layer** - Redis para lecturas frecuentes
5. **CDN** - Para assets estáticos

**P: ¿Es production-ready?**
R: Sí, incluye:
- Clean architecture
- Error handling
- Logging estructurado
- Health checks
- Metrics
- Docker support
- Security best practices
- Pero requiere:
  - Authentication/Authorization
  - Rate limiting
  - Input sanitization adicional
  - Security audit
  - Performance testing

### Configuración

**P: ¿Cómo cambio el puerto de la aplicación?**
R: Variable de entorno:
```bash
export SERVER_PORT=8080
```
O en `.env`:
```bash
SERVER_PORT=8080
```

**P: ¿Cómo deshabilitar auditoría?**
R: En `.env`:
```bash
HODEI_AUDIT_ENABLED=false
```

**P: ¿Cómo usar un database externo?**
R: Configurar `DATABASE_URL`:
```bash
export DATABASE_URL=postgresql://user:pass@host:5432/dbname
```

**P: ¿Cómo ver las queries SQL ejecutadas?**
R: Habilitar logging:
```bash
RUST_LOG=sqlx=debug cargo run
```

### Desarrollo

**P: ¿Cómo añadir un nuevo endpoint?**
R: Pasos:
1. Añadir route en `presentation/controllers.rs`
2. Crear handler function
3. Crear DTO si es necesario
4. Añadir test
5. Documentar en API.md

**P: ¿Cómo añadir un nuevo campo a una entidad?**
R: Pasos:
1. Actualizar struct en `domain/entities.rs`
2. Actualizar repository queries
3. Crear migration SQL
4. Actualizar DTOs
5. Actualizar tests

**P: ¿Cómo testear con Docker?**
R: Ejecutar tests dentro del contenedor:
```bash
docker-compose exec petclinic-app cargo test
```

**P: ¿Cómo debuggear con IntelliJ/Rust Analyzer?**
R:
1. Instalar Rust Analyzer plugin
2. Abrir project folder
3. Cargo.toml será detectado automáticamente
4. Usar Debug configuration para tests

### Integración

**P: ¿Cómo funciona la integración con hodei-audit?**
R: Ver sección [Integración con Hodei Audit Service](#integración-con-hodei-audit-service)

**P: ¿Puedo usar hodei-audit sin headers?**
R: Sí, pero `tenant_id` será "unknown" y `user_id` será "anonymous"

**P: ¿Cómo ver eventos de auditoría?**
R:
- Grafana: http://localhost:3001
- API directa: http://hodei-audit-service:50053
- ClickHouse: http://localhost:8123

**P: ¿Cómo cambiar el tenant_id?**
R: En configuración o header:
```bash
HODEI_AUDIT_TENANT_ID=mi-tenant-custom
```
O en request:
```bash
-H "x-tenant-id: mi-tenant-custom"
```

### Base de Datos

**P: ¿Cómo crear una nueva migration?**
R:
```bash
sqlx migrate add nombre_de_migration
# Editar archivo generado
sqlx migrate run
```

**P: ¿Cómo hacer seed de datos?**
R: Usar `02-data.sql` o crear script separado:
```bash
psql $DATABASE_URL < db/seed.sql
```

**P: ¿Cómo cambiar el schema?**
R: 1. Crear migration down (reversible)
2. Crear migration up (nuevo schema)
3. Testear
4. Aplicar: `sqlx migrate run`

**P: ¿Cómo hacer backup completo?**
R:
```bash
docker-compose exec postgres pg_dumpall -U petclinic > backup.sql
```

**P: ¿Cómo restaurar desde backup?**
R:
```bash
cat backup.sql | docker-compose exec -T postgres psql -U petclinic
```

### Docker

**P: ¿Cómo hacer rebuild completo?**
R:
```bash
docker-compose down -v
docker-compose build --no-cache
docker-compose up -d
```

**P: ¿Cómo acceder al shell del contenedor?**
R:
```bash
docker-compose exec petclinic-app /bin/sh
```

**P: ¿Cómo ver usage de recursos?**
R:
```bash
docker stats
```

**P: ¿Cómo limpiar Docker completamente?**
R:
```bash
docker-compose down -v
docker system prune -a
```

### Performance

**P: ¿La aplicación es lenta?**
R: Posibles soluciones:
1. Añadir índices a BD
2. Implementar cache (Redis)
3. Optimizar queries
4. Aumentar connection pool
5. Profiling con `cargo flamegraph`

**P: ¿Alto uso de memoria?**
R: Verificar:
1. Connection pool settings
2. Memory leaks (valgrind)
3. Cache size
4. Batch processing size

**P: ¿Cómo optimizar queries?**
R:
1. Usar `EXPLAIN ANALYZE`
2. Añadir índices apropiados
3. Evitar SELECT *
4. Usar LIMIT para tests
5. Paginación en list endpoints

---

## Recursos Adicionales

### Documentación

- [Axum Documentation](https://docs.rs/axum/0.8/)
- [SQLx Documentation](https://docs.rs/sqlx/0.7/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://docs.rs/tokio/1.0/)
- [Serde Documentation](https://serde.rs/)
- [Hodei Audit Service](https://github.com/rubentxu/hodei-trail)

### Artículos y Tutorials

- [Clean Architecture in Rust](https://herbertograca.com/2017/09/28/clean-architecture-standing-up/)
- [Repository Pattern in Rust](https://alex Bewegung.gitbook.io/rust-notes/repository-pattern)
- [Axum Web Framework Tutorial](https://github.com/ndmitchell/axum-start)
- [SQLx Tutorial](https://github.com/launchbadge/sqlx/tree/master/examples)

### Herramientas

- [Rustup](https://rustup.rs/) - Rust toolchain installer
- [Cargo](https://doc.rust-lang.org/cargo/) - Package manager
- [rustfmt](https://github.com/rust-lang/rustfmt) - Code formatter
- [clippy](https://github.com/rust-lang/rust-clippy) - Linter
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) - Database CLI
- [just](https://github.com/casey/just) - Task runner

### Books

- "The Rust Programming Language" (The Book)
- "Programming Rust" by Blandy & Orendorff
- "Rust in Action" by Tim McNamara
- "Command Line Rust" by Ken Youens-Clark

### Videos

- [RustConf Talks](https://www.youtube.com/c/RustVideos)
- [Tokio Tutorials](https://tokio.rs/tutorials)
- [Axum Intro](https://www.youtube.com/watch?v=n7J_UxqjJfc)

### Community

- [Rust Discord](https://discord.gg/rust-lang)
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust](https://www.reddit.com/r/rust/)
- [Stack Overflow - Rust](https://stackoverflow.com/questions/tagged/rust)

### Alternative Implementations

- [Spring Petclinic (Java)](https://github.com/spring-projects/spring-petclinic)
- [Django Pet Clinic (Python)](https://github.com/thisisankitbhusal/Django-Pet-Clinic)
- [ASP.NET Core Pet Clinic](https://github.com/dotnet-architecture/aspnetcore-petclinic-microservices)

---

## Changelog

### v0.1.0 (2024-01-15)

**Added**
- Initial implementation
- Clean architecture with domain, application, infrastructure, presentation layers
- PostgreSQL integration with SQLx
- REST API with Axum
- Full CRUD operations for Owner, Pet, Visit, Vet
- hodei-audit-service integration
- HRN system
- Docker Compose setup
- Database schema and sample data
- Integration tests
- Developer manual
- Prometheus metrics
- Grafana dashboards
- Adminer for database UI

**Features**
- Owner management
- Pet registration and management
- Visit scheduling
- Veterinarian management
- Pet type management
- Automatic audit logging
- Multi-tenancy support
- Health checks

**Documentation**
- Developer manual
- API documentation
- Database schema
- Architecture diagrams
- Setup instructions

---

## Apéndices

### A. Environment Variables

Ver `.env.example` para lista completa de variables de entorno.

### B. API Endpoints Summary

```
GET    /health
GET    /owners
GET    /owners/{id}
POST   /owners
PUT    /owners/{id}
DELETE /owners/{id}
GET    /owners/{id}/pets
POST   /owners/{id}/pets

GET    /pets
GET    /pets/{id}
PUT    /pets/{id}
DELETE /pets/{id}
GET    /pets/{id}/visits
POST   /pets/{id}/visits

GET    /visits
GET    /visits/{id}
POST   /visits
PUT    /visits/{id}
DELETE /visits/{id}

GET    /vets
GET    /vets/{id}

GET    /pet-types
```

### C. Database ERD

```
[OWNERS] 1 -----> * [PETS] 1 -----> * [VISITS]
    |                    |
    |                    |
    v                    v
[TYPES]             [SPECIALTIES]
                           |
                           v
                      * [VETS] *
```

### D. Docker Services

| Service | Port | Purpose |
|---------|------|---------|
| petclinic-app | 3000 | Main application |
| postgres | 5432 | Database |
| redis | 6379 | Cache |
| adminer | 8080 | DB UI |
| hodei-audit-service | 50052, 50053 | Audit service |
| clickhouse | 8123, 9000 | Audit storage |
| minio | 9000, 9001 | Archive storage |
| vector | 50051, 8686 | Data pipeline |
| prometheus | 9090 | Metrics |
| grafana | 3001 | Dashboards |

---

**© 2024 Pet Clinic Application - Apache 2.0 License**
