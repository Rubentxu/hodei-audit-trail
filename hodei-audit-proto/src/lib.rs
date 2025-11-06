//! Protocol Buffer definitions for Hodei Audit Service
//!
//! This crate contains all gRPC service definitions and message types
//! for the Hodei Audit ecosystem, inspired by AWS CloudTrail patterns.

pub mod audit_event {
    include!("audit_event.rs");
}

pub mod audit_control {
    include!("audit_control.rs");
}

pub mod audit_query {
    include!("audit_query.rs");
}

pub mod audit_crypto {
    include!("audit_crypto.rs");
}

pub mod vector_api {
    include!("vector_api.rs");
}
