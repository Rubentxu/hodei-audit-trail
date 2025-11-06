# Épica 5: Multi-Tenancy y Seguridad

## 📋 Resumen Ejecutivo

**Objetivo**: Implementar aislamiento multi-tenant completo con API Key management, Row-Level Security, quotas y compliance.

**Duración**: 2-3 semanas

---

## Historias Principales

### Historia 5.1: Tenant Isolation y Context

**Objetivo**: Asegurar aislamiento total entre tenants.

**Criterios de Aceptación**:
- [ ] Shared table con tenant_id obligatorio
- [ ] TenantContext en todas las requests
- [ ] gRPC interceptor para validación
- [ ] Row-Level Security en ClickHouse
- [ ] Test de aislamiento (no cross-tenant access)

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que shared table con tenant_id funciona
- [ ] Testear TenantContext en todas las requests
- [ ] Verificar gRPC interceptor para validación
- [ ] Testear Row-Level Security en ClickHouse
- [ ] Validar que test de aislamiento detecta cross-tenant access
- [ ] Testear que tenant_id es obligatorio
- [ ] Verificar que context propagation funciona
- [ ] Testear validation en interceptor

**Tests de Integración Requeridos**:
- [ ] Shared table con tenant_id obligatorio funcionando
- [ ] TenantContext implementado y propagado
- [ ] gRPC interceptor validando correctamente
- [ ] Row-Level Security en ClickHouse activo
- [ ] Test de aislamiento passing (NO cross-tenant access)
- [ ] Tests de seguridad passing
- [ ] Access control funcionando correctamente
- [ ] Data isolation verificado

**Comandos de Verificación**:
```bash
# Testear tenant isolation
cargo test -p hodei-audit-service tenant_isolation

# Testear gRPC interceptor
cargo test -p hodei-audit-service grpc_interceptor

# Testear Row-Level Security
cargo test -p hodei-audit-service row_level_security

# Testear tenant context
cargo test -p hodei-audit-service tenant_context

# Test cross-tenant access (debe FALLAR)
cargo test -p hodei-audit-service cross_tenant_access_should_fail

# Verificar ClickHouse RLS
clickhouse-client --query="SHOW GRANTS"
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Shared table con tenant_id funcionando
- [ ] Row-Level Security activo
- [ ] NO cross-tenant access (aislamiento total)
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Shared table con tenant_id obligatorio
- ✅ TenantContext en todas las requests
- ✅ gRPC interceptor para validación
- ✅ Row-Level Security en ClickHouse
- ✅ Test de aislamiento (no cross-tenant access)
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 5.2: API Key Management

**Objetivo**: Sistema de API keys por tenant con scopes granulares.

**Criterios de Aceptación**:
- [ ] TenantApiKey struct con scopes
- [ ] Hashing seguro de keys
- [ ] Validation service
- [ ] Scopes: AuditRead, AuditWrite, CryptoVerify
- [ ] Rate limiting por key

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar TenantApiKey struct con scopes
- [ ] Testear hashing seguro de keys
- [ ] Verificar validation service
- [ ] Testear scopes: AuditRead, AuditWrite, CryptoVerify
- [ ] Validar rate limiting por key
- [ ] Testear que key generation es segura
- [ ] Verificar que keys son únicas
- [ ] Testear expiration de keys

**Tests de Integración Requeridos**:
- [ ] TenantApiKey struct funcionando
- [ ] Hashing seguro de keys implementado
- [ ] Validation service operativo
- [ ] Scopes granulares funcionando
- [ ] Rate limiting por key activo
- [ ] API key authentication passing
- [ ] Unauthorized access blocked
- [ ] Key rotation working
- [ ] Security audit passing

**Comandos de Verificación**:
```bash
# Testear API key management
cargo test -p hodei-audit-service api_key_management

# Testear scopes validation
cargo test -p hodei-audit-service scopes_validation

# Testear rate limiting
cargo test -p hodei-audit-service rate_limiting

# Testear key hashing
cargo test -p hodei-audit-service key_hashing

# Testear key validation
cargo test -p hodei-audit-service key_validation

