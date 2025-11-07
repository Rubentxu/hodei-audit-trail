/**
 * Rate Limiting Middleware
 *
 * Advanced rate limiter with multiple algorithms:
 * - Fixed Window Counter
 * - Sliding Window Log
 * - Token Bucket
 *
 * Supports per-user, per-tenant, and per-endpoint limits with dynamic adjustment.
 */

export type RateLimitAlgorithm =
  | "fixed-window"
  | "sliding-window"
  | "token-bucket";

export interface RateLimitConfig {
  windowMs: number;
  maxRequests: number;
  algorithm: RateLimitAlgorithm;
  skipSuccessfulRequests?: boolean;
  skipFailedRequests?: boolean;
  burstCapacity?: number; // For token bucket
  refillRate?: number; // For token bucket
}

export interface RateLimitEntry {
  count: number;
  resetTime: number;
  lastRequest: number;
  requests: number[]; // For sliding window
  tokens: number; // For token bucket
}

export interface RateLimitMetrics {
  totalRequests: number;
  allowed: number;
  blocked: number;
  currentWindow: number;
  averageWindow: number;
  uniqueIdentifiers: number;
  topUsers: Array<{ identifier: string; count: number }>;
}

export interface RateLimitStats {
  totalRequests: number;
  allowed: number;
  blocked: number;
  blockRate: number;
  averageWindow: number;
  uniqueUsers: number;
}

class InMemoryRateLimiter {
  private store: Map<string, RateLimitEntry> = new Map();
  private config: RateLimitConfig;
  private metrics: RateLimitMetrics;
  private requestTimes: number[] = [];

  constructor(config: RateLimitConfig) {
    this.config = {
      algorithm: "fixed-window",
      skipSuccessfulRequests: false,
      skipFailedRequests: false,
      burstCapacity: config.maxRequests,
      refillRate: config.maxRequests,
      ...config,
    };

    this.metrics = {
      totalRequests: 0,
      allowed: 0,
      blocked: 0,
      currentWindow: 0,
      averageWindow: 0,
      uniqueIdentifiers: 0,
      topUsers: [],
    };
  }

  /**
   * Check if request is allowed using configured algorithm
   */
  isAllowed(key: string): {
    allowed: boolean;
    remaining: number;
    resetTime: number;
    limit: number;
  } {
    this.metrics.totalRequests++;
    this.requestTimes.push(Date.now());

    const now = Date.now();

    // Clean up old entries periodically
    if (this.metrics.totalRequests % 100 === 0) {
      this.cleanup();
    }

    let result: { allowed: boolean; remaining: number; resetTime: number };

    switch (this.config.algorithm) {
      case "sliding-window":
        result = this.checkSlidingWindow(key, now);
        break;
      case "token-bucket":
        result = this.checkTokenBucket(key, now);
        break;
      case "fixed-window":
      default:
        result = this.checkFixedWindow(key, now);
        break;
    }

    if (result.allowed) {
      this.metrics.allowed++;
    } else {
      this.metrics.blocked++;
    }

    return {
      ...result,
      limit: this.config.maxRequests,
    };
  }

  /**
   * Fixed Window Counter algorithm
   */
  private checkFixedWindow(
    key: string,
    now: number,
  ): { allowed: boolean; remaining: number; resetTime: number } {
    const entry = this.store.get(key);

    if (!entry || now > entry.resetTime) {
      // Initialize new window
      this.store.set(key, {
        count: 1,
        resetTime: now + this.config.windowMs,
        lastRequest: now,
        requests: [],
        tokens: this.config.burstCapacity || this.config.maxRequests,
      });
      return {
        allowed: true,
        remaining: this.config.maxRequests - 1,
        resetTime: now + this.config.windowMs,
      };
    }

    if (entry.count >= this.config.maxRequests) {
      return {
        allowed: false,
        remaining: 0,
        resetTime: entry.resetTime,
      };
    }

    entry.count++;
    entry.lastRequest = now;
    this.store.set(key, entry);

    return {
      allowed: true,
      remaining: this.config.maxRequests - entry.count,
      resetTime: entry.resetTime,
    };
  }

