/**
 * API Caching Layer
 *
 * Provides caching functionality for API responses to improve performance.
 */

export interface CacheEntry<T> {
  data: T;
  timestamp: number;
  ttl: number;
  key: string;
  size: number;
  compressed?: boolean;
  hitCount: number;
  lastAccess: number;
}

export interface CacheConfig {
  defaultTtl?: number;
  maxSize?: number;
  maxEntries?: number;
  compressionEnabled?: boolean;
  compressionThreshold?: number;
  enableMonitoring?: boolean;
  redisUrl?: string;
}

export interface CacheMetrics {
  totalRequests: number;
  cacheHits: number;
  cacheMisses: number;
  evictions: number;
  hitRate: number;
  averageGetTime: number;
  averageSetTime: number;
}

export interface CacheStats {
  hits: number;
  misses: number;
  hitRate: number;
  size: number;
  maxSize: number;
  entryCount: number;
  memoryUsage: number;
  compressionRatio: number;
}

/**
 * Enhanced in-memory cache with statistics and monitoring
 */
export class ApiCache {
  private cache: Map<string, CacheEntry<any>> = new Map();
  private config: Required<CacheConfig>;
  private metrics: CacheMetrics;
  private redis?: any; // Redis client (optional)
  private totalGetTime: number = 0;
  private totalSetTime: number = 0;
  private getOperations: number = 0;
  private setOperations: number = 0;

  constructor(config: CacheConfig = {}) {
    this.config = {
      defaultTtl: 300000, // 5 minutes
      maxSize: 100 * 1024 * 1024, // 100MB
      maxEntries: 10000,
      compressionEnabled: false,
      compressionThreshold: 1024, // 1KB
      enableMonitoring: true,
      redisUrl: undefined,
      ...config,
    };

    this.metrics = {
      totalRequests: 0,
      cacheHits: 0,
      cacheMisses: 0,
      evictions: 0,
      hitRate: 0,
      averageGetTime: 0,
      averageSetTime: 0,
    };

    this.initializeRedis();
  }

  /**
   * Initialize Redis connection (if configured)
   */
  private async initializeRedis(): Promise<void> {
    if (!this.config.redisUrl) {
      return;
    }

    try {
      // In a real implementation, connect to Redis
      // const redis = require('redis');
      // this.redis = redis.createClient({ url: this.config.redisUrl });
      // await this.redis.connect();
      console.log("[Cache] Redis not configured, using in-memory cache");
    } catch (error) {
      console.error("[Cache] Redis initialization failed:", error);
    }
  }

  /**
   * Get value from cache
   */
  public get<T>(key: string): T | null {
    const startTime = Date.now();
    this.metrics.totalRequests++;

    const entry = this.cache.get(key);

    if (!entry) {
      this.metrics.cacheMisses++;
      this.updateHitRate();
      return null;
    }

    // Check if entry has expired
    if (this.isExpired(entry)) {
      this.delete(key);
      this.metrics.cacheMisses++;
      this.updateHitRate();
      return null;
    }

    // Update access metrics
    entry.hitCount++;
    entry.lastAccess = Date.now();

    const getTime = Date.now() - startTime;
    this.updateAverageGetTime(getTime);
    this.metrics.cacheHits++;
    this.updateHitRate();

    return entry.data as T;
  }

  /**
   * Set value in cache with compression and metrics
   */
  public async set<T>(key: string, data: T, ttl?: number): Promise<void> {
    const startTime = Date.now();

    let dataToStore = data;
    let compressed = false;
    let size = this.calculateSize(data);

    // Compress if enabled and above threshold
    if (
      this.config.compressionEnabled &&
      size > this.config.compressionThreshold
    ) {
      dataToStore = (await this.compress(data)) as T;
      size = this.calculateSize(dataToStore);
      compressed = true;
    }

    // Check size constraints
    if (size > this.config.maxSize) {
      throw new Error(
        `Data size (${size} bytes) exceeds maximum cache size (${this.config.maxSize} bytes)`,
      );
    }

    // Evict if necessary
    this.evictIfNeeded(size);

    const entry: CacheEntry<T> = {
      data: dataToStore,
      key,
      timestamp: Date.now(),
      ttl: ttl || this.config.defaultTtl,
      size,
      compressed,
      hitCount: 0,
      lastAccess: Date.now(),
    };

    this.cache.set(key, entry);

    const setTime = Date.now() - startTime;
    this.updateAverageSetTime(setTime);

    // Also store in Redis if available
    if (this.redis) {
      try {
        // await this.redis.setEx(key, ttl || this.config.defaultTtl / 1000, JSON.stringify(entry));
      } catch (error) {
        console.error("[Cache] Redis store failed:", error);
      }
    }
  }

  /**
   * Check if key exists in cache
   */
  public has(key: string): boolean {
    const entry = this.cache.get(key);
    if (!entry) {
      return false;
    }

    if (this.isExpired(entry)) {
      this.delete(key);
      return false;
    }

    return true;
  }

