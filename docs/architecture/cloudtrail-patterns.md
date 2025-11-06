# Patrones CloudTrail para Hodei Audit

## 📋 Resumen Ejecutivo

Este documento analiza y define la adopción de los patrones de **AWS CloudTrail** en el diseño de Hodei Audit Service. El objetivo es aprovechar las mejores prácticas probadas en producción para el diseño de eventos de auditoría, asegurando compatibilidad y familiaridad para equipos con experiencia en AWS.

**Fecha**: 2025-01-15
**Versión**: 1.0
**Basado en**: AWS CloudTrail User Guide v2023

---

## 🎯 Objetivos

1. **Adoptar** la taxonomía de eventos de CloudTrail
2. **Mantener** compatibilidad con ecosistemas CloudTrail
3. **Aprovechar** patrones de seguridad probados
4. **Implementar** sistema de digest criptográfico
5. **Validar** contra casos de uso del PRD

---

## 📊 Taxonomía de Eventos CloudTrail

### Event Categories

CloudTrail define tres categorías principales de eventos:

#### 1. Management Events
**Descripción**: Operaciones que afectan la configuración o recursos de la infraestructura.

**Ejemplos en Hodei Audit**:
- Creación/modificación/eliminación de tenant
- Cambio de políticas de retención
- Configuración de storage backends
- User management (creación, eliminación, roles)
- Configuración de integraciones

**Campos Clave**:
```json
{
  "EventCategory": "Management",
  "ReadOnly": false,
  "EventType": "AwsApiCall",
  "SourceIPAddress": "10.0.0.1",
  "UserAgent": "hodei-audit-cli/1.0"
}
```

#### 2. Data Events
**Descripción**: Operaciones realizadas en los datos dentro de recursos.

**Ejemplos en Hodei Audit**:
- Creación de eventos de auditoría
- Consultas de eventos (QueryEvents)
- Exports de datos
- Accesos a storage (ClickHouse, S3)

**Campos Clave**:
```json
{
  "EventCategory": "Data",
  "ReadOnly": true,
  "EventType": "DataAccess",
  "Resources": [
    {
      "ResourceType": "AuditEvent",
      "ResourceName": "event-uuid-123"
    }
  ]
}
```

#### 3. Insight Events
**Descripción**: Eventos que indican actividad inusual o patrones anómalos.

**Ejemplos en Hodei Audit**:
- Volumen inusual de eventos
- Consultas lentas (>5s)
- Errores de autenticación repetidos
- Anomalías en patrones de acceso

**Campos Clave**:
```json
{
  "EventCategory": "Insight",
  "EventType": "Insight",
  "InsightType": "ApiCallRateInsight",
  "InsightSeverity": "Low|Medium|High|Critical"
}
```

---

## 🏗️ Estructura de Eventos CloudTrail-Compatible

### Estructura Base

```json
{
  "Version": "0",
  "EventID": "hodei-uuid-v4",
  "EventTime": "2025-01-15T10:30:00Z",
  "EventSource": "hodei.audit.service",
  "EventName": "PublishEvent",
  "EventCategory": "Management|Data|Insight",
  "EventType": "AwsApiCall|DataAccess|Insight",
  "ReadOnly": true|false,
  "RecipientAccountId": "tenant-123",
  "SourceIPAddress": "10.0.0.1",
  "UserAgent": "hodei-sdk/1.0",
  "Resources": [
    {
      "ResourceType": "HodeiResource",
      "ResourceName": "hrn:partition:service:tenant:region:type/path"
    }
  ],
  "ErrorCode": null,
  "ErrorMessage": null,
  "AdditionalEventData": {
    "SchemaVersion": "1.0",
    "HodeiSpecificFields": {}
  }
}
```

### Campos Obligatorios

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| **EventID** | String (UUID v4) | Identificador único del evento | `a1b2c3d4-e5f6-7890-abcd-ef1234567890` |
| **EventTime** | ISO 8601 | Timestamp del evento | `2025-01-15T10:30:00Z` |
| **EventSource** | String | Fuente del evento | `hodei.audit.service` |
| **EventName** | String | Nombre de la operación | `PublishEvent` |
| **EventCategory** | Enum | Management, Data, Insight | `Management` |
| **ReadOnly** | Boolean | Solo lectura vs modificación | `false` |

### Campos Opcionales

| Campo | Descripción | Uso |
|-------|-------------|-----|
| **ErrorCode** | Código de error específico | Errores de API |
| **ErrorMessage** | Descripción legible del error | Debugging |
| **AdditionalEventData** | JSON con datos específicos de Hodei | Extensibilidad |
| **ResponseElements** | Elementos de respuesta modificados | APIs que modifican recursos |
| **RequestParameters** | Parámetros de la request | Auditoría de parámetros |

---

## 🔐 Sistema de Digest Criptográfico

### Diseño de Digest

Hodei Audit adopta el patrón de **digest chain** de CloudTrail para garantizar **tamper-evidence**.

#### Algoritmos

1. **Hash Principal**: SHA-256
2. **Firma Digital**: ed25519 (Curve25519)

#### Estructura de Digest

```json
{
  "DigestChain": {
    "HashAlgorithm": "SHA-256",
    "SignatureAlgorithm": "ed25519",
    "PreviousDigestHash": "hex-string",
    "CurrentDigestHash": "hex-string",
    "DigestTimestamp": "2025-01-15T10:30:00Z",
    "PublicKeyFingerprint": "hex-string",
    "Signature": "hex-string"
  }
}
```

#### Proceso de Generación