  /**
   * Sliding Window Log algorithm
   */
  private checkSlidingWindow(
    key: string,
    now: number,
  ): { allowed: boolean; remaining: number; resetTime: number } {
    const entry = this.store.get(key) || {
      count: 0,
      resetTime: now + this.config.windowMs,
      lastRequest: now,
      requests: [],
      tokens: this.config.burstCapacity || this.config.maxRequests,
    };

    // Remove old requests outside the window
    const windowStart = now - this.config.windowMs;
    entry.requests = entry.requests.filter(
      (timestamp) => timestamp > windowStart,
    );

    if (entry.requests.length >= this.config.maxRequests) {
      const resetTime = entry.requests[0] + this.config.windowMs;
      return {
        allowed: false,
        remaining: 0,
        resetTime,
      };
    }

    // Add current request
    entry.requests.push(now);
    entry.count = entry.requests.length;
    entry.lastRequest = now;
    this.store.set(key, entry);

    const remaining = this.config.maxRequests - entry.requests.length;
    const resetTime = entry.requests[0] + this.config.windowMs;

    return {
      allowed: true,
      remaining,
      resetTime,
    };
  }

  /**
   * Token Bucket algorithm
   */
  private checkTokenBucket(
    key: string,
    now: number,
  ): { allowed: boolean; remaining: number; resetTime: number } {
    const entry = this.store.get(key) || {
      count: 0,
      resetTime: now + this.config.windowMs,
      lastRequest: now,
      requests: [],
      tokens: this.config.burstCapacity || this.config.maxRequests,
    };

    // Refill tokens based on elapsed time
    const timePassed = now - entry.lastRequest;
    const refillRate = this.config.refillRate || this.config.maxRequests;
    const tokensToAdd = Math.floor(
      (timePassed * refillRate) / this.config.windowMs,
    );

    if (tokensToAdd > 0) {
      entry.tokens = Math.min(
        this.config.burstCapacity || this.config.maxRequests,
        entry.tokens + tokensToAdd,
      );
      entry.lastRequest = now;
    }

    if (entry.tokens <= 0) {
      return {
        allowed: false,
        remaining: 0,
        resetTime:
          entry.lastRequest +
          this.config.windowMs /
            (this.config.refillRate || this.config.maxRequests),
      };
    }

    // Consume a token
    entry.tokens--;
    entry.count++;
    this.store.set(key, entry);

    return {
      allowed: true,
      remaining: entry.tokens,
      resetTime:
        entry.lastRequest +
        this.config.windowMs /
          (this.config.refillRate || this.config.maxRequests),
    };
  }

  /**
   * Get remaining requests for a key
   */
  getRemaining(key: string): number {
    const entry = this.store.get(key);
    if (!entry) {
      return this.config.maxRequests;
    }

    const now = Date.now();

    switch (this.config.algorithm) {
      case "sliding-window":
        const windowStart = now - this.config.windowMs;
        const validRequests = entry.requests.filter(
          (timestamp) => timestamp > windowStart,
        );
        return Math.max(0, this.config.maxRequests - validRequests.length);

      case "token-bucket":
        return entry.tokens;

      case "fixed-window":
      default:
        if (now > entry.resetTime) {
          return this.config.maxRequests;
        }
        return Math.max(0, this.config.maxRequests - entry.count);
    }
  }

  /**
   * Clean up expired entries
   */
  cleanup(): void {
    const now = Date.now();

    switch (this.config.algorithm) {
      case "sliding-window":
        for (const [key, entry] of this.store.entries()) {
          const windowStart = now - this.config.windowMs;
          entry.requests = entry.requests.filter(
            (timestamp) => timestamp > windowStart,
          );

          if (entry.requests.length === 0 && now > entry.resetTime) {
            this.store.delete(key);
          } else {
            this.store.set(key, entry);
          }
        }
        break;

      case "token-bucket":
        for (const [key, entry] of this.store.entries()) {
          if (
            now > entry.resetTime &&
            entry.tokens ===
              (this.config.burstCapacity || this.config.maxRequests)
          ) {
            this.store.delete(key);
          }
        }
        break;

      case "fixed-window":
      default:
        for (const [key, entry] of this.store.entries()) {
          if (now > entry.resetTime) {
            this.store.delete(key);
          }
        }
    }
  }

  /**
   * Get comprehensive statistics
   */
  getStats(): RateLimitStats {
    this.updateMetrics();
    return {
      totalRequests: this.metrics.totalRequests,
      allowed: this.metrics.allowed,
      blocked: this.metrics.blocked,
      blockRate:
        this.metrics.totalRequests > 0
          ? (this.metrics.blocked / this.metrics.totalRequests) * 100
          : 0,
      averageWindow: this.metrics.averageWindow,
      uniqueUsers: this.metrics.uniqueIdentifiers,
    };
  }

