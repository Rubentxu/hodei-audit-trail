/**
 * Authentication Middleware for API Protection
 *
 * Protects API endpoints with JWT validation, RBAC, and security features.
 */

import { NextRequest, NextResponse } from "next/server";
import { getRateLimiter, generateRateLimitKey } from "./rate-limit";

export interface AuthUser {
  id: string;
  username: string;
  email: string;
  role: "admin" | "auditor" | "analyst" | "viewer";
  tenantId: string;
  permissions: string[];
}

export interface AuthContext {
  user: AuthUser | null;
  tenantId: string | null;
  apiKey?: string;
  ip: string;
  userAgent: string;
  method: string;
  path: string;
}

/**
 * Role-based permissions mapping
 */
const ROLE_PERMISSIONS: Record<string, string[]> = {
  admin: [
    "view:all",
    "manage:all",
    "delete:all",
    "export:all",
    "admin:all",
  ],
  auditor: [
    "view:events",
    "view:analytics",
    "view:compliance",
    "generate:reports",
    "export:events",
    "export:analytics",
  ],
  analyst: [
    "view:events",
    "view:analytics",
    "view:compliance",
    "query:events",
  ],
  viewer: [
    "view:events",
    "view:analytics",
  ],
};

/**
 * Required permissions for different endpoints
 */
const ENDPOINT_PERMISSIONS: Record<string, string[]> = {
  "/api/events": ["view:events"],
  "/api/events/export": ["export:events"],
  "/api/analytics": ["view:analytics"],
  "/api/analytics/query": ["view:analytics"],
  "/api/compliance": ["view:compliance"],
  "/api/compliance/reports": ["view:compliance", "generate:reports"],
  "/api/compliance/keys": ["view:compliance", "manage:compliance"],
  "/api/sse/stream": ["view:events"],
};

/**
 * Validate JWT token (mock implementation)
 */
async function validateToken(token: string): Promise<AuthUser | null> {
  try {
    // In a real implementation, verify JWT signature and expiration
    if (!token || !token.startsWith("token_")) {
      return null;
    }

    // Mock user based on token
    // In production, decode and verify the JWT
    const mockUsers: Record<string, AuthUser> = {
      "admin_token": {
        id: "user-1",
        username: "admin",
        email: "admin@example.com",
        role: "admin",
        tenantId: "tenant-1",
        permissions: ROLE_PERMISSIONS.admin,
      },
      "auditor_token": {
        id: "user-2",
        username: "auditor",
        email: "auditor@example.com",
        role: "auditor",
        tenantId: "tenant-1",
        permissions: ROLE_PERMISSIONS.auditor,
      },
      "analyst_token": {
        id: "user-3",
        username: "analyst",
        email: "analyst@example.com",
        role: "analyst",
        tenantId: "tenant-1",
        permissions: ROLE_PERMISSIONS.analyst,
      },
      "viewer_token": {
        id: "user-4",
        username: "viewer",
        email: "viewer@example.com",
        role: "viewer",
        tenantId: "tenant-1",
        permissions: ROLE_PERMISSIONS.viewer,
      },
    };

    return mockUsers[token] || null;
  } catch (error) {
    console.error("[Auth] Token validation error:", error);
    return null;
  }
}

/**
 * Extract token from request
 */
function extractToken(request: NextRequest): { token?: string; apiKey?: string } {
  // Try Authorization header
  const authHeader = request.headers.get("authorization");
  if (authHeader?.startsWith("Bearer ")) {
    return { token: authHeader.substring(7) };
  }

  // Try x-api-key header
  const apiKey = request.headers.get("x-api-key");
  if (apiKey) {
    return { apiKey };
  }

  // Try query parameter (not recommended for production)
  const queryToken = request.nextUrl.searchParams.get("token");
  if (queryToken) {
    return { token: queryToken };
  }

  return {};
}

/**
 * Extract tenant ID from request
 */
function extractTenantId(request: NextRequest, user: AuthUser | null): string {
  // From user object
  if (user?.tenantId) {
    return user.tenantId;
  }

  // From header
  const tenantHeader = request.headers.get("x-tenant-id");
  if (tenantHeader) {
    return tenantHeader;
  }

  // From query parameter
  const tenantQuery = request.nextUrl.searchParams.get("tenantId");
  if (tenantQuery) {
    return tenantQuery;
  }

  // Default tenant for demo
  return "tenant-1";
}

/**
 * Check if user has required permissions
 */
function hasPermission(user: AuthUser, requiredPermissions: string[]): boolean {
  if (user.role === "admin") {
    return true; // Admin has all permissions
  }

  return requiredPermissions.every((permission) =>
    user.permissions.includes(permission) || user.permissions.includes("manage:all")
  );
}

/**
 * Get required permissions for endpoint
 */
function getRequiredPermissions(path: string, method: string): string[] {
  // Check specific endpoint
  if (ENDPOINT_PERMISSIONS[path]) {
    return ENDPOINT_PERMISSIONS[path];
  }

  // Check with method
  const fullPath = `${path}:${method.toLowerCase()}`;
  if (ENDPOINT_PERMISSIONS[fullPath]) {
    return ENDPOINT_PERMISSIONS[fullPath];
  }

  // Default permissions based on method
  switch (method.toUpperCase()) {
    case "GET":
      return ["view:all"];
    case "POST":
      return ["manage:all"];
    case "PUT":
    case "PATCH":
      return ["manage:all"];
    case "DELETE":
      return ["delete:all"];
    default:
      return ["view:all"];
  }
}

