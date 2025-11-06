//! HRN Resolver with LRU Cache
//!
//! This module provides the HrnResolver implementation with
//! LRU caching, pattern search, and async resolution.

use hodei_audit_types::hrn::{Hrn, HrnError, HrnMetadata, HrnResolver as HrnResolverTrait};
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info, warn};

/// Configuration for HRN Resolver
#[derive(Debug, Clone)]
pub struct HrnResolverConfig {
    /// Maximum number of entries in LRU cache
    pub cache_size: usize,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: u64,
    /// Whether to enable pattern search
    pub enable_pattern_search: bool,
}

/// Metadata store for HRN resolution
/// In a real implementation, this would query a database
#[derive(Debug, Clone, Default)]
pub struct InMemoryMetadataStore {
    /// In-memory store of HRN metadata
    metadata: HashMap<Hrn, HrnMetadata>,
}

impl InMemoryMetadataStore {
    /// Create a new in-memory metadata store
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or update metadata for an HRN
    pub fn upsert(&mut self, hrn: Hrn, metadata: HrnMetadata) {
        self.metadata.insert(hrn, metadata);
    }

    /// Get metadata for an HRN
    pub fn get(&self, hrn: &Hrn) -> Option<&HrnMetadata> {
        self.metadata.get(hrn)
    }

    /// Search for HRNs by pattern
    pub fn search_by_pattern(&self, pattern: &str) -> Vec<&HrnMetadata> {
        let mut results = Vec::new();
        let pattern = pattern.to_lowercase();

        for metadata in self.metadata.values() {
            let hrn_str = metadata.hrn.to_string().to_lowercase();
            if hrn_str.contains(&pattern) {
                results.push(metadata);
            }
        }

        results
    }

    /// Get all metadata
    pub fn get_all(&self) -> Vec<&HrnMetadata> {
        self.metadata.values().collect()
    }
}

/// Cache entry with timestamp
#[derive(Debug, Clone)]
struct CacheEntry {
    metadata: HrnMetadata,
    timestamp: std::time::Instant,
}

/// HRN Resolver with LRU cache
#[derive(Debug)]
pub struct HrnResolver {
    /// Configuration
    config: HrnResolverConfig,
    /// LRU cache for HRN metadata
    cache: Arc<Mutex<LruCache<Hrn, CacheEntry>>>,
    /// Metadata store
    store: Arc<RwLock<InMemoryMetadataStore>>,
    /// Cache hit counter
    hit_count: Arc<AtomicU64>,
    /// Cache miss counter
    miss_count: Arc<AtomicU64>,
}

