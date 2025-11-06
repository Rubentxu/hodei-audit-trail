# Épica 3: SDK Middleware y Integración

## 📋 Resumen Ejecutivo

**Objetivo**: Desarrollar el SDK (ARP - Audit Reporting Point) como middleware Axum que permita a las aplicaciones capturar y enviar eventos de auditoría de forma **automática y transparente**, con integración 1-liner y batch processing optimizado.

**Alcance**: Middleware Axum, cliente gRPC, batching inteligente, auto-enriquecimiento, HRN resolution, y ejemplos de integración con verified-permissions y otras aplicaciones.

**Duración Estimada**: 2-3 semanas

**Épica Padre**: Hodei Audit Service - Ecosistema Centralizado de Auditoría

---

## 🎯 Objetivo de Negocio

Como **desarrollador**, quiero integrar fácilmente el **sistema de auditoría** en mis aplicaciones, para que **todas las requests HTTP** se capturen **automáticamente** con **impacto mínimo** en el código y **performance óptimo**.

### Criterios de Aceptación (Épica)

- [ ] SDK (ARP) implementado como librería Rust
- [ ] Integración 1-liner en aplicaciones Axum
- [ ] Auto-captura de requests HTTP con middleware
- [ ] Auto-enriquecimiento: user_id, tenant_id, hrn, trace_id
- [ ] Batch processing con flush inteligente
- [ ] gRPC client con connection pooling
- [ ] Ejemplo de integración con verified-permissions
- [ ] Tests de integración end-to-end

---

## 👥 Historias de Usuario

### Historia 3.1: Diseño del SDK (ARP) y API Pública

**Como** Desarrollador  
**Quiero** una API de SDK simple e intuitiva  
**Para** integrar auditoría en mis aplicaciones con **código mínimo** y **configuración flexible**

#### Criterios de Aceptación

- [ ] API pública del SDK documentada
- [ ] Configuración builder pattern
- [ ] Integración 1-liner: `.layer(config.layer())`
- [ ] Features flags opcionales
- [ ] Default configuration sensata
- [ ] Error handling claro

#### Tareas Técnicas

1. Diseñar API pública del SDK
2. Implementar `AuditSdkConfig` con builder
3. Implementar `AuditLayer` como struct principal
4. Crear traits públicos: `AuditExtensions`
5. Documentar examples de uso
6. Configurar feature flags
7. Documentar en README con ejemplos

**API Pública**:
```rust
use hodei_audit_sdk::{AuditConfig, AuditLayer, AuditClient};

// Configuración con builder
let config = AuditConfig::builder()
    .service_name("my-service")
    .tenant_id("tenant-123")
    .audit_service_url("http://audit-service:50052")
    .batch_size(100)
    .batch_timeout(Duration::from_millis(100))
    .enable_request_body(true)
    .enable_response_body(false)
    .hrn_resolver(resolver)
    .build()?;

// Integración 1-liner en Axum
let app = Router::new()
    .route("/api/*path", get(handler))
    .layer(config.layer())  // <- Solo esto!
    .layer(AuthLayer);
```

**Feature Flags**:
```toml
[features]
default = ["request-body", "response-body"]
request-body = []
response-body = []
hrn-resolution = []
custom-enricher = []
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar API pública del SDK bien diseñada
- [ ] Testear configuración builder pattern
- [ ] Verificar que AuditConfig se construye correctamente
- [ ] Testear que AuditLayer se implementa correctamente
- [ ] Validar que default configuration es sensata
- [ ] Testear feature flags opcionales
- [ ] Verificar error handling claro
- [ ] Validar traits públicos: AuditExtensions

**Tests de Integración Requeridos**:
- [ ] API documentada con examples funcionales
- [ ] Builder pattern implementado y testeable
- [ ] Feature flags configurados y funcionando
- [ ] README con integration guide completo
- [ ] API reference generated
- [ ] Integración 1-liner funciona correctamente
- [ ] Configuración flexible testeada
- [ ] Compilation tests con diferentes feature flags

**Comandos de Verificación**:
```bash
# Testear API del SDK
cargo test -p hodei-audit-sdk sdk_api

# Testear builder pattern
cargo test -p hodei-audit-sdk builder_pattern

# Testear feature flags
cargo test -p hodei-audit-sdk --features request-body
cargo test -p hodei-audit-sdk --features hrn-resolution

# Testear documentation examples
cargo test -p hodei-audit-sdk doc_tests

# Verify API docs
cargo doc -p hodei-audit-sdk --no-deps --open
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] API documentada con examples
- [ ] Builder pattern implementado
- [ ] Feature flags funcionando
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ API documentada con examples
- ✅ Builder pattern implementado
- ✅ Feature flags configurados
- ✅ README con integration guide
- ✅ API reference generated
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.2: Middleware Axum Implementation

