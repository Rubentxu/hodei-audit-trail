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
}

export interface CacheConfig {
  defaultTtl: number;
  maxSize: number;
  compressionEnabled: boolean;
}

/**
 * In-memory cache implementation
 */
export class ApiCache {
  private cache: Map<string, CacheEntry<any>> = new Map();
  private config: CacheConfig;

  constructor(config: CacheConfig) {
    this.config = {
      defaultTtl: 300000, // 5 minutes
      maxSize: 1000,
      compressionEnabled: false,
      ...config,
    };
  }

  /**
   * Get value from cache
   */
  public get<T>(key: string): T | null {
    const entry = this.cache.get(key);

    if (!entry) {
      return null;
    }

    // Check if entry has expired
    if (this.isExpired(entry)) {
      this.delete(key);
      return null;
    }

    return entry.data as T;
  }

  /**
   * Set value in cache
   */
  public set<T>(key: string, data: T, ttl?: number): void {
    // Evict entries if cache is full
    if (this.cache.size >= this.config.maxSize) {
      this.evictOldest();
    }

    const entry: CacheEntry<T> = {
      data,
      key,
      timestamp: Date.now(),
      ttl: ttl || this.config.defaultTtl,
    };

    this.cache.set(key, entry);
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
  public clear(): void {
    this.cache.clear();
  }

  /**
   * Get cache statistics
   */
  public getStats(): {
    size: number;
    maxSize: number;
    hitRate: number;
    missRate: number;
  } {
    // Note: In a real implementation, you'd track hits and misses
    return {
      size: this.cache.size,
      maxSize: this.config.maxSize,
      hitRate: 0, // TODO: Implement hit/miss tracking
      missRate: 0,
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
  public warm(entries: Record<string, { data: any; ttl?: number }>): void {
    Object.entries(entries).forEach(([key, { data, ttl }]) => {
      this.set(key, data, ttl);
    });
  }

  /**
   * Invalidate cache by pattern
   */
  public invalidatePattern(pattern: string): number {
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
}

/**
 * Cache decorator for functions
 */
export function cached<T extends any[], R>(
  cache: ApiCache,
  keyGenerator: (...args: T) => string,
  ttl?: number
) {
  return function (target: any, propertyName: string, descriptor: PropertyDescriptor) {
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
 * Default cache instance
 */
export const defaultCache = new ApiCache({
  defaultTtl: 300000, // 5 minutes
  maxSize: 1000,
  compressionEnabled: true,
});

/**
 * Cache keys generator
 */
export const CacheKeys = {
  EVENTS: (tenantId: string, params?: any) => `events:${tenantId}:${JSON.stringify(params || {})}`,
  ANALYTICS: (tenantId: string, query: string) => `analytics:${tenantId}:${query}`,
  REPORT: (id: string) => `report:${id}`,
  REPORTS: (tenantId: string) => `reports:${tenantId}`,
  KEYS: (tenantId: string) => `keys:${tenantId}`,
  USER: (userId: string) => `user:${userId}`,
  SAVED_QUERY: (id: string) => `saved_query:${id}`,
};

/**
 * Cache invalidation helpers
 */
export function invalidateAll(cache: ApiCache): void {
  cache.clear();
}

export function invalidateUserData(cache: ApiCache, userId: string): void {
  cache.invalidatePattern(`^user:${userId}:`);
}

export function invalidateTenantData(cache: ApiCache, tenantId: string): void {
  cache.invalidatePattern(`.*${tenantId}.*`);
}