/**
 * Log authentication event
 */
function logAuthEvent(context: AuthContext, success: boolean, reason?: string): void {
  const logEntry = {
    timestamp: new Date().toISOString(),
    ip: context.ip,
    userAgent: context.userAgent,
    method: context.method,
    path: context.path,
    userId: context.user?.id || "anonymous",
    tenantId: context.tenantId,
    success,
    reason: reason || (success ? "authorized" : "unauthorized"),
  };

  console.log("[Auth]", JSON.stringify(logEntry));
}

/**
 * Main authentication middleware
 */
export async function authenticate(
  request: NextRequest,
  requiredPermissions?: string[]
): Promise<{ context: AuthContext; response?: NextResponse }> {
  const ip = request.headers.get("x-forwarded-for") || request.ip || "unknown";
  const userAgent = request.headers.get("user-agent") || "unknown";
  const method = request.method;
  const path = new URL(request.url).pathname;

  // Check rate limiting before authentication
  const rateLimiter = getRateLimiter(request.url);
  const rateLimitKey = generateRateLimitKey(request);
  const rateLimitResult = rateLimiter.isAllowed(rateLimitKey);

  if (!rateLimitResult.allowed) {
    const response = NextResponse.json(
      {
        success: false,
        error: {
          code: "RATE_LIMIT_EXCEEDED",
          message: "Rate limit exceeded. Try again later.",
          retryAfter: Math.ceil((rateLimitResult.resetTime - Date.now()) / 1000),
        },
      },
      {
        status: 429,
        headers: {
          "Retry-After": String(Math.ceil((rateLimitResult.resetTime - Date.now()) / 1000)),
          "X-RateLimit-Limit": String(rateLimitResult.limit),
          "X-RateLimit-Remaining": String(rateLimitResult.remaining),
          "X-RateLimit-Reset": String(rateLimitResult.resetTime),
        },
      }
    );

    console.warn(`[Auth] Rate limit exceeded for ${ip} on ${path}`);
    return {
      context: {
        user: null,
        tenantId: null,
        ip,
        userAgent,
        method,
        path,
      },
      response,
    };
  }

  // Extract authentication credentials
  const { token, apiKey } = extractToken(request);

  // Validate token
  const user = token ? await validateToken(token) : null;

  // If API key provided but no token, validate API key
  let validUser = user;
  if (!validUser && apiKey) {
    // Mock API key validation
    if (apiKey === "demo_api_key") {
      validUser = {
        id: "api-user",
        username: "api_user",
        email: "api@example.com",
        role: "viewer",
        tenantId: "tenant-1",
        permissions: ROLE_PERMISSIONS.viewer,
      };
    }
  }

  // Extract tenant ID
  const tenantId = extractTenantId(request, validUser);

  // Create auth context
  const context: AuthContext = {
    user: validUser,
    tenantId,
    apiKey,
    ip,
    userAgent,
    method,
    path,
  };

  // If no permissions specified, get from endpoint
  const requiredPerms = requiredPermissions || getRequiredPermissions(path, method);

  // Check authentication
  if (!validUser) {
    logAuthEvent(context, false, "no_token");

    const response = NextResponse.json(
      {
        success: false,
        error: {
          code: "UNAUTHENTICATED",
          message: "Authentication required. Please provide a valid token.",
        },
      },
      {
        status: 401,
        headers: {
          "WWW-Authenticate": "Bearer",
          "X-RateLimit-Limit": String(rateLimitResult.limit),
          "X-RateLimit-Remaining": String(rateLimitResult.remaining - 1),
        },
      }
    );

    return { context, response };
  }

  // Check authorization
  if (!hasPermission(validUser, requiredPerms)) {
    logAuthEvent(context, false, "insufficient_permissions");

    const response = NextResponse.json(
      {
        success: false,
        error: {
          code: "FORBIDDEN",
          message: "Insufficient permissions to access this resource.",
          required: requiredPerms,
        },
      },
      {
        status: 403,
        headers: {
          "X-RateLimit-Limit": String(rateLimitResult.limit),
          "X-RateLimit-Remaining": String(rateLimitResult.remaining - 1),
        },
      }
    );

    return { context, response };
  }

  // Log successful authentication
  logAuthEvent(context, true);

  return { context };
}

/**
 * Middleware wrapper for Next.js API routes
 */
export function withAuth(
  handler: (request: NextRequest, context: AuthContext) => Promise<NextResponse>,
  requiredPermissions?: string[]
) {
  return async (request: NextRequest) => {
    const { context, response } = await authenticate(request, requiredPermissions);

    if (response) {
      return response;
    }

    return handler(request, context);
  };
}

/**
 * Get auth user from context
 */
export function getAuthUser(request: NextRequest): AuthUser | null {
  const authHeader = request.headers.get("authorization");
  if (!authHeader?.startsWith("Bearer ")) {
    return null;
  }

  const token = authHeader.substring(7);
  // In a real implementation, decode JWT and return user
  return null; // Placeholder
}

/**
 * Check if user has specific role
 */
export function hasRole(user: AuthUser, role: string): boolean {
  return user.role === role;
}

/**
 * Check if user has any of the specified roles
 */
export function hasAnyRole(user: AuthUser, roles: string[]): boolean {
  return roles.includes(user.role);
}