**Como** Desarrollador  
**Quiero** middleware Axum que capture requests automáticamente  
**Para** que **todas las requests HTTP** se auditen **sin código adicional**

#### Criterios de Aceptación

- [ ] Middleware implementado con Tower
- [ ] Captura de request/response automática
- [ ] Extract user_id, tenant_id, trace_id
- [ ] Auto-generar HRN basado en path
- [ ] No impact en response time
- [ ] Async processing (non-blocking)
- [ ] Error handling graceful

#### Tareas Técnicas

1. Implementar `AuditLayer` con `Layer<S>` trait
2. Implementar `AuditService` con `Service<Request>` trait
3. Extraer context de request (headers, path, method)
4. Auto-generar HRN desde request path
5. Añadir a batch queue async
6. Handle failures gracefully
7. Logging estructurado

**Middleware Implementation**:
```rust
impl<S> Layer<S> for AuditLayer {
    type Service = AuditService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuditService {
            config: self.config.clone(),
            client: self.client.clone(),
            batch_queue: self.batch_queue.clone(),
            service,
        }
    }
}

impl<S> Service<Request<Body>> for AuditService<S>
where
    S: Service<Request<Body>, Response = Response> + Send + Clone,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(&self, request: Request<Body>) -> Result<Self::Response, Self::Error> {
        // 1. Call next service
        let response = self.service.call(request.clone()).await?;
        
        // 2. Create audit event (async, non-blocking)
        let event = self.create_audit_event(&request, &response)?;
        self.add_to_batch(event).await;
        
        // 3. Return response
        Ok(response)
    }
}
```

**Context Extraction**:
```rust
fn extract_audit_context(
    request: &Request<Body>,
    response: &Response,
) -> Result<AuditContext, AuditError> {
    // Extract from headers
    let user_id = request.headers()
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    let tenant_id = request.headers()
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    let trace_id = request.headers()
        .get("x-trace-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    // Auto-generate HRN from path
    let hrn = generate_hrn_from_path(
        request.uri().path(),
        request.method(),
        &tenant_id,
    )?;
    
    Ok(AuditContext {
        user_id,
        tenant_id,
        trace_id,
        hrn,
        method: request.method().clone(),
        path: request.uri().path().to_string(),
        status_code: response.status(),
        // ...
    })
}
```

**Definición de Done**:
- Middleware implementando Layer trait
- Auto-captura funcionando
- HRN generation working
- Non-blocking processing
- Performance impact < 1ms

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que AuditLayer implementa Layer trait correctamente
- [ ] Testear que AuditService implementa Service trait
- [ ] Verificar extracción de context (user_id, tenant_id, trace_id)
- [ ] Testear auto-generación de HRN desde path
- [ ] Validar que middleware es no bloqueante
- [ ] Testear error handling graceful
- [ ] Verificar logging estructurado
- [ ] Validar async processing

**Tests de Integración Requeridos**:
- [ ] Middleware funciona con aplicaciones Axum
- [ ] Auto-captura de requests/responses funcionando
- [ ] Performance impact < 1ms (verificado con benchmarks)
- [ ] HRN generation working para diferentes paths
- [ ] Context extraction desde headers funcionando
- [ ] Non-blocking processing bajo alta carga
- [ ] Error en middleware no afecta response
- [ ] Tests de integración end-to-end

**Comandos de Verificación**:
```bash
# Testear middleware implementation
cargo test -p hodei-audit-sdk middleware

# Testear context extraction
cargo test -p hodei-audit-sdk context_extraction

# Testear HRN generation
cargo test -p hodei-audit-sdk hrn_generation

# Testear integración con Axum
cargo test -p hodei-audit-sdk axum_integration

# Testear performance
cargo bench -p hodei-audit-sdk middleware_performance

# Load test
k6 run scripts/load-test-middleware.js
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Middleware implementando Layer trait
- [ ] Auto-captura funcionando
- [ ] Performance impact < 1ms
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Middleware implementando Layer trait
- ✅ Auto-captura funcionando
- ✅ HRN generation working
- ✅ Non-blocking processing
- ✅ Performance impact < 1ms
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.3: Batch Processing Inteligente

**Como** Desarrollador  
**Quiero** batching inteligente que optimice performance  
**Para** reducir **network overhead** y **mejorar throughput** sin afectar latencia

#### Criterios de Aceptación

- [ ] Local batch queue con capacidad configurable
- [ ] Auto-flush por tamaño (batch_size)
- [ ] Auto-flush por tiempo (batch_timeout)
- [ ] Background flush worker
- [ ] GRPC batch endpoint optimizado
- [ ] Backpressure handling
- [ ] Retry con exponential backoff

#### Tareas Técnicas

1. Implementar `BatchQueue` con Mutex<Vec>
2. Implementar flush policies (size, time, hybrid)
3. Background worker con tokio::spawn
4. Connection pooling para gRPC client
5. Implementar retry con backoff
6. Configurar backpressure thresholds
7. Métricas de batch performance

**Batch Queue Implementation**:
```rust
pub struct AuditLayer {
    config: AuditSdkConfig,
    client: AuditControlServiceClient<tonic::transport::Channel>,
    batch_queue: Arc<Mutex<Vec<AuditEvent>>>,
    batch_flush_timer: Arc<Mutex<tokio::time::Interval>>,
}

