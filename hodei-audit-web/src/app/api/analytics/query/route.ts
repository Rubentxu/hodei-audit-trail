/**
 * Analytics API Endpoint
 *
 * Provides analytics querying and aggregation functionality.
 */

import { NextRequest, NextResponse } from "next/server";
import { withAuth, AuthContext } from "@/lib/middleware/auth";
import { mockApis } from "@/lib/api/mock";
import { createApiClients } from "@/lib/api/client";
import { createEventServiceClient } from "@/lib/grpc/factory";
import {
  AnalyticsQuery,
  RunAnalyticsRequest,
  AnalyticsResult,
} from "@/lib/grpc/types";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * Create analytics service client
 */
const analyticsClient = USE_MOCK_API ? null : createEventServiceClient();

/**
 * POST /api/analytics/query
 * Run analytics query with aggregations and grouping
 */
async function handleAnalyticsQuery(
  request: NextRequest,
  context: AuthContext,
): Promise<NextResponse> {
  try {
    const body = await request.json();
    const { query } = body;

    if (!query) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: "query is required",
          },
        },
        { status: 400 },
      );
    }

    // Validate query structure
    const validation = validateAnalyticsQuery(query);
    if (!validation.valid) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: validation.message,
          },
        },
        { status: 400 },
      );
    }

    const startTime = Date.now();

    let result: AnalyticsResult;
    if (USE_MOCK_API) {
      const response = await mockApis.analytics.runQuery(query);
      result = response.data!;
    } else {
      // Use real gRPC client
      const apiClients = createApiClients(analyticsClient!);
      const response = await apiClients.analytics.runQuery(query);
      result = response.data!;
    }

    const queryTime = Date.now() - startTime;

    // Log analytics query
    console.log(
      `[Analytics] Query executed in ${queryTime}ms for user ${context.user?.id}`,
    );

    return NextResponse.json({
      success: true,
      data: result,
      meta: {
        queryTime,
        requestId: `req_${Date.now()}`,
        timestamp: new Date().toISOString(),
        userId: context.user?.id,
        tenantId: context.tenantId,
      },
    });
  } catch (error) {
    console.error("[Analytics] Query error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "ANALYTICS_ERROR",
          message: "Failed to execute analytics query",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 },
    );
  }
}

/**
 * Validate analytics query structure
 */
function validateAnalyticsQuery(query: any): {
  valid: boolean;
  message?: string;
} {
  if (!query) {
    return { valid: false, message: "Query object is required" };
  }

  if (!query.tenantId) {
    return { valid: false, message: "tenantId is required" };
  }

  if (!query.query && !query.aggregations) {
    return {
      valid: false,
      message: "Either query or aggregations must be provided",
    };
  }

  // Validate aggregations
  if (query.aggregations && !Array.isArray(query.aggregations)) {
    return { valid: false, message: "aggregations must be an array" };
  }

  // Validate time range if provided
  if (query.timeRange) {
    if (!query.timeRange.start || !query.timeRange.end) {
      return { valid: false, message: "timeRange requires both start and end" };
    }
  }

  return { valid: true };
}

export const POST = withAuth(handleAnalyticsQuery, ["view:analytics"]);
