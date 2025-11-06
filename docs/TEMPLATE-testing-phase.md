# Template: FASE DE TESTING OBLIGATORIA
## Para aplicar a TODAS las historias en todas las épicas

---

## ⚠️ SECCIÓN DE TESTING (AGREGAR DESPUÉS DE "Tareas Técnicas")

### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla Fundamental**: NO continuar a la siguiente historia hasta que TODOS los tests pasen en verde ✅

#### Tests Unitarios Requeridos
- [ ] Test 1 (específico de la historia)
- [ ] Test 2 (específico de la historia)
- [ ] Test 3 (específico de la historia)
- [ ] Test N (según necesidad)

#### Tests de Integración Requeridos  
- [ ] Test de integración 1 (específico)
- [ ] Test de integración 2 (específico)
- [ ] Test de integración 3 (específico)
- [ ] Test N (según necesidad)

#### Comandos de Verificación
```bash
# Tests unitarios
cargo test --workspace --lib --bins

# Tests de integración
cargo test --workspace --test '*'

# Coverage
cargo tarpaulin --workspace --out html

# Validación específica (ejemplo)
./scripts/test-[nombre-historia].sh
```

#### Criterios de Aceptación de Tests
- [ ] 100% de tests unitarios passing (`cargo test`)
- [ ] 100% de tests de integración passing
- [ ] Coverage > 80% (o según especificación)
- [ ] Todas las validaciones pasando
- [ ] **TODOS los criterios en verde ✅**

#### Definición de Done (ACTUALIZADA)
- [ ] ✅ Funcionalidad implementada
- [ ] ✅ Tests unitarios written
- [ ] ✅ Tests integración written
- [ ] ✅ **TODOS los tests passing (100%)** ⚠️
- [ ] ✅ Code review approved
- [ ] ✅ Documentation updated

---

## INSTRUCCIONES DE APLICACIÓN

### Para cada Historia en cada Épica:
1. Buscar la sección "Tareas Técnicas"
2. Agregar inmediatamente después la sección "⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)"
3. Personalizar los tests específicos para esa historia
4. Agregar comandos de verificación relevantes
5. Actualizar "Definición de Done" para incluir testing

### Para Criterios de Aceptación de Épica:
Agregar al inicio de cada épica:

### Criterios de Aceptación (Épica)
- [ ] TODAS las historias completadas
- [ ] **TODOS los tests (unitarios + integración) passing en todas las historias**
- [ ] Coverage agregado > 80%
- [ ] Documentación completa
- [ ] Code review aprobado

---

## CHECKLIST DE APLICACIÓN

### Épica 1: Fundación y Arquitectura Base
- [ ] Historia 1.1 - Definición de Arquitectura CAP/ARP
- [ ] Historia 1.2 - Análisis CloudTrail
- [ ] Historia 1.3 - Estructura de Proyecto Rust
- [ ] Historia 1.4 - Contratos gRPC
- [ ] Historia 1.5 - Entorno de Desarrollo
- [ ] Historia 1.6 - Sistema HRN
- [ ] Historia 1.7 - CI/CD Base

### Épica 2: Core Service y HRN System
- [ ] Historia 2.1 - Servicio gRPC Principal
- [ ] Historia 2.2 - Sistema HRN Completo
- [ ] Historia 2.3 - Event Enrichment
- [ ] Historia 2.4 - Query Engine
- [ ] Historia 2.5 - Storage Backend
- [ ] Historia 2.6 - ClickHouse Integration
- [ ] Historia 2.7 - S3/MinIO Integration

### Épica 3: SDK Middleware y Integración
- [ ] Historia 3.1 - SDK API Pública
- [ ] Historia 3.2 - Middleware Axum
- [ ] Historia 3.3 - Batch Processing
- [ ] Historia 3.4 - Auto-Enriquecimiento
- [ ] Historia 3.5 - Cliente Manual
- [ ] Historia 3.6 - Integración verified-permissions
- [ ] Historia 3.7 - Testing y QA

### Épica 4: Storage Backend y ClickHouse
- [ ] Historia 4.1 - ClickHouse Schema
- [ ] Historia 4.2 - Tiered Storage
- [ ] Historia 4.3 - S3/MinIO Integration

### Épica 5: Multi-Tenancy y Seguridad
- [ ] Historia 5.1 - Tenant Isolation
- [ ] Historia 5.2 - API Key Management
- [ ] Historia 5.3 - Resource Quotas
- [ ] Historia 5.4 - Compliance y Retention

### Épica 6: Digest Criptográfico y Compliance ⚠️ CRÍTICA
- [ ] Historia 6.1 - Digest Worker
- [ ] Historia 6.2 - Key Management
- [ ] Historia 6.3 - Verificación de Integridad

### Épica 7: Alto Rendimiento y Escalabilidad
- [ ] Historia 7.1 - Batching y Connection Pooling
- [ ] Historia 7.2 - Auto-Scaling
- [ ] Historia 7.3 - Performance Tuning

### Épica 8: Vector.dev y Persistencia Avanzada
- [ ] Historia 8.1 - Vector.dev Setup
- [ ] Historia 8.2 - Contrato CAP → Vector
- [ ] Historia 8.3 - Multi-Sink y Fan-out
- [ ] Historia 8.4 - Vector Metrics

### Épica 9: Observabilidad y Métricas
- [ ] Historia 9.1 - Prometheus Metrics
- [ ] Historia 9.2 - Grafana Dashboards
- [ ] Historia 9.3 - Logging Estructurado
- [ ] Historia 9.4 - Tracing Distribuido

### Épica 10: DevOps y Despliegue
- [ ] Historia 10.1 - CI/CD Pipeline
- [ ] Historia 10.2 - Kubernetes Deployment
- [ ] Historia 10.3 - Backup y Disaster Recovery
- [ ] Historia 10.4 - Production Readiness

---

**TOTAL: 40+ historias que requieren actualización**

**METODOLOGÍA**:
1. Aplicar template sistemáticamente
2. Personalizar tests por historia
3. Verificar que comandos existen
4. Asegurar que todos los criterios son "bloqueantes"
5. NO avanzar hasta 100% tests passing ✅