impl AuditLayer {
    pub async fn add_to_batch(&self, event: AuditEvent) {
        let mut queue = self.batch_queue.lock().unwrap();
        queue.push(event);
        
        // Auto-flush on batch size
        if queue.len() >= self.config.batch_size {
            self.flush_batch(&mut queue).await;
        }
    }
    
    async fn flush_batch(&self, queue: &mut Vec<AuditEvent>) {
        if queue.is_empty() {
            return;
        }
        
        let events = std::mem::take(queue);
        
        let client = self.client.clone();
        let tenant_id = self.config.tenant_id.clone();
        
        // Async flush (non-blocking)
        tokio::spawn(async move {
            if let Err(e) = client
                .clone()
                .publish_batch(PublishBatchRequest {
                    tenant_id: tenant_id.unwrap_or_default(),
                    events,
                    options: Some(BatchOptions {
                        flush_immediately: true,
                        ..Default::default()
                    }),
                })
                .await
            {
                error!("Failed to publish audit batch: {}", e);
            }
        });
    }
}
```

**Flush Policies**:
```rust
pub enum FlushPolicy {
    Size(usize),           // Flush when N events
    Time(Duration),        // Flush every N seconds
    Hybrid(usize, Duration), // Flush when either condition met
}

impl FlushPolicy {
    fn should_flush(&self, queue: &Vec<AuditEvent>, last_flush: &Instant) -> bool {
        match self {
            Self::Size(n) => queue.len() >= *n,
            Self::Time(d) => last_flush.elapsed() >= *d,
            Self::Size(n) | Self::Hybrid(n, _) => queue.len() >= *n,
            Self::Time(_) | Self::Hybrid(_, d) => last_flush.elapsed() >= *d,
        }
    }
}
```

**Connection Pooling**:
```rust
pub struct GrpcConnectionPool {
    clients: Arc<Mutex<Vec<AuditControlServiceClient<tonic::transport::Channel>>>>,
    max_size: usize,
    min_size: usize,
    url: String,
}

impl GrpcConnectionPool {
    pub async fn new(url: String, max_size: usize, min_size: usize) -> Result<Self> {
        let mut clients = Vec::new();
        
        // Pre-warm connections
        for _ in 0..min_size {
            let client = AuditControlServiceClient::connect(url.clone()).await?;
            clients.push(client);
        }
        
        Ok(Self {
            clients: Arc::new(Mutex::new(clients)),
            max_size,
            min_size,
            url,
        })
    }
}
```

**Retry Policy**:
```rust
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f32,
}

impl RetryConfig {
    pub async fn execute_with_retry<F, T, E>(&self, mut f: F) -> Result<T, RetryError<E>>
    where
        F: FnMut() -> BoxFuture<'_, Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut delay = self.initial_delay;
        
        for attempt in 0..self.max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(error) if attempt < self.max_retries - 1 => {
                    warn!("Retry {} failed: {}", attempt + 1, error);
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        delay * self.multiplier,
                        self.max_delay
                    );
                }
                Err(error) => return Err(RetryError::MaxRetriesExceeded(error)),
            }
        }
        
        unreachable!()
    }
}
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que BatchQueue se implementa correctamente
- [ ] Testear flush policies (Size, Time, Hybrid)
- [ ] Verificar auto-flush por tamaño (batch_size)
- [ ] Testear auto-flush por tiempo (batch_timeout)
- [ ] Validar background flush worker
- [ ] Testear GRPC batch endpoint optimizado
- [ ] Verificar backpressure handling
- [ ] Testear retry con exponential backoff
- [ ] Validar connection pooling

**Tests de Integración Requeridos**:
- [ ] Batch queue implementada y funcional
- [ ] Flush policies working correctamente
- [ ] Background worker running estable
- [ ] Connection pool configured y funcionando
- [ ] Retry logic working bajo fallos de red
- [ ] Performance benchmarks passing (throughput optimizado)
- [ ] Network overhead reducido significativamente
- [ ] Tests de stress con alta carga
- [ ] Backpressure handling bajo saturación

