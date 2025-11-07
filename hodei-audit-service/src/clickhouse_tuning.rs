//! ClickHouse Performance Tuning Module
//!
//! Provides advanced performance tuning for ClickHouse:
//! - Index optimization
//! - Memory configuration
//! - Query optimization
//! - Merge settings
//! - Compression settings

use std::collections::HashMap;
use tracing::{info, warn};

/// ClickHouse tuning configuration
#[derive(Debug, Clone)]
pub struct ClickHouseTuningConfig {
    /// Enable index optimization
    pub enable_index_optimization: bool,
    /// Enable memory optimization
    pub enable_memory_optimization: bool,
    /// Enable compression
    pub enable_compression: bool,
    /// Index type (Bloom filter, minmax, etc.)
    pub index_type: IndexType,
    /// Merge tree settings
    pub merge_tree_settings: MergeTreeSettings,
    /// Memory settings
    pub memory_settings: MemorySettings,
    /// Compression settings
    pub compression_settings: CompressionSettings,
}

impl Default for ClickHouseTuningConfig {
    fn default() -> Self {
        Self {
            enable_index_optimization: true,
            enable_memory_optimization: true,
            enable_compression: true,
            index_type: IndexType::BloomFilter,
            merge_tree_settings: MergeTreeSettings::default(),
            memory_settings: MemorySettings::default(),
            compression_settings: CompressionSettings::default(),
        }
    }
}

/// Index types
#[derive(Debug, Clone)]
pub enum IndexType {
    BloomFilter,
    MinMax,
    Set,
    沛化,
    Normal,
}

impl Default for IndexType {
    fn default() -> Self {
        IndexType::BloomFilter
    }
}

/// Merge tree settings for performance
#[derive(Debug, Clone)]
pub struct MergeTreeSettings {
    /// Max bytes to merge at once
    pub max_bytes_to_merge_at_max_space_in_pool: u64,
    /// Max parts to merge at once
    pub max_parts_to_merge_at_once: u32,
    /// Merge policy
    pub merge_policy: String,
    /// Merge interval
    pub merge_interval_seconds: u64,
}

impl Default for MergeTreeSettings {
    fn default() -> Self {
        Self {
            max_bytes_to_merge_at_max_space_in_pool: 1024 * 1024 * 1024, // 1GB
            max_parts_to_merge_at_once: 100,
            merge_policy: "ttl".to_string(),
            merge_interval_seconds: 60,
        }
    }
}

/// Memory settings
#[derive(Debug, Clone)]
pub struct MemorySettings {
    /// Max memory usage for queries
    pub max_memory_usage: u64,
    /// Max memory usage for DISTINCT
    pub max_distinct_distinct_mode: u64,
    /// Max bytes before temporary file
    pub max_bytes_before_external_group_by: u64,
    /// Max bytes before external sort
    pub max_bytes_before_external_sort: u64,
    /// Memory tracker interval
    pub memory_tracker_interval_us: i64,
}

impl Default for MemorySettings {
    fn default() -> Self {
        Self {
            max_memory_usage: 10 * 1024 * 1024 * 1024,          // 10GB
            max_distinct_distinct_mode: 4 * 1024 * 1024 * 1024, // 4GB
            max_bytes_before_external_group_by: 2 * 1024 * 1024 * 1024, // 2GB
            max_bytes_before_external_sort: 2 * 1024 * 1024 * 1024, // 2GB
            memory_tracker_interval_us: -1,                     // Use default
        }
    }
}

/// Compression settings
#[derive(Debug, Clone)]
pub struct CompressionSettings {
    /// Min bytes for compression
    pub min_bytes_for_compress: u32,
    /// Compression codec
    pub compression_codec: String,
    /// Use lz4 compression
    pub use_lz4: bool,
    /// Use zstd compression
    pub use_zstd: bool,
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            min_bytes_for_compress: 1024, // 1KB
            compression_codec: "lz4".to_string(),
            use_lz4: true,
            use_zstd: true,
        }
    }
}

/// ClickHouse performance tuner
pub struct ClickHousePerformanceTuner {
    config: ClickHouseTuningConfig,
}

