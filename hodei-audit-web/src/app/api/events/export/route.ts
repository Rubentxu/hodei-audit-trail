/**
 * Export Events API
 *
 * API endpoint to export events in various formats.
 */

import { NextRequest, NextResponse } from "next/server";
import { createApiClients } from "@/lib/api/client";
import { mockApis } from "@/lib/api/mock";
import { QueryEventsRequest } from "@/lib/grpc/types";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * POST /api/events/export
 * Export events in specified format (CSV, JSON, PDF)
 */
export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { tenantId, filters, pagination, sorting, timeRange, format } = body;

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

    if (!format || !["csv", "json", "pdf"].includes(format)) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: "format must be one of: csv, json, pdf",
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

    if (USE_MOCK_API) {
      // Return mock download URL
      const downloadUrl = `/api/events/export/mock/${Date.now()}.${format}`;

      // Simulate processing time
      await new Promise(resolve => setTimeout(resolve, 1000));

      return NextResponse.json({
        success: true,
        data: { downloadUrl },
        meta: {
          requestId: `req_${Date.now()}`,
          timestamp: new Date().toISOString(),
        },
      });
    } else {
      // Use real gRPC client and backend export service
      const { createEventServiceClient } = await import("@/lib/grpc/factory");
      const eventClient = createEventServiceClient();
      const apiClients = createApiClients(eventClient);

      const response = await apiClients.event.exportEvents({
        ...queryRequest,
        format,
      });

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
    }
  } catch (error) {
    console.error("Error exporting events:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "INTERNAL_ERROR",
          message: "Failed to export events",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 }
    );
  }
}

/**
 * GET /api/events/export/[id]/download
 * Download exported file
 */
export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const { id } = params;

    if (!id) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: "Export ID is required",
          },
        },
        { status: 400 }
      );
    }

    if (USE_MOCK_API) {
      // Return mock file data
      return NextResponse.json({
        success: true,
        data: {
          downloadUrl: `/api/events/export/mock/${id}`,
          expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
        },
      });
    } else {
      // In a real implementation, this would:
      // 1. Check if the export job is complete
      // 2. If complete, return the download URL
      // 3. If still processing, return job status
      // 4. If failed, return error

      return NextResponse.json({
        success: true,
        data: {
          downloadUrl: `/api/events/export/${id}/download`,
          expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
        },
      });
    }
  } catch (error) {
    console.error(`Error downloading export ${params.id}:`, error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "INTERNAL_ERROR",
          message: "Failed to download export",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 }
    );
  }
}