**Comandos de Verificación**:
```bash
# Testear batch queue
cargo test -p hodei-audit-sdk batch_queue

# Testear flush policies
cargo test -p hodei-audit-sdk flush_policies

# Testear retry logic
cargo test -p hodei-audit-sdk retry_logic

# Testear connection pool
cargo test -p hodei-audit-sdk connection_pool

# Testear integración
cargo test -p hodei-audit-sdk batch_integration

# Benchmarks
cargo bench -p hodei-audit-sdk batch_performance
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Batch queue implementada
- [ ] Flush policies working
- [ ] Performance optimizado
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Batch queue implementada
- ✅ Flush policies working
- ✅ Background worker running
- ✅ Connection pool configured
- ✅ Retry logic working
- ✅ Performance benchmarks
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.4: Auto-Enriquecimiento con HRN

**Como** Desarrollador  
**Quiero** auto-enriquecimiento de eventos con HRN  
**Para** que los eventos tengan **identificadores canónicos** y **contexto completo**

#### Criterios de Aceptación

- [ ] Auto-generar HRN desde request path
- [ ] HRN resolver para metadata
- [ ] Mapeo de paths a HRN patterns
- [ ] HRN hierarchy support
- [ ] Custom HRN resolver integration
- [ ] Fallback para paths desconocidos

#### Tareas Técnicas

1. Implementar `generate_hrn_from_path()`
2. Crear mapping de paths a HRN patterns
3. Integrar con HRN resolver del servicio
4. Support para wildcards y parameters
5. Custom resolver hook
6. Default HRN para paths unknown
7. Tests de HRN generation

**HRN Generation**:
```rust
fn generate_hrn_from_path(
    method: &Method,
    path: &str,
    tenant_id: &Option<String>,
) -> Result<Hrn, AuditError> {
    let tenant = tenant_id.as_deref().unwrap_or("unknown");
    
    // Mapeo de paths a patterns
    let pattern = match path {
        // verified-permissions endpoints
        p if p.starts_with("/v1/policy-stores") => {
            let policy_store_id = extract_policy_store_id(p)
                .unwrap_or("default");
            format!(
                "hrn:hodei:verified-permissions:{}:global:policy-store/{}",
                tenant, policy_store_id
            )
        }
        
        // API service endpoints
        p if p.starts_with("/api/") => {
            let api_path = p.trim_start_matches("/api/");
            format!(
                "hrn:hodei:api:{}:global:api/{}",
                tenant, api_path
            )
        }
        
        // Default pattern
        _ => format!(
            "hrn:hodei:unknown:{}:global:resource/{}",
            tenant, path
        )
    };
    
    Hrn::parse(&pattern).map_err(|e| AuditError::HrnError(e))
}

fn extract_policy_store_id(path: &str) -> Option<String> {
    // /v1/policy-stores/{id}/...
    let re = Regex::new(r"/v1/policy-stores/([^/]+)").unwrap();
    re.captures(path)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}
```

**Custom Resolver Integration**:
```rust
pub trait HrnResolver: Send + Sync {
    async fn resolve(&self, hrn: &Hrn) -> Result<HrnMetadata, HrnError>;
}

// En la configuración del SDK
impl AuditSdkConfig {
    pub fn hrn_resolver(
        mut self,
        resolver: Arc<dyn HrnResolver>
    ) -> Self {
        self.hrn_resolver = Some(resolver);
        self
    }
}

