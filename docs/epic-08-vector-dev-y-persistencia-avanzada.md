# Épica 8: Vector.dev y Persistencia Avanzada

## 📋 Resumen Ejecutivo

**Objetivo**: Integrar Vector.dev para ingesta unificada, fan-out automático, disk buffer persistente y multi-sink routing.

**Duración**: 3-4 semanas

---

## Historias Principales

### Historia 8.1: Vector.dev Setup y Configuración

**Objetivo**: Configurar Vector.dev como capa de ingesta unificada.

**Criterios de Aceptación**:
- [ ] Vector DaemonSet en Kubernetes
- [ ] gRPC source configurado
- [ ] vector.toml con sinks ClickHouse + S3
- [ ] Disk buffer persistente (1-5GB)
- [ ] Health checks y monitoring

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar Vector DaemonSet en Kubernetes
- [ ] Testear gRPC source configurado
- [ ] Verificar vector.toml con sinks ClickHouse + S3
- [ ] Testear disk buffer persistente (1-5GB)
- [ ] Validar health checks y monitoring
- [ ] Testear Vector configuration
- [ ] Verificar buffer persistence
- [ ] Testear vector startup/shutdown
- [ ] Validar metrics endpoint

**Tests de Integración Requeridos**:
- [ ] Vector DaemonSet running en Kubernetes
- [ ] gRPC source escuchando correctamente
- [ ] vector.toml configurado con sinks
- [ ] Disk buffer persistente working
- [ ] Health checks operativos
- [ ] Vector sending/receiving events
- [ ] Buffer size within limits
- [ ] Sinks healthy y receiving data
- [ ] Monitoring dashboard working
- [ ] Zero data loss verified

**Comandos de Verificación**:
```bash
# Verificar Vector DaemonSet
kubectl get daemonset vector
kubectl get pods -l app=vector

# Verificar gRPC source
grpcurl -plaintext localhost:50051 list

# Verificar sinks
vector test /etc/vector/vector.toml

# Verificar buffer
curl http://localhost:9598/metrics | grep buffer

# Health check
curl http://localhost:9598/health

# Verificar configuration
vector validate /etc/vector/vector.toml

# Logs check
kubectl logs -l app=vector | tail -f
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Vector DaemonSet running
- [ ] gRPC source working
- [ ] Disk buffer persistente active
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Vector DaemonSet en Kubernetes
- ✅ gRPC source configurado
- ✅ vector.toml con sinks ClickHouse + S3
- ✅ Disk buffer persistente (1-5GB)
- ✅ Health checks y monitoring
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 8.2: Contrato CAP → Vector

**Objetivo**: Definir contrato simple gRPC entre CAP y Vector.

**Criterios de Aceptación**:
- [ ] vector_api.proto definido
- [ ] EventBatchRequest/Response
- [ ] Cliente en CAP (VectorForwarder)
- [ ] Error handling robusto
- [ ] Test de contrato

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar vector_api.proto definido correctamente
- [ ] Testear EventBatchRequest/Response structures
- [ ] Verificar Cliente en CAP (VectorForwarder)
- [ ] Testear error handling robusto
- [ ] Validar test de contrato
- [ ] Testear proto compilation
- [ ] Verificar gRPC code generation
- [ ] Testear serialization/deserialization
- [ ] Validar contract versioning

**Tests de Integración Requeridos**:
- [ ] vector_api.proto compilado y funcional
- [ ] EventBatchRequest/Response working
- [ ] Cliente VectorForwarder operativo
- [ ] Error handling robusto funcionando
- [ ] Test de contrato passing
- [ ] CAP can send to Vector
- [ ] Vector can receive from CAP
- [ ] Batch delivery working
- [ ] Error recovery tested
- [ ] End-to-end contract verified

**Comandos de Verificación**:
```bash
# Compilar proto
cargo build -p hodei-audit-proto

# Testear VectorForwarder
cargo test -p hodei-audit-service vector_forwarder

# Testear contract
cargo test -p hodei-audit-service vector_contract

# Testear error handling
cargo test -p hodei-audit-service vector_error_handling

# gRPC test
grpcurl -plaintext -d '{"events":[]}' localhost:50051 vector_api.SendEventBatch

# Verificar client
curl http://localhost:50055/health

# Test integration
./scripts/test-vector-contract.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] vector_api.proto definido
- [ ] EventBatchRequest/Response working
- [ ] Cliente VectorForwarder operativo
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ vector_api.proto definido
- ✅ EventBatchRequest/Response
- ✅ Cliente en CAP (VectorForwarder)
- ✅ Error handling robusto
- ✅ Test de contrato
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 8.3: Multi-Sink y Fan-out

