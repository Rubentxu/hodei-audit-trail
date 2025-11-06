# Épica 1: Fundación y Arquitectura Base

## 📋 Resumen Ejecutivo

**Objetivo**: Establecer los cimientos del ecosistema Hodei Audit Service con arquitectura gRPC, diseño multi-tenant y patrones CloudTrail, utilizando Vector.dev para ingesta y fan-out.

**Alcance**: Definir arquitectura, crear estructura de proyecto, configurar entorno de desarrollo y establecer contratos gRPC con integración a Vector.dev.

**Duración Estimada**: 2-3 semanas

**Épica Padre**: Hodei Audit Service - Ecosistema Centralizado de Auditoría

---

## 🎯 Objetivo de Negocio

Como **arquitecto de software**, quiero establecer una **arquitectura sólida y escalable** para el sistema de auditoría, para que el equipo pueda **desarrollar funcionalidades** con **confianza** y **sin deuda técnica** en las iteraciones futuras, aprovechando **Vector.dev** para simplificar la gestión de ingesta, buffering y fan-out.

### Criterios de Aceptación (Épica)

- [ ] Arquitectura documentada y aprobada por stakeholders
- [ ] Estructura de proyecto creada y configurada
- [ ] Contratos gRPC definidos y versionados
- [ ] Patrones de diseño (CAP/ARP, CloudTrail) implementados
- [ ] Integración con Vector.dev planificada
- [ ] Entorno de desarrollo configurado y funcional
- [ ] Documentación técnica completa

---

## 👥 Historias de Usuario

### Historia 1.1: Definición de Arquitectura CAP/ARP con Vector.dev

**Como** Arquitecto de Software  
**Quiero** documentar la arquitectura CAP (Centralized Audit Point) y ARP (Audit Reporting Point) con Vector.dev  
**Para** establecer un lenguaje común y patrones de diseño para todo el equipo, aprovechando la simplicidad de Vector para ingesta y fan-out

#### Criterios de Aceptación

- [ ] Documento de arquitectura con diagramas CAP/ARP/Vector
- [ ] Comparativa con patrones PDP/PEP de verified-permissions
- [ ] Definición de responsabilidades: CAP (lógica) vs Vector (ingesta/routing)
- [ ] Flujo de datos: App → ARP → CAP → Vector → Storage documentado
- [ ] Revisión y aprobación del equipo técnico

#### Tareas Técnicas

1. Crear documento de arquitectura con Mermaid diagrams
2. Definir interfaces entre CAP y Vector (contrato gRPC simple)
3. Documentar el flujo de datos: App → ARP → CAP → Vector → ClickHouse/S3
4. Crear tabla de responsabilidades por componente
5. Revisar con el equipo y obtener aprobación

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Verificar diagramas arquitectónicos se generan correctamente
- [ ] Validar formato de documentos (markdown, links, imágenes)
- [ ] Verificar consistencia de nomenclatura (CAP/ARP/Vector)
- [ ] Validar flujo de datos documentado

**Tests de Integración Requeridos**:
- [ ] Documentación es accesible y navegable
- [ ] Links entre documentos funcionan correctamente
- [ ] Diagramas se renderizan correctamente en markdown
- [ ] Revisión de equipo completada y aprobada

