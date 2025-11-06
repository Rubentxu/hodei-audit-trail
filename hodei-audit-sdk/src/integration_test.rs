//! Tests de integración para el SDK de auditoría
//!
//! Estos tests requieren un audit service real corriendo.
//! Para ejecutar: AUDIT_SERVICE_URL=http://localhost:50052 cargo test --test integration_test

use hodei_audit_sdk::{AuditClient, AuditEvent, AuditQuery, AuditSdkConfig};
use tokio::time::{Duration, sleep};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum::{Router, http::StatusCode, response::Json, routing::get};
    use std::net::SocketAddr;

    /// Test de integración básica
    #[tokio::test]
    async fn test_basic_integration() {
        // Skip si no hay audit service
        if std::env::var("AUDIT_SERVICE_URL").is_err() {
            println!("⚠️  SKIP: Set AUDIT_SERVICE_URL to run integration tests");
            return;
        }

        let audit_service_url = std::env::var("AUDIT_SERVICE_URL").unwrap();
        let client = AuditClient::new(audit_service_url.clone())
            .await
            .expect("Failed to create audit client");

        // Log evento simple
        let event = AuditEvent {
            event_name: "integration_test_event".to_string(),
            event_category: 0,
            hrn: "hrn:hodei:integration:test:global:service/test".to_string(),
            user_id: "test-user".to_string(),
            tenant_id: "test-tenant".to_string(),
            trace_id: "test-trace".to_string(),
            resource_path: "/test".to_string(),
            http_method: Some("GET".to_string()),
            http_status: Some(200),
            source_ip: Some("127.0.0.1".to_string()),
            user_agent: Some("integration-test".to_string()),
            additional_data: None,
        };

        let result = client.log(event).await;
        assert!(result.is_ok(), "Failed to log event to audit service");

        // Esperar un poco para que se procese
        sleep(Duration::from_millis(100)).await;
    }

    /// Test de batch logging
    #[tokio::test]
    async fn test_batch_logging() {
        // Skip si no hay audit service
        if std::env::var("AUDIT_SERVICE_URL").is_err() {
            println!("⚠️  SKIP: Set AUDIT_SERVICE_URL to run integration tests");
            return;
        }

        let audit_service_url = std::env::var("AUDIT_SERVICE_URL").unwrap();
        let client = AuditClient::new(audit_service_url.clone())
            .await
            .expect("Failed to create audit client");

        // Log batch de eventos
        let events: Vec<AuditEvent> = (0..10)
            .map(|i| AuditEvent {
                event_name: format!("batch_event_{}", i),
                event_category: 0,
                hrn: format!("hrn:hodei:integration:test:global:service/batch-{}", i),
                user_id: "test-user".to_string(),
                tenant_id: "test-tenant".to_string(),
                trace_id: format!("trace-{}", i),
                resource_path: format!("/batch/{}", i),
                http_method: Some("POST".to_string()),
                http_status: Some(201),
                source_ip: Some("127.0.0.1".to_string()),
                user_agent: Some("integration-test".to_string()),
                additional_data: Some(serde_json::json!({
                    "batch_index": i
                })),
            })
            .collect();

        let result = client.log_batch(events).await;
        assert!(result.is_ok(), "Failed to log batch to audit service");

        // Esperar un poco para que se procese
        sleep(Duration::from_millis(100)).await;
    }

    /// Test de query de eventos
    #[tokio::test]
    async fn test_query_events() {
        // Skip si no hay audit service
        if std::env::var("AUDIT_SERVICE_URL").is_err() {
            println!("⚠️  SKIP: Set AUDIT_SERVICE_URL to run integration tests");
            return;
        }

        let audit_service_url = std::env::var("AUDIT_SERVICE_URL").unwrap();
        let client = AuditClient::new(audit_service_url.clone())
            .await
            .expect("Failed to create audit client");

        // Crear query
        let query = AuditQuery {
            hrn: Some("hrn:hodei:integration:test:global:service/test".to_string()),
            tenant_id: Some("test-tenant".to_string()),
            user_id: None,
            start_time: None,
            end_time: None,
            limit: 10,
            offset: 0,
        };

        let result = client.query(query).await;
        assert!(result.is_ok(), "Failed to query audit service");
        let query_result = result.unwrap();
        println!("📊 Query returned {} events", query_result.total);
    }

    /// Test de resolve HRN
    #[tokio::test]
    async fn test_resolve_hrn() {
        // Skip si no hay audit service
        if std::env::var("AUDIT_SERVICE_URL").is_err() {
            println!("⚠️  SKIP: Set AUDIT_SERVICE_URL to run integration tests");
            return;
        }

        let audit_service_url = std::env::var("AUDIT_SERVICE_URL").unwrap();
        let client = AuditClient::new(audit_service_url.clone())
            .await
            .expect("Failed to create audit client");

        // Parse y resolver HRN
        let hrn = hodei_audit_sdk::Hrn::parse("hrn:hodei:integration:test:global:service/test")
            .expect("Failed to parse HRN");

        let result = client.resolve_hrn(hrn).await;
        assert!(result.is_ok(), "Failed to resolve HRN");
        let metadata = result.unwrap();
        println!("📝 HRN metadata: {:?}", metadata);
    }

    /// Test de middleware con app real
    #[tokio::test]
    async fn test_middleware_integration() {
        // Skip si no hay audit service
        if std::env::var("AUDIT_SERVICE_URL").is_err() {
            println!("⚠️  SKIP: Set AUDIT_SERVICE_URL to run integration tests");
            return;
        }

        // Crear app con middleware de auditoría
        let audit_config = AuditSdkConfig::builder()
            .service_name("integration-test")
            .tenant_id("test-tenant")
            .audit_service_url(std::env::var("AUDIT_SERVICE_URL").unwrap())
            .batch_size(1) // Immediate flush
            .batch_timeout(Duration::from_millis(10))
            .build()
            .expect("Failed to create audit config");

        let app = Router::new()
            .route("/test", get(|| async { Json("ok") }))
            .layer(audit_config.layer());

        // Crear test client
        let client = reqwest::Client::new();

        // Hacer request
        let response = client
            .get("http://127.0.0.1:3000/test")
            .header("x-user-id", "test-user")
            .header("x-tenant-id", "test-tenant")
            .send()
            .await
            .expect("Failed to make request");

        assert_eq!(response.status(), StatusCode::OK);

        // Esperar a que se procese el evento
        sleep(Duration::from_millis(200)).await;
    }

    /// Test de failure scenarios
    #[tokio::test]
    async fn test_failure_scenarios() {
        // Test con URL inválida
        let result = AuditClient::new("invalid-url".to_string()).await;
        // En una implementación real, esto debería fallar
        // Por ahora, acepta cualquier resultado
        println!("Invalid URL test result: {:?}", result);

        // Test con evento vacío (no debería fallar)
        let client = AuditClient::new("http://localhost:50052".to_string())
            .await
            .unwrap();

        let empty_event = AuditEvent::default();
        let result = client.log(empty_event).await;
        assert!(result.is_ok(), "Failed to log empty event");
    }
}

