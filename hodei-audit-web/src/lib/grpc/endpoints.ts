/**
 * gRPC API Endpoints Configuration
 *
 * Centralized configuration for all gRPC service endpoints.
 */

export const GRPC_ENDPOINTS = {
  // Base gRPC-web proxy URL
  BASE_URL: process.env.NEXT_PUBLIC_GRPC_BASE_URL || "http://localhost:8080",

  // Service-specific endpoints
  SERVICES: {
    EVENT: {
      HOST: process.env.NEXT_PUBLIC_GRPC_HOST_EVENTS || "http://localhost:8081",
      PATH: "/event.EventService",
    },
    ANALYTICS: {
      HOST: process.env.NEXT_PUBLIC_GRPC_HOST_ANALYTICS || "http://localhost:8082",
      PATH: "/analytics.AnalyticsService",
    },
    COMPLIANCE: {
      HOST: process.env.NEXT_PUBLIC_GRPC_HOST_COMPLIANCE || "http://localhost:8083",
      PATH: "/compliance.ComplianceService",
    },
    AUTH: {
      HOST: process.env.NEXT_PUBLIC_GRPC_HOST_AUTH || "http://localhost:8080",
      PATH: "/auth.AuthService",
    },
    HEALTH: {
      HOST: process.env.NEXT_PUBLIC_GRPC_HOST || "http://localhost:8080",
      PATH: "/health.HealthService",
    },
  },

  // gRPC-web proxy paths
  PROXY_PATHS: {
    NODE: "/grpc-web-node",
    WEB: "/grpc-web",
  },

  // Timeout configurations (in milliseconds)
  TIMEOUTS: {
    DEFAULT: 10000,
    EVENT_QUERY: 30000,
    ANALYTICS: 60000,
    COMPLIANCE: 90000,
    AUTH: 10000,
    HEALTH: 5000,
  },

  // Retry configurations
  RETRY: {
    ATTEMPTS: 3,
    DELAY: 1000,
    BACKOFF_MULTIPLIER: 2,
    MAX_DELAY: 10000,
  },

  // Rate limiting (requests per minute)
  RATE_LIMITS: {
    DEFAULT: 100,
    EVENT_QUERY: 60,
    ANALYTICS: 30,
    COMPLIANCE: 20,
    AUTH: 10,
  },
} as const;

/**
 * Get service endpoint configuration
 */
export function getServiceEndpoint(serviceName: keyof typeof GRPC_ENDPOINTS.SERVICES) {
  const service = GRPC_ENDPOINTS.SERVICES[serviceName];
  return {
    host: service.HOST,
    path: service.PATH,
    url: `${service.HOST}${GRPC_ENDPOINTS.PROXY_PATHS.NODE}`,
  };
}

/**
 * Get timeout for a service
 */
export function getServiceTimeout(serviceName: keyof typeof GRPC_ENDPOINTS.SERVICES): number {
  switch (serviceName) {
    case "EVENT":
      return GRPC_ENDPOINTS.TIMEOUTS.EVENT_QUERY;
    case "ANALYTICS":
      return GRPC_ENDPOINTS.TIMEOUTS.ANALYTICS;
    case "COMPLIANCE":
      return GRPC_ENDPOINTS.TIMEOUTS.COMPLIANCE;
    case "AUTH":
      return GRPC_ENDPOINTS.TIMEOUTS.AUTH;
    case "HEALTH":
      return GRPC_ENDPOINTS.TIMEOUTS.HEALTH;
    default:
      return GRPC_ENDPOINTS.TIMEOUTS.DEFAULT;
  }
}

/**
 * Get rate limit for a service
 */
export function getServiceRateLimit(serviceName: keyof typeof GRPC_ENDPOINTS.SERVICES): number {
  switch (serviceName) {
    case "EVENT":
      return GRPC_ENDPOINTS.RATE_LIMITS.EVENT_QUERY;
    case "ANALYTICS":
      return GRPC_ENDPOINTS.RATE_LIMITS.ANALYTICS;
    case "COMPLIANCE":
      return GRPC_ENDPOINTS.RATE_LIMITS.COMPLIANCE;
    case "AUTH":
      return GRPC_ENDPOINTS.RATE_LIMITS.AUTH;
    default:
      return GRPC_ENDPOINTS.RATE_LIMITS.DEFAULT;
  }
}

/**
 * Environment validation
 */
export function validateEnvironment(): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!process.env.NEXT_PUBLIC_GRPC_BASE_URL) {
    errors.push("NEXT_PUBLIC_GRPC_BASE_URL is not set");
  }

  if (!process.env.NEXT_PUBLIC_GRPC_HOST_EVENTS) {
    console.warn("NEXT_PUBLIC_GRPC_HOST_EVENTS is not set, using default");
  }

  if (!process.env.NEXT_PUBLIC_GRPC_HOST_ANALYTICS) {
    console.warn("NEXT_PUBLIC_GRPC_HOST_ANALYTICS is not set, using default");
  }

  if (!process.env.NEXT_PUBLIC_GRPC_HOST_COMPLIANCE) {
    console.warn("NEXT_PUBLIC_GRPC_HOST_COMPLIANCE is not set, using default");
  }

  if (!process.env.NEXT_PUBLIC_GRPC_HOST_AUTH) {
    console.warn("NEXT_PUBLIC_GRPC_HOST_AUTH is not set, using default");
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