**Comandos de Verificación**:
```bash
# Ejecutar validación de documentación
./scripts/validate-docs.sh

# Verificar consistencia de arquitectura
./scripts/check-architecture-consistency.sh

# Validar enlaces
markdown-link-check docs/architecture/*.md
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Documentación validada sin errores
- [ ] Presentación al equipo completada y aprobada
- [ ] **TODOS los criterios en verde ✅**

**Flujo de Datos CANÓNICO**:
```
App (ARP/SDK) → gRPC → Hodei Audit Service (CAP) → gRPC → Vector.dev → ClickHouse (hot) + S3 (warm)
```

**Definición de Done (ACTUALIZADA)**:
- ✅ Documento de arquitectura aprobado en `docs/architecture/`
- ✅ Diagrama CAP/ARP/Vector generado y versionado
- ✅ Presentación al equipo completada
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.2: Análisis y Adopción de Patrones CloudTrail

**Como** Líder Técnico  
**Quiero** estudiar y adoptar los patrones de AWS CloudTrail para el diseño de Hodei Audit
**Para** aprovechar mejores prácticas probadas en producción y evitar reinvenciones

#### Criterios de Aceptación

- [ ] Documento de análisis CloudTrail completado
- [ ] Taxonomía de eventos adoptada (Management, Data, Insight)
- [ ] Estructura de eventos CloudTrail-compatibles definida
- [ ] Patrón de EventID y ReadOnly flags implementado
- [ ] Sistema de Digest Criptográfico diseñado
- [ ] Campos AdditionalEventData y Error handling definidos

#### Tareas Técnicas

1. Analizar documentación de CloudTrail
2. Mapear conceptos CloudTrail a Hodei
3. Definir estructuras de datos compatibles
4. Diseñar sistema de digest criptográfico (SHA-256 + ed25519)
5. Crear documento de decisiones arquitectónicas (ADR)
6. Validar con casos de uso del PRD

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Tests Unitarios Requeridos**:
- [ ] Validar mapeo CloudTrail → Hodei correcto
- [ ] Verificar taxonomía de eventos (Management/Data/Insight)
- [ ] Testear estructuras de datos compatibles
- [ ] Validar diseño de digest criptográfico

**Tests de Integración Requeridos**:
- [ ] Documento cloudtrail-patterns.md completo y validado
- [ ] ADR documentado y aprobado por equipo
- [ ] Casos de uso PRD validados contra diseño
- [ ] Revisión técnica completada

**Comandos de Verificación**:
```bash
# Validar documentación
./scripts/validate-adr.sh

# Verificar mapeo de conceptos
./scripts/validate-cloudtrail-mapping.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% tests unitarios passing
- [ ] 100% tests integración passing
- [ ] Documentación técnica validada
- [ ] Equipo aprobó ADR
- [ ] **TODOS los criterios en verde ✅**

**Conceptos CloudTrail Adoptables**:
- Event Categories: Management, Data, Insight
- Digest Chain para tamper-evidence
- EventID único y ReadOnly flag
- AdditionalEventData como JSON
- ErrorCode y ErrorMessage

**Definición de Done (ACTUALIZADA)**:
- ✅ Documento `docs/architecture/cloudtrail-patterns.md` creado
- ✅ Estructuras de datos definidas en `src/types/`
- ✅ Diseño de digest documentado
- ✅ ADR aprobado
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.3: Estructura de Proyecto Rust

**Como** Desarrollador  
**Quiero** una estructura de proyecto Rust bien organizada y modular  
**Para** que el código esté **separado por responsabilidades** y sea **fácil de mantener**

#### Criterios de Aceptación

- [ ] Workspace Rust configurado con crates separados
- [ ] `hodei-audit-service` como servicio principal
- [ ] `hodei-audit-sdk` como librería reutilizable
- [ ] `hodei-audit-proto` para contratos gRPC
- [ ] `hodei-audit-types` para tipos compartidos
- [ ] Estructura alineada con arquitectura hexagonal
- [ ] Configuración de Cargo.toml y dependencies

#### Tareas Técnicas

1. Crear workspace en `Cargo.toml` raíz
2. Configurar crate `hodei-audit-proto` (protobuf)
3. Configurar crate `hodei-audit-types` (tipos compartidos)
4. Configurar crate `hodei-audit-service` (servicio)
5. Configurar crate `hodei-audit-sdk` (middleware)
6. Configurar `justfile` para tareas comunes
7. Configurar `.github/workflows` para CI/CD

**Estructura de Directorios**:
```
hodei-trail/
├── Cargo.toml (workspace)
├── justfile
├── hodei-audit-proto/
│   ├── proto/
│   │   ├── audit_event.proto
│   │   ├── audit_control.proto
│   │   ├── audit_query.proto
│   │   └── vector_api.proto
│   └── src/
├── hodei-audit-types/
│   ├── src/
│   │   ├── audit_event.rs
│   │   ├── hrn.rs
│   │   └── mod.rs
├── hodei-audit-service/
│   ├── src/
│   │   ├── grpc/
│   │   ├── storage/
│   │   ├── crypto/
│   │   └── main.rs
│   └── Dockerfile
├── hodei-audit-sdk/
│   ├── src/
│   │   ├── middleware.rs
│   │   ├── client.rs
│   │   └── lib.rs
│   └── tests/
└── docs/
    ├── architecture/
    ├── development/
    └── api/
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Verificar workspace Cargo.toml se configura correctamente
- [ ] Validar que todos los crates tienen dependencias correctas
- [ ] Testear que `cargo build` compila sin errores en todos los crates
- [ ] Verificar estructura de directorios coincide con especificación
- [ ] Validar justfile se ejecuta correctamente
- [ ] Verificar configuración de CI/CD en `.github/workflows/`

**Tests de Integración Requeridos**:
- [ ] Build completo del workspace sin warnings
- [ ] Todos los módulos se importan correctamente
- [ ] CI pipeline ejecuta build exitosamente
- [ ] Documentación README en cada crate creada
- [ ] Cross-compilation para diferentes targets
- [ ] Justfile commands funcionan correctamente

**Comandos de Verificación**:
```bash
# Verificar workspace completo
cargo build --workspace

