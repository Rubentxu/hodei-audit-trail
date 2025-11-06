//! Example: verified-permissions con Hodei Audit SDK
//!
//! Este ejemplo muestra cómo integrar el Hodei Audit SDK
//! con una aplicación verified-permissions.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use hodei_audit_sdk::{AuditClient, AuditSdkConfig, AuditQuery};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio;

#[derive(Clone, State)]
struct AppState {
    audit_client: AuditClient,
}

// Handlers para verified-permissions endpoints

/// List policy stores
async fn list_policy_stores(
    State(state): State<AppState>,
) -> Result<Json<Vec<PolicyStore>>, StatusCode> {
    // Simular consulta a verified-permissions
    let policy_stores = vec![PolicyStore {
        id: "default".to_string(),
        name: "Default Policy Store".to_string(),
        description: None,
    }];

    // Log manual event (opcional, el middleware también lo hace)
    let event = hodei_audit_sdk::AuditEvent {
        event_name: "GET /v1/policy-stores".to_string(),
        event_category: 0,
        hrn: "hrn:hodei:verified-permissions:tenant-123:global:policy-store/list".to_string(),
        user_id: "system".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-123".to_string(),
        resource_path: "/v1/policy-stores".to_string(),
        http_method: Some("GET".to_string()),
        http_status: Some(200),
        source_ip: None,
        user_agent: None,
        additional_data: None,
    };

    let _ = state.audit_client.log(event).await;

    Ok(Json(policy_stores))
}

/// Create policy store
async fn create_policy_store(
    State(state): State<AppState>,
    Json(payload): Json<CreatePolicyStoreRequest>,
) -> Result<Json<PolicyStore>, StatusCode> {
    // Simular creación
    let policy_store = PolicyStore {
        id: "new-store".to_string(),
        name: payload.name,
        description: payload.description,
    };

    // Log event
    let event = hodei_audit_sdk::AuditEvent {
        event_name: "POST /v1/policy-stores".to_string(),
        event_category: 0,
        hrn: "hrn:hodei:verified-permissions:tenant-123:global:policy-store/list".to_string(),
        user_id: "admin".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-124".to_string(),
        resource_path: "/v1/policy-stores".to_string(),
        http_method: Some("POST".to_string()),
        http_status: Some(201),
        source_ip: None,
        user_agent: None,
        additional_data: Some(serde_json::json!({
            "store_name": policy_store.name
        })),
    };

    let _ = state.audit_client.log(event).await;

    Ok(Json(policy_store))
}

/// Get policy store by ID
async fn get_policy_store(
    State(state): State<AppState>,
    Path(store_id): Path<String>,
) -> Result<Json<PolicyStore>, StatusCode> {
    // Simular consulta
    let policy_store = PolicyStore {
        id: store_id.clone(),
        name: format!("Policy Store {}", store_id),
        description: None,
    };

    // Log event
    let event = hodei_audit_sdk::AuditEvent {
        event_name: format!("GET /v1/policy-stores/{}", store_id),
        event_category: 0,
        hrn: format!(
            "hrn:hodei:verified-permissions:tenant-123:global:policy-store/{}",
            store_id
        ),
        user_id: "admin".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-125".to_string(),
        resource_path: format!("/v1/policy-stores/{}", store_id),
        http_method: Some("GET".to_string()),
        http_status: Some(200),
        source_ip: None,
        user_agent: None,
        additional_data: None,
    };

    let _ = state.audit_client.log(event).await;

    Ok(Json(policy_store))
}

/// Update policy store
async fn update_policy_store(
    State(state): State<AppState>,
    Path(store_id): Path<String>,
    Json(payload): Json<UpdatePolicyStoreRequest>,
) -> Result<Json<PolicyStore>, StatusCode> {
    // Simular actualización
    let policy_store = PolicyStore {
        id: store_id.clone(),
        name: payload.name.unwrap_or_else(|| format!("Policy Store {}", store_id)),
        description: payload.description,
    };

    // Log event
    let event = hodei_audit_sdk::AuditEvent {
        event_name: format!("PUT /v1/policy-stores/{}", store_id),
        event_category: 0,
        hrn: format!(
            "hrn:hodei:verified-permissions:tenant-123:global:policy-store/{}",
            store_id
        ),
        user_id: "admin".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-126".to_string(),
        resource_path: format!("/v1/policy-stores/{}", store_id),
        http_method: Some("PUT".to_string()),
        http_status: Some(200),
        source_ip: None,
        user_agent: None,
        additional_data: Some(serde_json::json!({
            "updated_fields": ["name", "description"]
        })),
    };

    let _ = state.audit_client.log(event).await;

    Ok(Json(policy_store))
}

