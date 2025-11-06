//! Hodei Audit SDK
//!
//! Middleware para captura de eventos de auditoría en aplicaciones cliente.
//!
//! # Ejemplo de Uso
//!
//! ```rust
//! use hodei_audit_sdk::{AuditClient, AuditConfig, EventBuilder};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AuditConfig::builder()
//!         .endpoint("http://localhost:50052")
//!         .tenant_id("tenant-123")
//!         .build();
//!
//!     let client = AuditClient::new(config).await?;
//!
//!     let event = EventBuilder::new()
//!         .event_name("UserLogin")
//!         .event_category(cloudtrail_patterns::EventCategory::Management)
//!         .hrn("hrn:hodei:auth:tenant-123:global:user/admin")
//!         .build()?;
//!
//!     client.publish_event(event).await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod middleware;
pub mod models;

pub use client::AuditClient;
pub use config::AuditConfig;
pub use error::AuditError;
pub use middleware::AuditMiddleware;
pub use models::{AuditEvent, EventBuilder};

/// Resultado de operaciones del SDK
pub type Result<T> = std::result::Result<T, AuditError>;