  /**
   * Get detailed metrics
   */
  getMetrics(): RateLimitMetrics {
    this.updateMetrics();
    return { ...this.metrics };
  }

  /**
   * Reset all metrics
   */
  resetMetrics(): void {
    this.metrics = {
      totalRequests: 0,
      allowed: 0,
      blocked: 0,
      currentWindow: 0,
      averageWindow: 0,
      uniqueIdentifiers: 0,
      topUsers: [],
    };
    this.requestTimes = [];
  }

  /**
   * Update metrics
   */
  private updateMetrics(): void {
    this.metrics.uniqueIdentifiers = this.store.size;
    this.metrics.currentWindow = this.requestTimes.filter(
      (time) => Date.now() - time < this.config.windowMs,
    ).length;

    if (this.requestTimes.length > 0) {
      this.metrics.averageWindow =
        this.requestTimes.reduce((sum, time) => {
          return sum + (Date.now() - time);
        }, 0) / this.requestTimes.length;
    }

    // Calculate top users
    const userCounts = new Map<string, number>();
    for (const [key, entry] of this.store.entries()) {
      const identifier = key.split(":")[0];
      userCounts.set(
        identifier,
        (userCounts.get(identifier) || 0) + entry.count,
      );
    }

    this.metrics.topUsers = Array.from(userCounts.entries())
      .map(([identifier, count]) => ({ identifier, count }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);
  }

  /**
   * Dynamic limit adjustment
   */
  public adjustLimit(newMaxRequests: number): void {
    this.config.maxRequests = newMaxRequests;
    if (this.config.burstCapacity) {
      this.config.burstCapacity = newMaxRequests;
    }
    if (this.config.refillRate) {
      this.config.refillRate = newMaxRequests;
    }
  }

  /**
   * Get current configuration
   */
  getConfig(): RateLimitConfig {
    return { ...this.config };
  }
}

/**
 * Rate limiters for different endpoints with optimized algorithms
 */
const rateLimiters = {
  // General API: 100 requests per minute (sliding window)
  default: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 100,
    algorithm: "sliding-window",
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
  }),

  // Event queries: 60 requests per minute (fixed window - simpler for high throughput)
  events: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 60,
    algorithm: "fixed-window",
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
  }),

  // Analytics: 30 requests per minute (token bucket - smooths bursts)
  analytics: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 30,
    algorithm: "token-bucket",
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
    burstCapacity: 40,
    refillRate: 30,
  }),

  // Compliance: 20 requests per minute (sliding window - strict)
  compliance: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 20,
    algorithm: "sliding-window",
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
  }),

  // Auth: 10 requests per minute (sliding window - security critical)
  auth: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 10,
    algorithm: "sliding-window",
    skipSuccessfulRequests: false,
    skipFailedRequests: true, // Don't count failed auth attempts
  }),

  // Export: 5 requests per minute (token bucket - allows short bursts)
  export: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 5,
    algorithm: "token-bucket",
    skipSuccessfulRequests: false,
    skipFailedRequests: false,
    burstCapacity: 8,
    refillRate: 5,
  }),
};

/**
 * Per-user and per-tenant rate limiters
 */
export class PerUserRateLimiter {
  private userLimiters: Map<string, InMemoryRateLimiter> = new Map();
  private tenantLimiters: Map<string, InMemoryRateLimiter> = new Map();
  private globalLimiter: InMemoryRateLimiter;

  constructor(globalConfig: RateLimitConfig) {
    this.globalLimiter = new InMemoryRateLimiter(globalConfig);
  }

  /**
   * Get or create per-user rate limiter
   */
  getUserLimiter(userId: string, config: RateLimitConfig): InMemoryRateLimiter {
    if (!this.userLimiters.has(userId)) {
      this.userLimiters.set(userId, new InMemoryRateLimiter(config));
    }
    return this.userLimiters.get(userId)!;
  }

  /**
   * Get or create per-tenant rate limiter
   */
  getTenantLimiter(
    tenantId: string,
    config: RateLimitConfig,
  ): InMemoryRateLimiter {
    if (!this.tenantLimiters.has(tenantId)) {
      this.tenantLimiters.set(tenantId, new InMemoryRateLimiter(config));
    }
    return this.tenantLimiters.get(tenantId)!;
  }

