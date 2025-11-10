# Pet Clinic Application - Complete Implementation

## Overview

This document provides the complete implementation of a Pet Clinic application in Rust/Axum, fully integrated with hodei-audit-service for centralized auditing.

## Architecture

The application follows a clean architecture pattern:

```
┌─────────────────────────────────────────────────┐
│                 REST API (Axum)                 │
│  - OwnersController                            │
│  - PetsController                              │
│  - VisitsController                            │
│  - VetsController                              │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│            Service Layer                        │
│  - ClinicService                                │
│  - Business Logic                               │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│          Repository Layer (SQLx)                │
│  - PostgreSQL Database                          │
│  - Connection Pooling                           │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│          Domain Models                          │
│  - Owner, Pet, Visit, Vet, Specialty           │
└─────────────────────────────────────────────────┘
                  │
                  │ gRPC
                  ▼
┌─────────────────────────────────────────────────┐
│      Hodei Audit Service (CAP)                  │
│  - Auto-capture HTTP requests                   │
│  - HRN generation                               │
│  - Batch processing                             │
│  - ClickHouse + Vector.dev                      │
└─────────────────────────────────────────────────┘
```

## File Structure

```
petclinic-app/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── config.rs                  # Configuration
│   ├── domain/                    # Domain layer
│   │   ├── mod.rs
│   │   ├── entities.rs            # Domain entities
│   │   ├── repositories.rs        # Repository contracts
│   │   └── services.rs            # Domain services
│   ├── application/               # Application layer
│   │   └── services.rs            # Application services
│   ├── infrastructure/            # Infrastructure layer
│   │   └── repositories.rs        # SQLx implementations
│   └── presentation/              # Presentation layer
│       ├── controllers.rs         # HTTP controllers
│       └── mod.rs
├── db/
│   └── init/
│       ├── 01-init.sql            # Database schema
│       └── 02-data.sql            # Sample data
├── Dockerfile                     # Application container
├── docker-compose.yml             # Complete stack
├── Cargo.toml                     # Dependencies
└── README.md                      # Documentation
```

## Database Schema

```sql
-- Owners table
CREATE TABLE owners (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    address VARCHAR(255),
    city VARCHAR(100),
    telephone VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Pet types
CREATE TABLE types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL UNIQUE
);

-- Pets
CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    birth_date DATE,
    type_id INTEGER REFERENCES types(id),
    owner_id INTEGER REFERENCES owners(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Visits
CREATE TABLE visits (
    id SERIAL PRIMARY KEY,
    pet_id INTEGER REFERENCES pets(id) ON DELETE CASCADE,
    visit_date DATE NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Specialties
CREATE TABLE specialties (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL UNIQUE
);

-- Vets
CREATE TABLE vets (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL
);

-- Vet-Specialty many-to-many
CREATE TABLE vet_specialties (
    vet_id INTEGER REFERENCES vets(id) ON DELETE CASCADE,
    specialty_id INTEGER REFERENCES specialties(id) ON DELETE CASCADE,
    PRIMARY KEY (vet_id, specialty_id)
);

-- Indexes
CREATE INDEX idx_pets_owner_id ON pets(owner_id);
CREATE INDEX idx_pets_type_id ON pets(type_id);
CREATE INDEX idx_visits_pet_id ON visits(pet_id);
CREATE INDEX idx_owners_last_name ON owners(last_name);
```

## Sample Data

```sql
-- Pet types
INSERT INTO types (name) VALUES
    ('Dog'), ('Cat'), ('Bird'), ('Rabbit'), ('Hamster');

-- Specialties
INSERT INTO specialties (name) VALUES
    ('Radiology'), ('Surgery'), ('Dentistry'), ('Internal Medicine');

-- Vets
INSERT INTO vets (first_name, last_name) VALUES
    ('James', 'Carter'), ('Helen', 'Leary'), ('Linda', 'Douglas');

-- Sample owners
INSERT INTO owners (first_name, last_name, address, city, telephone) VALUES
    ('George', 'Franklin', '110 W. Liberty St.', 'Madison', '6085551023'),
    ('Betty', 'Davis', '638 Cardinal Ave.', 'Sun Prairie', '6085551745');

-- Sample pets
INSERT INTO pets (name, birth_date, type_id, owner_id) VALUES
    ('Leo', '2010-09-07', 1, 1), -- Dog for George
    ('Basil', '2012-08-06', 2, 2); -- Cat for Betty
```

