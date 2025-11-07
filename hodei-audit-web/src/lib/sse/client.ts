/**
 * Server-Sent Events Client
 *
 * Client-side utilities for connecting to SSE endpoints.
 */

export interface SSEConfig {
  url: string;
  eventTypes?: string[];
  filters?: Record<string, any>;
  onOpen?: (event: Event) => void;
  onMessage?: (event: MessageEvent) => void;
  onError?: (event: Event) => void;
  onClose?: (event: CloseEvent) => void;
  heartbeatInterval?: number;
}

export interface SSEEvent {
  type: string;
  data: any;
  id?: string;
  timestamp?: string;
}

export class SSEClient {
  private config: Required<SSEConfig>;
  private eventSource: EventSource | null = null;
  private listeners: Map<string, Set<(data: any) => void>> = new Map();
  private heartbeatTimer: NodeJS.Timeout | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;

  constructor(config: SSEConfig) {
    this.config = {
      url: config.url,
      eventTypes: config.eventTypes || [],
      filters: config.filters || {},
      onOpen: config.onOpen || (() => {}),
      onMessage: config.onMessage || (() => {}),
      onError: config.onError || (() => {}),
      onClose: config.onClose || (() => {}),
      heartbeatInterval: config.heartbeatInterval || 30000,
    };
  }

  /**
   * Connect to SSE endpoint
   */
  public connect(): void {
    const url = this.buildUrl();
    console.log(`[SSE] Connecting to ${url}`);

    this.eventSource = new EventSource(url);

    this.eventSource.onopen = (event) => {
      console.log("[SSE] Connected");
      this.reconnectAttempts = 0;
      this.config.onOpen(event);
      this.startHeartbeat();
    };

    this.eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        this.config.onMessage(event);
        this.emit("message", data);
      } catch (error) {
        console.error("[SSE] Failed to parse message:", error);
      }
    };

    this.eventSource.onerror = (event) => {
      console.error("[SSE] Error:", event);
      this.config.onError(event);
      this.stopHeartbeat();

      if (this.eventSource?.readyState === EventSource.CLOSED) {
        this.attemptReconnect();
      }
    };

    this.eventSource.addEventListener("connected", (event) => {
      console.log("[SSE] Connection confirmed");
      const data = JSON.parse((event as MessageEvent).data);
      this.emit("connected", data);
    });

    this.eventSource.addEventListener("welcome", (event) => {
      const data = JSON.parse((event as MessageEvent).data);
      this.emit("welcome", data);
    });

    this.eventSource.addEventListener("event", (event) => {
      const data = JSON.parse((event as MessageEvent).data);
      this.emit("event", data);
    });

    this.eventSource.addEventListener("heartbeat", (event) => {
      // Heartbeat received, no action needed
    });

    this.eventSource.addEventListener("ready", (event) => {
      const data = JSON.parse((event as MessageEvent).data);
      this.emit("ready", data);
    });
  }

  /**
   * Disconnect from SSE endpoint
   */
  public disconnect(): void {
    console.log("[SSE] Disconnecting");
    this.stopHeartbeat();
    this.reconnectAttempts = this.maxReconnectAttempts; // Prevent reconnection

    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }
  }

  /**
   * Subscribe to event type
   */
  public on(eventType: string, callback: (data: any) => void): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, new Set());
    }
    this.listeners.get(eventType)!.add(callback);
  }

  /**
   * Unsubscribe from event type
   */
  public off(eventType: string, callback: (data: any) => void): void {
    const listeners = this.listeners.get(eventType);
    if (listeners) {
      listeners.delete(callback);
      if (listeners.size === 0) {
        this.listeners.delete(eventType);
      }
    }
  }

  /**
   * Get connection state
   */
  public getState(): "connecting" | "open" | "closed" {
    if (!this.eventSource) {
      return "closed";
    }

    switch (this.eventSource.readyState) {
      case EventSource.CONNECTING:
        return "connecting";
      case EventSource.OPEN:
        return "open";
      case EventSource.CLOSED:
        return "closed";
      default:
        return "closed";
    }
  }

  /**
   * Build SSE URL with parameters
   */
  private buildUrl(): string {
    const url = new URL(this.config.url);

    if (this.config.eventTypes.length > 0) {
      url.searchParams.set("eventTypes", this.config.eventTypes.join(","));
    }

    Object.entries(this.config.filters).forEach(([key, value]) => {
      url.searchParams.set(`filter_${key}`, String(value));
    });

    return url.toString();
  }

  /**
   * Emit event to listeners
   */
  private emit(eventType: string, data: any): void {
    const listeners = this.listeners.get(eventType);
    if (listeners) {
      listeners.forEach((callback) => callback(data));
    }
  }

  /**
   * Attempt to reconnect
   */
  private attemptReconnect(): void {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.log("[SSE] Max reconnection attempts reached");
      this.config.onClose(new CloseEvent("close", { reason: "max_reconnect_attempts" }));
      return;
    }

    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

    console.log(`[SSE] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.maxReconnectAttempts})`);

    setTimeout(() => {
      this.connect();
    }, delay);
  }

  /**
   * Start heartbeat monitoring
   */
  private startHeartbeat(): void {
    this.stopHeartbeat();
    this.heartbeatTimer = setInterval(() => {
      if (this.getState() === "open") {
        // In SSE, heartbeats are sent by the server
        // We just monitor that we receive them
        // If no heartbeat received in 2 intervals, we can consider connection lost
      }
    }, this.config.heartbeatInterval);
  }

  /**
   * Stop heartbeat monitoring
   */
  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }
}

/**
 * Create SSE client
 */
export function createSSEClient(config: SSEConfig): SSEClient {
  return new SSEClient(config);
}

/**
 * Hook for using SSE in React components
 */
export function useSSE(config: SSEConfig) {
  const [isConnected, setIsConnected] = useState(false);
  const [events, setEvents] = useState<any[]>([]);
  const clientRef = useRef<SSEClient | null>(null);

  useEffect(() => {
    const client = createSSEClient({
      ...config,
      onOpen: () => {
        setIsConnected(true);
        config.onOpen?.(new Event("open"));
      },
      onMessage: config.onMessage,
      onError: (event) => {
        setIsConnected(false);
        config.onError?.(event);
      },
    });

    clientRef.current = client;
    client.connect();

    return () => {
      client.disconnect();
    };
  }, [config.url]);

  const subscribe = (eventType: string, callback: (data: any) => void) => {
    clientRef.current?.on(eventType, callback);

    // Also track in events array if it's a general event
    if (eventType === "event") {
      clientRef.current?.on("event", (data) => {
        setEvents((prev) => [data, ...prev.slice(0, 99)]);
        callback(data);
      });
    }
  };

  const unsubscribe = (eventType: string, callback: (data: any) => void) => {
    clientRef.current?.off(eventType, callback);
  };

  return {
    isConnected,
    events,
    subscribe,
    unsubscribe,
    client: clientRef.current,
  };
}
