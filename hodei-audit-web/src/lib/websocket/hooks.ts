/**
 * WebSocket Hooks for React Components
 *
 * Custom hooks for using WebSocket in React components.
 */

"use client";

import { useEffect, useRef, useState, useCallback } from "react";
import { WebSocketClient, EventMessage } from "./client";

/**
 * Hook to use WebSocket connection
 */
export function useWebSocket(
  url: string,
  options?: {
    protocols?: string[];
    onConnect?: () => void;
    onDisconnect?: () => void;
    onError?: (error: Event) => void;
    autoConnect?: boolean;
  }
) {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<
    "connecting" | "connected" | "reconnecting" | "disconnected" | "error"
  >("disconnected");
  const [error, setError] = useState<Error | null>(null);

  const wsClientRef = useRef<WebSocketClient | null>(null);
  const reconnectAttempts = useRef(0);

  const connect = useCallback(async () => {
    try {
      setConnectionStatus("connecting");
      setError(null);

      if (!wsClientRef.current) {
        wsClientRef.current = new WebSocketClient({
          url,
          protocols: options?.protocols,
        });

        wsClientRef.current.onStatusChange((status) => {
          setConnectionStatus(status);
          setIsConnected(status === "connected");

          if (status === "connected") {
            reconnectAttempts.current = 0;
            options?.onConnect?.();
          } else if (status === "disconnected") {
            options?.onDisconnect?.();
          }
        });
      }

      await wsClientRef.current.connect();
    } catch (err) {
      setError(err as Error);
      setIsConnected(false);
      setConnectionStatus("error");
      options?.onError?.(err as Error);
    }
  }, [url, options]);

  const disconnect = useCallback(() => {
    if (wsClientRef.current) {
      wsClientRef.current.disconnect();
      wsClientRef.current = null;
    }
    setIsConnected(false);
    setConnectionStatus("disconnected");
  }, []);

  useEffect(() => {
    if (options?.autoConnect !== false) {
      connect();
    }

    return () => {
      disconnect();
    };
  }, [connect, disconnect, options?.autoConnect]);

  return {
    isConnected,
    connectionStatus,
    error,
    connect,
    disconnect,
    client: wsClientRef.current,
  };
}

/**
 * Hook to subscribe to WebSocket events
 */
export function useWebSocketSubscription(
  client: WebSocketClient | null,
  eventType: string,
  callback: (data: any) => void,
  filters?: Record<string, any>
) {
  const subscriptionIdRef = useRef<string | null>(null);

  const subscribe = useCallback(() => {
    if (!client) {
      return null;
    }

    const subscriptionId = client.subscribe(eventType, callback, filters);
    subscriptionIdRef.current = subscriptionId;
    return subscriptionId;
  }, [client, eventType, callback, filters]);

  const unsubscribe = useCallback(() => {
    if (!client || !subscriptionIdRef.current) {
      return;
    }

    client.unsubscribe(subscriptionIdRef.current);
    subscriptionIdRef.current = null;
  }, [client]);

  useEffect(() => {
    if (client && client.getStatus() === "connected") {
      const subscriptionId = subscribe();
      return () => {
        unsubscribe();
      };
    }
  }, [client, subscribe, unsubscribe]);

  return { subscribe, unsubscribe };
}

/**
 * Hook for real-time events
 */
export function useRealtimeEvents(
  client: WebSocketClient | null,
  eventTypes: string[]
) {
  const [events, setEvents] = useState<any[]>([]);
  const [eventCounts, setEventCounts] = useState<Record<string, number>>({});

  const handleEvent = useCallback((eventType: string) => {
    return (data: any) => {
      setEvents((prev) => [data, ...prev.slice(0, 99)]); // Keep last 100 events
      setEventCounts((prev) => ({
        ...prev,
        [eventType]: (prev[eventType] || 0) + 1,
      }));
    };
  }, []);

  const subscriptions = eventTypes.map((eventType) =>
    useWebSocketSubscription(client, eventType, handleEvent(eventType))
  );

  const clearEvents = useCallback(() => {
    setEvents([]);
    setEventCounts({});
  }, []);

  return {
    events,
    eventCounts,
    clearEvents,
    subscriptions,
  };
}

/**
 * Hook for user activity monitoring
 */
export function useUserActivity(
  client: WebSocketClient | null,
  userId?: string
) {
  const [activities, setActivities] = useState<any[]>([]);
  const [isOnline, setIsOnline] = useState(false);

  const handleActivity = useCallback((data: any) => {
    if (userId && data.userId !== userId) {
      return;
    }

    setActivities((prev) => [data, ...prev.slice(0, 49)]); // Keep last 50 activities
    setIsOnline(true);
  }, [userId]);

  useWebSocketSubscription(
    client,
    "user_activity",
    handleActivity,
    userId ? { userId } : undefined
  );

  // Mark user offline after 5 minutes of no activity
  useEffect(() => {
    if (activities.length === 0) {
      return;
    }

    const timer = setTimeout(() => {
      setIsOnline(false);
    }, 5 * 60 * 1000);

    return () => clearTimeout(timer);
  }, [activities]);

  return {
    activities,
    isOnline,
    lastActivity: activities[0] || null,
  };
}

/**
 * Hook for system status updates
 */
export function useSystemStatus(client: WebSocketClient | null) {
  const [status, setStatus] = useState<any | null>(null);
  const [alerts, setAlerts] = useState<any[]>([]);

  const handleStatusUpdate = useCallback((data: any) => {
    setStatus(data);
  }, []);

  const handleAlert = useCallback((data: any) => {
    setAlerts((prev) => [data, ...prev.slice(0, 19)]); // Keep last 20 alerts
  }, []);

  useWebSocketSubscription(client, "system_status", handleStatusUpdate);
  useWebSocketSubscription(client, "security_alert", handleAlert);

  const clearAlerts = useCallback(() => {
    setAlerts([]);
  }, []);

  return {
    status,
    alerts,
    clearAlerts,
    hasActiveAlerts: alerts.length > 0,
  };
}
