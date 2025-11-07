//! Gestión de claves criptográficas
//!
//! Módulo para gestión segura de claves Ed25519 con rotación automática.

pub mod adapters;
pub mod ports;

// Re-exports
pub use adapters::{file_key_store::FileKeyStore, standalone_key_manager::StandaloneKeyManager};
pub use ports::{key_manager, key_store};
