/**
 * WebSocket Server (Mock Implementation)
 *
 * Mock WebSocket server for development and testing.
 * In production, this would be implemented in the backend.
 */

import { EventEmitter } from "events";
import { EventMessage } from "./client";

export interface WebSocketServerConfig {
  port: number;
  host: string;
  maxConnections?: number;
  heartbeatInterval?: number;
}

export class MockWebSocketServer extends EventEmitter {
  private config: Required<WebSocketServerConfig>;
  private connections: Set<any> = new Set();
  private subscriptions: Map<string, Set<any>> = new Map();
  private heartbeatTimer: NodeJS.Timeout | null = null;

  constructor(config: WebSocketServerConfig) {
    super();
    this.config = {
      port: config.port,
      host: config.host,
      maxConnections: config.maxConnections || 100,
      heartbeatInterval: config.heartbeatInterval || 30000,
    };
  }

  /**
   * Start the server
   */
  public start(): Promise<void> {
    return new Promise((resolve) => {
      console.log(`[WebSocket Server] Starting on ${this.config.host}:${this.config.port}`);

      // In a real implementation, this would start a WebSocket server
      // For now, we'll just simulate starting
      setTimeout(() => {
        console.log("[WebSocket Server] Started");
        this.startHeartbeat();
        this.startEventGenerator();
        resolve();
      }, 1000);
    });
  }

  /**
   * Stop the server
   */
  public stop(): void {
    console.log("[WebSocket Server] Stopping");
    this.stopHeartbeat();
    this.connections.forEach((conn: any) => {
      conn.close();
    });
    this.connections.clear();
    this.subscriptions.clear();
  }

  /**
   * Broadcast event to subscribers
   */
  public broadcast(event: EventMessage): void {
    console.log(`[WebSocket Server] Broadcasting event: ${event.type}`);

    const subscribers = this.subscriptions.get(event.type);
    if (subscribers) {
      subscribers.forEach((ws: any) => {
        if (ws.readyState === 1) { // OPEN
          ws.send(JSON.stringify(event));
        }
      });
    }
  }

  /**
   * Add a new connection
   */
  private addConnection(ws: any): void {
    if (this.connections.size >= this.config.maxConnections) {
      ws.close(1008, "Server overloaded");
      return;
    }

    this.connections.add(ws);
    console.log(`[WebSocket Server] New connection. Total: ${this.connections.size}`);

    ws.on("message", (data: Buffer) => {
      this.handleMessage(ws, data);
    });

    ws.on("close", () => {
      this.removeConnection(ws);
    });

    ws.on("error", (error: Error) => {
      console.error("[WebSocket Server] Connection error:", error);
      this.removeConnection(ws);
    });

    // Send welcome message
    ws.send(
      JSON.stringify({
        type: "connected",
        data: {
          message: "Welcome to Hodei Audit WebSocket",
          timestamp: new Date().toISOString(),
        },
      })
    );
  }

  /**
   * Remove a connection
   */
  private removeConnection(ws: any): void {
    this.connections.delete(ws);

    // Remove from all subscriptions
    this.subscriptions.forEach((subscribers) => {
      subscribers.delete(ws);
    });

    console.log(`[WebSocket Server] Connection closed. Total: ${this.connections.size}`);
  }

  /**
   * Handle incoming message
   */
  private handleMessage(ws: any, data: Buffer): void {
    try {
      const message = JSON.parse(data.toString());
      console.log("[WebSocket Server] Received message:", message.type);

      switch (message.type) {
        case "subscribe":
          this.handleSubscribe(ws, message);
          break;
        case "unsubscribe":
          this.handleUnsubscribe(ws, message);
          break;
        case "ping":
          ws.send(
            JSON.stringify({
              type: "pong",
              timestamp: Date.now(),
            })
          );
          break;
        default:
          console.log("[WebSocket Server] Unknown message type:", message.type);
      }
    } catch (error) {
      console.error("[WebSocket Server] Message parse error:", error);
    }
  }

  /**
   * Handle subscribe message
   */
  private handleSubscribe(ws: any, message: any): void {
    const { id, eventType, filters } = message;

    if (!eventType) {
      ws.send(
        JSON.stringify({
          type: "error",
          data: {
            message: "eventType is required",
            subscriptionId: id,
          },
        })
      );
      return;
    }

    if (!this.subscriptions.has(eventType)) {
      this.subscriptions.set(eventType, new Set());
    }

    this.subscriptions.get(eventType)!.add(ws);

    ws.send(
      JSON.stringify({
        type: "subscribed",
        data: {
          subscriptionId: id,
          eventType,
          filters,
        },
      })
    );
  }

  /**
   * Handle unsubscribe message
   */
  private handleUnsubscribe(ws: any, message: any): void {
    const { id, eventType } = message;

    if (eventType) {
      const subscribers = this.subscriptions.get(eventType);
      if (subscribers) {
        subscribers.delete(ws);
      }
    }

    ws.send(
      JSON.stringify({
        type: "unsubscribed",
        data: {
          subscriptionId: id,
          eventType,
        },
      })
    );
  }

  /**
   * Start heartbeat
   */
  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      this.connections.forEach((ws: any) => {
        if (ws.readyState === 1) {
          ws.ping();
        }
      });
    }, this.config.heartbeatInterval);
  }

  /**
   * Stop heartbeat
   */
  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }

  /**
   * Start generating mock events
   */
  private startEventGenerator(): void {
    // Generate a mock event every 10 seconds
    setInterval(() => {
      const event: EventMessage = {
        id: `evt_${Date.now()}`,
        type: "new_event",
        timestamp: new Date().toISOString(),
        source: "mock_server",
        data: {
          id: `evt_${Date.now()}`,
          tenantId: "tenant-1",
          action: "user_action",
          resource: "dashboard",
          sourceIp: "192.168.1.100",
          status: "success",
        },
      };

      this.broadcast(event);
    }, 10000);

    // Generate system status every 30 seconds
    setInterval(() => {
      const event: EventMessage = {
        id: `status_${Date.now()}`,
        type: "system_status",
        timestamp: new Date().toISOString(),
        source: "mock_server",
        data: {
          status: "healthy",
          uptime: Math.floor(Math.random() * 1000000),
          memory: {
            used: Math.floor(Math.random() * 1000),
            total: 2000,
          },
          cpu: Math.floor(Math.random() * 100),
        },
      };

      this.broadcast(event);
    }, 30000);
  }
}

/**
 * Global mock server instance
 */
let mockServer: MockWebSocketServer | null = null;

/**
 * Initialize mock WebSocket server
 */
export function initializeMockWebSocketServer(): MockWebSocketServer {
  if (mockServer) {
    return mockServer;
  }

  mockServer = new MockWebSocketServer({
    port: parseInt(process.env.WS_PORT || "8080"),
    host: process.env.WS_HOST || "localhost",
  });

  return mockServer;
}

/**
 * Get mock WebSocket server
 */
export function getMockWebSocketServer(): MockWebSocketServer | null {
  return mockServer;
}