/// Delete policy store
async fn delete_policy_store(
    State(state): State<AppState>,
    Path(store_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // Simular eliminación
    // Log event
    let event = hodei_audit_sdk::AuditEvent {
        event_name: format!("DELETE /v1/policy-stores/{}", store_id),
        event_category: 0,
        hrn: format!(
            "hrn:hodei:verified-permissions:tenant-123:global:policy-store/{}",
            store_id
        ),
        user_id: "admin".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-127".to_string(),
        resource_path: format!("/v1/policy-stores/{}", store_id),
        http_method: Some("DELETE".to_string()),
        http_status: Some(204),
        source_ip: None,
        user_agent: None,
        additional_data: None,
    };

    let _ = state.audit_client.log(event).await;

    Ok(StatusCode::NO_CONTENT)
}

/// Authorization check
async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthorizeRequest>,
) -> Result<Json<AuthorizeResponse>, StatusCode> {
    // Simular authorization check
    let decision = if payload.principal == "User:alice" && payload.action == "read" {
        "allow"
    } else {
        "deny"
    };

    // Log authorization event
    let event = hodei_audit_sdk::AuditEvent {
        event_name: "POST /v1/authorize".to_string(),
        event_category: 1, // Data event
        hrn: "hrn:hodei:verified-permissions:tenant-123:global:authorization/check".to_string(),
        user_id: "system".to_string(),
        tenant_id: "tenant-123".to_string(),
        trace_id: "trace-128".to_string(),
        resource_path: "/v1/authorize".to_string(),
        http_method: Some("POST".to_string()),
        http_status: Some(200),
        source_ip: None,
        user_agent: None,
        additional_data: Some(serde_json::json!({
            "principal": payload.principal,
            "action": payload.action,
            "resource": payload.resource,
            "decision": decision,
            "policies_applied": ["policy-1", "policy-2"]
        })),
    };

    let _ = state.audit_client.log(event).await;

    Ok(Json(AuthorizeResponse {
        decision: decision.to_string(),
        obligations: vec![],
        advices: vec![],
    }))
}

// Data types

#[derive(Debug, Serialize, Deserialize)]
struct PolicyStore {
    id: String,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePolicyStoreRequest {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePolicyStoreRequest {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizeRequest {
    principal: String,
    action: String,
    resource: String,
    context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizeResponse {
    decision: String,
    obligations: Vec<String>,
    advices: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure audit
    println!("🚀 Starting verified-permissions with Hodei Audit SDK...");

    let audit_config = AuditSdkConfig::builder()
        .service_name("verified-permissions")
        .tenant_id("tenant-123")
        .audit_service_url("http://audit-service:50052")
        .batch_size(100)
        .batch_timeout(Duration::from_millis(100))
        .enable_request_body(false)
        .enable_response_body(false)
        .build()?;

    // 2. Create audit client (for manual logging)
    let audit_client = AuditClient::with_config(audit_config.clone()).await?;

    // 3. Create app with audit layer
    let app = Router::new()
        .route("/v1/policy-stores", get(list_policy_stores))
        .route("/v1/policy-stores", post(create_policy_store))
        .route("/v1/policy-stores/:id", get(get_policy_store))
        .route("/v1/policy-stores/:id", put(update_policy_store))
        .route("/v1/policy-stores/:id", delete(delete_policy_store))
        .route("/v1/authorize", post(authorize))
        // Add audit layer (captures ALL requests!)
        .layer(audit_config.layer())
        // Add state
        .with_state(AppState { audit_client });

    // 4. Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    println!("✅ Server running on http://{}", addr);
    println!("📊 Audit middleware enabled - all requests will be logged");
    println!("🔗 Health check: curl http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
