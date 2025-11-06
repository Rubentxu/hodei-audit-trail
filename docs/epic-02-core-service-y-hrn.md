# Épica 2: Core Service y HRN System

## 📋 Resumen Ejecutivo

**Objetivo**: Implementar el servicio central Hodei Audit Service (CAP) con todas las funcionalidades core: ingestión gRPC, query engine, resolución HRN, event enrichment y configuración de almacenamiento.

**Alcance**: Desarrollo del servicio principal, pipeline de eventos, motor de consultas, sistema HRN completo y conectores de almacenamiento (ClickHouse + S3/MinIO).

**Duración Estimada**: 3-4 semanas

**Épica Padre**: Hodei Audit Service - Ecosistema Centralizado de Auditoría

---

## 🎯 Objetivo de Negocio

Como **arquitecto**, quiero implementar el **servicio central (CAP)** con todas las funcionalidades core, para que las aplicaciones puedan **enviar eventos de auditoría** de forma **confiable** y los usuarios puedan **consultar** y **analizar** eventos con **latencia baja** y **alta disponibilidad**.

### Criterios de Aceptación (Épica)

- [ ] Hodei Audit Service desplegado y funcional
- [ ] APIs gRPC (ingestión, query, crypto) implementadas
- [ ] Sistema HRN completamente funcional con cache
- [ ] Event enrichment pipeline operativo
- [ ] Query engine con filtros avanzados
- [ ] Almacenamiento tiered (Hot/Warm/Cold) configurado
- [ ] Performance: 10K+ events/sec ingestión, <100ms queries

---

## 👥 Historias de Usuario

### Historia 2.1: Implementación de Servicio gRPC Principal

**Como** Desarrollador Backend  
**Quiero** implementar el servicio Hodei Audit Service (CAP) con gRPC  
**Para** que las aplicaciones puedan enviar eventos de auditoría vía gRPC de forma **type-safe**

#### Criterios de Aceptación

- [ ] Servicio gRPC escuchando en puertos 50052-50054
- [ ] AuditControlService (ingestión) implementado
- [ ] AuditQueryService (consultas) implementado
- [ ] AuditCryptoService (digest) implementado
- [ ] Graceful shutdown configurado
- [ ] Health checks funcionando
- [ ] Error handling robusto

#### Tareas Técnicas

1. Implementar `main.rs` con tonic::transport::Server
2. Implementar `AuditControlService` con PublishEvent/PublishBatch
3. Implementar `AuditQueryService` con QueryEvents/ResolveHrn
4. Implementar `AuditCryptoService` con VerifyDigest/GetPublicKeys
5. Configurar TLS para producción
6. Implementar health checks (health = serving/not_serving)
7. Añadir logging estructurado (json)
8. Crear tests de integración con clientes gRPC

**Arquitectura del Servicio**:
```rust
pub struct AuditService {
    config: AuditConfig,
    storage: Arc<dyn StorageBackend>,
    hrn_resolver: Arc<HrnResolver>,
    enricher: Arc<EventEnricher>,
    query_engine: Arc<QueryEngine>,
    metrics: Arc<AuditMetrics>,
}
```

**Endpoints gRPC**:

Puerto 50052 - Ingestión:
- `PublishEvent`: Envío de evento individual
- `PublishBatch`: Envío de lote de eventos (optimizado)

Puerto 50053 - Query:
- `QueryEvents`: Consultar eventos con filtros
- `ResolveHrn`: Resolver metadata de HRN

Puerto 50054 - Crypto:
- `VerifyDigest`: Verificar integridad de digest
- `GetPublicKeys`: Obtener claves públicas para verificación

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que servicio gRPC se inicia correctamente
- [ ] Verificar que todos los puertos se abren (50052-50054)
- [ ] Testear que AuditControlService responde a PublishEvent
- [ ] Testear que AuditControlService responde a PublishBatch
- [ ] Testear que AuditQueryService responde a QueryEvents
- [ ] Testear que AuditQueryService responde a ResolveHrn
- [ ] Testear que AuditCryptoService responde a VerifyDigest
- [ ] Testear que AuditCryptoService responde a GetPublicKeys
- [ ] Verificar graceful shutdown funciona
- [ ] Testear health checks (serving/not_serving)
- [ ] Validar error handling y logging