```
digest = SHA-256(previous_digest_hash + event_data + timestamp)
signature = ed25519_sign(digest, private_key)
```

#### Ventajas

✅ **Detección de manipulación**: Cualquier cambio rompe la cadena  
✅ **Orden cronológico**: Digest incluye timestamp  
✅ **Verificabilidad**: Firma digital permite validación  
✅ **Non-repudiation**: Firma digital prueba origen  

#### Ejemplo de Evento con Digest

```json
{
  "EventID": "event-uuid",
  "EventTime": "2025-01-15T10:30:00Z",
  "EventName": "PublishEvent",
  "AdditionalEventData": {
    "DigestChain": {
      "HashAlgorithm": "SHA-256",
      "SignatureAlgorithm": "ed25519",
      "PreviousDigestHash": "abc123...",
      "CurrentDigestHash": "def456...",
      "DigestTimestamp": "2025-01-15T10:30:00Z",
      "PublicKeyFingerprint": "key789...",
      "Signature": "sig012..."
    }
  }
}
```

---

## 🔄 Mapeo CloudTrail → Hodei

### Correspondencia de Conceptos

| CloudTrail | Hodei | Descripción |
|------------|-------|-------------|
| AWS Account | Tenant | Multi-tenancy nativo |
| Region | Region | Distribución geográfica |
| Resource | HRN | Identificación jerárquica |
| CloudTrail Lake | ClickHouse | Almacenamiento analítico |
| S3 | S3/MinIO | Almacenamiento a largo plazo |

### Ejemplos de Eventos

#### Management Event: Creación de Tenant

```json
{
  "EventID": "evt-001",
  "EventTime": "2025-01-15T10:30:00Z",
  "EventName": "CreateTenant",
  "EventCategory": "Management",
  "ReadOnly": false,
  "EventSource": "hodei.audit.service",
  "Resources": [
    {
      "ResourceType": "Tenant",
      "ResourceName": "tenant-123"
    }
  ],
  "RequestParameters": {
    "tenant_name": "acme-corp",
    "region": "eu-west-1"
  }
}
```

#### Data Event: Consulta de Eventos

```json
{
  "EventID": "evt-002",
  "EventTime": "2025-01-15T10:35:00Z",
  "EventName": "QueryEvents",
  "EventCategory": "Data",
  "ReadOnly": true,
  "EventSource": "hodei.audit.service",
  "Resources": [
    {
      "ResourceType": "AuditEvent",
      "ResourceName": "hrn:hodei:audit:tenant-123:global:query/filter-456"
    }
  ],
  "RequestParameters": {
    "start_time": "2025-01-15T00:00:00Z",
    "end_time": "2025-01-15T23:59:59Z",
    "event_category": "Management"
  }
}
```

#### Insight Event: Anomalía de Volumen

```json
{
  "EventID": "evt-003",
  "EventTime": "2025-01-15T10:40:00Z",
  "EventName": "VolumeAnomaly",
  "EventCategory": "Insight",
  "EventType": "Insight",
  "InsightType": "ApiCallRateInsight",
  "InsightSeverity": "High",
  "EventSource": "hodei.audit.service",
  "AdditionalEventData": {
    "Baseline": 100,
    "Observed": 5000,
    "AnomalyScore": 0.95,
    "DurationSeconds": 300
  }
}
```

---

## 🛠️ Implementación en Rust

### Estructura de Datos

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CloudTrailEvent {
    pub event_id: String,
    pub event_time: DateTime<Utc>,
    pub event_source: String,
    pub event_name: String,
    pub event_category: EventCategory,
    pub read_only: bool,
    pub resources: Vec<Resource>,
    pub source_ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_event_data: Option<AdditionalEventData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_elements: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalEventData {
    pub schema_version: String,
    pub digest_chain: Option<DigestChain>,
    pub hodei_specific_fields: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigestChain {
    pub hash_algorithm: String,
    pub signature_algorithm: String,
    pub previous_digest_hash: String,
    pub current_digest_hash: String,
    pub digest_timestamp: DateTime<Utc>,
    pub public_key_fingerprint: String,
    pub signature: String,
}
```

---

## ✅ Validación contra PRD

### Casos de Uso Validados

#### Caso de Uso 1: Multi-Tenant Audit
- ✅ Management Events: Creación/modificación de tenants
- ✅ Data Events: Eventos de auditoría por tenant
- ✅ Resource isolation via HRN

#### Caso de Uso 2: Compliance y Forense
- ✅ Digest Chain: Tamper-evidence
- ✅ ReadOnly flag: Trazabilidad de solo lectura
- ✅ ErrorCode/ErrorMessage: Auditoría de errores

#### Caso de Uso 3: Observabilidad
- ✅ Insight Events: Anomalías y patrones
- ✅ SourceIPAddress: Rastreo de origen
- ✅ UserAgent: Identificación de cliente

#### Caso de Uso 4: Performance
- ✅ Batching de eventos (Data Events)
- ✅ Filtering por EventCategory
- ✅ Query por timestamp ranges

---

## 📚 Referencias

- [AWS CloudTrail User Guide](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/)
- [CloudTrail Event Reference](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-event-reference.html)
- [CloudTrail Lake Schema](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake-schema.html)
- [RFC 8032: EdDSA](https://www.rfc-editor.org/rfc/rfc8032)
- [NIST FIPS 180-4: SHA-256](https://csrc.nist.gov/publications/detail/fips/180/4/final)

---

**Autor**: Equipo de Arquitectura Hodei  
**Basado en**: AWS CloudTrail Documentation  
**Aprobado**: ⏳ Pendiente de revisión
