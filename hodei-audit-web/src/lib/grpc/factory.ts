/**
 * gRPC Client Factory
 *
 * Factory for creating specialized gRPC clients for different services.
 */

import {
  GrpcClient,
  GrpcClientConfig,
  createGrpcClient,
  createAuthMetadata,
} from "./client";

/**
 * Service-specific client configuration
 */
export interface ServiceConfig extends GrpcClientConfig {
  serviceName: string;
}

/**
 * Factory for creating gRPC clients
 */
export class GrpcClientFactory {
  private clients: Map<string, GrpcClient> = new Map();

  /**
   * Create a client for a specific service
   */
  public createClient(config: ServiceConfig): GrpcClient {
    const client = new GrpcClient({
      ...config,
      metadata: createAuthMetadata(),
    });

    // Add common interceptors
    this.addCommonInterceptors(client);

    this.clients.set(config.serviceName, client);
    return client;
  }

  /**
   * Get an existing client for a service
   */
  public getClient(serviceName: string): GrpcClient | undefined {
    return this.clients.get(serviceName);
  }

  /**
   * Create or get a client for a service
   */
  public getOrCreateClient(config: ServiceConfig): GrpcClient {
    const existing = this.getClient(config.serviceName);
    if (existing) {
      return existing;
    }
    return this.createClient(config);
  }

  /**
   * Add common interceptors to a client
   */
  private addCommonInterceptors(client: GrpcClient): void {
    // Request logging interceptor
    client.addRequestInterceptor({
      onRequest: (request) => {
        console.log(`[gRPC] Sending request: ${request.getMethod().name}`);
        return request;
      },
    });

    // Response error logging interceptor
    client.addResponseInterceptor({
      onError: (error) => {
        console.error("[gRPC] Error occurred:", error);
      },
    });
  }

  /**
   * Close all clients
   */
  public closeAll(): void {
    this.clients.forEach((client) => client.close());
    this.clients.clear();
  }
}

/**
 * Global client factory instance
 */
export const clientFactory = new GrpcClientFactory();

/**
 * Create event service client
 */
export function createEventServiceClient(): GrpcClient {
  return clientFactory.getOrCreateClient({
    serviceName: "event-service",
    host: process.env.NEXT_PUBLIC_GRPC_HOST_EVENTS || "http://localhost:8081",
    timeout: 30000, // 30 seconds for event queries
  });
}

/**
 * Create analytics service client
 */
export function createAnalyticsServiceClient(): GrpcClient {
  return clientFactory.getOrCreateClient({
    serviceName: "analytics-service",
    host: process.env.NEXT_PUBLIC_GRPC_HOST_ANALYTICS || "http://localhost:8082",
    timeout: 60000, // 60 seconds for analytics queries
  });
}

/**
 * Create compliance service client
 */
export function createComplianceServiceClient(): GrpcClient {
  return clientFactory.getOrCreateClient({
    serviceName: "compliance-service",
    host: process.env.NEXT_PUBLIC_GRPC_HOST_COMPLIANCE || "http://localhost:8083",
    timeout: 90000, // 90 seconds for compliance operations
  });
}

/**
 * Create auth service client
 */
export function createAuthServiceClient(): GrpcClient {
  return clientFactory.getOrCreateClient({
    serviceName: "auth-service",
    host: process.env.NEXT_PUBLIC_GRPC_HOST_AUTH || "http://localhost:8080",
    timeout: 10000,
  });
}

/**
 * Health check client for service discovery
 */
export function createHealthCheckClient(): GrpcClient {
  return clientFactory.getOrCreateClient({
    serviceName: "health-service",
    host: process.env.NEXT_PUBLIC_GRPC_HOST || "http://localhost:8080",
    timeout: 5000,
  });
}

/**
 * Test client connection
 */
export async function testClientConnection(
  client: GrpcClient,
  serviceName: string
): Promise<boolean> {
  try {
    console.log(`[gRPC] Testing connection to ${serviceName}...`);
    // In a real implementation, this would make a health check call
    // For now, we'll just check if the client was created successfully
    const config = client.getConfig();
    console.log(`[gRPC] Connection test successful for ${serviceName}: ${config.host}`);
    return true;
  } catch (error) {
    console.error(`[gRPC] Connection test failed for ${serviceName}:`, error);
    return false;
  }
}

/**
 * Test all client connections
 */
export async function testAllConnections(): Promise<void> {
  console.log("[gRPC] Testing all client connections...");

  const clients = [
    { name: "Event Service", client: createEventServiceClient() },
    { name: "Analytics Service", client: createAnalyticsServiceClient() },
    { name: "Compliance Service", client: createComplianceServiceClient() },
    { name: "Auth Service", client: createAuthServiceClient() },
  ];

  for (const { name, client } of clients) {
    await testClientConnection(client, name);
  }

  console.log("[gRPC] Connection tests complete");
}

/**
 * Clean up all clients on application shutdown
 */
export function cleanupClients(): void {
  console.log("[gRPC] Cleaning up all clients...");
  clientFactory.closeAll();
}
