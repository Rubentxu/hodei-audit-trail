/**
 * Error Handling and Response Standardization
 *
 * Provides standardized error responses and error tracking.
 */

import { NextRequest, NextResponse } from "next/server";

/**
 * Standard error codes
 */
export const ERROR_CODES = {
  // Authentication
  UNAUTHENTICATED: "UNAUTHENTICATED",
  INVALID_TOKEN: "INVALID_TOKEN",
  TOKEN_EXPIRED: "TOKEN_EXPIRED",

  // Authorization
  FORBIDDEN: "FORBIDDEN",
  INSUFFICIENT_PERMISSIONS: "INSUFFICIENT_PERMISSIONS",

  // Validation
  VALIDATION_ERROR: "VALIDATION_ERROR",
  INVALID_INPUT: "INVALID_INPUT",
  MISSING_FIELD: "MISSING_FIELD",

  // Resources
  NOT_FOUND: "NOT_FOUND",
  ALREADY_EXISTS: "ALREADY_EXISTS",
  CONFLICT: "CONFLICT",

  // Rate limiting
  RATE_LIMIT_EXCEEDED: "RATE_LIMIT_EXCEEDED",
  QUOTA_EXCEEDED: "QUOTA_EXCEEDED",

  // Server errors
  INTERNAL_ERROR: "INTERNAL_ERROR",
  SERVICE_UNAVAILABLE: "SERVICE_UNAVAILABLE",
  TIMEOUT: "TIMEOUT",
  DATABASE_ERROR: "DATABASE_ERROR",

  // Business logic
  INVALID_OPERATION: "INVALID_OPERATION",
  BUSINESS_RULE_VIOLATION: "BUSINESS_RULE_VIOLATION",

  // External
  EXTERNAL_SERVICE_ERROR: "EXTERNAL_SERVICE_ERROR",
  GRPC_ERROR: "GRPC_ERROR",
} as const;

/**
 * Error response structure
 */
export interface ApiErrorResponse {
  success: false;
  error: {
    code: string;
    message: string;
    details?: any;
    fieldErrors?: Record<string, string[]>;
    requestId?: string;
    timestamp: string;
  };
}

/**
 * Success response structure
 */
export interface ApiSuccessResponse<T = any> {
  success: true;
  data: T;
  meta?: {
    requestId?: string;
    timestamp?: string;
    [key: string]: any;
  };
}

/**
 * Create standardized error response
 */
export function createErrorResponse(
  code: string,
  message: string,
  options?: {
    statusCode?: number;
    details?: any;
    fieldErrors?: Record<string, string[]>;
    requestId?: string;
  }
): NextResponse {
  const { statusCode = 400, details, fieldErrors, requestId } = options || {};

  const errorResponse: ApiErrorResponse = {
    success: false,
    error: {
      code,
      message,
      details,
      fieldErrors,
      requestId,
      timestamp: new Date().toISOString(),
    },
  };

  return NextResponse.json(errorResponse, { status: statusCode });
}

/**
 * Create standardized success response
 */
export function createSuccessResponse<T>(
  data: T,
  options?: {
    statusCode?: number;
    meta?: any;
    headers?: Record<string, string>;
  }
): NextResponse {
  const { statusCode = 200, meta, headers } = options || {};

  const response: ApiSuccessResponse<T> = {
    success: true,
    data,
    meta: {
      ...meta,
      timestamp: new Date().toISOString(),
    },
  };

  const nextResponse = NextResponse.json(response, { status: statusCode });

  // Add custom headers
  if (headers) {
    Object.entries(headers).forEach(([key, value]) => {
      nextResponse.headers.set(key, value);
    });
  }

  return nextResponse;
}

/**
 * Generate unique request ID
 */
function generateRequestId(): string {
  return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Error handler for API routes
 */
export function withErrorHandling(
  handler: (request: NextRequest, context: any) => Promise<NextResponse>
) {
  return async (request: NextRequest, context: any) => {
    const requestId = generateRequestId();
    const startTime = Date.now();

    try {
      const response = await handler(request, context);

      // Add request ID to successful responses
      if (!response.headers.has("X-Request-ID")) {
        response.headers.set("X-Request-ID", requestId);
      }

      return response;
    } catch (error) {
      console.error(`[Error] ${requestId}:`, error);

      const responseTime = Date.now() - startTime;
      const url = new URL(request.url);
      const method = request.method;

      // Log error details
      console.error(`[Error] Request: ${method} ${url.pathname}`);
      console.error(`[Error] User: ${context.user?.id || "anonymous"}`);
      console.error(`[Error] Time: ${responseTime}ms`);
      console.error(`[Error] Details:`, error);

      // Determine error type and create appropriate response
      if (error instanceof ValidationError) {
        return createErrorResponse(
          ERROR_CODES.VALIDATION_ERROR,
          error.message,
          {
            statusCode: 400,
            fieldErrors: error.fieldErrors,
            requestId,
          }
        );
      }

      if (error instanceof NotFoundError) {
        return createErrorResponse(
          ERROR_CODES.NOT_FOUND,
          error.message,
          {
            statusCode: 404,
            requestId,
          }
        );
      }

      if (error instanceof UnauthorizedError) {
        return createErrorResponse(
          ERROR_CODES.UNAUTHENTICATED,
          error.message,
          {
            statusCode: 401,
            requestId,
          }
        );
      }

      if (error instanceof ForbiddenError) {
        return createErrorResponse(
          ERROR_CODES.FORBIDDEN,
          error.message,
          {
            statusCode: 403,
            requestId,
          }
        );
      }

      if (error instanceof ConflictError) {
        return createErrorResponse(
          ERROR_CODES.CONFLICT,
          error.message,
          {
            statusCode: 409,
            details: error.details,
            requestId,
          }
        );
      }

      if (error instanceof TimeoutError) {
        return createErrorResponse(
          ERROR_CODES.TIMEOUT,
          error.message,
          {
            statusCode: 408,
            requestId,
          }
        );
      }

      // Generic internal error
      return createErrorResponse(
        ERROR_CODES.INTERNAL_ERROR,
        "An unexpected error occurred",
        {
          statusCode: 500,
          details: process.env.NODE_ENV === "development" ? error : undefined,
          requestId,
        }
      );
    }
  };
}

// Custom error classes
export class ValidationError extends Error {
  public fieldErrors?: Record<string, string[]>;

  constructor(message: string, fieldErrors?: Record<string, string[]>) {
    super(message);
    this.name = "ValidationError";
    this.fieldErrors = fieldErrors;
  }
}

export class NotFoundError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NotFoundError";
  }
}

export class UnauthorizedError extends Error {
  constructor(message: string = "Authentication required") {
    super(message);
    this.name = "UnauthorizedError";
  }
}

export class ForbiddenError extends Error {
  constructor(message: string = "Insufficient permissions") {
    super(message);
    this.name = "ForbiddenError";
  }
}

export class ConflictError extends Error {
  public details?: any;

  constructor(message: string, details?: any) {
    super(message);
    this.name = "ConflictError";
    this.details = details;
  }
}

export class TimeoutError extends Error {
  constructor(message: string = "Request timeout") {
    super(message);
    this.name = "TimeoutError";
  }
}