**Tests de Integración Requeridos**:
- [ ] Cliente gRPC puede conectarse y enviar eventos
- [ ] Servicio maneja PublishEvent correctamente
- [ ] Servicio maneja PublishBatch correctamente (lotes)
- [ ] Query engine responde con datos correctos
- [ ] TLS configurado y funcional (si habilitado)
- [ ] Logging estructurado en formato JSON
- [ ] Servicios múltiples pueden correr simultáneamente
- [ ] Tests de stress con múltiples clientes concurrentes
- [ ] Graceful shutdown bajo load

**Comandos de Verificación**:
```bash
# Testear servicio gRPC
cargo test -p hodei-audit-service grpc_service

# Testear health checks
curl http://localhost:50055/healthz

# Testear con cliente gRPC
grpcurl -plaintext localhost:50052 list

# Testear integración completa
cargo test -p hodei-audit-service grpc_integration

# Testear graceful shutdown
./scripts/test-graceful-shutdown.sh

# Load testing
k6 run scripts/load-test-grpc.js
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Servicio corriendo sin errores
- [ ] Todos los endpoints gRPC funcionando
- [ ] Health checks respondiendo correctamente
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Servicio corre sin errores
- ✅ Todos los endpoints gRPC implementados
- ✅ Health check responde correctamente
- ✅ Tests de integración passing
- ✅ Documentación API actualizada
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.2: Sistema HRN Completo

**Como** Desarrollador  
**Quiero** un sistema HRN completo con resolución y cache  
**Para** que todos los recursos tengan **identificadores únicos** y **metadata asociada**

#### Criterios de Aceptación

- [ ] Parser HRN con validación completa
- [ ] Operaciones: parse, to_string, parent, is_child_of, hierarchy
- [ ] HrnResolver con LRU cache (TTL configurable)
- [ ] Búsqueda por patrones (wildcards)
- [ ] Resolución de metadata async
- [ ] Cache invalidation strategy
- [ ] 100% test coverage

#### Tareas Técnicas

1. Implementar struct `Hrn` con todas las validaciones
2. Implementar `HrnMetadata` para metadata de recursos
3. Implementar `HrnResolver` con cache LRU
4. Implementar operaciones: parent(), is_child_of(), hierarchy()
5. Implementar búsqueda con patrones
6. Configurar TTL y políticas de cache
7. Integrar con base de datos de metadata
8. Crear tests comprensivos

**Estructura HRN**:
```rust
pub struct Hrn {
    pub partition: String,      // "hodei"
    pub service: ServiceId,     // "verified-permissions"
    pub tenant_id: TenantId,    // "tenant-123"
    pub region: Option<String>, // "eu-west-1" o None
    pub resource_type: String,  // "policy-store"
    pub resource_path: String,  // "default/policies/123"
}
```

**Operaciones HRN**:
- `parse(s)`: Parse string a Hrn con validación
- `to_string()`: Convertir Hrn a string canónico
- `parent()`: Obtener HRN padre (sin último componente)
- `is_child_of(parent)`: Verificar si es hijo de otro HRN
- `hierarchy()`: Obtener jerarquía completa

**Cache Strategy**:
- LRU cache con tamaño configurable
- TTL por entrada
- Invalidación manual para updates
- Metrics: hit/miss ratio, cache size

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Parser HRN parsea strings válidos correctamente
- [ ] Parser HRN rechaza strings inválidos con errores específicos
- [ ] Validar operaciones: parse, to_string, parent, is_child_of
- [ ] Testear operación `hierarchy()` retorna jerarquía completa
- [ ] Validar que HrnMetadata se almacena correctamente
- [ ] Testear HrnResolver con LRU cache
- [ ] Verificar TTL del cache funciona
- [ ] Testear búsqueda con patrones (wildcards)
- [ ] Validar cache invalidation manual
- [ ] Testear edge cases (HRNs complejos, caracteres especiales)
- [ ] Verificar que cache hit/miss metrics se registran

**Tests de Integración Requeridos**:
- [ ] HrnResolver funciona con base de datos de metadata
- [ ] Cache LRU almacena y recupera entradas correctamente
- [ ] Resolución de metadata async funciona
- [ ] Performance del cache bajo load (hit rate > 80%)
- [ ] Integración con audit events exitosa
- [ ] Tests comprensivos con 100% coverage
- [ ] Performance benchmarks passing
- [ ] Integración con storage backend

**Comandos de Verificación**:
```bash
# Testear parser HRN
cargo test -p hodei-audit-types hrn_parsing

# Testear operaciones HRN
cargo test -p hodei-audit-types hrn_operations

# Testear cache HRN
cargo test -p hodei-audit-types hrn_cache

