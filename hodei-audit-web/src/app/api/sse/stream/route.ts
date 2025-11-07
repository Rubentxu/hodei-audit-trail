/**
 * Server-Sent Events (SSE) Endpoint
 *
 * Provides real-time event streaming using SSE protocol.
 */

import { NextRequest } from "next/server";
import { mockApis } from "@/lib/api/mock";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * SSE Connection Manager
 */
class SSEConnectionManager {
  private connections: Map<string, Response> = new Map();
  private controllers: Map<string, ReadableStreamDefaultController> = new Map();

  /**
   * Add a new SSE connection
   */
  public addConnection(connectionId: string, response: Response, controller: ReadableStreamDefaultController): void {
    this.connections.set(connectionId, response);
    this.controllers.set(connectionId, controller);
    console.log(`[SSE] New connection: ${connectionId}. Total: ${this.connections.size}`);
  }

  /**
   * Remove SSE connection
   */
  public removeConnection(connectionId: string): void {
    this.connections.delete(connectionId);
    this.controllers.delete(connectionId);
    console.log(`[SSE] Connection closed: ${connectionId}. Total: ${this.connections.size}`);
  }

  /**
   * Send event to specific connection
   */
  public sendEvent(connectionId: string, event: string, data: any): void {
    const controller = this.controllers.get(connectionId);
    if (!controller) {
      return;
    }

    const eventData = `event: ${event}\ndata: ${JSON.stringify(data)}\n\n`;
    controller.enqueue(new TextEncoder().encode(eventData));
  }

  /**
   * Broadcast event to all connections
   */
  public broadcastEvent(event: string, data: any): void {
    const eventData = `event: ${event}\ndata: ${JSON.stringify(data)}\n\n`;
    const encoded = new TextEncoder().encode(eventData);

    this.controllers.forEach((controller) => {
      controller.enqueue(encoded);
    });

    console.log(`[SSE] Broadcasted event: ${event} to ${this.connections.size} connections`);
  }

  /**
   * Send heartbeat to all connections
   */
  public sendHeartbeat(): void {
    const heartbeat = `event: heartbeat\ndata: ${JSON.stringify({ timestamp: Date.now() })}\n\n`;
    const encoded = new TextEncoder().encode(heartbeat);

    this.controllers.forEach((controller) => {
      controller.enqueue(encoded);
    });
  }

  /**
   * Get connection count
   */
  public getConnectionCount(): number {
    return this.connections.size;
  }
}

// Global connection manager
const sseManager = new SSEConnectionManager();

/**
 * Generate unique connection ID
 */
function generateConnectionId(): string {
  return `sse_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * Setup SSE event stream
 */
async function setupSSEStream(connectionId: string, request: NextRequest): Promise<ReadableStream> {
  return new ReadableStream({
    start(controller) {
      // Add to connection manager
      const response = new Response(null, { status: 200 });
      sseManager.addConnection(connectionId, response, controller);

      // Send initial connection event
      const initEvent = {
        type: "connected",
        connectionId,
        timestamp: new Date().toISOString(),
        message: "SSE connection established",
      };
      controller.enqueue(new TextEncoder().encode(`event: connected\ndata: ${JSON.stringify(initEvent)}\n\n`));

      // Handle client disconnect
      const signal = request.signal;
      signal.addEventListener("abort", () => {
        console.log(`[SSE] Client disconnected: ${connectionId}`);
        sseManager.removeConnection(connectionId);
        controller.close();
      });

      // Send welcome message
      const welcomeEvent = {
        type: "welcome",
        data: {
          message: "Welcome to Hodei Audit SSE",
          version: "1.0",
          features: ["events", "heartbeat", "reconnect"],
        },
      };
      controller.enqueue(new TextEncoder().encode(`event: welcome\ndata: ${JSON.stringify(welcomeEvent)}\n\n`));
    },
    cancel() {
      sseManager.removeConnection(connectionId);
    },
  });
}

/**
 * Parse SSE parameters from query string
 */
function parseSSEParams(searchParams: URLSearchParams): {
  tenantId: string;
  eventTypes: string[];
  filters: Record<string, any>;
} {
  const tenantId = searchParams.get("tenantId") || "tenant-1";
  const eventTypesParam = searchParams.get("eventTypes") || "";
  const eventTypes = eventTypesParam ? eventTypesParam.split(",") : ["*"];

  // Parse filters from query string
  const filters: Record<string, any> = {};
  searchParams.forEach((value, key) => {
    if (key.startsWith("filter_")) {
      const filterKey = key.replace("filter_", "");
      filters[filterKey] = value;
    }
  });

  return { tenantId, eventTypes, filters };
}

/**
 * GET /api/sse/stream
 * Create new SSE stream
 */
export async function GET(request: NextRequest) {
  try {
    const connectionId = generateConnectionId();
    const searchParams = request.nextUrl.searchParams;
    const params = parseSSEParams(searchParams);

    console.log(`[SSE] New stream request: ${connectionId}`, params);

    // Create SSE stream
    const stream = await setupSSEStream(connectionId, request);

    // Set up periodic heartbeat
    const heartbeatInterval = setInterval(() => {
      sseManager.sendHeartbeat();
    }, 30000); // Send heartbeat every 30 seconds

    // Clean up on client disconnect
    request.signal.addEventListener("abort", () => {
      clearInterval(heartbeatInterval);
    });

    // If mock mode, start generating events
    if (USE_MOCK_API) {
      startMockEventGenerator(connectionId, params);
    }

    // Return SSE response
    return new Response(stream, {
      headers: {
        "Content-Type": "text/event-stream",
        "Cache-Control": "no-cache, no-transform",
        "Connection": "keep-alive",
        "X-Accel-Buffering": "no", // Disable nginx buffering
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Headers": "Cache-Control, X-Requested-With",
        "Access-Control-Expose-Headers": "X-Event-Count",
      },
    });
  } catch (error) {
    console.error("[SSE] Error creating stream:", error);
    return new Response(
      JSON.stringify({
        error: {
          code: "SSE_ERROR",
          message: "Failed to create SSE stream",
        },
      }),
      {
        status: 500,
        headers: {
          "Content-Type": "application/json",
        },
      }
    );
  }
}

/**
 * Start mock event generator for a connection
 */
function startMockEventGenerator(connectionId: string, params: any): void {
  // Generate a welcome event
  setTimeout(() => {
    sseManager.sendEvent(connectionId, "ready", {
      connectionId,
      tenantId: params.tenantId,
      eventTypes: params.eventTypes,
    });
  }, 1000);

  // Generate random events
  const eventInterval = setInterval(() => {
    const eventTypes = ["user_activity", "system_status", "security_alert", "new_event"];
    const eventType = eventTypes[Math.floor(Math.random() * eventTypes.length)];

    const event = {
      id: `evt_${Date.now()}`,
      type: eventType,
      timestamp: new Date().toISOString(),
      tenantId: params.tenantId,
      data: {
        action: eventType === "user_activity" ? "login" : "status_check",
        sourceIp: `192.168.1.${Math.floor(Math.random() * 255) + 1}`,
        userId: `user-${Math.floor(Math.random() * 10) + 1}`,
        status: "success",
      },
    };

    sseManager.sendEvent(connectionId, "event", event);
  }, 15000); // Generate event every 15 seconds

  // Clean up when connection is closed
  setTimeout(() => {
    clearInterval(eventInterval);
  }, 5 * 60 * 1000); // Stop after 5 minutes
}

/**
 * GET /api/sse/events
 * Get SSE connection statistics
 */
export async function OPTIONS(request: NextRequest) {
  return new Response(null, {
    status: 200,
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    },
  });
}
