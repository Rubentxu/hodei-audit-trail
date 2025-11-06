# Épica 6: Digest Criptográfico y Compliance

## 📋 Resumen Ejecutivo

**Objetivo**: Implementar sistema de digest criptográfico con firma ed25519, verificación de integridad y compliance SOC2/PCI-DSS.

**Duración**: 2-3 semanas  
**Criticidad**: CRÍTICA - Deal-breaker para producción

---

## Historias Principales

### Historia 6.1: Digest Worker con ed25519

**Objetivo**: Generar digest criptográfico hourly para tamper-evidence.

**Criterios de Aceptación**:
- [ ] DigestWorker implementado
- [ ] SHA-256 de archivos Parquet
- [ ] Firma ed25519 de digest
- [ ] Chain de hashes (previous → current)
- [ ] CronJob Kubernetes para ejecución

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅
**⚠️ CRÍTICO**: Este es un deal-breaker para producción

**Tests Unitarios Requeridos**:
- [ ] Validar DigestWorker implementado correctamente
- [ ] Testear SHA-256 de archivos Parquet
- [ ] Verificar firma ed25519 de digest
- [ ] Testear chain de hashes (previous → current)
- [ ] Validar que ChronJob configuración es correcta
- [ ] Testear digest generation algorithm
- [ ] Verificar que digest es único por hour
- [ ] Testear tamper-evidence mechanism

**Tests de Integración Requeridos**:
- [ ] DigestWorker funcionando en producción
- [ ] SHA-256 de archivos Parquet working
- [ ] Firma ed25519 validada
- [ ] Chain de hashes validado
- [ ] CronJob Kubernetes ejecutando correctamente
- [ ] Tamper-evidence verificado
- [ ] Digest verification passing
- [ ] Performance acceptable (< 1 hour para 1TB data)
- [ ] Security audit passing
- [ ] SOC2/PCI-DSS compliance verified

**Comandos de Verificación**:
```bash
# Testear DigestWorker
cargo test -p hodei-audit-service digest_worker

# Testear SHA-256 hashing
cargo test -p hodei-audit-service sha256_hashing

# Testear ed25519 signature
cargo test -p hodei-audit-service ed25519_signature

# Testear hash chain
cargo test -p hodei-audit-service hash_chain

# Testear tamper evidence
cargo test -p hodei-audit-service tamper_evidence

# Verificar Kubernetes CronJob
kubectl get cronjobs
kubectl logs -l job=digest-worker

# Manual verification
./scripts/verify-digest.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] DigestWorker funcionando
- [ ] SHA-256 hashing working
- [ ] ed25519 signature valid
- [ ] Hash chain verified
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ DigestWorker implementado
- ✅ SHA-256 de archivos Parquet
- ✅ Firma ed25519 de digest
- ✅ Chain de hashes (previous → current)
- ✅ CronJob Kubernetes para ejecución
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 6.2: Key Management y Rotación

**Objetivo**: Gestión segura de claves criptográficas.

**Criterios de Aceptación**:
- [ ] StandaloneKeyManager (archivo)
- [ ] VaultKeyManager (futuro)
- [ ] Key rotation cada 90 días
- [ ] Public key manifest
- [ ] Key distribution service

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅
**⚠️ CRÍTICO**: Gestión de claves es fundamental para security

**Tests Unitarios Requeridos**:
- [ ] Validar StandaloneKeyManager (archivo)
- [ ] Testear VaultKeyManager (estructura para futuro)
- [ ] Verificar key rotation cada 90 días
- [ ] Testear public key manifest
- [ ] Validar key distribution service
- [ ] Testear secure key storage
- [ ] Verificar key generation
- [ ] Testear key deletion
- [ ] Validar key versioning

**Tests de Integración Requeridos**:
- [ ] StandaloneKeyManager funcionando
- [ ] Key rotation automática cada 90 días
- [ ] Public key manifest updated
- [ ] Key distribution service operativo
- [ ] Keys almacenadas securely
- [ ] Rotation working sin downtime
- [ ] Key history preserved
- [ ] Security audit passing
- [ ] Compliance verified
- [ ] Performance acceptable

**Comandos de Verificación**:
```bash
# Testear key management
cargo test -p hodei-audit-service key_management

# Testear key rotation
cargo test -p hodei-audit-service key_rotation

# Testear public key manifest
cargo test -p hodei-audit-service public_key_manifest

# Testear key distribution
cargo test -p hodei-audit-service key_distribution

# Testear security
cargo test -p hodei-audit-service key_security

# Verificar key rotation
./scripts/test-key-rotation.sh

# Security check
./scripts/security-audit-keys.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] StandaloneKeyManager funcionando
- [ ] Key rotation automática operativa
- [ ] Public key manifest valid
- [ ] Key distribution working
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ StandaloneKeyManager (archivo)
- ✅ VaultKeyManager (futuro)
- ✅ Key rotation cada 90 días
- ✅ Public key manifest
- ✅ Key distribution service
- ✅ **TODOS los tests passing (100%)** ⚠️

### Historia 6.3: Verificación de Integridad

**Objetivo**: APIs para que auditores verifiquen integridad.

**Criterios de Aceptación**:
- [ ] gRPC VerifyDigest endpoint
- [ ] CLI tool para auditoría manual
- [ ] Verificación de firma y chain
- [ ] Reportes de compliance
- [ ] Auditor dashboard

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅
**⚠️ CRÍTICO**: Compliance SOC2/PCI-DSS depende de esto

**Tests Unitarios Requeridos**:
- [ ] Validar gRPC VerifyDigest endpoint
- [ ] Testear CLI tool para auditoría manual
- [ ] Verificar verificación de firma y chain
- [ ] Testear reportes de compliance
- [ ] Validar auditor dashboard
- [ ] Testear digest chain verification
- [ ] Verificar tamper detection
- [ ] Testear compliance reporting
- [ ] Validar audit trail

**Tests de Integración Requeridos**:
- [ ] gRPC VerifyDigest endpoint operativo
- [ ] CLI tool funcional para auditoría manual
- [ ] Verificación de firma y chain passing
- [ ] Reportes de compliance generated
- [ ] Auditor dashboard displaying correctly
- [ ] External auditor can verify
- [ ] Compliance audit passing
- [ ] SOC2/PCI-DSS requirements met
- [ ] Tamper detection active
- [ ] Audit reports accurate
- [ ] End-to-end verification working

**Comandos de Verificación**:
```bash
# Testear VerifyDigest endpoint
cargo test -p hodei-audit-service verify_digest

# Testear CLI tool
cargo test -p hodei-audit-service cli_tool

# Testear compliance reports
cargo test -p hodei-audit-service compliance_reports

# Testear auditor dashboard
cargo test -p hodei-audit-service auditor_dashboard

# End-to-end verification
./scripts/verify-integrity-e2e.sh

# CLI verification
./target/debug/hodei-audit-cli verify --digest <digest>

# Compliance check
./scripts/validate-soc2-compliance.sh
./scripts/validate-pci-dss-compliance.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] gRPC VerifyDigest endpoint operativo
- [ ] CLI tool funcional
- [ ] Verificación de firma y chain passing
- [ ] SOC2/PCI-DSS compliance verified
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ gRPC VerifyDigest endpoint
- ✅ CLI tool para auditoría manual
- ✅ Verificación de firma y chain
- ✅ Reportes de compliance
- ✅ Auditor dashboard
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## ⏭️ Siguiente Épica

[Épica 7: Alto Rendimiento y Escalabilidad](epic-07-alto-rendimiento-y-escalabilidad.md)