  /**
   * Check rate limit across all levels
   */
  checkAll(
    userId: string,
    tenantId: string,
    globalKey: string,
    userKey: string,
    tenantKey: string,
  ): {
    global: {
      allowed: boolean;
      remaining: number;
      resetTime: number;
      limit: number;
    };
    user: {
      allowed: boolean;
      remaining: number;
      resetTime: number;
      limit: number;
    };
    tenant: {
      allowed: boolean;
      remaining: number;
      resetTime: number;
      limit: number;
    };
  } {
    const globalResult = this.globalLimiter.isAllowed(globalKey);
    const userLimiter = this.userLimiters.get(userId);
    const tenantLimiter = this.tenantLimiters.get(tenantId);

    const userResult = userLimiter
      ? userLimiter.isAllowed(userKey)
      : { allowed: true, remaining: 999, resetTime: 0, limit: 999 };
    const tenantResult = tenantLimiter
      ? tenantLimiter.isAllowed(tenantKey)
      : { allowed: true, remaining: 999, resetTime: 0, limit: 999 };

    return {
      global: globalResult,
      user: userResult,
      tenant: tenantResult,
    };
  }

  /**
   * Get global limiter
   */
  getGlobalLimiter(): InMemoryRateLimiter {
    return this.globalLimiter;
  }
}

/**
 * Get rate limiter for endpoint
 */
export function getRateLimiter(pathname: string): InMemoryRateLimiter {
  if (pathname.includes("/api/events/export")) {
    return rateLimiters.export;
  }
  if (pathname.includes("/api/events")) {
    return rateLimiters.events;
  }
  if (pathname.includes("/api/analytics")) {
    return rateLimiters.analytics;
  }
  if (pathname.includes("/api/compliance")) {
    return rateLimiters.compliance;
  }
  if (pathname.includes("/api/auth")) {
    return rateLimiters.auth;
  }
  return rateLimiters.default;
}

/**
 * Generate rate limit key
 */
export function generateRateLimitKey(
  request: Request,
  identifier?: string,
): string {
  // Use user ID if available, otherwise use IP
  const userId = identifier || "anonymous";
  return `ratelimit:${userId}:${request.url}`;
}

/**
 * Check rate limit
 */
export function checkRateLimit(
  request: Request,
  identifier?: string,
): { allowed: boolean; remaining: number; resetTime: number; limit: number } {
  const pathname = new URL(request.url).pathname;
  const limiter = getRateLimiter(pathname);
  const key = generateRateLimitKey(request, identifier);

  const result = limiter.isAllowed(key);

  return {
    allowed: result.allowed,
    remaining: result.remaining,
    resetTime: result.resetTime,
    limit: limiter.config.maxRequests,
  };
}

/**
 * Advanced multi-level rate limiting (global, tenant, user, endpoint)
 */
export function checkMultiLevelRateLimit(
  request: Request,
  userId: string,
  tenantId: string,
  endpoint: string,
): {
  allowed: boolean;
  global: {
    allowed: boolean;
    remaining: number;
    resetTime: number;
    limit: number;
  };
  tenant: {
    allowed: boolean;
    remaining: number;
    resetTime: number;
    limit: number;
  };
  user: {
    allowed: boolean;
    remaining: number;
    resetTime: number;
    limit: number;
  };
  endpoint: {
    allowed: boolean;
    remaining: number;
    resetTime: number;
    limit: number;
  };
  reason?: string;
} {
  const pathname = new URL(request.url).pathname;

  // Create a per-user rate limiter
  const perUserLimiter = new PerUserRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 1000,
    algorithm: "sliding-window",
  });

  // Check global limit
  const globalKey = `global:${request.url}`;
  const globalResult = perUserLimiter.getGlobalLimiter().isAllowed(globalKey);

  // Check tenant limit
  const tenantKey = `tenant:${tenantId}:${request.url}`;
  const tenantLimiter = perUserLimiter.getTenantLimiter(tenantId, {
    windowMs: 60 * 1000,
    maxRequests: 500,
    algorithm: "sliding-window",
  });
  const tenantResult = tenantLimiter.isAllowed(tenantKey);

  // Check user limit
  const userKey = `user:${userId}:${request.url}`;
  const userLimiter = perUserLimiter.getUserLimiter(userId, {
    windowMs: 60 * 1000,
    maxRequests: 200,
    algorithm: "sliding-window",
  });
  const userResult = userLimiter.isAllowed(userKey);

  // Check endpoint limit
  const endpointLimiter = getRateLimiter(pathname);
  const endpointKey = `endpoint:${endpoint}:${request.url}`;
  const endpointResult = endpointLimiter.isAllowed(endpointKey);

  // Determine if all checks pass
  const allowed =
    globalResult.allowed &&
    tenantResult.allowed &&
    userResult.allowed &&
    endpointResult.allowed;

  let reason: string | undefined;
  if (!allowed) {
    if (!globalResult.allowed) reason = "Global rate limit exceeded";
    else if (!tenantResult.allowed) reason = "Tenant rate limit exceeded";
    else if (!userResult.allowed) reason = "User rate limit exceeded";
    else if (!endpointResult.allowed) reason = "Endpoint rate limit exceeded";
  }

  return {
    allowed,
    global: globalResult,
    tenant: tenantResult,
    user: userResult,
    endpoint: endpointResult,
    reason,
  };
}

