# Épica 4: Storage Backend y ClickHouse

## 📋 Resumen Ejecutivo

**Objetivo**: Implementar el backend de almacenamiento con ClickHouse como Hot Tier, S3/MinIO como Warm/Cold Tier, y lifecycle policies automáticas para optimizar costo y performance.

**Duración**: 3-4 semanas

---

## Historias Principales

### Historia 4.1: ClickHouse Schema y Optimización

**Objetivo**: Crear schema optimizado con índices y particionamiento.

**Criterios de Aceptación**:
- [ ] Schema creado con particionamiento mensual
- [ ] Índices en tenant_id, hrn, timestamp
- [ ] TTL configurado (7 días para Hot)
- [ ] Connection pooling implementado
- [ ] Batch inserts optimizados

**Tareas**:
1. Crear DDL scripts con MergeTree engine
2. Configurar índices bloom para tenant_id
3. Implementar ClickHouseStorage trait
4. Batch inserts con 1000-10000 events/batch
5. Connection pool con 10-50 connections
6. Tests de performance

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar schema DDL se crea correctamente
- [ ] Testear particionamiento mensual funciona
- [ ] Verificar índices bloom para tenant_id
- [ ] Testear ClickHouseStorage trait implementation
- [ ] Validar batch inserts con 1000-10000 events/batch
- [ ] Testear connection pool (10-50 connections)
- [ ] Verificar TTL configurado (7 días para Hot)
- [ ] Testear optimizaciones de performance

**Tests de Integración Requeridos**:
- [ ] Schema creado y funcionando
- [ ] Índices en tenant_id, hrn, timestamp
- [ ] TTL configurado correctamente
- [ ] Connection pooling implementado y estable
- [ ] Batch inserts optimizados funcionando
- [ ] Insert Throughput >= 10K events/sec
- [ ] Query Latency < 10ms (Hot tier)
- [ ] Tests de performance passing

**Comandos de Verificación**:
```bash
# Testear schema creation
cargo test -p hodei-audit-service clickhouse_schema

# Testear batch inserts
cargo test -p hodei-audit-service clickhouse_batch_inserts

# Testear connection pool
cargo test -p hodei-audit-service clickhouse_pool

# Testear performance
cargo bench -p hodei-audit-service clickhouse_performance

# Verificar índices
clickhouse-client --query="SHOW INDEX FROM audit_events"

# Verificar TTL
clickhouse-client --query="SELECT * FROM system.tables WHERE name='audit_events'"
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Schema creado con optimizaciones
- [ ] Query Latency < 10ms
- [ ] Insert Throughput >= 10K events/sec
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Schema creado con particionamiento mensual
- ✅ Índices en tenant_id, hrn, timestamp
- ✅ TTL configurado (7 días para Hot)
- ✅ Connection pooling implementado
- ✅ Batch inserts optimizados
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 4.2: Tiered Storage Orchestrator

**Objetivo**: Unificar acceso a múltiples tiers de almacenamiento.

**Criterios de Aceptación**:
- [ ] TieredStorage orchestrator
- [ ] Query planning automático
- [ ] Cross-tier query execution
- [ ] Lifecycle policies automáticas
- [ ] Cost estimation

**Tareas**:
1. Implementar StorageBackend trait
2. Query planner que determina tiers
3. Parallel execution across tiers
4. Lifecycle management
5. Cost optimizer

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar StorageBackend trait se implementa correctamente
- [ ] Testear TieredStorage orchestrator
- [ ] Verificar query planner determina tiers automáticamente
- [ ] Testear cross-tier query execution
- [ ] Validar lifecycle policies automáticas
- [ ] Testear cost estimation
- [ ] Verificar parallel execution across tiers
- [ ] Testear que lifecycle management funciona

**Tests de Integración Requeridos**:
- [ ] TieredStorage orchestrator funcionando
- [ ] Query planning automático operativo
- [ ] Cross-tier query execution working
- [ ] Lifecycle policies automáticas activas
- [ ] Cost estimation accurate
- [ ] Parallel queries across tiers optimizadas
- [ ] Tests de performance passing
- [ ] Migration entre tiers automática

**Comandos de Verificación**:
```bash
# Testear tiered storage
cargo test -p hodei-audit-service tiered_storage

# Testear query planner
cargo test -p hodei-audit-service query_planner

# Testear lifecycle policies
cargo test -p hodei-audit-service lifecycle_policies

# Testear cross-tier queries
cargo test -p hodei-audit-service cross_tier_queries

# Benchmarks
cargo bench -p hodei-audit-service tiered_performance

# Verificar cost estimation
./scripts/validate-cost-estimation.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] TieredStorage orchestrator funcionando
- [ ] Query planning automático operativo
- [ ] Cross-tier queries funcionando
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ TieredStorage orchestrator
- ✅ Query planning automático
- ✅ Cross-tier query execution
- ✅ Lifecycle policies automáticas
- ✅ Cost estimation
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 4.3: S3/MinIO Integration (Warm/Cold)

**Objetivo**: Implementar storage económico para datos históricos.

**Criterios de Aceptación**:
- [ ] S3/MinIO client configurado
- [ ] Parquet format para analytics
- [ ] Partitioning por fecha/tenant
- [ ] Lifecycle rules S3 Standard → IA → Glacier
- [ ] Athena/Trino queries

**Tareas**:
1. Configurar S3 client
2. Implementar Parquet writer
3. Partitioning strategy
4. Lifecycle policies
5. Query integration

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar S3/MinIO client configurado correctamente
- [ ] Testear Parquet format para analytics
- [ ] Verificar partitioning por fecha/tenant
- [ ] Testear lifecycle rules S3 Standard → IA → Glacier
- [ ] Validar que Athena/Trino queries funcionan
- [ ] Testear Parquet writer
- [ ] Verificar query integration
- [ ] Testear S3 client operations

**Tests de Integración Requeridos**:
- [ ] S3/MinIO client configurado y accesible
- [ ] Parquet format optimizado para analytics
- [ ] Partitioning por fecha/tenant funcionando
- [ ] Lifecycle rules activas y automáticas
- [ ] Athena/Trino queries working
- [ ] Query Latency < 500ms (Warm tier)
- [ ] Storage Cost < $0.023/GB-month
- [ ] Tests de performance passing

**Comandos de Verificación**:
```bash
# Testear S3 client
cargo test -p hodei-audit-service s3_minio_client

# Testear Parquet writer
cargo test -p hodei-audit-service parquet_writer

# Testear lifecycle policies
cargo test -p hodei-audit-service s3_lifecycle_policies

# Testear Athena/Trino queries
cargo test -p hodei-audit-service athena_trino_queries

# Benchmarks
cargo bench -p hodei-audit-service s3_performance

# Verificar cost
aws s3 ls --recursive s3://hodei-audit-warm/ --summarize
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] S3/MinIO client configurado
- [ ] Parquet format optimizado
- [ ] Query Latency < 500ms
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ S3/MinIO client configurado
- ✅ Parquet format para analytics
- ✅ Partitioning por fecha/tenant
- ✅ Lifecycle rules S3 Standard → IA → Glacier
- ✅ Athena/Trino queries
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## Métricas

| Métrica | Objetivo |
|---------|----------|
| Query Latency (Hot) | < 10ms |
| Query Latency (Warm) | < 500ms |
| Insert Throughput | 10K+ events/sec |
| Storage Cost | < $0.023/GB-month |

---

## ⏭️ Siguiente Épica

[Épica 5: Multi-Tenancy y Seguridad](epic-05-multi-tenancy-y-seguridad.md)
