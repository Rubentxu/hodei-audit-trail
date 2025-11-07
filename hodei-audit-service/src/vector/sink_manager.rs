//! Vector sink configuration and management
//!
//! This module provides utilities for managing Vector sinks (ClickHouse, S3, Blackhole)
//! and handling multi-sink fan-out.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Sink configuration for Vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSinkConfig {
    /// Sink name
    pub name: String,
    /// Sink type
    pub sink_type: VectorSinkType,
    /// Connection details
    pub connection: SinkConnection,
    /// Buffer configuration
    pub buffer: BufferConfig,
    /// Retry policy
    pub retry: RetryConfig,
    /// Whether sink is enabled
    pub enabled: bool,
}

/// Types of Vector sinks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorSinkType {
    ClickHouse,
    S3,
    Blackhole,
    HTTP,
    Kafka,
}

impl std::fmt::Display for VectorSinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorSinkType::ClickHouse => write!(f, "clickhouse"),
            VectorSinkType::S3 => write!(f, "s3"),
            VectorSinkType::Blackhole => write!(f, "blackhole"),
            VectorSinkType::HTTP => write!(f, "http"),
            VectorSinkType::Kafka => write!(f, "kafka"),
        }
    }
}

/// Connection configuration for a sink
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkConnection {
    /// For ClickHouse
    pub clickhouse: Option<ClickHouseConnection>,
    /// For S3
    pub s3: Option<S3Connection>,
    /// For HTTP
    pub http: Option<HttpConnection>,
    /// For Kafka
    pub kafka: Option<KafkaConnection>,
}

/// ClickHouse connection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickHouseConnection {
    pub endpoint: String,
    pub database: String,
    pub table: String,
    pub username: String,
    pub password: String,
    pub compression: bool,
}

/// S3 connection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Connection {
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub key_prefix: Option<String>,
    pub compression: bool,
}

/// HTTP connection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConnection {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

/// Kafka connection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConnection {
    pub brokers: Vec<String>,
    pub topic: String,
    pub sasl: Option<HashMap<String, String>>,
}

/// Buffer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferConfig {
    /// Maximum events in buffer
    pub max_events: usize,
    /// Buffer type
    pub buffer_type: BufferType,
    /// When buffer is full
    pub when_full: WhenFull,
    /// Maximum file size for disk buffer
    pub max_file_size: Option<usize>,
}

/// Types of buffers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BufferType {
    Memory,
    Disk,
}

impl std::fmt::Display for BufferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferType::Memory => write!(f, "memory"),
            BufferType::Disk => write!(f, "disk"),
        }
    }
}

/// Behavior when buffer is full
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhenFull {
    Block,
    DropNewest,
    DropOldest,
}

impl std::fmt::Display for WhenFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WhenFull::Block => write!(f, "block"),
            WhenFull::DropNewest => write!(f, "drop_newest"),
            WhenFull::DropOldest => write!(f, "drop_oldest"),
        }
    }
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_interval: u64,
    pub max_interval: u64,
    pub multiplier: f64,
}

/// Vector sink manager
#[derive(Debug, Clone)]
pub struct VectorSinkManager {
    /// All configured sinks
    sinks: HashMap<String, VectorSinkConfig>,
    /// Active sinks (enabled and healthy)
    active_sinks: Vec<String>,
}

impl VectorSinkManager {
    /// Create a new sink manager
    pub fn new() -> Self {
        Self {
            sinks: HashMap::new(),
            active_sinks: Vec::new(),
        }
    }

    /// Add a sink configuration
    pub fn add_sink(&mut self, sink: VectorSinkConfig) {
        info!(sink = sink.name, "Adding Vector sink");
        self.sinks.insert(sink.name.clone(), sink);
    }

    /// Remove a sink
    pub fn remove_sink(&mut self, name: &str) {
        info!(sink = name, "Removing Vector sink");
        self.sinks.remove(name);
        self.active_sinks.retain(|s| s != name);
    }

    /// Get a sink by name
    pub fn get_sink(&self, name: &str) -> Option<&VectorSinkConfig> {
        self.sinks.get(name)
    }

    /// Get all sinks
    pub fn get_all_sinks(&self) -> HashMap<String, &VectorSinkConfig> {
        self.sinks.iter().map(|(k, v)| (k.clone(), v)).collect()
    }

    /// Get active sinks
    pub fn get_active_sinks(&self) -> Vec<&VectorSinkConfig> {
        self.active_sinks
            .iter()
            .filter_map(|name| self.sinks.get(name))
            .collect()
    }

    /// Mark sink as active
    pub fn mark_sink_active(&mut self, name: &str) {
        if !self.active_sinks.iter().any(|s| s == name) {
            self.active_sinks.push(name.to_string());
        }
    }

