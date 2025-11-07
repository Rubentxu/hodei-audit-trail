//! Structured Logging module
//!
//! This module provides comprehensive structured logging for the Hodei Audit Service:
//! - JSON structured logs
//! - Correlation IDs for request tracking
//! - Appropriate log levels
//! - Sensitive data filtering
//! - Centralized logging support (ELK/Fluentd)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp in ISO 8601 format
    pub timestamp: String,
    /// Log level
    pub level: LogLevel,
    /// Service name
    pub service: String,
    /// Correlation ID for request tracking
    pub correlation_id: Option<String>,
    /// Tenant ID (if applicable)
    pub tenant_id: Option<String>,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Log message
    pub message: String,
    /// Additional context fields
    pub context: HashMap<String, serde_json::Value>,
    /// Stack trace (for errors)
    pub stack_trace: Option<String>,
    /// Log source location
    pub source: String,
}

/// Log context builder
#[derive(Debug, Clone, Default)]
pub struct LogContext {
    correlation_id: Option<String>,
    tenant_id: Option<String>,
    user_id: Option<String>,
    context: HashMap<String, serde_json::Value>,
}

impl LogContext {
    /// Create a new log context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set correlation ID
    pub fn correlation_id(mut self, id: &str) -> Self {
        self.correlation_id = Some(id.to_string());
        self
    }

    /// Set tenant ID
    pub fn tenant_id(mut self, id: &str) -> Self {
        self.tenant_id = Some(id.to_string());
        self
    }

    /// Set user ID
    pub fn user_id(mut self, id: &str) -> Self {
        self.user_id = Some(id.to_string());
        self
    }

    /// Add context field
    pub fn field<T: Serialize>(mut self, key: &str, value: T) -> Self {
        match serde_json::to_value(value) {
            Ok(v) => {
                self.context.insert(key.to_string(), v);
            }
            Err(_) => {
                self.context.insert(
                    key.to_string(),
                    serde_json::Value::String("invalid".to_string()),
                );
            }
        }
        self
    }

    /// Build the context
    pub fn build(self) -> LogContext {
        self
    }
}

/// Sensitive data detector
#[derive(Debug, Clone)]
pub struct SensitiveDataDetector {
    /// Patterns to detect (e.g., passwords, tokens, etc.)
    patterns: Vec<String>,
}

impl SensitiveDataDetector {
    /// Create a new detector with default patterns
    pub fn new() -> Self {
        Self {
            patterns: vec![
                "password".to_string(),
                "passwd".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "key".to_string(),
                "api_key".to_string(),
                "access_token".to_string(),
                "refresh_token".to_string(),
                "credential".to_string(),
            ],
        }
    }

    /// Check if a field name contains sensitive data
    pub fn is_sensitive(&self, field_name: &str) -> bool {
        let field_name = field_name.to_lowercase();
        self.patterns
            .iter()
            .any(|pattern| field_name.contains(&pattern.to_lowercase()))
    }

    /// Mask sensitive data in a value
    pub fn mask(&self, field_name: &str, value: &str) -> String {
        if self.is_sensitive(field_name) {
            "[REDACTED]".to_string()
        } else {
            value.to_string()
        }
    }

    /// Filter sensitive data from context
    pub fn filter_context(
        &self,
        context: HashMap<String, serde_json::Value>,
    ) -> HashMap<String, serde_json::Value> {
        let mut filtered = HashMap::new();
        for (key, value) in context {
            if self.is_sensitive(&key) {
                filtered.insert(key, serde_json::Value::String("[REDACTED]".to_string()));
            } else {
                // Convert value to string, redact if sensitive
                match value {
                    serde_json::Value::String(s) => {
                        let masked = self.mask(&key, &s);
                        filtered.insert(key, serde_json::Value::String(masked));
                    }
                    _ => {
                        filtered.insert(key, value);
                    }
                }
            }
        }
        filtered
    }
}

/// Structured logger
#[derive(Debug, Clone)]
pub struct StructuredLogger {
    service_name: String,
    sensitive_detector: SensitiveDataDetector,
}

