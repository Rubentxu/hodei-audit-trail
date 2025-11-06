//! Shared types for Hodei Audit Service
//!
//! This crate contains common types used across the hodei-audit ecosystem

pub mod audit_event;
pub mod hrn;

pub use audit_event::{AuditEvent, AuditEventBuilder, EventCategory, Outcome};
pub use hrn::{Hrn, HrnError, HrnMetadata, HrnResolver};