# Testear búsqueda con patrones
cargo test -p hodei-audit-types hrn_pattern_search

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
- [ ] LRU cache con hit rate > 80%
- [ ] Coverage >= 95% en módulo HRN
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Código en `hodei-audit-service/src/hrn/`
- ✅ Tests unitarios en `tests/hrn/`
- ✅ Performance benchmarks
- ✅ Documentación de uso
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.3: Event Enrichment Pipeline

**Como** Ingeniero de Datos  
**Quiero** un pipeline de enriquecimiento de eventos  
**Para** que los eventos de auditoría tengan **metadata adicional** y contexto completo

#### Criterios de Aceptación

- [ ] EventEnricher implementado
- [ ] Enriquecimiento con HRN metadata
- [ ] Geo-location (MaxMindDB) para IPs
- [ ] User context (desde user service)
- [ ] Calculated fields (processed_at, enriched)
- [ ] Async enrichment (no bloquea ingestión)
- [ ] Error handling graceful (enrichment optional)

#### Tareas Técnicas

1. Implementar `EventEnricher` struct
2. Integrar con HRN resolver para metadata
3. Integrar con MaxMindDB para geo-location
4. Integrar con user service para contexto
5. Añadir calculated fields
6. Configurar como middleware (no bloquea)
7. Implementar fallbacks para enrichment failures
8. Métricas de enrichment rate

**Pipeline de Enriquecimiento**:
```rust
impl EventEnricher {
    pub async fn enrich(&self, mut event: AuditEvent) -> Result<AuditEvent> {
        // 1. HRN metadata
        if let Ok(metadata) = self.hrn_resolver.resolve(&event.hrn).await {
            event.metadata["hrn_display_name"] = serde_json::to_value(&metadata.display_name)?;
            event.metadata["hrn_tags"] = serde_json::to_value(&metadata.tags)?;
        }
        
        // 2. Geo-location
        if let (Some(ip), Some(geo_db)) = (
            event.metadata.get("client_ip"),
            &self.geoip_db
        ) {
            if let Ok(location) = geo_db.lookup(ip.as_str()?) {
                event.metadata["geo_location"] = serde_json::to_value(location)?;
            }
        }
        
        // 3. User context
        if let (Ok(user_id), Some(user_client)) = (
            parse_user_id(&event.user_id),
            &self.user_context_client
        ) {
            if let Ok(user_context) = user_client.get_user_context(user_id).await {
                event.metadata["user_context"] = serde_json::to_value(user_context)?;
            }
        }
        
        // 4. Calculated fields
        event.processed_at = Some(Utc::now());
        event.enriched = true;
        
        Ok(event)
    }
}
```

**Fuentes de Enriquecimiento**:
1. **HRN Metadata**: Display name, description, tags
2. **GeoIP**: Country, city, region from IP
3. **User Service**: User roles, department, manager
4. **Calculated**: Processing time, enrichment status

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que EventEnricher se inicializa correctamente
- [ ] Testear enriquecimiento con HRN metadata
- [ ] Testear enriquecimiento con GeoIP (MaxMindDB)
- [ ] Testear enriquecimiento con User context
- [ ] Validar que calculated fields se añaden correctamente
- [ ] Testear fallbacks para enrichment failures
- [ ] Validar que enrichment no bloquea ingestión
- [ ] Testear error handling graceful
- [ ] Verificar que métricas se registran correctamente
- [ ] Testear pipeline async completo

**Tests de Integración Requeridos**:
- [ ] Pipeline de enriquecimiento end-to-end funcionando
- [ ] Integración con HRN resolver funciona
- [ ] MaxMindDB integration funciona
- [ ] User service integration funciona
- [ ] Enrichment rate > 95% success bajo load
- [ ] No impact en latencia de ingestión
- [ ] Eventos enriched se almacenan correctamente
- [ ] Tests de performance con 10K+ events/sec
- [ ] Fallback behavior cuando fuentes fallan