**Objetivo**: Distribución automática a múltiples destinos.

**Criterios de Aceptación**:
- [ ] ClickHouse sink (hot tier)
- [ ] S3 sink (warm/cold tier)
- [ ] Blackhole sink (emergency)
- [ ] Parallel delivery
- [ ] Reintentos automáticos

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar ClickHouse sink (hot tier)
- [ ] Testear S3 sink (warm/cold tier)
- [ ] Verificar Blackhole sink (emergency)
- [ ] Testear parallel delivery
- [ ] Validar reintentos automáticos
- [ ] Testear sink configuration
- [ ] Verificar fan-out logic
- [ ] Testear delivery guarantees
- [ ] Validar sink health checks

**Tests de Integración Requeridos**:
- [ ] ClickHouse sink working (hot tier)
- [ ] S3 sink working (warm/cold tier)
- [ ] Blackhole sink operativa (emergency)
- [ ] Parallel delivery funcionando
- [ ] Reintentos automáticos activos
- [ ] Events delivered to all sinks
- [ ] Fan-out working correctly
- [ ] No data loss verified
- [ ] Sink failover tested
- [ ] Load balancing across sinks
- [ ] Error recovery working

**Comandos de Verificación**:
```bash
# Testear sinks
cargo test -p hodei-audit-service vector_sinks

# Testear fan-out
cargo test -p hodei-audit-service vector_fanout

# Testear parallel delivery
cargo test -p hodei-audit-service parallel_delivery

# Verificar sinks health
curl http://localhost:9598/metrics | grep sink

# ClickHouse sink test
clickhouse-client --query="SELECT COUNT(*) FROM audit_events"

# S3 sink test
aws s3 ls s3://hodei-audit-warm/

# Load test
k6 run scripts/load-test-multisink.js
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] ClickHouse sink working (hot tier)
- [ ] S3 sink working (warm/cold tier)
- [ ] Parallel delivery funcionando
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ ClickHouse sink (hot tier)
- ✅ S3 sink (warm/cold tier)
- ✅ Blackhole sink (emergency)
- ✅ Parallel delivery
- ✅ Reintentos automáticos
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 8.4: Vector Metrics y Observabilidad

**Objetivo**: Monitoreo completo de Vector.dev.

**Criterios de Aceptación**:
- [ ] Prometheus metrics endpoint
- [ ] Grafana dashboard para Vector
- [ ] Alerts configurados
- [ ] Buffer size monitoring
- [ ] Delivery rate tracking

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar Prometheus metrics endpoint
- [ ] Testear Grafana dashboard para Vector
- [ ] Verificar alerts configurados
- [ ] Testear buffer size monitoring
- [ ] Validar delivery rate tracking
- [ ] Testear metrics collection
- [ ] Verificar alerting rules
- [ ] Testear dashboard queries
- [ ] Validar metric definitions

**Tests de Integración Requeridos**:
- [ ] Prometheus metrics endpoint operativo
- [ ] Grafana dashboard displaying Vector metrics
- [ ] Alerts configurados y firing correctamente
- [ ] Buffer size monitoring activo
- [ ] Delivery rate tracking working
- [ ] Metrics exported correctly
- [ ] Dashboard updating in real-time
- [ ] Alerts working under conditions
- [ ] Monitoring comprehensive
- [ ] Observability complete

**Comandos de Verificación**:
```bash
# Verificar Prometheus metrics
curl http://localhost:9598/metrics

# Testear metrics endpoint
cargo test -p hodei-audit-service vector_metrics

# Verificar Grafana dashboard
open http://localhost:3000/d/vector

# Verificar alerts
kubectl get prometheusrules

# Buffer monitoring
curl -s http://localhost:9598/metrics | grep buffer_size

# Delivery rate
curl -s http://localhost:9598/metrics | grep delivery_rate

# Dashboard validation
./scripts/validate-vector-dashboard.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Prometheus metrics endpoint operativo
- [ ] Grafana dashboard working
- [ ] Alerts configurados y funcionando
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Prometheus metrics endpoint
- ✅ Grafana dashboard para Vector
- ✅ Alerts configurados
- ✅ Buffer size monitoring
- ✅ Delivery rate tracking
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## ⏭️ Siguiente Épica

[Épica 9: Observabilidad y Métricas](epic-09-observabilidad-y-metricas.md)