impl HrnResolver {
    /// Create a new HRN resolver
    pub fn new(config: HrnResolverConfig) -> Self {
        info!(
            "Initializing HRN Resolver with cache_size={}, ttl_seconds={}",
            config.cache_size, config.ttl_seconds
        );

        let cache = LruCache::new(
            NonZeroUsize::new(config.cache_size).unwrap_or(NonZeroUsize::new(100).unwrap()),
        );
        let store = InMemoryMetadataStore::new();

        Self {
            config,
            cache: Arc::new(Mutex::new(cache)),
            store: Arc::new(RwLock::new(store)),
            hit_count: Arc::new(AtomicU64::new(0)),
            miss_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Create a new HRN resolver with default configuration
    pub fn new_default() -> Self {
        Self::new(HrnResolverConfig {
            cache_size: 10_000,
            ttl_seconds: 3600, // 1 hour
            enable_pattern_search: true,
        })
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let hit_count = self.hit_count.load(Ordering::Relaxed);
        let miss_count = self.miss_count.load(Ordering::Relaxed);
        let total = hit_count + miss_count;
        let hit_rate = if total > 0 {
            (hit_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        CacheStats {
            hits: hit_count,
            misses: miss_count,
            hit_rate,
        }
    }

    /// Manually invalidate cache entry for an HRN
    pub async fn invalidate(&self, hrn: &Hrn) {
        let mut cache = self.cache.lock().await;
        cache.pop(hrn);
        debug!("Invalidated cache entry for HRN: {}", hrn);
    }

    /// Manually invalidate all cache entries
    pub async fn invalidate_all(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
        info!("Invalidated all cache entries");
    }

    /// Seed the metadata store with sample data (for testing/demo)
    pub async fn seed_sample_data(&self) {
        let mut store = self.store.write().await;

        let sample_hrns = vec![
            (
                Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store/default")
                    .unwrap(),
                HrnMetadata {
                    hrn: Hrn::parse(
                        "hrn:hodei:verified-permissions:tenant-123:global:policy-store/default",
                    )
                    .unwrap(),
                    display_name: "Default Policy Store".to_string(),
                    description: Some("Default policy store for tenant-123".to_string()),
                    tags: {
                        let mut tags = std::collections::BTreeMap::new();
                        tags.insert("env".to_string(), "prod".to_string());
                        tags.insert("owner".to_string(), "security-team".to_string());
                        tags
                    },
                    owner: Some("security-team@company.com".to_string()),
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                },
            ),
            (
                Hrn::parse("hrn:hodei:api:tenant-123:eu-west-1:api/gateway").unwrap(),
                HrnMetadata {
                    hrn: Hrn::parse("hrn:hodei:api:tenant-123:eu-west-1:api/gateway").unwrap(),
                    display_name: "API Gateway".to_string(),
                    description: Some("Main API Gateway for tenant-123".to_string()),
                    tags: {
                        let mut tags = std::collections::BTreeMap::new();
                        tags.insert("env".to_string(), "prod".to_string());
                        tags.insert("region".to_string(), "eu-west-1".to_string());
                        tags
                    },
                    owner: Some("platform-team@company.com".to_string()),
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                },
            ),
            (
                Hrn::parse(
                    "hrn:hodei:verified-permissions:tenant-123:global:policy/default/allow-all",
                )
                .unwrap(),
                HrnMetadata {
                    hrn: Hrn::parse(
                        "hrn:hodei:verified-permissions:tenant-123:global:policy/default/allow-all",
                    )
                    .unwrap(),
                    display_name: "Allow All Policy".to_string(),
                    description: Some("Default allow-all policy".to_string()),
                    tags: {
                        let mut tags = std::collections::BTreeMap::new();
                        tags.insert("type".to_string(), "authorization".to_string());
                        tags
                    },
                    owner: Some("security-team@company.com".to_string()),
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                },
            ),
        ];

        for (hrn, metadata) in &sample_hrns {
            store.upsert(hrn.clone(), metadata.clone());
        }

        info!("Seeded metadata store with {} entries", sample_hrns.len());
    }

    /// Get cache size
    pub async fn get_cache_size(&self) -> usize {
        let cache = self.cache.lock().await;
        cache.len()
    }

    /// Check if an entry is expired
    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let elapsed = entry.timestamp.elapsed();
        elapsed.as_secs() > self.config.ttl_seconds
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

#[async_trait::async_trait]
impl HrnResolverTrait for HrnResolver {
    async fn resolve(&self, hrn: &Hrn) -> Result<HrnMetadata, HrnError> {
        // Try cache first
        let cache_entry_opt = {
            let mut cache = self.cache.lock().await;
            cache.get(hrn).cloned()
        };

        if let Some(entry) = cache_entry_opt {
            // Check if entry is expired
            if self.is_expired(&entry) {
                // Entry expired, will refresh from store
            } else {
                // Cache hit
                self.hit_count.fetch_add(1, Ordering::Relaxed);
                debug!("Cache hit for HRN: {}", hrn);
                return Ok(entry.metadata);
            }
        }

        // Cache miss or expired, check store
        debug!("Cache miss for HRN: {}", hrn);
        self.miss_count.fetch_add(1, Ordering::Relaxed);

        let store = self.store.read().await;
        if let Some(metadata) = store.get(hrn) {
            // Found in store, update cache
            let metadata = metadata.clone();
            drop(store);

            {
                let mut cache = self.cache.lock().await;
                cache.put(
                    hrn.clone(),
                    CacheEntry {
                        metadata: metadata.clone(),
                        timestamp: std::time::Instant::now(),
                    },
                );
            }

            Ok(metadata)
        } else {
            Err(HrnError::InvalidFormat {
                input: hrn.to_string(),
                reason: "HRN not found in metadata store".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hodei_audit_types::hrn::Hrn;

    #[tokio::test]
    async fn test_hrn_resolver_cache() {
        let resolver = HrnResolver::new(HrnResolverConfig {
            cache_size: 100,
            ttl_seconds: 3600,
            enable_pattern_search: true,
        });

        // Seed sample data
        resolver.seed_sample_data().await;

        // First resolve - cache miss
        let hrn =
            Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store/default")
                .unwrap();
        let metadata = resolver.resolve(&hrn).await.unwrap();
        assert_eq!(metadata.display_name, "Default Policy Store");

        // Check stats
        let stats = resolver.get_cache_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 1);

        // Second resolve - cache hit
        let metadata2 = resolver.resolve(&hrn).await.unwrap();
        assert_eq!(metadata2.display_name, "Default Policy Store");

        let stats = resolver.get_cache_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_hrn_resolver_invalidation() {
        let resolver = HrnResolver::new(HrnResolverConfig {
            cache_size: 100,
            ttl_seconds: 3600,
            enable_pattern_search: true,
        });

        resolver.seed_sample_data().await;

        let hrn =
            Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store/default")
                .unwrap();

        // Resolve to populate cache
        resolver.resolve(&hrn).await.unwrap();

        // Invalidate
        resolver.invalidate(&hrn).await;

        // Resolve again - should be cache miss
        resolver.resolve(&hrn).await.unwrap();

        let stats = resolver.get_cache_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 2);
    }

    #[tokio::test]
    async fn test_hrn_resolver_not_found() {
        let resolver = HrnResolver::new_default();

        let hrn =
            Hrn::parse("hrn:hodei:verified-permissions:tenant-123:global:policy-store/nonexistent")
                .unwrap();

        let result = resolver.resolve(&hrn).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let resolver = HrnResolver::new_default();
        resolver.seed_sample_data().await;

        let hrn = Hrn::parse("hrn:hodei:api:tenant-123:eu-west-1:api/gateway").unwrap();

        // Multiple resolves
        for _ in 0..5 {
            let _ = resolver.resolve(&hrn).await;
        }

        let stats = resolver.get_cache_stats();
        assert_eq!(stats.hits, 4); // First is miss, then 4 hits
        assert_eq!(stats.misses, 1);
        assert!(stats.hit_rate > 70.0); // Should be around 80%
    }
}
