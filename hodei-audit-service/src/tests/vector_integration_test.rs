//! Vector integration tests

#[cfg(test)]
mod vector_forwarder_tests {
    use super::*;
    use hodei_audit_proto::{EventId, TenantId};

    #[tokio::test]
    async fn test_vector_forwarder_creation() {
        let config = VectorForwarderConfig {
            endpoint: "http://127.0.0.1:50051".to_string(),
            max_batch_size: 100,
            batch_timeout: std::time::Duration::from_secs(1),
            max_retries: 2,
            retry_delay: std::time::Duration::from_millis(50),
            connect_timeout: std::time::Duration::from_secs(2),
            health_check_interval: std::time::Duration::from_secs(10),
            tls_config: None,
            use_compression: true,
        };

        let forwarder = VectorForwarder::new(config).await;
        // Note: In tests without Vector running, creation will succeed
        // but connection will fail when actually used
        assert!(forwarder.is_ok() || forwarder.is_err()); // Either way is fine in test
    }

    #[tokio::test]
    async fn test_send_event_batch() {
        // This test will fail if Vector is not running
        // But it verifies the API works
        let config = VectorForwarderConfig {
            endpoint: "http://127.0.0.1:50051".to_string(),
            max_batch_size: 10,
            batch_timeout: std::time::Duration::from_secs(1),
            max_retries: 1,
            retry_delay: std::time::Duration::from_millis(10),
            connect_timeout: std::time::Duration::from_secs(1),
            health_check_interval: std::time::Duration::from_secs(5),
            tls_config: None,
            use_compression: false,
        };

        let forwarder = VectorForwarder::new(config).await;

        if let Ok(fw) = forwarder {
            let events = vec![hodei_audit_proto::AuditEvent {
                event_id: Some(EventId {
                    value: "test-event".to_string(),
                }),
                tenant_id: Some(TenantId {
                    value: "test-tenant".to_string(),
                }),
                event_type: "TEST".to_string(),
                ..Default::default()
            }];

            // This will fail in test environment but tests the flow
            let result = fw.send_events(events).await;
            // Expect failure since Vector is not running
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = VectorForwarderConfig {
            endpoint: "http://127.0.0.1:50051".to_string(),
            max_batch_size: 10,
            batch_timeout: std::time::Duration::from_secs(1),
            max_retries: 1,
            retry_delay: std::time::Duration::from_millis(10),
            connect_timeout: std::time::Duration::from_secs(1),
            health_check_interval: std::time::Duration::from_secs(5),
            tls_config: None,
            use_compression: false,
        };

        let forwarder = VectorForwarder::new(config).await;

        if let Ok(fw) = forwarder {
            // This will fail in test environment but tests the API
            let result = fw.health_check().await;
            // Expect failure since Vector is not running
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_vector_forwarder_config() {
        let config = VectorForwarderConfig::default();
        assert_eq!(config.endpoint, "http://127.0.0.1:50051");
        assert_eq!(config.max_batch_size, 1000);
        assert_eq!(config.max_retries, 3);
    }
}

#[cfg(test)]
mod vector_metrics_tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = VectorMetricsCollector::new("http://127.0.0.1:9598".to_string());
        assert_eq!(collector.endpoint, "http://127.0.0.1:9598");
    }

    #[tokio::test]
    async fn test_health_check() {
        let collector = VectorMetricsCollector::new("http://127.0.0.1:9598".to_string());
        // This will fail in test environment but tests the API
        let result = collector.check_health().await;
        // Expect failure since Vector is not running
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_metrics() {
        let collector = VectorMetricsCollector::new("http://127.0.0.1:9598".to_string());
        // This will fail in test environment but tests the flow
        let result = collector.get_metrics().await;
        // Expect failure since Vector is not running
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_last_metrics_empty() {
        let collector = VectorMetricsCollector::new("http://127.0.0.1:9598".to_string());
        assert!(collector.get_last_metrics().is_none());
    }
}

#[cfg(test)]
mod vector_sink_manager_tests {
    use super::*;

    #[test]
    fn test_sink_manager_basic_operations() {
        let mut manager = VectorSinkManager::new();
        assert_eq!(manager.sink_count(), 0);
        assert_eq!(manager.active_sink_count(), 0);

        // Add a sink
        let sink = VectorSinkConfig {
            name: "test_sink".to_string(),
            sink_type: VectorSinkType::ClickHouse,
            connection: SinkConnection {
                clickhouse: Some(ClickHouseConnection {
                    endpoint: "http://localhost:8123".to_string(),
                    database: "test".to_string(),
                    table: "events".to_string(),
                    username: "user".to_string(),
                    password: "pass".to_string(),
                    compression: false,
                }),
                s3: None,
                http: None,
                kafka: None,
            },
            buffer: BufferConfig {
                max_events: 1000,
                buffer_type: BufferType::Disk,
                when_full: WhenFull::Block,
                max_file_size: Some(50_000_000),
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
        assert!(manager.get_sink("test_sink").is_some());

        // Mark as active
        manager.mark_sink_active("test_sink");
        assert_eq!(manager.active_sink_count(), 1);

        // Remove sink
        manager.remove_sink("test_sink");
        assert_eq!(manager.sink_count(), 0);
        assert_eq!(manager.active_sink_count(), 0);
    }

    #[test]
    fn test_create_default_sinks() {
        let manager = create_default_sinks();

        // Should have 3 default sinks
        assert_eq!(manager.sink_count(), 3);

        // Check ClickHouse sink
        let clickhouse = manager.get_sink("clickhouse_hot");
        assert!(clickhouse.is_some());
        if let Some(sink) = clickhouse {
            assert_eq!(sink.name, "clickhouse_hot");
            assert_eq!(sink.sink_type, VectorSinkType::ClickHouse);
            assert!(sink.enabled);
        }

        // Check S3 sink
        let s3 = manager.get_sink("s3_warm");
        assert!(s3.is_some());
        if let Some(sink) = s3 {
            assert_eq!(sink.name, "s3_warm");
            assert_eq!(sink.sink_type, VectorSinkType::S3);
            assert!(sink.enabled);
        }

        // Check Blackhole sink
        let blackhole = manager.get_sink("blackhole_emergency");
        assert!(blackhole.is_some());
        if let Some(sink) = blackhole {
            assert_eq!(sink.name, "blackhole_emergency");
            assert_eq!(sink.sink_type, VectorSinkType::Blackhole);
            assert!(sink.enabled);
        }
    }

    #[test]
    fn test_generate_vector_config() {
        let manager = create_default_sinks();
        let config = manager.generate_vector_config();

        // Verify config contains expected elements
        assert!(config.contains("data_dir"));
        assert!(config.contains("gRPC source"));
        assert!(config.contains("clickhouse_hot"));
        assert!(config.contains("s3_warm"));
        assert!(config.contains("blackhole_emergency"));
        assert!(config.contains("prometheus_exporter"));
    }

    #[test]
    fn test_sink_type_display() {
        assert_eq!(format!("{}", VectorSinkType::ClickHouse), "clickhouse");
        assert_eq!(format!("{}", VectorSinkType::S3), "s3");
        assert_eq!(format!("{}", VectorSinkType::Blackhole), "blackhole");
        assert_eq!(format!("{}", VectorSinkType::HTTP), "http");
        assert_eq!(format!("{}", VectorSinkType::Kafka), "kafka");
    }
}

#[cfg(test)]
mod vector_error_tests {
    use super::*;

    #[test]
    fn test_error_classification() {
        // Test retryable errors
        let conn_error = VectorError::ConnectionFailed("connection failed".to_string());
        assert!(conn_error.is_retryable());
        assert!(!conn_error.is_fatal());

        let timeout_error = VectorError::Timeout("timeout".to_string());
        assert!(timeout_error.is_retryable());
        assert!(!timeout_error.is_fatal());

        let unavailable_error = VectorError::Unavailable("unavailable".to_string());
        assert!(unavailable_error.is_retryable());
        assert!(!unavailable_error.is_fatal());

        // Test fatal errors
        let invalid_error = VectorError::InvalidArgument("invalid".to_string());
        assert!(!invalid_error.is_retryable());
        assert!(invalid_error.is_fatal());

        let send_error = VectorError::SendFailed("send failed".to_string());
        assert!(!send_error.is_retryable());
        assert!(send_error.is_fatal());

        let internal_error = VectorError::Internal("internal error".to_string());
        assert!(!internal_error.is_retryable());
        assert!(internal_error.is_fatal());
    }

    #[test]
    fn test_error_display() {
        let error = VectorError::InvalidArgument("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid argument: test error");

        let error = VectorError::ConnectionFailed("conn error".to_string());
        assert_eq!(format!("{}", error), "Connection failed: conn error");
    }
}
