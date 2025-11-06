# API gRPC - Hodei Audit Service

## 📋 Resumen Ejecutivo

Este documento describe los contratos gRPC para Hodei Audit Service, definiendo los servicios de ingestión, consulta, criptografía y comunicación con Vector.dev.

**Versión**: 1.0  
**Última Actualización**: 2025-01-15

---

## 🏗️ Arquitectura de Servicios

### Puertos

| Servicio | Puerto | Propósito |
|----------|--------|-----------|
| **AuditControlService** | 50052 | Ingestión desde ARPs (SDKs) |
| **AuditQueryService** | 50053 | Query para clientes |
| **AuditCryptoService** | 50054 | Crypto/Digest para compliance |
| **VectorApi** | 50051 | CAP → Vector (fan-out) |

---

## 📥 AuditControlService (Puerto 50052)

Servicio para la ingestión de eventos de auditoría desde aplicaciones cliente (ARPs/SDKs).

### Servicios

#### PublishEvent
Publicar un evento individual de auditoría.

**Request**:
```json
{
  "tenant_id": "tenant-123",
  "event": { /* AuditEvent */ },
  "options": { /* PublishOptions (optional) */ }
}
```

**Response**:
```json
{
  "receipt_id": "receipt-uuid-123",
  "receipt_time": "2025-01-15T10:30:00Z"
}
```

**Ejemplo grpcurl**:
```bash
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "event": {
    "event_id": "evt-123",
    "event_time": "2025-01-15T10:30:00Z",
    "event_name": "UserLogin",
    "event_category": "MANAGEMENT",
    "hrn": "hrn:hodei:auth:tenant-123:global:user/admin"
  }
}' localhost:50052 hodei.audit.AuditControlService/PublishEvent
```

#### PublishBatch
Publicar múltiples eventos en un solo request (recomendado para performance).

**Request**:
```json
{
  "tenant_id": "tenant-123",
  "events": [ /* Array of AuditEvent */ ],
  "options": { /* BatchOptions (optional) */ }
}
```

**Response**:
```json
{
  "received_count": 100,
  "batch_id": "batch-uuid-123",
  "receipt_time": "2025-01-15T10:30:00Z",
  "failed_events": []
}
```

**Ejemplo grpcurl**:
```bash
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "events": [
    {
      "event_id": "evt-001",
      "event_name": "UserLogin",
      "hrn": "hrn:hodei:auth:tenant-123:global:user/admin"
    },
    {
      "event_id": "evt-002",
      "event_name": "CreateResource",
      "hrn": "hrn:hodei:storage:tenant-123:global:bucket/uploads"
    }
  ]
}' localhost:50052 hodei.audit.AuditControlService/PublishBatch
```

---

## 🔍 AuditQueryService (Puerto 50053)

Servicio para consultar eventos de auditoría.

#### QueryEvents
Consultar eventos con filtros y paginación.

**Request**:
```json
{
  "tenant_id": "tenant-123",
  "start_time": "2025-01-15T00:00:00Z",
  "end_time": "2025-01-15T23:59:59Z",
  "event_categories": ["MANAGEMENT", "DATA"],
  "event_names": ["UserLogin", "CreateResource"],
  "hrn_prefix": "hrn:hodei:auth:",
  "page_size": 100,
  "next_token": "token-123" // Para paginación
}
```

**Response**:
```json
{
  "events": [ /* Array of AuditEvent */ ],
  "next_token": "token-124",
  "total_count": 150
}
```

**Ejemplo grpcurl**:
```bash
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "start_time": "2025-01-15T00:00:00Z",
  "end_time": "2025-01-15T23:59:59Z",
  "page_size": 50
}' localhost:50053 hodei.audit.AuditQueryService/QueryEvents
```

#### ResolveHrn
Resolver un HRN a su metadata.

**Request**:
```json
{
  "hrn": "hrn:hodei:auth:tenant-123:global:user/admin"
}
```

**Response**:
```json
{
  "metadata": {
    "resource_type": "user",
    "tenant_id": "tenant-123",
    "region": "global",
    "path": "/user/admin",
    "attributes": {
      "username": "admin",
      "role": "administrator"
    }
  }
}
```

**Ejemplo grpcurl**:
```bash
grpcurl -plaintext -d '{
  "hrn": "hrn:hodei:auth:tenant-123:global:user/admin"
}' localhost:50053 hodei.audit.AuditQueryService/ResolveHrn
```

---

## 🔐 AuditCryptoService (Puerto 50054)

Servicio para operaciones criptográficas y verificación de digests.

#### VerifyDigest
Verificar la integridad de un digest.

**Request**:
```json
{
  "event_id": "evt-123",
  "digest_hash": "abc123...",
  "signature": "def456...",
  "public_key_fingerprint": "key-789..."
}
```

**Response**:
```json
{
  "valid": true,
  "previous_digest_hash": "prev-123...",
  "verification_details": {
    "algorithm": "SHA-256",
    "signature_algorithm": "ed25519"
  }
}
```

#### GetPublicKeys
Obtener las claves públicas para verificación.

**Request**:
```json
{
  "tenant_id": "tenant-123"
}
```

**Response**:
```json
{
  "keys": [
    {
      "fingerprint": "key-789...",
      "algorithm": "ed25519",
      "public_key": "base64-encoded-key",
      "valid_from": "2025-01-01T00:00:00Z",
      "valid_to": "2026-01-01T00:00:00Z"
    }
  ]
}
```