**Comandos de Verificación**:
```bash
# Testear EventEnricher
cargo test -p hodei-audit-service event_enrichment

# Testear enrichment rate
cargo test -p hodei-audit-service enrichment_rate -- --nocapture

# Testear integración completa
cargo test -p hodei-audit-service enrichment_integration

# Verificar métricas
curl http://localhost:9090/metrics | grep enrichment

# Load test enrichment
k6 run scripts/load-test-enrichment.js
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Enrichment rate >= 95%
- [ ] No impact en latencia de ingestión
- [ ] Pipeline async funcionando
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Pipeline implementado y testeado
- ✅ Enrichment rate > 95% success
- ✅ No impact en latencia de ingestión
- ✅ Documentación de campos enriquecidos
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.4: Query Engine Avanzado

**Como** Usuario Final  
**Quiero** un motor de consultas potente y flexible  
**Para** poder **buscar y filtrar** eventos de auditoría de forma **eficiente**

#### Criterios de Aceptación

- [ ] QueryEngine implementado con filtros
- [ ] Filtros: tenant_id, hrn, user_id, time_range, action, outcome
- [ ] Paginación eficiente (cursor-based)
- [ ] Sorting por timestamp, hrn, user_id
- [ ] Limit configurable
- [ ] Query planning y optimization
- [ ] Response time < 100ms (p95)

#### Tareas Técnicas

1. Implementar struct `AuditQuery` con filtros
2. Implementar `QueryEngine` con execute()
3. Traducir queries a SQL optimizado
4. Implementar cursor-based pagination
5. Añadir sorting y limiting
6. Optimizar índices (tenant_id, hrn, timestamp)
7. Implementar query cache (opcional)
8. Métricas de query performance

**Filtros de Query**:
```rust
pub struct AuditQuery {
    pub tenant_id: Option<TenantId>,
    pub hrn: Option<Hrn>,
    pub user_id: Option<UserId>,
    pub action: Option<String>,    // LIKE search
    pub outcome: Option<Outcome>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub limit: usize,              // Max 1000
    pub cursor: Option<String>,    // Pagination
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}
```

**Query Optimization**:
- Índices en (tenant_id, hrn, timestamp)
- Partition pruning por fecha
- Predicate pushdown
- Limit early termination
- Streaming para large results

**Ejemplos de Query**:
```rust
// Query por HRN
let query = AuditQuery {
    hrn: Some(hrn.parse()?),
    limit: 100,
    ..Default::default()
};

// Query por usuario y rango de tiempo
let query = AuditQuery {
    user_id: Some("user-123".to_string()),
    start_time: Some(Utc::now() - Duration::days(7)),
    end_time: Some(Utc::now()),
    limit: 1000,
    sort_by: Some(SortField::Timestamp),
    sort_order: Some(SortOrder::Desc),
};
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que AuditQuery se construye correctamente
- [ ] Testear filtros individuales: tenant_id, hrn, user_id
- [ ] Testear filtros de time_range (start_time, end_time)
- [ ] Testear filtros de action y outcome
- [ ] Validar cursor-based pagination
- [ ] Testear sorting por timestamp, hrn, user_id
- [ ] Verificar que limit se aplica correctamente
- [ ] Testear query optimization (índices, partition pruning)
- [ ] Validar que query cache funciona (si configurado)
- [ ] Testear traducción a SQL optimizado

**Tests de Integración Requeridos**:
- [ ] Query engine ejecuta todos los filtros correctamente
- [ ] Performance < 100ms p95 en queries reales
- [ ] Paginación funciona end-to-end
- [ ] Sorting funciona correctamente
- [ ] Query optimization reduce latencia significativamente
- [ ] Tests de query comprensivos passing
- [ ] Query con múltiples filtros simultáneos
- [ ] Queries de edge cases (límites, fechas extremas)
- [ ] Streaming para large results funciona

**Comandos de Verificación**:
```bash
# Testear QueryEngine
cargo test -p hodei-audit-service query_engine

# Testear performance de queries
cargo test -p hodei-audit-service query_performance -- --nocapture

# Testear paginación
cargo test -p hodei-audit-service query_pagination

# Testear filtros
cargo test -p hodei-audit-service query_filters

# Testear integration
cargo test -p hodei-audit-service query_integration

# Benchmark queries
cargo bench -p hodei-audit-service query_bench
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Performance < 100ms p95
- [ ] Paginación funcionando al 100%
- [ ] Todos los filtros implementados
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Query engine implementando todos los filtros
- ✅ Performance < 100ms p95
- ✅ Paginación funcionando
- ✅ Tests de query comprensivos
- ✅ Documentación de syntax
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.5: Storage Backend (Tiered)

**Como** DevOps Engineer  
**Quiero** un sistema de almacenamiento tiered (Hot/Warm/Cold)  
**Para** optimizar **costo y performance** según la edad de los datos

#### Criterios de Aceptación

- [ ] ClickHouse storage (Hot tier, 0-7 días)
- [ ] S3/MinIO storage (Warm tier, 7-365 días)
- [ ] Glacier storage (Cold tier, 1-7 años)
- [ ] Lifecycle policies automáticas
- [ ] Query unificado across tiers
- [ ] Partitioning strategy optimizada
- [ ] Retention policies por tipo de tenant

#### Tareas Técnicas

1. Implementar `StorageBackend` trait
2. Implementar `ClickHouseStorage` (Hot)
3. Implementar `S3Storage` (Warm)
4. Implementar `GlacierStorage` (Cold)
5. Implementar `TieredStorage` orchestrator
6. Configurar ClickHouse schema con índices
7. Configurar S3 bucket con lifecycle
8. Implementar query planning across tiers

**Arquitectura de Storage**:
```rust
pub enum StorageTier {
    Hot(ClickHouseStorage),    // 0-7 días, <10ms query
    Warm(S3Storage),           // 7-365 días, <500ms query
    Cold(GlacierStorage),      // 1-7 años, minutos
}

