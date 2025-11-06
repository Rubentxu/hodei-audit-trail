//! Shared types for Hodei Audit Service
//!
//! This crate contains common types used across the hodei-audit ecosystem

pub mod hrn;

pub use hrn::{Hrn, HrnError, HrnMetadata, HrnResolver};