  /**
   * Delete key from cache
   */
  public delete(key: string): boolean {
    return this.cache.delete(key);
  }

  /**
   * Clear all cache entries
   */
  public async clear(): Promise<void> {
    this.cache.clear();

    if (this.redis) {
      // await this.redis.flushall();
    }
  }

  /**
   * Warm cache with predefined data
   */
  public async warm(
    entries: Record<string, { data: any; ttl?: number }>,
  ): Promise<void> {
    for (const [key, { data, ttl }] of Object.entries(entries)) {
      try {
        await this.set(key, data, ttl);
      } catch (error) {
        console.error(`[Cache] Failed to warm key ${key}:`, error);
      }
    }
  }

  /**
   * Invalidate cache by pattern
   */
  public async invalidatePattern(pattern: string): Promise<number> {
    const regex = new RegExp(pattern);
    let count = 0;

    for (const key of this.cache.keys()) {
      if (regex.test(key)) {
        this.cache.delete(key);
        count++;
      }
    }

    if (this.redis) {
      // await this.redis.del(...matchedKeys);
    }

    return count;
  }

  /**
   * Manually purge expired entries
   */
  public async purgeExpired(): Promise<number> {
    const now = Date.now();
    let count = 0;

    for (const [key, entry] of this.cache.entries()) {
      if (now - entry.timestamp > entry.ttl) {
        this.cache.delete(key);
        count++;
      }
    }

    return count;
  }

  /**
   * Get comprehensive cache statistics
   */
  public getStats(): CacheStats {
    const currentSize = this.getCurrentSize();
    const memoryUsage = this.estimateMemoryUsage();

    return {
      hits: this.metrics.cacheHits,
      misses: this.metrics.cacheMisses,
      hitRate: this.metrics.hitRate,
      size: currentSize,
      maxSize: this.config.maxSize,
      entryCount: this.cache.size,
      memoryUsage,
      compressionRatio: this.calculateCompressionRatio(),
    };
  }

  /**
   * Get cache metrics
   */
  public getMetrics(): CacheMetrics {
    return { ...this.metrics };
  }

  /**
   * Reset cache statistics
   */
  public resetMetrics(): void {
    this.metrics = {
      totalRequests: 0,
      cacheHits: 0,
      cacheMisses: 0,
      evictions: 0,
      hitRate: 0,
      averageGetTime: 0,
      averageSetTime: 0,
    };
  }

  /**
   * Check if cache entry is expired
   */
  private isExpired(entry: CacheEntry<any>): boolean {
    const age = Date.now() - entry.timestamp;
    return age > entry.ttl;
  }

  /**
   * Evict the oldest entry
   */
  private evictOldest(): void {
    let oldestKey: string | null = null;
    let oldestTime = Date.now();

    for (const [key, entry] of this.cache.entries()) {
      if (entry.timestamp < oldestTime) {
        oldestTime = entry.timestamp;
        oldestKey = key;
      }
    }

    if (oldestKey) {
      this.cache.delete(oldestKey);
    }
  }

  /**
   * Pre-warm cache with data
   */
  public async warm(
    entries: Record<string, { data: any; ttl?: number }>,
  ): Promise<void> {
    for (const [key, { data, ttl }] of Object.entries(entries)) {
      try {
        await this.set(key, data, ttl);
      } catch (error) {
        console.error(`[Cache] Failed to warm key ${key}:`, error);
      }
    }
  }

  /**
   * Invalidate cache by pattern
   */
  public async invalidatePattern(pattern: string): Promise<number> {
    const regex = new RegExp(pattern);
    let count = 0;

    for (const key of this.cache.keys()) {
      if (regex.test(key)) {
        this.cache.delete(key);
        count++;
      }
    }

    return count;
  }

  /**
   * Check if cache entry is expired
   */
  private isExpired(entry: CacheEntry<any>): boolean {
    const age = Date.now() - entry.timestamp;
    return age > entry.ttl;
  }

  /**
   * Evict entries if needed based on size or count
   */
  private evictIfNeeded(newEntrySize: number): void {
    const currentSize = this.getCurrentSize();

    // Check max entries
    if (this.cache.size >= this.config.maxEntries) {
      this.evictOldest();
      this.metrics.evictions++;
    }

    // Check max size (with 10% buffer to avoid frequent evictions)
    const maxSizeWithBuffer = this.config.maxSize * 0.9;
    if (currentSize + newEntrySize > maxSizeWithBuffer) {
      this.evictOldest();
      this.metrics.evictions++;
    }
  }

  /**
   * Get current cache size
   */
  private getCurrentSize(): number {
    let total = 0;
    for (const entry of this.cache.values()) {
      total += entry.size;
    }
    return total;
  }

