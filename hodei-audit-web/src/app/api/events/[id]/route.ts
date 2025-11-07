/**
 * Get Single Event API
 *
 * API endpoint to retrieve a single event by ID.
 */

import { NextRequest, NextResponse } from "next/server";
import { createApiClients } from "@/lib/api/client";
import { mockApis } from "@/lib/api/mock";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * GET /api/events/[id]
 * Get a single event by ID
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
            message: "Event ID is required",
          },
        },
        { status: 400 }
      );
    }

    let response;
    if (USE_MOCK_API) {
      response = await mockApis.event.getEvent(id);
    } else {
      // Use real gRPC client
      const { createEventServiceClient } = await import("@/lib/grpc/factory");
      const eventClient = createEventServiceClient();
      const apiClients = createApiClients(eventClient);
      response = await apiClients.event.getEvent(id);
    }

    if (!response.success) {
      return NextResponse.json(
        {
          success: false,
          error: response.error,
        },
        { status: 404 }
      );
    }

    return NextResponse.json(response);
  } catch (error) {
    console.error(`Error fetching event ${params.id}:`, error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "INTERNAL_ERROR",
          message: "Failed to fetch event",
          details: error instanceof Error ? error.message : undefined,
        },
      },
      { status: 500 }
    );
  }
}