# Verificar que todos los tests pasan
cargo test --workspace

# Verificar justfile
just --list
just setup

# Verificar estructura de proyecto
./scripts/validate-project-structure.sh

# Verificar CI/CD configuration
./scripts/validate-ci-config.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Build exitoso en todos los crates
- [ ] Estructura de directorios validada
- [ ] CI/CD pipeline configurado y funcional
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ `cargo build` funciona en todos los crates
- ✅ Tests unitarios passing en todos los módulos
- ✅ Documentación README en cada crate
- ✅ CI pipeline configurado
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.4: Definición de Contratos gRPC

**Como** Desarrollador Backend  
**Quiero** contratos gRPC claros y versionados para la comunicación entre componentes  
**Para** garantizar **type safety** y **compatibilidad** entre versiones

#### Criterios de Aceptación

- [ ] Proto files definidos para AuditControlService (CAP ← ARP)
- [ ] Proto files definidos para AuditQueryService (CAP → Client)
- [ ] Contrato simple CAP → Vector (batch → confirmation)
- [ ] Mensajes de request/response documentados
- [ ] Códigos de error gRPC definidos
- [ ] Versioning strategy documentada
- [ ] Cliente gRPC generado y testeable

#### Tareas Técnicas

1. Definir `audit_event.proto` con estructura CloudTrail
2. Definir `audit_control.proto` para ingestión desde ARP
3. Definir `audit_query.proto` para consultas
4. Definir `vector_api.proto` para comunicación CAP → Vector
5. Generar código Rust con tonic y prost
6. Crear tests de integración básicos
7. Documentar API con grpcurl examples

**Contratos gRPC Principales**:

```protobuf
// Puerto 50052: Ingestión desde ARPs (SDKs)
service AuditControlService {
  rpc PublishEvent(PublishEventRequest) returns (PublishEventResponse);
  rpc PublishBatch(PublishBatchRequest) returns (PublishBatchResponse);
}

// Puerto 50053: Query para clientes
service AuditQueryService {
  rpc QueryEvents(AuditQueryRequest) returns (AuditQueryResponse);
  rpc ResolveHrn(ResolveHrnRequest) returns (ResolveHrnResponse);
}

// Puerto 50054: Crypto/Digest para compliance
service AuditCryptoService {
  rpc VerifyDigest(VerifyDigestRequest) returns (VerifyDigestResponse);
  rpc GetPublicKeys(GetPublicKeysRequest) returns (GetPublicKeysResponse);
}

// Puerto 50051: Vector para fan-out (CAP → Vector)
service VectorApi {
  rpc SendEventBatch(EventBatchRequest) returns (EventBatchResponse);
}
```

**Contrato CAP → Vector (SIMPLE)**:
```protobuf
message EventBatchRequest {
  repeated AuditEvent events = 1;
}

message EventBatchResponse {
  bool success = 1;  // Solo confirmación
}
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que proto files se compilan correctamente
- [ ] Verificar que todas las estructuras de mensajes están bien definidas
- [ ] Testear que gRPC services se generan sin errores
- [ ] Validar códigos de error gRPC definidos
- [ ] Verificar versioning strategy documentada
- [ ] Testear compatibilidad de contratos entre versiones

**Tests de Integración Requeridos**:
- [ ] Código Rust generado con tonic y prost funciona
- [ ] Cliente gRPC puede conectarse a servicios
- [ ] Tests de integración básicos passing
- [ ] Documentación API con grpcurl examples validada
- [ ] Contratos se versionan correctamente
- [ ] Compatibilidad entre CAP y Vector verificada

**Comandos de Verificación**:
```bash
# Compilar proto files
cargo build -p hodei-audit-proto

