//! Vector error types and handling

use thiserror::Error;
use tonic::Status;

/// Result type for Vector operations
pub type VectorResult<T> = Result<T, VectorError>;

/// Error types for Vector operations
#[derive(Error, Debug, Clone)]
pub enum VectorError {
    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Connection to Vector failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Send operation failed
    #[error("Send failed: {0}")]
    SendFailed(String),

    /// Operation timed out
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Vector is unavailable
    #[error("Vector unavailable: {0}")]
    Unavailable(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl VectorError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            VectorError::ConnectionFailed(_)
                | VectorError::Timeout(_)
                | VectorError::Unavailable(_)
        )
    }

    /// Check if error is fatal (not retryable)
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            VectorError::InvalidArgument(_)
                | VectorError::SendFailed(_)
                | VectorError::Internal(_)
                | VectorError::Serialization(_)
                | VectorError::Deserialization(_)
        )
    }
}

impl From<tonic::Status> for VectorError {
    fn from(status: Status) -> Self {
        match status.code() {
            tonic::Code::InvalidArgument => {
                VectorError::InvalidArgument(status.message().to_string())
            }
            tonic::Code::Unavailable => VectorError::Unavailable(status.message().to_string()),
            tonic::Code::DeadlineExceeded => VectorError::Timeout(status.message().to_string()),
            tonic::Code::Internal => VectorError::Internal(status.message().to_string()),
            _ => VectorError::Internal(status.message().to_string()),
        }
    }
}

impl From<serde_json::Error> for VectorError {
    fn from(err: serde_json::Error) -> Self {
        VectorError::Serialization(err.to_string())
    }
}

impl From<tonic::transport::Error> for VectorError {
    fn from(err: tonic::transport::Error) -> Self {
        VectorError::ConnectionFailed(err.to_string())
    }
}

impl From<std::io::Error> for VectorError {
    fn from(err: std::io::Error) -> Self {
        VectorError::Internal(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for VectorError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        VectorError::ConnectionFailed(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_errors() {
        let retryable = VectorError::ConnectionFailed("connection lost".to_string());
        assert!(retryable.is_retryable());
        assert!(!retryable.is_fatal());

        let timeout = VectorError::Timeout("request timed out".to_string());
        assert!(timeout.is_retryable());
        assert!(!timeout.is_fatal());

        let unavailable = VectorError::Unavailable("service down".to_string());
        assert!(unavailable.is_retryable());
        assert!(!unavailable.is_fatal());
    }

    #[test]
    fn test_fatal_errors() {
        let invalid = VectorError::InvalidArgument("bad request".to_string());
        assert!(!invalid.is_retryable());
        assert!(invalid.is_fatal());

        let send_failed = VectorError::SendFailed("rejected".to_string());
        assert!(!send_failed.is_retryable());
        assert!(send_failed.is_fatal());

        let internal = VectorError::Internal("bug".to_string());
        assert!(!internal.is_retryable());
        assert!(internal.is_fatal());
    }
}