    /// Mark sink as inactive
    pub fn mark_sink_inactive(&mut self, name: &str) {
        self.active_sinks.retain(|s| s != name);
    }

    /// Get sink count
    pub fn sink_count(&self) -> usize {
        self.sinks.len()
    }

    /// Get active sink count
    pub fn active_sink_count(&self) -> usize {
        self.active_sinks.len()
    }

    /// Generate Vector TOML configuration
    pub fn generate_vector_config(&self) -> String {
        let mut config = String::new();

        // Add global settings
        config.push_str("data_dir = \"/var/lib/vector\"\n");
        config.push_str("log_schema = \"vector\"\n\n");

        // Add source (gRPC)
        config.push_str("# Source\ngRPC source from CAP\n");
        config.push_str("[sources.cap_grpc]\n");
        config.push_str("type = \"grpc_server\"\n");
        config.push_str("address = \"0.0.0.0:50051\"\n");
        config.push_str("decoding = { type = \"json\" }\n\n");

        // Add API
        config.push_str("# API\n[api]\n");
        config.push_str("enabled = true\n");
        config.push_str("address = \"0.0.0.0:9598\"\n\n");

        // Add transforms
        config.push_str("# Transform\n[transforms.enrich]\n");
        config.push_str("type = \"remap\"\n");
        config.push_str("inputs = [\"cap_grpc\"]\n");
        config.push_str("storage_tier = \"hot\"\n\n");

        // Add buffer configuration
        config.push_str("# Buffers\n[buffers.default]\n");
        config.push_str("type = \"disk\"\n");
        config.push_str("max_events = 50000\n");
        config.push_str("when_full = \"block\"\n\n");

        // Add sinks
        for (name, sink) in &self.sinks {
            if !sink.enabled {
                continue;
            }

            config.push_str(&format!("# Sink: {}\n[transforms.to_{}]\n", name, name));
            config.push_str("type = \"remap\"\n");
            config.push_str("inputs = [\"enrich\"]\n");
            config.push_str(&format!("{}_payload = {{\n", name.to_lowercase()));
            config.push_str("  \"event_id\" = .event_id.value,\n");
            config.push_str("  \"tenant_id\" = .tenant_id.value,\n");
            config.push_str("  \"timestamp\" = .timestamp,\n");
            config.push_str("  \"event_type\" = .event_type,\n");
            config.push_str("  \"event_data\" = to_string!(.event_data)\n");
            config.push_str("}\n\n");

            config.push_str(&format!("[sinks.{}]\n", name));
            config.push_str(&format!(
                "type = \"{}\"\n",
                match sink.sink_type {
                    VectorSinkType::ClickHouse => "clickhouse",
                    VectorSinkType::S3 => "aws_s3",
                    VectorSinkType::Blackhole => "blackhole",
                    VectorSinkType::HTTP => "http",
                    VectorSinkType::Kafka => "kafka",
                }
            ));
            config.push_str(&format!("inputs = [\"to_{}\"]\n", name));

            match sink.sink_type {
                VectorSinkType::ClickHouse => {
                    if let Some(conn) = &sink.connection.clickhouse {
                        config.push_str(&format!("endpoint = \"{}\"\n", conn.endpoint));
                        config.push_str(&format!("database = \"{}\"\n", conn.database));
                        config.push_str(&format!("table = \"{}\"\n", conn.table));
                        if conn.compression {
                            config.push_str("compression = \"gzip\"\n");
                        }
                    }
                }
                VectorSinkType::S3 => {
                    if let Some(conn) = &sink.connection.s3 {
                        config.push_str(&format!("bucket = \"{}\"\n", conn.bucket));
                        config.push_str(&format!("region = \"{}\"\n", conn.region));
                        if let Some(endpoint) = &conn.endpoint {
                            config.push_str(&format!("endpoint = \"{}\"\n", endpoint));
                        }
                        if conn.compression {
                            config.push_str("compression = \"gzip\"\n");
                        }
                    }
                }
                _ => {}
            }

            // Add buffer
            config.push_str(&format!("buffer = \"{}\"\n", "default"));

            // Add retry
            config.push_str("\n[config]\n");
            config.push_str(&format!("max_attempts = {}\n", sink.retry.max_attempts));
            config.push_str(&format!(
                "initial_interval_secs = {}\n",
                sink.retry.initial_interval
            ));
            config.push_str(&format!(
                "max_interval_secs = {}\n",
                sink.retry.max_interval
            ));
            config.push_str(&format!("multiplier = {}\n", sink.retry.multiplier));

            config.push_str("\n");
        }

        // Add metrics sink
        config.push_str("# Metrics\n[transforms._internal_metrics]\n");
        config.push_str("type = \"internal_metrics\"\n");
        config.push_str("namespace = \"vector\"\n\n");

        config.push_str("[sinks.prometheus]\n");
        config.push_str("type = \"prometheus_exporter\"\n");
        config.push_str("inputs = [\"_internal_metrics\"]\n");
        config.push_str("host = \"0.0.0.0\"\n");
        config.push_str("port = 9598\n");
        config.push_str("default_namespace = \"vector\"\n");

        config
    }
}