impl StructuredLogger {
    /// Create a new structured logger
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            sensitive_detector: SensitiveDataDetector::new(),
        }
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> String {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        // Convert to ISO 8601 format
        chrono::DateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos() as u32)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "invalid-timestamp".to_string())
    }

    /// Create a log entry
    fn create_log_entry(
        &self,
        level: LogLevel,
        message: String,
        context: Option<LogContext>,
        stack_trace: Option<String>,
        source: &str,
    ) -> LogEntry {
        let ctx = context.unwrap_or_default();
        let filtered_context = self.sensitive_detector.filter_context(ctx.context);

        LogEntry {
            timestamp: self.get_timestamp(),
            level,
            service: self.service_name.clone(),
            correlation_id: ctx.correlation_id,
            tenant_id: ctx.tenant_id,
            user_id: ctx.user_id,
            message,
            context: filtered_context,
            stack_trace,
            source: source.to_string(),
        }
    }

    /// Log at trace level
    pub fn trace(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry =
            self.create_log_entry(LogLevel::Trace, message.to_string(), context, None, source);
        self.write_log(entry);
    }

    /// Log at debug level
    pub fn debug(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry =
            self.create_log_entry(LogLevel::Debug, message.to_string(), context, None, source);
        self.write_log(entry);
    }

    /// Log at info level
    pub fn info(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry =
            self.create_log_entry(LogLevel::Info, message.to_string(), context, None, source);
        self.write_log(entry);
    }

    /// Log at warn level
    pub fn warn(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry =
            self.create_log_entry(LogLevel::Warn, message.to_string(), context, None, source);
        self.write_log(entry);
    }

    /// Log at error level
    pub fn error(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry =
            self.create_log_entry(LogLevel::Error, message.to_string(), context, None, source);
        self.write_log(entry);
    }

    /// Log at critical level
    pub fn critical(&self, message: &str, context: Option<LogContext>, source: &str) {
        let entry = self.create_log_entry(
            LogLevel::Critical,
            message.to_string(),
            context,
            None,
            source,
        );
        self.write_log(entry);
    }

    /// Log error with stack trace
    pub fn error_with_stack(
        &self,
        message: &str,
        context: Option<LogContext>,
        stack_trace: &str,
        source: &str,
    ) {
        let entry = self.create_log_entry(
            LogLevel::Error,
            message.to_string(),
            context,
            Some(stack_trace.to_string()),
            source,
        );
        self.write_log(entry);
    }

    /// Write log entry (in real implementation, this would send to ELK/Fluentd)
    fn write_log(&self, entry: LogEntry) {
        // In a real implementation, this would send the log to:
        // - Elasticsearch via Fluentd
        // - Logstash
        // - Or other centralized logging systems
        let json = serde_json::to_string(&entry).unwrap_or_default();
        println!("{}", json);
    }

    /// Export logs to JSON (for testing or manual review)
    pub fn export_logs(&self, entries: &[LogEntry]) -> String {
        serde_json::to_string_pretty(entries).unwrap_or_default()
    }
}

impl Default for StructuredLogger {
    fn default() -> Self {
        Self::new("hodei-audit-service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_log_context_builder() {
        let context = LogContext::new()
            .correlation_id("req-123")
            .tenant_id("tenant-1")
            .user_id("user-1")
            .field("key1", "value1")
            .field("key2", 42)
            .build();

        assert_eq!(context.correlation_id, Some("req-123".to_string()));
        assert_eq!(context.tenant_id, Some("tenant-1".to_string()));
        assert_eq!(context.user_id, Some("user-1".to_string()));
        assert!(context.context.contains_key("key1"));
        assert!(context.context.contains_key("key2"));
    }

    #[test]
    fn test_sensitive_data_detector() {
        let detector = SensitiveDataDetector::new();

        assert!(detector.is_sensitive("password"));
        assert!(detector.is_sensitive("api_key"));
        assert!(detector.is_sensitive("access_token"));
        assert!(!detector.is_sensitive("username"));
        assert!(!detector.is_sensitive("email"));
    }

    #[test]
    fn test_sensitive_data_masking() {
        let detector = SensitiveDataDetector::new();

        assert_eq!(detector.mask("password", "secret123"), "[REDACTED]");
        assert_eq!(detector.mask("username", "john"), "john");
    }

    #[test]
    fn test_context_filtering() {
        let detector = SensitiveDataDetector::new();
        let mut context = HashMap::new();
        context.insert(
            "password".to_string(),
            serde_json::Value::String("secret".to_string()),
        );
        context.insert(
            "username".to_string(),
            serde_json::Value::String("john".to_string()),
        );
        context.insert(
            "token".to_string(),
            serde_json::Value::String("abc123".to_string()),
        );

        let filtered = detector.filter_context(context);

        assert_eq!(
            filtered.get("password"),
            Some(&serde_json::Value::String("[REDACTED]".to_string()))
        );
        assert_eq!(
            filtered.get("username"),
            Some(&serde_json::Value::String("john".to_string()))
        );
        assert_eq!(
            filtered.get("token"),
            Some(&serde_json::Value::String("[REDACTED]".to_string()))
        );
    }

    #[test]
    fn test_structured_logger_info() {
        let logger = StructuredLogger::new("test-service");

        let context = LogContext::new()
            .correlation_id("req-123")
            .tenant_id("tenant-1")
            .build();

        logger.info("Test info message", Some(context), "test.rs:10");
    }

    #[test]
    fn test_structured_logger_error() {
        let logger = StructuredLogger::new("test-service");

        let context = LogContext::new()
            .correlation_id("req-456")
            .field("error_code", 500)
            .build();

        logger.error("Test error message", Some(context), "test.rs:20");
    }

    #[test]
    fn test_log_entry_serialization() {
        let mut context = HashMap::new();
        context.insert(
            "key".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        let entry = LogEntry {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            level: LogLevel::Info,
            service: "test-service".to_string(),
            correlation_id: Some("req-123".to_string()),
            tenant_id: Some("tenant-1".to_string()),
            user_id: Some("user-1".to_string()),
            message: "Test message".to_string(),
            context,
            stack_trace: None,
            source: "test.rs:10".to_string(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("Test message"));
        assert!(json.contains("test-service"));
    }

    #[test]
    fn test_log_entry_with_sensitive_data() {
        let logger = StructuredLogger::new("test-service");

        let context = LogContext::new()
            .correlation_id("req-123")
            .field("password", "secret123")
            .field("username", "john")
            .build();

        // This should redact the password
        logger.info("Login attempt", Some(context), "test.rs:30");
    }
}