# Generar código gRPC
cargo build -p hodei-audit-service

# Testear contratos
cargo test -p hodei-audit-proto --lib

# Testear integración gRPC
cargo test -p hodei-audit-service grpc_integration

# Validar documentación API
./scripts/validate-grpc-docs.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Proto files compilados sin errores
- [ ] Cliente gRPC testeable y funcional
- [ ] Documentación API validada
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Proto files en `hodei-audit-proto/proto/`
- ✅ Código generado en `hodei-audit-service/src/grpc/`
- ✅ Documentación API en `docs/api/`
- ✅ Tests de contrato passing
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.5: Configuración de Entorno de Desarrollo

**Como** Desarrollador  
**Quiero** un entorno de desarrollo completo y documentado  
**Para** que cualquier miembro del equipo pueda **setup en menos de 30 minutos**

#### Criterios de Aceptación

- [ ] Docker Compose configurado con servicios base
- [ ] Vector.dev incluido en entorno de desarrollo
- [ ] Script de setup automatizado
- [ ] Documentación de instalación completa
- [ ] Variables de entorno documentadas
- [ ] Debugging configurado (vscode, intellij)
- [ ] Herramientas de desarrollo instaladas

#### Tareas Técnicas

1. Configurar `docker-compose.dev.yml` con:
   - ClickHouse para desarrollo
   - Vector.dev para ingesta
   - Prometheus para métricas
   - MinIO para S3-compatible storage
2. Crear script `scripts/setup-dev.sh`
3. Crear `.env.example` con todas las variables
4. Configurar launch configs para vscode
5. Documentar en `docs/development/`
6. Configurar rust-analyzer y extensiones

**Servicios en Docker Compose**:
```yaml
services:
  clickhouse:
    image: clickhouse/clickhouse-server:23.8
    ports: ["8123:8123", "9000:9000"]
  
  vector:
    image: timberio/vector:latest-alpine
    ports: ["50051:50051", "9598:9598"]
    volumes:
      - ./config/vector/vector.toml:/etc/vector/vector.toml:ro
      - vector_data:/var/lib/vector
  
  minio:
    image: minio/minio:latest
    ports: ["9000:9000", "9001:9001"]
  
  prometheus:
    image: prom/prometheus:v2.47
    ports: ["9090:9090"]
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar configuración docker-compose.dev.yml es válida
- [ ] Verificar que todas las imágenes están en las versiones correctas
- [ ] Testear que puertos no colisionan con otros servicios
- [ ] Validar variables de entorno en .env.example
- [ ] Verificar configuración de volúmenes
- [ ] Validar scripts de setup son ejecutables

**Tests de Integración Requeridos**:
- [ ] `./scripts/setup-dev.sh` ejecuta end-to-end sin errores
- [ ] Vector.dev levanta y acepta conexiones gRPC
- [ ] ClickHouse acepta conexiones y es accesible
- [ ] MinIO levanta y es accesible
- [ ] Prometheus levanta y recolecta métricas
- [ ] Documentación `docs/development/setup.md` validada
- [ ] Team puede hacer setup sin ayuda
- [ ] Todos los servicios health checks passing

**Comandos de Verificación**:
```bash
# Validar configuración Docker Compose
docker-compose -f docker-compose.dev.yml config

# Ejecutar setup completo
./scripts/setup-dev.sh

# Verificar servicios
./scripts/health-check.sh

# Testear que Vector recibe eventos
./scripts/test-vector-connection.sh

# Verificar documentación
./scripts/validate-dev-docs.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Docker Compose válido y funcional
- [ ] Setup automatizado funcional
- [ ] Todos los servicios en estado healthy
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ `./scripts/setup-dev.sh` funciona end-to-end
- ✅ Vector.dev levanta y acepta conexiones gRPC
- ✅ Documentación `docs/development/setup.md` completa
- ✅ Team puede hacer setup sin ayuda
- ✅ Todos los servicios levantan correctamente
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.6: Sistema HRN (Hodei Resource Names)

**Como** Desarrollador  
**Quiero** un sistema HRN completo y funcional  
**Para** que todos los recursos tengan **identificadores únicos y jerárquicos**

#### Criterios de Aceptación

- [ ] Parser HRN implementado y testeado
- [ ] Formato HRN validado (hrn:partition:service:tenant:region:type/path)
- [ ] Operaciones HRN (parent, child, is_child_of) implementadas
- [ ] Cache de metadata HRN funcional
- [ ] Resolución de HRN a metadata
- [ ] Tests unitarios con 100% coverage

