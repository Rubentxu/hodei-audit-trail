/**
 * WebSocket Client for Real-time Updates
 *
 * Manages WebSocket connections for receiving real-time event updates.
 */

type EventCallback = (event: any) => void;
type ErrorCallback = (error: Event) => void;
type StatusCallback = (status: "connecting" | "connected" | "reconnecting" | "disconnected" | "error") => void;

export interface WebSocketConfig {
  url: string;
  protocols?: string[];
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
  heartbeatInterval?: number;
  timeout?: number;
}

export interface Subscription {
  id: string;
  eventType: string;
  filters?: Record<string, any>;
  callback: EventCallback;
}

export interface EventMessage {
  id: string;
  type: string;
  data: any;
  timestamp: string;
  tenantId?: string;
  source: string;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private config: Required<WebSocketConfig>;
  private subscriptions: Map<string, Subscription> = new Map();
  private reconnectAttempts = 0;
  private reconnectTimer: NodeJS.Timeout | null = null;
  private heartbeatTimer: NodeJS.Timeout | null = null;
  private isIntentionallyClosed = false;
  private statusCallbacks: Set<StatusCallback> = new Set();

  constructor(config: WebSocketConfig) {
    this.config = {
      url: config.url,
      protocols: config.protocols || [],
      reconnectInterval: config.reconnectInterval || 5000,
      maxReconnectAttempts: config.maxReconnectAttempts || 5,
      heartbeatInterval: config.heartbeatInterval || 30000,
      timeout: config.timeout || 10000,
    };
  }

  /**
   * Connect to WebSocket server
   */
  public connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.isIntentionallyClosed = false;
        this.updateStatus("connecting");

        console.log(`[WebSocket] Connecting to ${this.config.url}`);
        this.ws = new WebSocket(this.config.url, this.config.protocols);

        const timeoutId = setTimeout(() => {
          this.ws?.close();
          reject(new Error("Connection timeout"));
        }, this.config.timeout);

        this.ws.onopen = () => {
          clearTimeout(timeoutId);
          console.log("[WebSocket] Connected");
          this.reconnectAttempts = 0;
          this.updateStatus("connected");
          this.startHeartbeat();
          this.resubscribeAll();
          resolve();
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(event);
        };

        this.ws.onerror = (error) => {
          console.error("[WebSocket] Error:", error);
          this.updateStatus("error");
          reject(error);
        };

        this.ws.onclose = (event) => {
          clearTimeout(timeoutId);
          console.log("[WebSocket] Closed:", event.code, event.reason);
          this.cleanup();

          if (!this.isIntentionallyClosed && this.reconnectAttempts < this.config.maxReconnectAttempts) {
            this.scheduleReconnect();
          } else {
            this.updateStatus("disconnected");
          }
        };
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Disconnect from WebSocket server
   */
  public disconnect(): void {
    this.isIntentionallyClosed = true;
    this.cleanup();

    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.close(1000, "Client disconnect");
    }