impl Default for VectorSinkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create default multi-sink configuration
pub fn create_default_sinks() -> VectorSinkManager {
    let mut manager = VectorSinkManager::new();

    // ClickHouse sink (Hot tier)
    manager.add_sink(VectorSinkConfig {
        name: "clickhouse_hot".to_string(),
        sink_type: VectorSinkType::ClickHouse,
        connection: SinkConnection {
            clickhouse: Some(ClickHouseConnection {
                endpoint: "http://clickhouse:8123".to_string(),
                database: "hodei_audit".to_string(),
                table: "audit_events".to_string(),
                username: "hodei".to_string(),
                password: "hodei123".to_string(),
                compression: true,
            }),
            s3: None,
            http: None,
            kafka: None,
        },
        buffer: BufferConfig {
            max_events: 10000,
            buffer_type: BufferType::Disk,
            when_full: WhenFull::Block,
            max_file_size: Some(52428800), // 50MB
        },
        retry: RetryConfig {
            max_attempts: 5,
            initial_interval: 1,
            max_interval: 10,
            multiplier: 2.0,
        },
        enabled: true,
    });

    // S3 sink (Warm/Cold tier)
    manager.add_sink(VectorSinkConfig {
        name: "s3_warm".to_string(),
        sink_type: VectorSinkType::S3,
        connection: SinkConnection {
            clickhouse: None,
            s3: Some(S3Connection {
                bucket: "hodei-audit-warm".to_string(),
                region: "us-east-1".to_string(),
                endpoint: Some("http://minio:9000".to_string()),
                access_key: Some("minioadmin".to_string()),
                secret_key: Some("minioadmin123".to_string()),
                key_prefix: Some("audit/".to_string()),
                compression: true,
            }),
            http: None,
            kafka: None,
        },
        buffer: BufferConfig {
            max_events: 50000,
            buffer_type: BufferType::Disk,
            when_full: WhenFull::Block,
            max_file_size: Some(104857600), // 100MB
        },
        retry: RetryConfig {
            max_attempts: 5,
            initial_interval: 1,
            max_interval: 10,
            multiplier: 2.0,
        },
        enabled: true,
    });

    // Blackhole sink (Emergency)
    manager.add_sink(VectorSinkConfig {
        name: "blackhole_emergency".to_string(),
        sink_type: VectorSinkType::Blackhole,
        connection: SinkConnection {
            clickhouse: None,
            s3: None,
            http: None,
            kafka: None,
        },
        buffer: BufferConfig {
            max_events: 1000,
            buffer_type: BufferType::Memory,
            when_full: WhenFull::DropNewest,
            max_file_size: None,
        },
        retry: RetryConfig {
            max_attempts: 0,
            initial_interval: 0,
            max_interval: 0,
            multiplier: 1.0,
        },
        enabled: true,
    });

    manager
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sink_manager_add_remove() {
        let mut manager = VectorSinkManager::new();
        assert_eq!(manager.sink_count(), 0);

        let sink = VectorSinkConfig {
            name: "test".to_string(),
            sink_type: VectorSinkType::ClickHouse,
            connection: SinkConnection {
                clickhouse: None,
                s3: None,
                http: None,
                kafka: None,
            },
            buffer: BufferConfig {
                max_events: 1000,
                buffer_type: BufferType::Memory,
                when_full: WhenFull::Block,
                max_file_size: None,
            },
            retry: RetryConfig {
                max_attempts: 3,
                initial_interval: 1,
                max_interval: 10,
                multiplier: 2.0,
            },
            enabled: true,
        };

        manager.add_sink(sink.clone());
        assert_eq!(manager.sink_count(), 1);
        assert!(manager.get_sink("test").is_some());

        manager.remove_sink("test");
        assert_eq!(manager.sink_count(), 0);
    }

    #[test]
    fn test_create_default_sinks() {
        let manager = create_default_sinks();
        assert_eq!(manager.sink_count(), 3);
        assert!(manager.get_sink("clickhouse_hot").is_some());
        assert!(manager.get_sink("s3_warm").is_some());
        assert!(manager.get_sink("blackhole_emergency").is_some());
    }

    #[test]
    fn test_buffer_type_display() {
        assert_eq!(format!("{}", BufferType::Memory), "memory");
        assert_eq!(format!("{}", BufferType::Disk), "disk");
    }
}