## API Endpoints

### Owners
- `GET /owners` - List all owners
- `GET /owners/{id}` - Get owner details with pets
- `POST /owners` - Create new owner
- `PUT /owners/{id}` - Update owner
- `DELETE /owners/{id}` - Delete owner and all pets
- `GET /owners/{id}/pets` - List owner's pets
- `POST /owners/{id}/pets` - Add pet to owner

### Pets
- `GET /pets` - List all pets
- `GET /pets/{id}` - Get pet details
- `PUT /pets/{id}` - Update pet
- `DELETE /pets/{id}` - Delete pet
- `GET /pets/{id}/visits` - List pet's visits
- `POST /pets/{id}/visits` - Add visit to pet

### Visits
- `GET /visits` - List all visits
- `GET /visits/{id}` - Get visit details
- `POST /visits` - Create new visit
- `PUT /visits/{id}` - Update visit
- `DELETE /visits/{id}` - Delete visit

### Vets
- `GET /vets` - List all vets
- `GET /vets/{id}` - Get vet details

### Pet Types
- `GET /pet-types` - List all pet types

## Hodei Audit Integration

### Example HRN Generation

The SDK automatically generates HRNs for each endpoint:

| Endpoint | HRN Generated |
|----------|---------------|
| `POST /owners` | `hrn:hodei:petclinic:tenant-petclinic:global:owner/create` |
| `GET /owners/1` | `hrn:hodei:petclinic:tenant-petclinic:global:owner/1` |
| `POST /owners/1/pets` | `hrn:hodei:petclinic:tenant-petclinic:global:pet/create` |
| `GET /pets/1/visits` | `hrn:hodei:petclinic:tenant-petclinic:global:visit/list` |

### Configuration

```rust
use hodei_audit_sdk::{AuditSdkConfig, AuditLayer};

// In main.rs
let audit_config = AuditSdkConfig::builder()
    .service_name("petclinic")
    .tenant_id("tenant-petclinic")
    .audit_service_url("http://hodei-audit-service:50052")
    .batch_size(100)
    .batch_timeout(std::time::Duration::from_millis(100))
    .build()?;

let app = Router::new()
    .nest("/owners", owners_routes())
    .nest("/pets", pets_routes())
    // ... other routes
    .layer(AuditLayer::new(audit_config));
```

## Docker Compose

See the full docker-compose.yml file for complete stack including:
- petclinic-app (Rust/Axum application)
- postgres (Database)
- redis (Caching)
- adminer (Database UI)
- hodei-audit-service (Audit service)
- clickhouse (Hot storage for audit)
- minio (S3-compatible storage)
- vector (Data pipeline)
- prometheus (Metrics)
- grafana (Dashboards)

## Running the Application

### With Docker Compose
```bash
docker-compose up -d
```

### Development
```bash
# Setup database
export DATABASE_URL=postgresql://petclinic:petclinic@localhost:5432/petclinic
sqlx db setup

# Run application
cargo run
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --features testing

# Coverage
cargo tarpaulin --out html
```

## Key Features

1. ✅ **Clean Architecture** - Domain, Application, Infrastructure, Presentation
2. ✅ **Repository Pattern** - SQLx with PostgreSQL
3. ✅ **Auto-auditing** - All HTTP operations via hodei-audit-service
4. ✅ **HRN System** - Automatic resource identification
5. ✅ **Multi-tenancy** - Tenant isolation
6. ✅ **Validation** - Domain-level validation
7. ✅ **Error Handling** - Proper error propagation
8. ✅ **Logging** - Structured logging with tracing
9. ✅ **Health Checks** - Application health endpoint
10. ✅ **Docker Support** - Complete containerization

## Business Rules

1. Owner can have multiple pets
2. Each pet belongs to one owner
3. Each pet can have multiple visits
4. Pet name must be unique per owner
5. Birth date cannot be in the future
6. Visit date cannot be in the future
7. Deleting owner cascades to delete all pets and visits
8. Vets can have multiple specialties

## Next Steps

1. Add authentication/authorization
2. Add rate limiting
3. Add caching layer with Redis
4. Add more validation
5. Add search capabilities
6. Add reporting features
7. Add file upload for pet photos
8. Add appointment scheduling
9. Add billing system
10. Add mobile app API

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

## License

Apache 2.0