---

## 🚀 VectorApi (Puerto 50051)

Contrato simple para comunicación CAP → Vector.dev (fan-out).

#### SendEventBatch
Enviar batch de eventos a Vector para fan-out.

**Request**:
```json
{
  "events": [ /* Array of AuditEvent */ ],
  "tenant_id": "tenant-123",
  "batch_id": "batch-uuid-123"
}
```

**Response**:
```json
{
  "success": true,
  "message": "Batch accepted for fan-out",
  "accepted_count": 100
}
```

**Ejemplo grpcurl**:
```bash
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "batch_id": "batch-123",
  "events": [
    {
      "event_id": "evt-001",
      "event_name": "TestEvent"
    }
  ]
}' localhost:50051 hodei.audit.VectorApi/SendEventBatch
```

---

## 📊 AuditEvent Structure

Todos los eventos siguen la estructura CloudTrail-compatible:

```json
{
  "event_id": "evt-uuid-123",
  "event_time": "2025-01-15T10:30:00Z",
  "event_source": "hodei.audit.service",
  "event_name": "UserLogin",
  "event_category": "MANAGEMENT",
  "read_only": false,
  "hrn": "hrn:hodei:auth:tenant-123:global:user/admin",
  "source_ip": "10.0.0.1",
  "user_agent": "hodei-sdk/1.0",
  "error_code": null,
  "error_message": null,
  "additional_event_data": {
    "schema_version": "1.0",
    "digest_chain": {
      "hash_algorithm": "SHA-256",
      "signature_algorithm": "ed25519",
      "previous_digest_hash": "abc123...",
      "current_digest_hash": "def456...",
      "signature": "sig012..."
    }
  }
}
```

---

## 🔄 Códigos de Error gRPC

| Código | Descripción | Uso |
|--------|-------------|-----|
| **OK** | Éxito | Operación completada correctamente |
| **INVALID_ARGUMENT** | Argumento inválido | Parámetros de request inválidos |
| **NOT_FOUND** | No encontrado | HRN o recurso no existe |
| **ALREADY_EXISTS** | Ya existe | Intento de crear recurso duplicado |
| **PERMISSION_DENIED** | Permiso denegado | Tenant sin permisos |
| **RESOURCE_EXHAUSTED** | Recurso agotado | Rate limit excedido |
| **FAILED_PRECONDITION** | Precondición fallida | Estado inválido del sistema |
| **ABORTED** | Abortado | Conflicto de concurrencia |
| **OUT_OF_RANGE** | Fuera de rango | Parámetros fuera de rango |
| **UNIMPLEMENTED** | No implementado | Método no disponible |
| **INTERNAL** | Error interno | Error del servidor |
| **UNAVAILABLE** | No disponible | Servicio temporalmente no disponible |
| **DATA_LOSS** | Pérdida de datos | Error crítico de datos |
| **UNAUTHENTICATED** | No autenticado | Token inválido o expirado |

---

## 📝 Ejemplos Completos

### Cliente SDK (Rust)

```rust
use hodei_audit_sdk::{AuditClient, AuditConfig, EventBuilder};
use cloudtrail_patterns::EventCategory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AuditConfig::builder()
        .endpoint("http://localhost:50052")
        .tenant_id("tenant-123")
        .build()?;

    let client = AuditClient::new(config).await?;

    let event = EventBuilder::new()
        .event_name("UserLogin")
        .event_category(EventCategory::Management)
        .hrn("hrn:hodei:auth:tenant-123:global:user/admin")
        .source_ip("10.0.0.1")
        .user_agent("my-app/1.0")
        .build()?;

    let receipt = client.publish_event(event).await?;
    println!("Event published: {:?}", receipt);
    
    Ok(())
}
```

### Consulta de Eventos (grpcurl)

```bash
# Listar todos los eventos del día
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "start_time": "2025-01-15T00:00:00Z",
  "end_time": "2025-01-15T23:59:59Z",
  "page_size": 100
}' localhost:50053 hodei.audit.AuditQueryService/QueryEvents

# Filtrar por tipo de evento
grpcurl -plaintext -d '{
  "tenant_id": "tenant-123",
  "event_names": ["UserLogin", "UserLogout"],
  "page_size": 50
}' localhost:50053 hodei.audit.AuditQueryService/QueryEvents

# Resolver HRN
grpcurl -plaintext -d '{
  "hrn": "hrn:hodei:auth:tenant-123:global:user/admin"
}' localhost:50053 hodei.audit.AuditQueryService/ResolveHrn
```

---

## 🔐 Autenticación y Autorización

Los servicios utilizan mTLS para autenticación mutua:

- **Cliente → Server**: Certificado de cliente (tenant)
- **Server → Cliente**: Certificado de servidor

Headers requeridos:
```
Authorization: Bearer <jwt-token>
x-tenant-id: <tenant-id>
```

---

## 📈 Versioning Strategy

### Versión Actual: v1

- **Ruta gRPC**: localhost:50052/50053/50054
- **Compatibilidad**: Backwards compatible
- **Deprecación**: Aviso con 6 meses de antelación

### Futuras Versiones

- **v2**: Breaking changes (nueva ruta de puerto)
- **Migración**: Documentada con guías de migración

---

**Autor**: Equipo de Arquitectura Hodei  
**Contacto**: dev@hodei.io