/**
 * Rate limiting middleware for Next.js
 */
export function createRateLimitMiddleware() {
  return async (request: Request, userId?: string, tenantId?: string) => {
    const result = checkRateLimit(request, userId);

    if (!result.allowed) {
      return {
        success: false,
        error: "Rate limit exceeded",
        resetTime: result.resetTime,
        limit: result.limit,
        remaining: 0,
      };
    }

    return {
      success: true,
      limit: result.limit,
      remaining: result.remaining,
      resetTime: result.resetTime,
    };
  };
}

/**
 * Rate limit headers for HTTP responses
 */
export function getRateLimitHeaders(result: {
  allowed: boolean;
  remaining: number;
  resetTime: number;
  limit: number;
}): Record<string, string> {
  const resetEpoch = Math.floor(result.resetTime / 1000);

  return {
    "x-ratelimit-limit": result.limit.toString(),
    "x-ratelimit-remaining": result.remaining.toString(),
    "x-ratelimit-reset": resetEpoch.toString(),
    "retry-after": result.allowed
      ? "0"
      : Math.ceil((result.resetTime - Date.now()) / 1000).toString(),
  };
}

/**
 * Get all rate limiter statistics
 */
export function getAllRateLimitStats(): Record<string, RateLimitStats> {
  const stats: Record<string, RateLimitStats> = {};

  for (const [name, limiter] of Object.entries(rateLimiters)) {
    stats[name] = limiter.getStats();
  }

  return stats;
}

/**
 * Reset all rate limiter metrics
 */
export function resetAllRateLimitMetrics(): void {
  for (const limiter of Object.values(rateLimiters)) {
    limiter.resetMetrics();
  }
}

/**
 * Dynamic rate limit adjustment
 */
export function adjustRateLimit(
  endpoint: string,
  newMaxRequests: number,
): boolean {
  const limiter = rateLimiters[endpoint as keyof typeof rateLimiters];
  if (limiter) {
    limiter.adjustLimit(newMaxRequests);
    return true;
  }
  return false;
}

/**
 * Rate limit tier management
 */
export class RateLimitTier {
  private tiers: Map<string, RateLimitConfig> = new Map();

  constructor() {
    // Free tier
    this.tiers.set("free", {
      windowMs: 60 * 1000,
      maxRequests: 50,
      algorithm: "fixed-window",
    });

    // Basic tier
    this.tiers.set("basic", {
      windowMs: 60 * 1000,
      maxRequests: 200,
      algorithm: "sliding-window",
    });

    // Pro tier
    this.tiers.set("pro", {
      windowMs: 60 * 1000,
      maxRequests: 1000,
      algorithm: "sliding-window",
    });

    // Enterprise tier
    this.tiers.set("enterprise", {
      windowMs: 60 * 1000,
      maxRequests: 5000,
      algorithm: "sliding-window",
    });
  }

  /**
   * Get configuration for a tier
   */
  getTierConfig(tier: string): RateLimitConfig | null {
    return this.tiers.get(tier) || null;
  }

  /**
   * Create limiter for a tier
   */
  createLimiterForTier(tier: string): InMemoryRateLimiter | null {
    const config = this.getTierConfig(tier);
    if (!config) {
      return null;
    }
    return new InMemoryRateLimiter(config);
  }

  /**
   * List all available tiers
   */
  getTiers(): string[] {
    return Array.from(this.tiers.keys());
  }

  /**
   * Add custom tier
   */
  addTier(name: string, config: RateLimitConfig): void {
    this.tiers.set(name, config);
  }
}

// Export default tier manager
export const rateLimitTier = new RateLimitTier();