#### Tareas Técnicas

1. Implementar struct `Hrn` con parsing y validation
2. Implementar `HrnResolver` con LRU cache
3. Implementar operaciones: parse, to_string, parent, is_child_of
4. Crear tests unitarios comprensivos
5. Documentar ejemplos HRN en `docs/hrn/`
6. Integrar con tipos de audit events

**Formato HRN**:
```
hrn:<partition>:<service>:<tenant>:<region>:<resource-type>/<resource-path>

Ejemplos:
- hrn:hodei:verified-permissions:tenant-123:global:policy-store/default
- hrn:hodei:api:tenant-123:eu-west-1:api/user-profile
- hrn:hodei:storage:tenant-123:global:bucket/uploads
- hrn:hodei:verified-permissions:tenant-123:global:authorization/user-123
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Parser HRN parsea strings válidos correctamente
- [ ] Parser HRN rechaza strings inválidos con errores descriptivos
- [ ] Validar formato HRN (hrn:partition:service:tenant:region:type/path)
- [ ] Testear operación `parent()` retorna el HRN padre correcto
- [ ] Testear operación `child()` crea un HRN hijo válido
- [ ] Testear operación `is_child_of()` detecta jerarquía correctamente
- [ ] Testear `HrnResolver` con LRU cache
- [ ] Validar resolución de HRN a metadata
- [ ] Testear performance con HRNs complejos
- [ ] Validar casos edge (HRNs malformados, caracteres especiales)

**Tests de Integración Requeridos**:
- [ ] Sistema HRN funciona con audit events
- [ ] Cache de metadata HRN funciona correctamente
- [ ] Integración con tipos compartidos exitosa
- [ ] Documentación `docs/hrn/README.md` validada
- [ ] Benchmarks de performance passing
- [ ] Tests comprensivos con 100% coverage
- [ ] Ejemplos de HRN en documentación son correctos

**Comandos de Verificación**:
```bash
# Testear parser HRN
cargo test -p hodei-audit-types hrn_parsing

# Testear operaciones HRN
cargo test -p hodei-audit-types hrn_operations

# Testear cache HRN
cargo test -p hodei-audit-types hrn_cache

# Testear integración
cargo test -p hodei-audit-service hrn_integration

# Verificar coverage
cargo tarpaulin -p hodei-audit-types --out xml

# Benchmarking
cargo bench -p hodei-audit-types hrn
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Parser HRN funciona al 100%
- [ ] Cache LRU funciona correctamente
- [ ] Coverage >= 95% en módulo HRN
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Código en `hodei-audit-service/src/hrn/`
- ✅ Tests en `hodei-audit-service/tests/hrn/`
- ✅ Documentación `docs/hrn/README.md`
- ✅ Benchmarks de performance
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 1.7: Configuración de CI/CD Base

**Como** DevOps Engineer  
**Quiero** un pipeline CI/CD básico pero robusto  
**Para** automatizar **builds, tests y quality gates**

#### Criterios de Aceptación

- [ ] GitHub Actions configurado
- [ ] Build automatizado en cada push
- [ ] Tests unitarios ejecutados automáticamente
- [ ] Linting (clippy, rustfmt) automatizado
- [ ] Security scanning (cargo-audit) configurado
- [ ] Artifacts almacenados

#### Tareas Técnicas

1. Crear `.github/workflows/ci.yml`
2. Configurar matrix de testing (stable, nightly)
3. Configurar cargo-audit para security
4. Configurar cargo-tarpaulin para coverage
5. Configurar sonarqube (opcional)
6. Configurar badge de status en README