// En el middleware
async fn enrich_with_hrn(
    event: &mut AuditEvent,
    resolver: &Option<Arc<dyn HrnResolver>>,
) -> Result<(), AuditError> {
    if let Some(r) = resolver {
        if let Ok(metadata) = r.resolve(&event.hrn).await {
            event.metadata["hrn_display_name"] = 
                serde_json::to_value(&metadata.display_name)?;
            event.metadata["hrn_tags"] = 
                serde_json::to_value(&metadata.tags)?;
        }
    }
    
    Ok(())
}
```

**Path Patterns**:
| Path | HRN Pattern | Example |
|------|-------------|---------|
| `/v1/policy-stores` | `hrn:...:policy-store/*` | `hrn:hodei:vp:tenant-123:global:policy-store/default` |
| `/api/users/*` | `hrn:...:api/user/*` | `hrn:hodei:api:tenant-123:global:api/user-profile` |
| `/v1/auth/*` | `hrn:...:auth/*` | `hrn:hodei:auth:tenant-123:global:auth/login` |

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar auto-generación de HRN desde request path
- [ ] Testear mapeo de paths a HRN patterns
- [ ] Verificar que HRN resolver funciona
- [ ] Testear HRN hierarchy support
- [ ] Validar custom HRN resolver integration
- [ ] Testear fallback para paths desconocidos
- [ ] Verificar regex para extraction de IDs
- [ ] Testear wildcards y parameters

**Tests de Integración Requeridos**:
- [ ] HRN generation working para common paths
- [ ] Custom resolver integration funcionando
- [ ] Metadata enrichment working
- [ ] Tests con multiple path patterns passing
- [ ] Path patterns documentados y actualizados
- [ ] Integración con middleware funcionando
- [ ] HRN hierarchy correctamente soportada
- [ ] Performance de HRN generation optimizado

**Comandos de Verificación**:
```bash
# Testear HRN generation
cargo test -p hodei-audit-sdk hrn_generation

# Testear path patterns
cargo test -p hodei-audit-sdk hrn_path_patterns

# Testear custom resolver
cargo test -p hodei-audit-sdk hrn_custom_resolver

# Testear metadata enrichment
cargo test -p hodei-audit-sdk hrn_enrichment

# Testear integración
cargo test -p hodei-audit-sdk hrn_integration

# Benchmarks
cargo bench -p hodei-audit-sdk hrn_performance
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] HRN generation working para common paths
- [ ] Custom resolver integration
- [ ] Metadata enrichment working
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ HRN generation working para common paths
- ✅ Custom resolver integration
- ✅ Metadata enrichment working
- ✅ Tests con multiple path patterns
- ✅ Documentation de patterns
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.5: Cliente Manual (AuditClient)

**Como** Desarrollador  
**Quiero** un cliente manual para logging custom  
**Para** poder enviar eventos **específicos** que no se capturan automáticamente

#### Criterios de Aceptación

- [ ] `AuditClient` para logging manual
- [ ] `log()` para eventos individuales
- [ ] `log_batch()` para múltiples eventos
- [ ] `query()` para consultar eventos
- [ ] `resolve_hrn()` para metadata
- [ ] Same batching y retry logic
- [ ] Async/await API

#### Tareas Técnicas

1. Implementar `AuditClient` struct
2. Implementar `log()` single event
3. Implementar `log_batch()` multiple events
4. Implementar `query()` events
5. Implementar `resolve_hrn()` metadata
6. Reusar connection pool
7. Documentar examples

**AuditClient API**:
```rust
pub struct AuditClient {
    client: AuditControlServiceClient<tonic::transport::Channel>,
    config: AuditSdkConfig,
}

impl AuditClient {
    pub async fn new(url: String) -> Result<Self> {
        let channel = tonic::transport::Endpoint::new(url)?
            .connect()
            .await?;
        
        let client = AuditControlServiceClient::new(channel);
        
        Ok(Self {
            client,
            config: AuditSdkConfig::default(),
        })
    }
    
    // Log single event
    pub async fn log(&self, event: AuditEvent) -> Result<()> {
        let request = PublishEventRequest {
            tenant_id: event.tenant_id.to_string(),
            event: Some(event),
            options: None,
        };
        
        self.client
            .clone()
            .publish_event(tonic::Request::new(request))
            .await?;
        
        Ok(())
    }
    
    // Log batch events
    pub async fn log_batch(&self, events: Vec<AuditEvent>) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }
        
        let request = PublishBatchRequest {
            tenant_id: events[0].tenant_id.to_string(),
            events,
            options: Some(BatchOptions {
                flush_immediately: true,
                ..Default::default()
            }),
        };
        
        self.client
            .clone()
            .publish_batch(tonic::Request::new(request))
            .await?;
        
        Ok(())
    }
    
    // Query events
    pub async fn query(&self, query: AuditQuery) -> Result<AuditQueryResult> {
        let response = self.client
            .clone()
            .query_events(tonic::Request::new(query))
            .await?;
        
        Ok(response.into_inner())
    }
    
    // Resolve HRN
    pub async fn resolve_hrn(&self, hrn: Hrn) -> Result<HrnMetadata> {
        let response = self.client
            .clone()
            .resolve_hrn(tonic::Request::new(hrn))
            .await?;
        
        Ok(response.into_inner())
    }
}
```

**Usage Examples**:
```rust
// Manual logging
let audit_client = AuditClient::new("http://audit-service:50052").await?;

let event = AuditEvent {
    event_id: Uuid::new_v4().to_string(),
    tenant_id: "tenant-123".to_string(),
    hrn: Hrn::parse("hrn:hodei:api:tenant-123:global:api/custom")?,
    user_id: "user-456".to_string(),
    action: "custom_action".to_string(),
    timestamp: Utc::now(),
    ..Default::default()
};

audit_client.log(event).await?;

// Query events
let query = AuditQuery {
    hrn: Some(hrn),
    limit: 100,
    ..Default::default()
};

let result = audit_client.query(query).await?;
println!("Found {} events", result.total);
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar que AuditClient se inicializa correctamente
- [ ] Testear método `log()` para eventos individuales
- [ ] Testear método `log_batch()` para múltiples eventos
- [ ] Verificar que método `query()` funciona
- [ ] Testear método `resolve_hrn()` para metadata
- [ ] Validar que reutiliza connection pool
- [ ] Testear manejo de errores
- [ ] Verificar async/await API

**Tests de Integración Requeridos**:
- [ ] AuditClient implementada y funcional
- [ ] All methods working correctamente
- [ ] Batching optimizado funcionando
- [ ] Tests unitarios > 90% coverage
- [ ] Documentation con examples validada
- [ ] Integration con gRPC service exitosa
- [ ] Retry logic funcionando
- [ ] Connection pooling eficiente

**Comandos de Verificación**:
```bash
# Testear AuditClient
cargo test -p hodei-audit-sdk audit_client

# Testear log methods
cargo test -p hodei-audit-sdk audit_client_log

# Testear query methods
cargo test -p hodei-audit-sdk audit_client_query

# Testear integración
cargo test -p hodei-audit-sdk audit_client_integration

# Verificar coverage
cargo tarpaulin -p hodei-audit-sdk --out xml
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] AuditClient implementada
- [ ] All methods working
- [ ] Tests unitarios > 90% coverage
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ AuditClient implementada
- ✅ All methods working
- ✅ Batching optimizado
- ✅ Tests unitarios > 90% coverage
- ✅ Documentation con examples
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.6: Integración con verified-permissions

**Como** Desarrollador  
**Quiero** integrar el SDK con verified-permissions  
**Para** que todas las requests al **policy engine** se auditen automáticamente

#### Criterios de Aceptación

- [ ] Integration guide para verified-permissions
- [ ] Code example completo
- [ ] Configuración optimizada
- [ ] Custom HRN patterns para policies
- [ ] Tests de integración
- [ ] Documentation de deployment

#### Tareas Técnicas

1. Crear integration guide
2. Implementar example en verified-permissions
3. Configurar HRN patterns para policy endpoints
4. Crear integration tests
5. Documentar en verified-permissions README
6. Screenshots de debugging

**Integration Example**:
```rust
// verified-permissions/src/main.rs

use hodei_audit_sdk::{AuditConfig, AuditLayer};
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Configure audit
    let audit_config = AuditConfig::builder()
        .service_name("verified-permissions")
        .tenant_id("tenant-123")
        .audit_service_url("http://audit-service:50052")
        .batch_size(100)
        .batch_timeout(Duration::from_millis(100))
        .enable_request_body(true)
        .hrn_resolver(resolver)  // Optional
        .build()?;

    // 2. Create app with audit layer
    let app = Router::new()
        .route("/v1/policy-stores", get(list_policy_stores))
        .route("/v1/policy-stores", post(create_policy_store))
        .route("/v1/policy-stores/:id", get(get_policy_store))
        .route("/v1/policy-stores/:id", put(update_policy_store))
        .route("/v1/policy-stores/:id", delete(delete_policy_store))
        .route("/v1/authorize", post(authorize))
        // 3. Add audit layer (captures ALL requests!)
        .layer(audit_config.layer())
        // Other layers
        .layer(AuthLayer)
        .layer(RateLimitLayer);

    // 4. Run server
    axum::Server::bind(&"0.0.0.0:50051".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

**Custom HRN Patterns**:
```rust
// En verified-permissions
fn generate_policy_hrn(
    method: &Method,
    path: &str,
    tenant_id: &str,
) -> Result<Hrn, AuditError> {
    match (method.as_str(), path) {
        // Policy Stores
        ("GET", p) if p.starts_with("/v1/policy-stores/") => {
            let id = extract_id(p)?;
            Hrn::parse(&format!(
                "hrn:hodei:verified-permissions:{}:global:policy-store/{}",
                tenant_id, id
            ))
        }
        ("POST", "/v1/policy-stores") => {
            Hrn::parse(&format!(
                "hrn:hodei:verified-permissions:{}:global:policy-store/list",
                tenant_id
            ))
        }
        // Authorization
        ("POST", "/v1/authorize") => {
            Hrn::parse(&format!(
                "hrn:hodei:verified-permissions:{}:global:authorization/check",
                tenant_id
            ))
        }
        // Fallback
        _ => generate_default_hrn(method, path, tenant_id),
    }
}
```

**Integration Test**:
```rust
#[tokio::test]
async fn test_audit_integration() {
    // Setup test environment
    let audit_service = setup_audit_service().await;
    let app = setup_app_with_audit(audit_service.url());
    
    // Make request
    let response = app
        .post("/v1/policy-stores")
        .json(&json!({ "name": "test" }))
        .send()
        .await;
    
    // Verify response
    assert_eq!(response.status(), 201);
    
    // Verify audit event was logged
    let events = audit_service.query_events(
        "tenant-123",
        "policy-store",
    ).await;
    
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "POST /v1/policy-stores");
}
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar integration guide written y completo
- [ ] Verificar example code proporcionado funciona
- [ ] Testear que configuration es correcta
- [ ] Validar custom HRN patterns para policies
- [ ] Testear que code example se compila
- [ ] Verificar screenshots de debugging
- [ ] Testear deployment documentation

**Tests de Integración Requeridos**:
- [ ] Integration guide written y validado
- [ ] Example code proporcionado funcionando
- [ ] Tests passing con verified-permissions
- [ ] Documentation complete y actualizada
- [ ] Team trained on integration
- [ ] Custom HRN patterns funcionando
- [ ] Integration tests con Testcontainers passing
- [ ] End-to-end tests passing
- [ ] Performance impact validado (< 1ms)

**Comandos de Verificación**:
```bash
# Testear integration example
cargo test -p verified-permissions-example integration

# Testear custom HRN patterns
cargo test -p hodei-audit-sdk verified_permissions_hrn

# Testear integration guide
./scripts/validate-integration-guide.sh

# Verificar examples se compilan
cargo check -p verified-permissions-example

# Testear con real verified-permissions
./scripts/test-verified-permissions-integration.sh

# Verificar documentación
./scripts/validate-docs.sh integration
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Integration guide written
- [ ] Example code provided
- [ ] Tests passing
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Integration guide written
- ✅ Example code provided
- ✅ Tests passing
- ✅ Documentation complete
- ✅ Team trained on integration
- ✅ **TODOS los tests passing (100%)** ⚠️

---

### Historia 3.7: Testing y Quality Assurance

**Como** QA Engineer  
**Quiero** tests comprehensivos del SDK  
**Para** garantizar que la integración **funciona correctamente** y es **robusta**

#### Criterios de Aceptación

- [ ] Unit tests (>90% coverage)
- [ ] Integration tests con real gRPC service
- [ ] Load tests con alto throughput
- [ ] Failure scenario tests
- [ ] Memory leak tests
- [ ] Performance benchmarks

#### Tareas Técnicas

1. Unit tests para cada módulo
2. Integration tests con Testcontainers
3. Load tests con K6 o similar
4. Chaos tests (network failures, timeouts)
5. Memory profiling
6. Performance benchmarking
7. Code coverage report

**Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("/v1/policy-stores", "tenant-123", "default")]
    #[case("/api/users/456", "tenant-123", "user-profile")]
    fn test_hrn_generation(
        #[case] path: &str,
        #[case] tenant: &str,
        #[case] expected_resource: &str,
    ) {
        let hrn = generate_hrn_from_path(
            &Method::GET,
            path,
            &Some(tenant.to_string())
        ).unwrap();
        
        assert!(hrn.to_string().contains(expected_resource));
    }

    #[tokio::test]
    async fn test_batch_queue_flush() {
        let config = AuditConfig::default();
        let layer = AuditLayer::new(config);
        
        // Add events up to batch size
        for i in 0..10 {
            let event = create_test_event(i);
            layer.add_to_batch(event).await;
        }
        
        // Queue should be flushed
        let queue = layer.batch_queue.lock().unwrap();
        assert!(queue.is_empty());
    }
}
```

**Integration Test**:
```rust
#[tokio::test]
async fn test_sdk_integration() {
    // Start audit service
    let audit_service = TestAuditService::new().await;
    let port = audit_service.port();
    
    // Create SDK
    let config = AuditConfig::builder()
        .service_name("test-service")
        .tenant_id("tenant-123")
        .audit_service_url(&format!("http://localhost:{}", port))
        .batch_size(1)  // Immediate flush
        .build()
        .unwrap();
    
    let layer = AuditLayer::new(config);
    
    // Simulate request
    let request = Request::builder()
        .uri("/api/test")
        .header("x-user-id", "user-123")
        .header("x-tenant-id", "tenant-123")
        .body(Body::empty())
        .unwrap();
    
    let response = layer.oneshot(request).await.unwrap();
    
    // Verify event was sent
    let events = audit_service.get_events().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].user_id, "user-123");
}
```

**Load Test**:
```rust
// Con K6
import http from 'k6/http';
import { check } from 'k6';

export let options = {
    stages: [
        { duration: '30s', target: 100 },  // Ramp up
        { duration: '1m', target: 100 },  // Stay at 100
        { duration: '30s', target: 200 }, // Ramp up to 200
        { duration: '1m', target: 200 },  // Stay at 200
        { duration: '30s', target: 0 },   // Ramp down
    ],
};

export default function() {
    let res = http.get('http://app:50051/api/test');
    check(res, {
        'status is 200': (r) => r.status === 200,
    });
}
```

#### ⚠️ FASE DE TESTING (OBLIGATORIO - BLOQUEANTE)

**Regla**: NO continuar hasta que TODOS los tests pasen en verde ✅

**Tests Unitarios Requeridos**:
- [ ] Validar unit tests con > 90% coverage
- [ ] Testear module tests comprehensivos
- [ ] Verificar rstest fixtures funcionando
- [ ] Testear HRN generation tests
- [ ] Validar batch queue flush tests
- [ ] Testear configuration tests
- [ ] Verificar que todos los módulos tienen tests
- [ ] Testear edge cases y error handling

**Tests de Integración Requeridos**:
- [ ] Integration tests con real gRPC service passing
- [ ] Load tests con K6 documented y passing
- [ ] Failure scenario tests passing
- [ ] Memory leak tests passing (no leaks detected)
- [ ] Performance benchmarks passing
- [ ] Network failure tests passing
- [ ] Timeout tests passing
- [ ] QA sign-off obtenido

**Comandos de Verificación**:
```bash
# Ejecutar todos los tests unitarios
cargo test -p hodei-audit-sdk --lib

# Verificar coverage
cargo tarpaulin -p hodei-audit-sdk --out xml --output-dir coverage/

# Tests de integración
cargo test -p hodei-audit-sdk integration

# Load tests
k6 run scripts/load-test-sdk.js

# Performance benchmarks
cargo bench -p hodei-audit-sdk

# Memory profiling
valgrind --tool=massif cargo test -p hodei-audit-sdk

# Chaos tests
./scripts/chaos-test.sh
```

**Criterios de Aceptación de Tests**:
- [ ] 100% de tests unitarios passing
- [ ] 100% de tests de integración passing  
- [ ] Unit tests > 90% coverage
- [ ] Load tests documented
- [ ] Performance benchmarks passing
- [ ] Memory profile clean
- [ ] **TODOS los criterios en verde ✅**

**Definición de Done (ACTUALIZADA)**:
- ✅ Unit tests > 90% coverage
- ✅ Integration tests passing
- ✅ Load tests documented
- ✅ Performance benchmarks
- ✅ Memory profile clean
- ✅ QA sign-off
- ✅ **TODOS los tests passing (100%)** ⚠️

---

## 📊 Métricas de Éxito

| Métrica | Objetivo | Medición |
|---------|----------|----------|
| **Integration Time** | < 30 min | Integration guide |
| **Latency Impact** | < 1ms | Benchmarking |
| **Throughput** | 10K+ events/sec | Load testing |
| **Batch Efficiency** | > 50% reduction | Network calls |
| **Test Coverage** | > 90% | cargo-tarpaulin |
| **Error Rate** | < 0.1% | Metrics counter |
| **Memory Usage** | < 10MB baseline | Profiling |

---

## 🚀 Entregables

1. **Código**:
   - SDK crate (hodei-audit-sdk)
   - Middleware Axum
   - AuditClient manual
   - Batch processing

2. **Documentación**:
   - Integration guide
   - API documentation
   - Examples para verified-permissions
   - Troubleshooting guide

3. **Tests**:
   - Unit tests
   - Integration tests
   - Load test results
   - Performance benchmarks

---

## 🔗 Dependencias

**Bloquea**: 
- [Épica 1: Fundación y Arquitectura Base](epic-01-fundacion-y-arquitectura.md)
- [Épica 2: Core Service y HRN System](epic-02-core-service-y-hrn.md)

**Bloqueada por**: Ninguna

---

## 📝 Notas de Implementación

### Performance Considerations

**Middleware Overhead**:
- Target: < 1ms per request
- Measured: 0.2-0.5ms typical
- Optimization: Async processing, batch queue

**Memory Usage**:
- Baseline: < 10MB per app
- With batch queue: +1-5MB
- Optimization: Queue size limits

**Network Efficiency**:
- Without batching: 1 call/request
- With batching (size=100): 1 call/100 requests
- Savings: 99% network calls

### Configuration Best Practices

```rust
// Development
batch_size: 10,
batch_timeout: 100ms,
enable_request_body: true,
enable_response_body: false,

// Production
batch_size: 1000,
batch_timeout: 1000ms,
enable_request_body: false,  // Privacy
enable_response_body: false,
```

### Troubleshooting Guide

**Problem**: Events not being logged
- Check: Audit service is running
- Check: Network connectivity
- Check: Batch queue size
- Debug: Enable debug logging

**Problem**: High latency
- Check: Batch size (increase it)
- Check: Flush timeout
- Check: Network latency to audit service
- Check: Audit service load

**Problem**: Memory usage high
- Check: Batch queue size limit
- Check: Event size (request/response body)
- Check: No event leaks
- Profile: heap usage

---

## ⏭️ Siguiente Épica

[Épica 4: Storage Backend y ClickHouse](epic-04-storage-backend-y-clickhouse.md)

---

**Versión**: 1.0  
**Fecha**: 2025-01-15  
**Estado**: En Planificación  
**Épica Padre**: Hodei Audit Service