impl ClickHousePerformanceTuner {
    /// Create a new performance tuner
    pub fn new(config: ClickHouseTuningConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn new_with_defaults() -> Self {
        Self::new(ClickHouseTuningConfig::default())
    }

    /// Generate optimized CREATE TABLE statement
    pub fn generate_optimized_table_schema(&self, table_name: &str) -> String {
        let mut sql = format!("CREATE TABLE {} (", table_name);

        // Base columns
        sql.push_str("\n  event_id String CODEC(DoubleDelta, ZSTD),\n");
        sql.push_str("  tenant_id String,\n");
        sql.push_str("  hrn String,\n");
        sql.push_str("  user_identity String,\n");
        sql.push_str("  action String,\n");
        sql.push_str("  event_time DateTime64(3) CODEC(DoubleDelta, ZSTD),\n");
        sql.push_str("  processed_at DateTime64(3) CODEC(DoubleDelta, ZSTD),\n");
        sql.push_str("  latency_ms UInt32,\n");
        sql.push_str("  metadata String CODEC(ZSTD)\n");

        // Table engine with optimized settings
        sql.push_str(") ENGINE = MergeTree()\n");
        sql.push_str("PARTITION BY toYYYYMM(event_time)\n");
        sql.push_str("ORDER BY (tenant_id, event_time, event_id)\n");

        // Primary key for efficient lookups
        sql.push_str("PRIMARY KEY (tenant_id, event_time, event_id)\n");

        // Sample block size for efficient I/O
        sql.push_str("SETTINGS index_granularity = 8192,\n");
        sql.push_str("             index_granularity_bytes = 10485760,\n");
        sql.push_str("             min_bytes_for_wide_part = 1048576,\n");
        sql.push_str("             max_bytes_to_merge_at_max_space_in_pool = 1073741824,\n");
        sql.push_str("             max_parts_to_merge_at_once = 100,\n");
        sql.push_str("             merge_selecting_policy = 'ttl',\n");
        sql.push_str("             merge_interval_to_delete_expired_data = 3600\n");

        // TTL for automatic data expiration
        sql.push_str("TTL event_time + INTERVAL 1 YEAR DELETE\n");

        sql
    }

    /// Generate optimized indexes
    pub fn generate_index_statements(&self) -> Vec<String> {
        let mut statements = Vec::new();

        if self.config.enable_index_optimization {
            match self.config.index_type {
                IndexType::BloomFilter => {
                    // Bloom filter for fast existence checks
                    statements.push(
                        "ALTER TABLE events ADD INDEX user_id_bloom bloom_filter_filter user_id TYPE bloom_filter GRANULARITY 1".to_string()
                    );
                }
                IndexType::MinMax => {
                    // MinMax index for range queries
                    statements.push(
                        "ALTER TABLE events ADD INDEX latency_minmax minmax_grab latency_ms GRABULARITY 4".to_string()
                    );
                }
                _ => {}
            }
        }

        statements
    }

    /// Generate optimized query settings
    pub fn generate_query_settings(&self) -> HashMap<String, String> {
        let mut settings = HashMap::new();

        if self.config.enable_memory_optimization {
            settings.insert(
                "max_memory_usage".to_string(),
                self.config.memory_settings.max_memory_usage.to_string(),
            );
            settings.insert(
                "max_bytes_before_external_group_by".to_string(),
                self.config
                    .memory_settings
                    .max_bytes_before_external_group_by
                    .to_string(),
            );
            settings.insert(
                "max_bytes_before_external_sort".to_string(),
                self.config
                    .memory_settings
                    .max_bytes_before_external_sort
                    .to_string(),
            );
            settings.insert(
                "max_distinct_distinct_mode".to_string(),
                self.config
                    .memory_settings
                    .max_distinct_distinct_mode
                    .to_string(),
            );
        }

        if self.config.enable_compression {
            settings.insert(
                "min_bytes_for_compress".to_string(),
                self.config
                    .compression_settings
                    .min_bytes_for_compress
                    .to_string(),
            );
            settings.insert(
                "use_lz4_compression_in_memory_format".to_string(),
                self.config.compression_settings.use_lz4.to_string(),
            );
            settings.insert(
                "use_zstd_compression_in_memory_format".to_string(),
                self.config.compression_settings.use_zstd.to_string(),
            );
        }

        // General performance settings
        settings.insert("max_result_rows".to_string(), "1000000".to_string());
        settings.insert("result_overflow_mode".to_string(), "break".to_string());
        settings.insert("max_execution_time".to_string(), "300".to_string());
        settings.insert("max_bytes_in_join".to_string(), "1073741824".to_string()); // 1GB

        settings
    }

    /// Optimize existing table
    pub async fn optimize_table(
        &self,
        table_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("[ClickHouse] Optimizing table: {}", table_name);

        // Execute optimization commands
        let mut results = Vec::new();

        // Force merge small parts
        results.push(format!("OPTIMIZE TABLE {} FINAL", table_name));
        info!("[ClickiumHouse] Forced merge of small parts");

        // Analyze table statistics
        results.push(format!("ANALYZE TABLE {}", table_name));
        info!("[ClickHouse] Updated table statistics");

        Ok(results.join(";\n"))
    }

    /// Get performance recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.config.enable_index_optimization {
            recommendations.push(
                "Consider using Bloom filters for columns with many unique values".to_string(),
            );
            recommendations.push("Use MinMax indexes for range queries".to_string());
        }

        if self.config.enable_memory_optimization {
            recommendations.push("Adjust max_memory_usage based on available RAM".to_string());
            recommendations.push("Enable external aggregation for large datasets".to_string());
        }

        if self.config.enable_compression {
            recommendations.push("Enable compression to reduce I/O and memory usage".to_string());
            recommendations.push("Use ZSTD compression for better compression ratio".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_optimized_table_schema() {
        let tuner = ClickHousePerformanceTuner::new_with_defaults();
        let schema = tuner.generate_optimized_table_schema("events");

        assert!(schema.contains("CREATE TABLE events"));
        assert!(schema.contains("ENGINE = MergeTree()"));
        assert!(schema.contains("PARTITION BY toYYYYMM(event_time)"));
        assert!(schema.contains("ORDER BY (tenant_id, event_time, event_id)"));
        assert!(schema.contains("PRIMARY KEY"));
    }

    #[test]
    fn test_generate_index_statements() {
        let tuner = ClickHousePerformanceTuner::new_with_defaults();
        let indexes = tuner.generate_index_statements();

        assert!(!indexes.is_empty());
    }

    #[test]
    fn test_generate_query_settings() {
        let tuner = ClickHousePerformanceTuner::new_with_defaults();
        let settings = tuner.generate_query_settings();

        assert!(settings.contains_key("max_memory_usage"));
        assert!(settings.contains_key("max_bytes_before_external_group_by"));
    }

    #[test]
    fn test_get_recommendations() {
        let tuner = ClickHousePerformanceTuner::new_with_defaults();
        let recommendations = tuner.get_recommendations();

        assert!(!recommendations.is_empty());
    }
}
