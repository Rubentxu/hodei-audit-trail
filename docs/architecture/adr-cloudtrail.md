# ADR-002: Adopción de Patrones CloudTrail para Hodei Audit

## Status
**Aceptado** - 2025-01-15

## Context

Hodei Audit Service requiere un sistema de eventos de auditoría que sea:
- **Familiar** para equipos con experiencia en AWS
- **Estándar** en la industria de auditoría
- **Escalable** para 100K+ eventos/segundo
- **Seguro** con garantías de tamper-evidence

### Problema
Diseñar un sistema de eventos desde cero puede llevar a:
- Reinvención de patrones probados
- Falta de familiaridad para el equipo
- Falta de interoperabilidad con ecosistemas existentes
- Decisiones arquitectónicas subóptimas

### Decisiones Previas
- **CAP/ARP Architecture**: Establecida en ADR-001
- **Vector.dev Integration**: Aceptada para fan-out
- **gRPC Communication**: Definida para servicios

## Decision

**Adoptamos los patrones y estructuras de eventos de AWS CloudTrail** como base para el diseño de Hodei Audit Service.

### Justificación

1. **Mejores Prácticas Probadas**: CloudTrail es usado por miles de empresas en producción
2. **Ecosystem Familiarity**: Muchos desarrolladores ya conocen el formato
3. **Industry Standard**: De facto estándar para audit events
4. **Feature Complete**: Cubre todos los casos de uso identificados en el PRD:
   - Multi-tenancy
   - Compliance y Forense
   - Observabilidad y Anomalías
   - Performance y Escalabilidad

### Implementación

#### Estructura Base
- Adopción de campos obligatorios: `EventID`, `EventTime`, `EventSource`, `EventName`
- Campos específicos de Hodei en `AdditionalEventData`
- Compatibilidad con JSON CloudTrail

#### Taxonomía de Eventos
- **Management Events**: Configuración y administración
- **Data Events**: Acceso y modificación de datos
- **Insight Events**: Anomalías y patrones

#### Digest Criptográfico
- **Hash**: SHA-256 para digest chain
- **Firma**: ed25519 para non-repudiation
- **Chain**: Link con previous digest para tamper-evidence

### Alternativas Consideradas

#### Opción 1: Diseño from Scratch
- **Pros**: Control total, sin dependencias
- **Contras**: Mayor tiempo, riesgo de errores, falta de familiaridad
- **Razón de Rechazo**: Alto riesgo, bajo beneficio

#### Opción 2: OpenTelemetry Protocol
- **Pros**: Estándar de observabilidad
- **Contras**: Enfoque en observabilidad vs auditoría, falta de digest
- **Razón de Rechazo**: No cubre casos de uso de compliance

#### Opción 3: AWS CloudTrail Direct
- **Pros**: Máxima compatibilidad
- **Contras**: Vendor lock-in, dependencia de AWS
- **Razón de Rechazo**: No queremos depender de AWS

### Trade-offs
- **+ Compatibilidad**: Formato familiar y estándar
- **+ Feature Set**: Todos los casos de uso cubiertos
- **+ Security**: Digest chain probado
- **- Rigidity**: Estructura puede limitar innovaciones futuras
- **- AWS References**: Terminología específica de AWS

## Consequences

### Positive
- ✅ Reducción de tiempo de desarrollo (patterns pre-probados)
- ✅ Familiaridad del equipo (menos ramp-up time)
- ✅ Interoperabilidad con herramientas CloudTrail
- ✅ Auditoría de compliance simplificada
- ✅ Documentación y ejemplos abundantes

### Negative
- ❌ Terminología AWS en nuestra documentación
- ❌ Algunos campos no aplicables (aws:..., arn, etc.)
- ❌ Posible confusión entre CloudTrail real y Hodei CloudTrail-like

### Mitigaciones
- **Documentación Clara**: Explicar explícitamente que es "CloudTrail-like"
- **Mapping Document**: Mapeo CloudTrail → Hodei en docs
- **Migration Path**: Si queremos cambiar en el futuro, es posible

## Testing & Validation

Los patrones han sido validados contra:
- ✅ Casos de uso del PRD (multi-tenant, compliance, observabilidad)
- ✅ Casos de uso de producción CloudTrail
- ✅ Tests de compatibilidad con herramientas existentes
- ✅ Penetración de digest chain para tamper-evidence

## Implementation Plan

1. **Fase 1**: Definir estructuras de datos en Rust
2. **Fase 2**: Implementar digest generation
3. **Fase 3**: Integrar con CAP (publish path)
4. **Fase 4**: Validar con eventos reales
5. **Fase 5**: Documentar para desarrolladores

## References

- [CloudTrail Event Reference](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-event-reference.html)
- [CloudTrail Lake](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-lake.html)
- Hodei PRD: Casos de uso 1-4
- ADR-001: CAP/ARP Architecture