**Pipeline Stages**:
```yaml
1. Checkout code
2. Setup Rust toolchain
3. Cache dependencies
4. Run rustfmt (check)
5. Run clippy (lint)
6. Run cargo-audit (security)
7. Run tests
8. Generate coverage
9. Build artifacts
10. Upload artifacts
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar configuración de GitHub Actions es válida
- [ ] Verificar workflow se ejecuta en pushes correctos
- [ ] Testear matrix de testing (stable, nightly)
- [ ] Validar cargo-audit configuration
- [ ] Verificar cargo-tarpaulin para coverage
- [ ] Testear artifact storage configuration
- [ ] Validar cache de dependencias
- [ ] Verificar que badges de status están bien configurados

**Tests de Integración Requeridos**:
- [ ] Pipeline CI/CD corre en PRs sin errores
- [ ] Build automatizado funciona en cada push
- [ ] Tests unitarios se ejecutan automáticamente
- [ ] Linting (clippy, rustfmt) automatizado passing
- [ ] Security scanning (cargo-audit) passing sin vulnerabilidades
- [ ] Coverage report se genera correctamente
- [ ] Build artifacts se almacenan y son accesibles
- [ ] Documentación del pipeline completa
- [ ] SonarQube analysis (si configurado) passing

**Comandos de Verificación**:
```bash
# Validar workflow de GitHub Actions
gh workflow list
gh workflow run ci.yml --dry-run

# Verificar pipeline localmente (con act)
act -P ubuntu-latest=nektos/act-environments-ubuntu:18.04

# Testear linting
cargo fmt --check
cargo clippy --all-targets --all-features

# Testear security
cargo audit

# Verificar coverage
cargo tarpaulin --out xml --output-dir coverage/

# Build test
cargo build --release
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] CI pipeline funcional en GitHub Actions
- [ ] Security scan sin vulnerabilidades críticas
- [ ] Coverage report >= 80%
- [ ] Build artifacts generados correctamente
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Pipeline corriendo en PRs
- ✅ Coverage report generado
- ✅ Security scan passing
- ✅ Build artifacts disponibles
- ✅ Documentación del pipeline
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## 📊 Métricas de Éxito

| Métrica | Objetivo | Medición |
|---------|----------|----------|
| **Tiempo de setup** | < 30 min | Script de setup documentado |
| **Coverage** | > 80% | cargo-tarpaulin report |
| **Build time** | < 5 min | GitHub Actions metrics |
| **Documentación** | 100% de APIs | docs/coverage checklist |
| **Static analysis** | 0 warnings | clippy output |

---

## 🚀 Entregables

1. **Documentación**:
   - `docs/architecture/cap-arp.md`
   - `docs/architecture/cloudtrail-patterns.md`
   - `docs/development/setup.md`
   - `docs/hrn/README.md`
   - `docs/api/grpc-contracts.md`

2. **Código**:
   - Estructura de proyecto completa
   - Sistema HRN implementado
   - Contratos gRPC definidos
   - Tests unitarios passing

3. **Infraestructura**:
   - Docker Compose con Vector.dev
   - CI/CD pipeline configurado
   - Scripts de automatización

---

## 🔗 Dependencias

**Bloquea**: Ninguna (es la primera épica)  
**Bloqueada por**: Ninguna

---

## 📝 Notas de Implementación

### Decisiones Arquitectónicas (ADR)

1. **ADR-001**: Adopción de arquitectura CAP/ARP/Vector
2. **ADR-002**: Patrones CloudTrail para event taxonomy
3. **ADR-003**: HRN como sistema de naming jerárquico
4. **ADR-004**: gRPC para comunicación entre componentes
5. **ADR-005**: Vector.dev para ingesta, buffering y fan-out
6. **ADR-006**: Contrato simple CAP → Vector (batch → confirmation)
7. **ADR-007**: Workspace Rust con crates modulares

### Ventajas de usar Vector.dev

✅ **Fan-out automático**: Múltiples sinks (ClickHouse, S3, etc.) sin código  
✅ **Buffer persistente**: Disk buffer para zero-loss  
✅ **Reintentos**: Backoff exponencial automático  
✅ **Operacional simple**: Un componente vs múltiples soluciones  
✅ **Métricas nativas**: Prometheus metrics integradas  
✅ **Configuración declarativa**: vector.toml vs código  

### Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| Complejidad de HRN | Media | Alto | Implementar gradualmente, tests comprehensivos |
| gRPC breaking changes | Baja | Medio | Versioning strategy desde día 1 |
| Team unfamiliar con Vector | Media | Medio | Documentación detallada y examples |
| Contrato CAP → Vector complejo | Baja | Alto | Mantener contrato simple: batch → confirmation |

---

## ⏭️ Siguiente Épica

[Épica 2: Core Service y HRN System](epic-02-core-service-y-hrn.md)

---

**Versión**: 1.1 (Actualizada con Vector.dev)  
**Fecha**: 2025-01-15  
**Estado**: En Planificación  
**Épica Padre**: Hodei Audit Service