# Security tests
cargo test -p hodei-audit-service security_tests
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] TenantApiKey struct funcionando
- [ ] Hashing seguro implementado
- [ ] Scopes granulares validados
- [ ] Rate limiting activo
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ TenantApiKey struct con scopes
- ✅ Hashing seguro de keys
- ✅ Validation service
- ✅ Scopes: AuditRead, AuditWrite, CryptoVerify
- ✅ Rate limiting por key
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 5.3: Resource Quotas y Rate Limiting

**Objetivo**: Controlar uso de recursos por tenant.

**Criterios de Aceptación**:
- [ ] Quota enforcement (events/sec, storage)
- [ ] Rate limiting por API key
- [ ] Usage tracking y alertas
- [ ] Per-tenant billing metrics
- [ ] Abuse detection

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar quota enforcement (events/sec, storage)
- [ ] Testear rate limiting por API key
- [ ] Verificar usage tracking y alertas
- [ ] Testear per-tenant billing metrics
- [ ] Validar abuse detection
- [ ] Testear que quotas se respetan
- [ ] Verificar que limits se aplican
- [ ] Testear enforcement mechanisms

**Tests de Integración Requeridos**:
- [ ] Quota enforcement activo (events/sec, storage)
- [ ] Rate limiting por API key funcionando
- [ ] Usage tracking y alertas operativas
- [ ] Per-tenant billing metrics registradas
- [ ] Abuse detection activo
- [ ] Exceeded quotas rejected
- [ ] Billing reports generated
- [ ] Performance under load maintained
- [ ] Tests de stress passing

**Comandos de Verificación**:
```bash
# Testear quotas
cargo test -p hodei-audit-service quotas

# Testear rate limiting
cargo test -p hodei-audit-service rate_limiting_quotas

# Testear usage tracking
cargo test -p hodei-audit-service usage_tracking

# Testear abuse detection
cargo test -p hodei-audit-service abuse_detection

# Load test
k6 run scripts/load-test-quotas.js

# Verificar metrics
curl http://localhost:9090/metrics | grep quota
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Quota enforcement activo
- [ ] Rate limiting funcionando
- [ ] Usage tracking operativo
- [ ] Abuse detection activo
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Quota enforcement (events/sec, storage)
- ✅ Rate limiting por API key
- ✅ Usage tracking y alertas
- ✅ Per-tenant billing metrics
- ✅ Abuse detection
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 5.4: Compliance y Retention

**Objetivo**: Políticas de retención por tipo de tenant.

**Criterios de Aceptación**:
- [ ] Enterprise: 7 años retención
- [ ] SME: 1-5 años configurable
- [ ] Legal hold support
- [ ] GDPR compliance
- [ ] Audit trail de deletions

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar Enterprise: 7 años retención
- [ ] Testear SME: 1-5 años configurable
- [ ] Verificar legal hold support
- [ ] Testear GDPR compliance
- [ ] Validar audit trail de deletions
- [ ] Testear que policies se aplican automáticamente
- [ ] Verificar que retention se respeta
- [ ] Testear data deletion audit

**Tests de Integración Requeridos**:
- [ ] Enterprise: 7 años retention configurado
- [ ] SME: 1-5 años retention configurable
- [ ] Legal hold support operativo
- [ ] GDPR compliance verificado
- [ ] Audit trail de deletions recording
- [ ] Automatic data deletion working
- [ ] Legal hold prevents deletion
- [ ] GDPR requests processed
- [ ] Compliance audit passing
- [ ] Data retention policies enforced

**Comandos de Verificación**:
```bash
# Testear retention policies
cargo test -p hodei-audit-service retention_policies

# Testear legal hold
cargo test -p hodei-audit-service legal_hold

# Testear GDPR compliance
cargo test -p hodei-audit-service gdpr_compliance

# Testear audit trail
cargo test -p hodei-audit-service audit_trail

# Verificar policy enforcement
clickhouse-client --query="SELECT * FROM system.events WHERE event='Delete'"

# Compliance check
./scripts/validate-gdpr-compliance.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Enterprise retention configured (7 años)
- [ ] SME retention configurable
- [ ] GDPR compliance verified
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Enterprise: 7 años retención
- ✅ SME: 1-5 años configurable
- ✅ Legal hold support
- ✅ GDPR compliance
- ✅ Audit trail de deletions
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## ⏭️ Siguiente Épica

[Épica 6: Digest Criptográfico y Compliance](epic-06-digest-criptografico-y-compliance.md)
