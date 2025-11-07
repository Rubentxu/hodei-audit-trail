//! Adapters (implementaciones) para el dominio criptográfico
//!
//! Estos adapters implementan los ports definidos,
//! conectando el dominio con la infraestructura.

pub mod sha256_hasher;
pub mod ed25519_signer;
pub mod in_memory_digest_chain;