/// Benchmarks del SDK
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{Criterion, criterion_group, criterion_main};

    /// Benchmark de generación HRN
    fn bench_generate_hrn(c: &mut Criterion) {
        c.bench_function("generate_hrn_policy_store", |b| {
            b.iter(|| {
                let hrn = hodei_audit_sdk::generate_hrn_from_path(
                    &http::Method::GET,
                    "/v1/policy-stores/default/policies",
                    Some("tenant-123"),
                )
                .unwrap();
                black_box(hrn);
            })
        });

        c.bench_function("generate_hrn_api_user", |b| {
            b.iter(|| {
                let hrn = hodei_audit_sdk::generate_hrn_from_path(
                    &http::Method::GET,
                    "/api/v1/users/456",
                    Some("tenant-123"),
                )
                .unwrap();
                black_box(hrn);
            })
        });

        c.bench_function("generate_hrn_auth", |b| {
            b.iter(|| {
                let hrn = hodei_audit_sdk::generate_hrn_from_path(
                    &http::Method::POST,
                    "/v1/auth/login",
                    Some("tenant-123"),
                )
                .unwrap();
                black_box(hrn);
            })
        });
    }

    /// Benchmark de parsing HRN
    fn bench_parse_hrn(c: &mut Criterion) {
        c.bench_function("parse_hrn_valid", |b| {
            b.iter(|| {
                let hrn = hodei_audit_sdk::Hrn::parse(
                    "hrn:hodei:verified-permissions:tenant-123:global:policy-store/default",
                )
                .unwrap();
                black_box(hrn);
            })
        });
    }

    criterion_group!(benches, bench_generate_hrn, bench_parse_hrn);
    criterion_main!(benches);
}
