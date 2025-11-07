//! Puertos (abstracciones) para el dominio criptográfico
//!
//! Estos ports definen las interfaces que el dominio necesita,
//! permitiendo la separación clara entre dominio e infraestructura.

pub mod hashing;
pub mod signing;
pub mod digest_chain;
