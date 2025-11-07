/**
 * Event Query Service API
 *
 * REST API endpoints that wrap gRPC calls for event querying.
 */

import { NextRequest, NextResponse } from "next/server";
import { createApiClients } from "@/lib/api/client";
import { mockApis } from "@/lib/api/mock";
import { QueryEventsRequest } from "@/lib/grpc/types";
import { getServiceTimeout } from "@/lib/grpc/endpoints";
import { createEventServiceClient } from "@/lib/grpc/factory";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * Create event service client
 */
const eventClient = USE_MOCK_API ? null : createEventServiceClient();

/**
 * GET /api/events
 * Query events with filters, pagination, and sorting
 */
export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const tenantId = searchParams.get("tenantId") || "tenant-1"; // Default for demo

    // Parse query parameters
    const page = parseInt(searchParams.get("page") || "1");
    const pageSize = parseInt(searchParams.get("pageSize") || "50");
    const sortBy = searchParams.get("sortBy") || "timestamp";
    const sortOrder = searchParams.get("sortOrder") || "desc";

    // Parse filters from query string
    const filters: any[] = [];
    searchParams.forEach((value, key) => {
      if (key.startsWith("filter_")) {
        const field = key.replace("filter_", "");
        const [operator, val] = value.split(":");
        filters.push({
          field,
          operator: operator || "eq",
          value: val,
        });
      }
    });

    // Parse time range
    const timeRange = {
      start: searchParams.get("startDate") || new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
      end: searchParams.get("endDate") || new Date().toISOString(),
    };

    const queryRequest: QueryEventsRequest = {
      tenantId,
      pagination: {
        page,
        pageSize,
      },
      sorting: {
        field: sortBy,
        direction: sortOrder as "asc" | "desc",
      },
      timeRange,
      filters: filters.length > 0 ? filters : undefined,
    };

    let response;
    if (USE_MOCK_API) {
      response = await mockApis.event.queryEvents(queryRequest);
    } else {
      // Use real gRPC client
      const apiClients = createApiClients(eventClient!);
      response = await apiClients.event.queryEvents(queryRequest);
    }

    if (!response.success) {
      return NextResponse.json(
        {
          success: false,
          error: response.error,
        },
        { status: 400 }
      );
    }

    return NextResponse.json(response);
  } catch (error) {
    console.error("Error querying events:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "INTERNAL_ERROR",
          message: "Failed to query events",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 }
    );
  }
}

/**
 * POST /api/events/query
 * Advanced event query with complex filters
 */
export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { tenantId, filters, pagination, sorting, timeRange } = body;

    if (!tenantId) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: "tenantId is required",
          },
        },
        { status: 400 }
      );
    }

    const queryRequest: QueryEventsRequest = {
      tenantId,
      filters,
      pagination,
      sorting,
      timeRange,
    };

    let response;
    if (USE_MOCK_API) {
      response = await mockApis.event.queryEvents(queryRequest);
    } else {
      const apiClients = createApiClients(eventClient!);
      response = await apiClients.event.queryEvents(queryRequest);
    }

    if (!response.success) {
      return NextResponse.json(
        {
          success: false,
          error: response.error,
        },
        { status: 400 }
      );
    }

    return NextResponse.json(response);
  } catch (error) {
    console.error("Error in advanced event query:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "INTERNAL_ERROR",
          message: "Failed to execute query",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 }
    );
  }
}