  /**
   * Evict the oldest entry
   */
  private evictOldest(): void {
    let oldestKey: string | null = null;
    let oldestTime = Date.now();

    for (const [key, entry] of this.cache.entries()) {
      if (entry.timestamp < oldestTime) {
        oldestTime = entry.timestamp;
        oldestKey = key;
      }
    }

    if (oldestKey) {
      this.cache.delete(oldestKey);
    }
  }

  /**
   * Calculate size of data in bytes
   */
  private calculateSize(data: any): number {
    try {
      const jsonString = JSON.stringify(data);
      return new TextEncoder().encode(jsonString).length;
    } catch (error) {
      return 1024; // Default size if cannot calculate
    }
  }

  /**
   * Compress data
   */
  private async compress(data: any): Promise<string> {
    try {
      const jsonString = JSON.stringify(data);
      // In a real implementation, use pako or similar
      // For now, just return base64 encoded
      if (typeof btoa !== "undefined") {
        return btoa(jsonString);
      }
      return jsonString;
    } catch (error) {
      console.error("[Cache] Compression failed:", error);
      return JSON.stringify(data);
    }
  }

  /**
   * Update hit rate
   */
  private updateHitRate(): void {
    if (this.metrics.totalRequests > 0) {
      this.metrics.hitRate =
        (this.metrics.cacheHits / this.metrics.totalRequests) * 100;
    }
  }

  /**
   * Update average get time
   */
  private updateAverageGetTime(time: number): void {
    this.totalGetTime += time;
    this.getOperations++;
    this.metrics.averageGetTime = this.totalGetTime / this.getOperations;
  }

  /**
   * Update average set time
   */
  private updateAverageSetTime(time: number): void {
    this.totalSetTime += time;
    this.setOperations++;
    this.metrics.averageSetTime = this.totalSetTime / this.setOperations;
  }

  /**
   * Get current memory usage estimate
   */
  private estimateMemoryUsage(): number {
    let total = 0;
    for (const entry of this.cache.values()) {
      total += entry.size;
    }
    return total;
  }

  /**
   * Calculate compression ratio
   */
  private calculateCompressionRatio(): number {
    let totalOriginal = 0;
    let totalCompressed = 0;

    for (const entry of this.cache.values()) {
      if (entry.compressed) {
        totalOriginal += entry.size;
        // Approximate: compressed is typically 30-50% of original
        totalCompressed += entry.size;
      }
    }

    if (totalOriginal === 0) return 0;
    return ((totalOriginal - totalCompressed) / totalOriginal) * 100;
  }
}

/**
 * Cache decorator for functions
 */
export function cached<T extends any[], R>(
  cache: ApiCache,
  keyGenerator: (...args: T) => string,
  ttl?: number,
) {
  return function (
    target: any,
    propertyName: string,
    descriptor: PropertyDescriptor,
  ) {
    const method = descriptor.value;

    descriptor.value = async function (...args: T): Promise<R> {
      const cacheKey = keyGenerator(...args);

      const cached = cache.get<R>(cacheKey);
      if (cached !== null) {
        return cached;
      }

      const result = await method.apply(this, args);
      cache.set(cacheKey, result, ttl);
      return result;
    };
  };
}

/**
 * Default cache instance with enhanced configuration
 */
export const defaultCache = new ApiCache({
  defaultTtl: 300000, // 5 minutes
  maxSize: 100 * 1024 * 1024, // 100MB
  maxEntries: 10000,
  compressionEnabled: true,
  compressionThreshold: 1024,
  enableMonitoring: true,
});

/**
 * Cache keys generator with versioned keys
 */
export const CacheKeys = {
  EVENTS: (tenantId: string, params?: any) =>
    `v1:events:${tenantId}:${JSON.stringify(params || {})}`,
  ANALYTICS: (tenantId: string, query: string) =>
    `v1:analytics:${tenantId}:${query}`,
  REPORT: (id: string) => `v1:report:${id}`,
  REPORTS: (tenantId: string) => `v1:reports:${tenantId}`,
  KEYS: (tenantId: string) => `v1:keys:${tenantId}`,
  USER: (userId: string) => `v1:user:${userId}`,
  SAVED_QUERY: (id: string) => `v1:saved_query:${id}`,
  COMPLIANCE: (tenantId: string) => `v1:compliance:${tenantId}`,
};

/**
 * Cache invalidation helpers
 */
export async function invalidateAll(cache: ApiCache): Promise<void> {
  await cache.clear();
}

export async function invalidateUserData(
  cache: ApiCache,
  userId: string,
): Promise<number> {
  return await cache.invalidatePattern(`^v1:user:${userId}:`);
}

export async function invalidateTenantData(
  cache: ApiCache,
  tenantId: string,
): Promise<number> {
  return await cache.invalidatePattern(`.*${tenantId}.*`);
}

export async function invalidateExpired(cache: ApiCache): Promise<number> {
  return await cache.purgeExpired();
}