    this.updateStatus("disconnected");
  }

  /**
   * Subscribe to event type
   */
  public subscribe(
    eventType: string,
    callback: EventCallback,
    filters?: Record<string, any>
  ): string {
    const subscriptionId = `sub_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

    this.subscriptions.set(subscriptionId, {
      id: subscriptionId,
      eventType,
      filters,
      callback,
    });

    // If connected, send subscription message
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.sendSubscribeMessage(subscriptionId);
    }

    return subscriptionId;
  }

  /**
   * Unsubscribe from event type
   */
  public unsubscribe(subscriptionId: string): boolean {
    const subscription = this.subscriptions.get(subscriptionId);

    if (!subscription) {
      return false;
    }

    // Send unsubscribe message if connected
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.sendUnsubscribeMessage(subscriptionId);
    }

    this.subscriptions.delete(subscriptionId);
    return true;
  }

  /**
   * Get connection status
   */
  public getStatus(): "connecting" | "connected" | "reconnecting" | "disconnected" | "error" {
    if (!this.ws) {
      return "disconnected";
    }

    switch (this.ws.readyState) {
      case WebSocket.CONNECTING:
        return this.reconnectAttempts > 0 ? "reconnecting" : "connecting";
      case WebSocket.OPEN:
        return "connected";
      case WebSocket.CLOSING:
        return "disconnected";
      case WebSocket.CLOSED:
        return this.isIntentionallyClosed ? "disconnected" : "error";
      default:
        return "disconnected";
    }
  }

  /**
   * Add status change callback
   */
  public onStatusChange(callback: StatusCallback): void {
    this.statusCallbacks.add(callback);
  }

  /**
   * Remove status change callback
   */
  public offStatusChange(callback: StatusCallback): void {
    this.statusCallbacks.delete(callback);
  }

  /**
   * Send message to server
   */
  public send(data: any): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      console.warn("[WebSocket] Cannot send message: not connected");
      return;
    }

    try {
      this.ws.send(JSON.stringify(data));
    } catch (error) {
      console.error("[WebSocket] Send error:", error);
    }
  }

  /**
   * Handle incoming message
   */
  private handleMessage(event: MessageEvent): void {
    try {
      const message: EventMessage = JSON.parse(event.data);

      console.log(`[WebSocket] Received message: ${message.type}`);

      // Notify all subscriptions that match this event type
      for (const subscription of this.subscriptions.values()) {
        if (subscription.eventType === message.type || subscription.eventType === "*") {
          // Check filters if provided
          if (subscription.filters && !this.matchesFilters(message.data, subscription.filters)) {
            continue;
          }

          subscription.callback(message.data);
        }
      }
    } catch (error) {
      console.error("[WebSocket] Message parse error:", error);
    }
  }

  /**
   * Check if data matches filters
   */
  private matchesFilters(data: any, filters: Record<string, any>): boolean {
    return Object.entries(filters).every(([key, value]) => {
      return data[key] === value;
    });
  }

  /**
   * Send subscription message to server
   */
  private sendSubscribeMessage(subscriptionId: string): void {
    const subscription = this.subscriptions.get(subscriptionId);
    if (!subscription) {
      return;
    }

    this.send({
      type: "subscribe",
      id: subscriptionId,
      eventType: subscription.eventType,
      filters: subscription.filters,
    });
  }

  /**
   * Send unsubscribe message to server
   */
  private sendUnsubscribeMessage(subscriptionId: string): void {
    this.send({
      type: "unsubscribe",
      id: subscriptionId,
    });
  }

  /**
   * Resubscribe to all subscriptions after reconnection
   */
  private resubscribeAll(): void {
    for (const subscriptionId of this.subscriptions.keys()) {
      this.sendSubscribeMessage(subscriptionId);
    }
  }

  /**
   * Schedule reconnection attempt
   */
  private scheduleReconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }

    this.reconnectAttempts++;
    this.updateStatus("reconnecting");

    const delay = this.config.reconnectInterval * Math.pow(2, this.reconnectAttempts - 1);

    console.log(`[WebSocket] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.config.maxReconnectAttempts})`);

    this.reconnectTimer = setTimeout(() => {
      this.connect().catch((error) => {
        console.error("[WebSocket] Reconnect failed:", error);
      });
    }, delay);
  }

  /**
   * Start heartbeat/ping
   */
  private startHeartbeat(): void {
    this.stopHeartbeat();

    this.heartbeatTimer = setInterval(() => {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        this.send({ type: "ping", timestamp: Date.now() });
      }
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
   * Clean up resources
   */
  private cleanup(): void {
    this.stopHeartbeat();

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
  }

  /**
   * Update connection status
   */
  private updateStatus(status: "connecting" | "connected" | "reconnecting" | "disconnected" | "error"): void {
    this.statusCallbacks.forEach(callback => callback(status));
  }
}

/**
 * Create default WebSocket client
 */
export function createWebSocketClient(config: WebSocketConfig): WebSocketClient {
  return new WebSocketClient(config);
}

/**
 * Default client instance
 */
export const defaultWebSocketClient = createWebSocketClient({
  url: process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8080/ws",
});

/**
 * Event subscription helpers
 */
export const EventSubscriptions = {
  NEW_EVENT: "new_event",
  UPDATED_EVENT: "updated_event",
  DELETED_EVENT: "deleted_event",
  BULK_EVENTS: "bulk_events",
  USER_ACTIVITY: "user_activity",
  SECURITY_ALERT: "security_alert",
  COMPLIANCE_EVENT: "compliance_event",
  SYSTEM_STATUS: "system_status",
} as const;
