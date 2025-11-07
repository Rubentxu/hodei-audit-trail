/**
 * Rate Limiting Middleware
 *
 * Simple in-memory rate limiter for API endpoints.
 */

interface RateLimitConfig {
  windowMs: number;
  maxRequests: number;
  skipSuccessfulRequests?: boolean;
  skipFailedRequests?: boolean;
}

interface RateLimitEntry {
  count: number;
  resetTime: number;
  lastRequest: number;
}

class InMemoryRateLimiter {
  private store: Map<string, RateLimitEntry> = new Map();
  private config: RateLimitConfig;

  constructor(config: RateLimitConfig) {
    this.config = config;
  }

  /**
   * Check if request is allowed
   */
  isAllowed(key: string): { allowed: boolean; remaining: number; resetTime: number } {
    const now = Date.now();
    const entry = this.store.get(key);

    if (!entry || now > entry.resetTime) {
      // Initialize new window
      this.store.set(key, {
        count: 1,
        resetTime: now + this.config.windowMs,
        lastRequest: now,
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
   * Get remaining requests for a key
   */
  getRemaining(key: string): number {
    const entry = this.store.get(key);
    if (!entry) {
      return this.config.maxRequests;
    }

    const now = Date.now();
    if (now > entry.resetTime) {
      return this.config.maxRequests;
    }

    return Math.max(0, this.config.maxRequests - entry.count);
  }

  /**
   * Clean up expired entries
   */
  cleanup(): void {
    const now = Date.now();
    for (const [key, entry] of this.store.entries()) {
      if (now > entry.resetTime) {
        this.store.delete(key);
      }
    }
  }
}

/**
 * Rate limiters for different endpoints
 */
const rateLimiters = {
  // General API: 100 requests per minute
  default: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 100,
  }),

  // Event queries: 60 requests per minute
  events: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 60,
  }),

  // Analytics: 30 requests per minute
  analytics: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 30,
  }),

  // Compliance: 20 requests per minute
  compliance: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 20,
  }),

  // Auth: 10 requests per minute
  auth: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 10,
  }),

  // Export: 5 requests per minute
  export: new InMemoryRateLimiter({
    windowMs: 60 * 1000,
    maxRequests: 5,
  }),
};

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
  identifier?: string
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
  identifier?: string
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