pub struct TieredStorage {
    hot: Arc<ClickHouseStorage>,
    warm: Arc<S3Storage>,
    cold: Arc<GlacierStorage>,
    lifecycle_policy: LifecyclePolicy,
    partition_strategy: PartitionStrategy,
}
```

**ClickHouse Schema**:
```sql
CREATE TABLE audit_events (
    event_id String,
    tenant_id String,
    hrn String,
    user_id String,
    action String,
    timestamp DateTime64(3),
    metadata_json String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (tenant_id, hrn, timestamp)
TTL timestamp + INTERVAL 7 DAY;  -- Auto-delete después de 7 días
```

**Lifecycle Policy**:
- Hot → Warm: 7 días
- Warm → Cold: 1 año
- Cold → Delete: 7 años

**Query Flow**:
1. Query entra al TieredStorage
2. Query plan determina qué tiers tocar
3. Parallel query en tiers relevantes
4. Merge results y sort
5. Apply limit y paginación

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que StorageBackend trait se implementa correctamente
- [ ] Testear ClickHouseStorage (Hot tier)
- [ ] Testear S3Storage (Warm tier)
- [ ] Testear GlacierStorage (Cold tier)
- [ ] Validar TieredStorage orchestrator
- [ ] Testear que lifecycle policies se aplican
- [ ] Verificar partitioning strategy
- [ ] Testear query planning across tiers
- [ ] Validar retention policies

**Tests de Integración Requeridos**:
- [ ] Storage tiered funcionando end-to-end
- [ ] Lifecycle policies automáticas activas
- [ ] Query unificado across tiers working
- [ ] ClickHouse Hot tier < 10ms query latency
- [ ] S3 Warm tier < 500ms query latency
- [ ] Glacier Cold tier < minutos query latency
- [ ] Migration entre tiers automática
- [ ] Performance benchmarks passing
- [ ] Cost estimation tool funcional

**Comandos de Verificación**:
```bash
# Testear storage tiers
cargo test -p hodei-audit-service storage_tiered

# Testear ClickHouse
cargo test -p hodei-audit-service clickhouse_storage

# Testear lifecycle
cargo test -p hodei-audit-service lifecycle_policies

# Testear query across tiers
cargo test -p hodei-audit-service query_tiered

# Benchmarks
cargo bench -p hodei-audit-service storage_bench
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Storage tiered funcionando
- [ ] Lifecycle policies automáticas
- [ ] Query unificado working
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Storage tiered funcionando
- ✅ Lifecycle policies automáticas
- ✅ Query unificado working
- ✅ Performance benchmarks documentados
- ✅ Cost estimation tool
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.6: ClickHouse Integration

**Como** Desarrollador  
**Quiero** integración robusta con ClickHouse  
**Para** almacenar y consultar eventos de auditoría con **bajo latency**

#### Criterios de Aceptación

- [ ] ClickHouse client configurado
- [ ] Schema de tablas optimizado
- [ ] Índices apropiados (tenant_id, hrn, timestamp)
- [ ] Batch inserts optimizados
- [ ] Connection pooling
- [ ] Retry policies para failures
- [ ] Monitoring de performance

#### Tareas Técnicas

1. Configurar `clickhouse-rs` client
2. Crear schema con PARTITION BY y ORDER BY
3. Optimizar índices para queries comunes
4. Implementar batch inserts (1000 events/batch)
5. Configurar connection pool (10-50 connections)
6. Implementar retry con backoff
7. Métricas: insert latency, query latency
8. Backup/restore procedures

**Schema Optimizado**:
```sql
-- Tabla principal con particionamiento por mes
CREATE TABLE audit_events (
    event_id String,
    tenant_id String,
    hrn String,
    user_id String,
    action String,
    path String,
    method String,
    status_code UInt16,
    outcome String,
    latency_ms UInt64,
    metadata_json String,
    timestamp DateTime64(3),
    processed_at DateTime64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (tenant_id, timestamp, hrn)
SETTINGS index_granularity = 8192;

-- Índice bloom para tenant_id
CREATE INDEX idx_tenant ON audit_events (tenant_id) TYPE bloom_filter GRANULARITY 1;

-- TTL: 7 días en Hot tier
ALTER TABLE audit_events MODIFY TTL timestamp + INTERVAL 7 DAY;
```

**Performance Tuning**:
- Batch size: 1000-10000 events
- Insert concurrency: 4-8 parallel
- Max memory: 1-2GB per query
- Query timeout: 30 seconds

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar configuración de clickhouse-rs client
- [ ] Testear que schema se crea correctamente
- [ ] Verificar que índices se crean apropiadamente
- [ ] Testear batch inserts (1000 events/batch)
- [ ] Validar connection pool configuration
- [ ] Testear retry policies para failures
- [ ] Verificar metrics collection (insert latency, query latency)
- [ ] Testear backup/restore procedures

**Tests de Integración Requeridos**:
- [ ] ClickHouse cluster healthy y accesible
- [ ] Insert throughput > 10K/sec
- [ ] Query latency < 10ms p95
- [ ] Connection pool working correctamente
- [ ] Monitoring dashboard funcional
- [ ] Schema optimizado funcionando
- [ ] Performance tuning aplicando correctamente
- [ ] Batch inserts con alta concurrencia
- [ ] Query timeout manejado correctamente

**Comandos de Verificación**:
```bash
# Testear ClickHouse client
cargo test -p hodei-audit-service clickhouse_client

# Testear schema y tablas
cargo test -p hodei-audit-service clickhouse_schema

# Testear inserts
cargo test -p hodei-audit-service clickhouse_inserts

# Testear queries
cargo test -p hodei-audit-service clickhouse_queries

# Testear connection pool
cargo test -p hodei-audit-service clickhouse_pool

# Benchmarks
cargo bench -p hodei-audit-service clickhouse_bench
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] ClickHouse cluster healthy
- [ ] Insert throughput > 10K/sec
- [ ] Query latency < 10ms p95
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ ClickHouse cluster healthy
- ✅ Insert throughput > 10K/sec
- ✅ Query latency < 10ms p95
- ✅ Connection pool working
- ✅ Monitoring dashboard
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 2.7: S3/MinIO Integration

**Como** DevOps Engineer  
**Quiero** integración con S3/MinIO para Warm/Cold tier  
**Para** almacenar datos históricos de forma **económica**

#### Criterios de Aceptación

- [ ] S3 client configurado (aws-sdk o minio)
- [ ] Parquet format para optimización
- [ ] Partitioning por fecha y tenant
- [ ] Compression (gzip/lz4)
- [ ] Lifecycle policies
- [ ] Athena-style queries
- [ ] Cost optimization

#### Tareas Técnicas

1. Configurar `aws-sdk-s3` o `minio` client
2. Crear bucket con versioning
3. Implementar Parquet writer
4. Configurar partitioning strategy
5. Implementar compression
6. Configurar lifecycle rules
7. Integrar con Athena/Trino para queries
8. Cost monitoring y alerts

**Partitioning Strategy**:
```
warm/tenant_id=tenant-123/year=2024/month=01/day=15/
  ├── audit_events_2024-01-15_00.parquet
  ├── audit_events_2024-01-15_01.parquet
  └── _meta/partition.json
```

**Parquet Schema**:
```rust
// Optimizado para analytics
{
    "event_id": "string",
    "tenant_id": "string",
    "hrn": "string",
    "user_id": "string",
    "action": "string",
    "timestamp": "timestamp",
    "outcome": "string"
}
```

**Lifecycle Rules**:
- Tiering: S3 Standard → IA → Glacier
- Warm retention: 365 días
- Cold retention: 7 años
- Auto-delete después de retention

**Athena/Trino Query**:
```sql
SELECT hrn, COUNT(*) as event_count
FROM audit_events
WHERE tenant_id = 'tenant-123'
  AND timestamp BETWEEN '2024-01-01' AND '2024-01-31'
GROUP BY hrn
ORDER BY event_count DESC
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar configuración de aws-sdk-s3 o minio client
- [ ] Testear creación de bucket con versioning
- [ ] Verificar que Parquet writer funciona
- [ ] Validar partitioning strategy
- [ ] Testear compression (gzip/lz4)
- [ ] Verificar que lifecycle rules se configuran
- [ ] Testear integración con Athena/Trino
- [ ] Validar cost monitoring y alerts
- [ ] Verificar Parquet schema correcto

**Tests de Integración Requeridos**:
- [ ] S3/MinIO bucket configured y accessible
- [ ] Parquet files being written correctamente
- [ ] Lifecycle policies active y funcionando
- [ ] Athena/Trino queries working correctamente
- [ ] Cost under control (alerts configurados)
- [ ] Partitioning strategy optimizando queries
- [ ] Compression reduciendo storage cost
- [ ] Versioning funcionando para recovery
- [ ] Query performance < 500ms p95 (warm tier)

**Comandos de Verificación**:
```bash
# Testear S3 client
cargo test -p hodei-audit-service s3_client

# Testear Parquet writer
cargo test -p hodei-audit-service parquet_writer

# Testear lifecycle policies
cargo test -p hodei-audit-service lifecycle_rules

# Testear Athena queries
cargo test -p hodei-audit-service athena_integration

# Verificar cost monitoring
./scripts/check-s3-costs.sh

# Testear bucket y files
aws s3 ls --recursive s3://hodei-audit-warm/
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] S3/MinIO bucket configured
- [ ] Parquet files being written
- [ ] Lifecycle policies active
- [ ] Cost under control
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ S3/MinIO bucket configured
- ✅ Parquet files being written
- ✅ Lifecycle policies active
- ✅ Athena queries working
- ✅ Cost under control
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## 📊 Métricas de Éxito

| Métrica | Objetivo | Medición |
|---------|----------|----------|
| **Ingest Throughput** | 10K+ events/sec | Load testing con K6 |
| **Query Latency (Hot)** | < 10ms p95 | ClickHouse query timing |
| **Query Latency (Warm)** | < 500ms p95 | S3/Parquet query timing |
| **Storage Cost** | < $0.023/GB-month | AWS pricing calculator |
| **Enrichment Success** | > 95% | Metrics counter |
| **HRN Cache Hit Rate** | > 80% | LRU cache metrics |
| **Availability** | 99.9% | Uptime monitoring |

---

## 🚀 Entregables

1. **Código**:
   - Hodei Audit Service (CAP) corriendo
   - Sistema HRN completo
   - Event enrichment pipeline
   - Query engine optimizado
   - Storage tiered (Hot/Warm/Cold)

2. **Infraestructura**:
   - ClickHouse cluster configurado
   - S3/MinIO bucket con lifecycle
   - Storage schemas y migrations
   - Backup/restore procedures

3. **Documentación**:
   - API documentation (gRPC)
   - Storage architecture doc
   - Query examples
   - Performance tuning guide

---

## 🔗 Dependencias

**Bloquea**: 
- [Épica 1: Fundación y Arquitectura Base](epic-01-fundacion-y-arquitectura.md)

**Bloqueada por**: Ninguna

---

## 📝 Notas de Implementación

### Decisiones Arquitectónicas (ADR)

1. **ADR-008**: ClickHouse para Hot tier (OLAP optimized)
2. **ADR-009**: S3 + Parquet para Warm tier (analytics)
3. **ADR-010**: Athena/Trino para query warm tier
4. **ADR-011**: LRU cache para HRN (TTL configurable)
5. **ADR-012**: Async enrichment (non-blocking)
6. **ADR-013**: Cursor-based pagination

### Configuración de Performance

**ClickHouse**:
```xml
<max_connections>256</max_connections>
<max_concurrent_queries>100</max_concurrent_queries>
<max_memory_usage>10000000000</max_memory_usage>
<max_bytes_before_external_group_by>5000000000</max_bytes_before_external_group_by>
```

**Connection Pool**:
```rust
pub struct ConnectionPool {
    max_size: 20,
    min_size: 5,
    acquire_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(1800),
}
```

### Testing Strategy

1. **Unit Tests**: HRN, enrichment, query engine
2. **Integration Tests**: ClickHouse, S3/MinIO
3. **Load Tests**: K6 para ingestión y queries
4. **Chaos Tests**: Fallos de red, timeouts

---

## ⏭️ Siguiente Épica

[Épica 3: SDK Middleware y Integración](epic-03-sdk-middleware-y-integracion.md)

---

**Versión**: 1.1  
**Fecha**: 2025-11-06  
**Estado**: En Desarrollo (PARCIAL - ~35% completo)  
**Épica Padre**: Hodei Audit Service

---

## 📊 Estado Actual de Implementación (Actualizado: 2025-11-06)

### ✅ COMPLETADO (35%)

**Historia 2.1 - Servicio gRPC Principal:**
- ✅ `main.rs` implementado con estructura base
- ✅ `AuditControlService` (puerto 50052) - estructura base
- ✅ `AuditQueryService` (puerto 50053) - estructura base  
- ✅ Health checks básico implementado
- ✅ Logging estructurado con tracing
- ✅ Graceful shutdown configurado
- ✅ Puerto 50051 para Vector API

**Historia 2.2 - Sistema HRN:**
- ✅ `Hrn` struct con validaciones completas
- ✅ Parser HRN (parse, to_string)
- ✅ Operaciones: parent(), is_child_of()
- ✅ `HrnMetadata` struct
- ✅ Trait `HrnResolver` definido
- ✅ Tests unitarios básicos

**Infraestructura:**
- ✅ Estructura de workspace con 4 crates
- ✅ Protocol buffers definidos
- ✅ Tests de integración básicos
- ✅ Dependencias base configuradas

### ⚠️ EN PROGRESO / PARCIAL

**Historia 2.1 - Servicio gRPC:**
- ⚠️ `AuditControlService` - TODOs en persistencia y Vector integration
- ⚠️ `AuditQueryService` - TODOs en query engine real
- ⚠️ `AuditCryptoService` - solo módulo crypto, falta servidor gRPC

### ❌ PENDIENTE (65%)

**Historia 2.2 - Sistema HRN:**
- ❌ HrnResolver con LRU cache (TTL configurable)
- ❌ Búsqueda con patrones (wildcards)
- ❌ Resolución async con base de datos
- ❌ Cache invalidation strategy

**Historia 2.3 - Event Enrichment Pipeline:**
- ❌ EventEnricher struct
- ❌ Enriquecimiento con HRN metadata
- ❌ Geo-location (MaxMindDB)
- ❌ User context integration
- ❌ Calculated fields
- ❌ Pipeline async no-bloqueante

**Historia 2.4 - Query Engine:**
- ❌ AuditQuery struct con filtros
- ❌ QueryEngine con execute()
- ❌ SQL optimization
- ❌ Cursor-based pagination
- ❌ Sorting y limiting
- ❌ Performance optimization

**Historia 2.5 - Storage Backend Tiered:**
- ❌ ClickHouseStorage implementation
- ❌ S3Storage implementation
- ❌ GlacierStorage implementation
- ❌ TieredStorage orchestrator
- ❌ Lifecycle policies
- ❌ Partitioning strategy

**Historia 2.6 - ClickHouse Integration:**
- ❌ clickhouse-rs client
- ❌ Schema creation
- ❌ Batch inserts optimizados
- ❌ Connection pooling
- ❌ Retry policies
- ❌ Performance monitoring

**Historia 2.7 - S3/MinIO Integration:**
- ❌ aws-sdk-s3 client
- ❌ Parquet writer
- ❌ Partitioning strategy
- ❌ Compression
- ❌ Lifecycle rules
- ❌ Athena/Trino integration

### 🚨 PROBLEMAS IDENTIFICADOS

1. **Backend de Storage Inexistente**: Los servicios retornan datos vacíos por falta de implementación
2. **Event Enrichment Faltante**: No hay pipeline de enriquecimiento de eventos
3. **Query Engine No Implementado**: Las consultas no acceden a datos reales
4. **Cache HRN Faltante**: Sin optimización para resolución de HRN
5. **Tests de Integración**: Solo tests básicos, faltan tests comprensivos
6. **Performance**: Sin benchmarks ni métricas reales

### 📋 PRÓXIMOS PASOS RECOMENDADOS

**Prioridad ALTA:**
1. Completar implementación de storage backend (ClickHouse)
2. Implementar EventEnricher pipeline
3. Conectar AuditControlService con storage real

**Prioridad MEDIA:**
4. Implementar QueryEngine con filtros
5. Completar HrnResolver con LRU cache
6. Implementar AuditCryptoService gRPC

**Prioridad BAJA:**
7. S3/MinIO integration
8. Performance tuning
9. Métricas y monitoring avanzado
