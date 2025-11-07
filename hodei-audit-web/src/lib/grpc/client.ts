/**
 * gRPC-Web Client Configuration
 *
 * This module provides a centralized gRPC-web client setup for communicating
 * with the Hodei Audit backend services.
 */

import { grpc } from "@improbable-eng/grpc-web";
import { NodeHttp } from "@improbable-eng/grpc-web-node-http-transport";

// Configure gRPC-web to use Node.js HTTP transport
grpc.setDefaultTransport(NodeHttp);

// Client configuration
export interface GrpcClientConfig {
  host: string;
  timeout?: number;
  retryAttempts?: number;
  retryDelay?: number;
  withCredentials?: boolean;
  metadata?: grpc.Metadata;
}

// Default configuration
const DEFAULT_CONFIG: Required<GrpcClientConfig> = {
  host: process.env.NEXT_PUBLIC_GRPC_HOST || "http://localhost:8080",
  timeout: 10000, // 10 seconds
  retryAttempts: 3,
  retryDelay: 1000, // 1 second
  withCredentials: true,
  metadata: new grpc.Metadata(),
};

// Request/Response interceptors
export interface Interceptor {
  onRequest?: (request: grpc.Request) => grpc.Request;
  onResponse?: (response: grpc.Response) => grpc.Response;
  onError?: (error: Error) => void;
}

export class GrpcClient {
  private config: Required<GrpcClientConfig>;
  private requestInterceptors: Interceptor[] = [];
  private responseInterceptors: Interceptor[] = [];

  constructor(config: GrpcClientConfig) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Get the client configuration
   */
  public getConfig(): Required<GrpcClientConfig> {
    return this.config;
  }

  /**
   * Create a new unary call
   */
  public createUnaryCall<TRequest, TResponse>(
    service: grpc.UnaryServiceDefinition<TRequest, TResponse>,
    method: grpc.UnaryMethodDefinition<TRequest, TResponse>
  ): grpc.UnaryCall<TRequest, TResponse> {
    const request = new grpc.Request();
    request.setMethod(method);
    request.setService(service);
    request.setHost(this.config.host);
    request.setTimeout(this.config.timeout);
    request.setWithCredentials(this.config.withCredentials);
    request.setMetadata(this.config.metadata);

    return new grpc.UnaryCall(request);
  }

  /**
   * Create a new server streaming call
   */
  public createServerStreamingCall<TRequest, TResponse>(
    service: grpc.ServiceDefinition<TRequest, TResponse>,
    method: grpc.MethodDefinition<TRequest, TResponse>
  ): grpc.ServerStreamingCall<TRequest, TResponse> {
    const request = new grpc.Request();
    request.setMethod(method);
    request.setService(service);
    request.setHost(this.config.host);
    request.setTimeout(this.config.timeout);
    request.setWithCredentials(this.config.withCredentials);
    request.setMetadata(this.config.metadata);

    return new grpc.ServerStreamingCall(request);
  }

  /**
   * Add request interceptor
   */
  public addRequestInterceptor(interceptor: Interceptor): void {
    this.requestInterceptors.push(interceptor);
  }

  /**
   * Add response interceptor
   */
  public addResponseInterceptor(interceptor: Interceptor): void {
    this.responseInterceptors.push(interceptor);
  }

  /**
   * Make a unary call with retry logic
   */
  public async unaryCall<TRequest, TResponse>(
    service: grpc.UnaryServiceDefinition<TRequest, TResponse>,
    method: grpc.UnaryMethodDefinition<TRequest, TResponse>,
    requestData: TRequest
  ): Promise<TResponse> {
    let attempt = 0;
    let lastError: Error | null = null;

    while (attempt < this.config.retryAttempts) {
      try {
        const result = await this.performUnaryCall(
          service,
          method,
          requestData,
          attempt
        );
        return result;
      } catch (error) {
        lastError = error as Error;
        attempt++;

        if (attempt >= this.config.retryAttempts) {
          break;
        }

        // Wait before retry
        await this.delay(this.config.retryDelay * attempt);
      }
    }

    throw lastError;
  }

  /**
   * Perform a single unary call
   */
  private performUnaryCall<TRequest, TResponse>(
    service: grpc.UnaryServiceDefinition<TRequest, TResponse>,
    method: grpc.UnaryMethodDefinition<TRequest, TResponse>,
    requestData: TRequest,
    attempt: number
  ): Promise<TResponse> {
    return new Promise((resolve, reject) => {
      const call = this.createUnaryCall(service, method);

      // Apply request interceptors
      let interceptedRequest = call;
      for (const interceptor of this.requestInterceptors) {
        if (interceptor.onRequest) {
          interceptedRequest = interceptor.onRequest(interceptedRequest);
        }
      }

      // Set up response handler
      call.onMessage((message: TResponse) => {
        resolve(message);
      });

      // Set up error handler
      call.onError((error: Error) => {
        // Apply error interceptors
        for (const interceptor of this.responseInterceptors) {
          if (interceptor.onError) {
            interceptor.onError(error);
          }
        }
        reject(error);
      });

      // Start the call and send the request
      interceptedRequest.onStatus((status: grpc.Status) => {
        if (status.code !== grpc.StatusCode.OK) {
          reject(new Error(`gRPC call failed: ${status.code} - ${status.detail}`));
        }
      });

      interceptedRequest.start({
        onMessage: (message: TResponse) => {
          // This will be handled by the onMessage handler above
        },
        onError: (error: Error) => {
          // This will be handled by the onError handler above
        },
        onStatus: (status: grpc.Status) => {
          // This will be handled by the onStatus handler above
        },
      });

      interceptedRequest.write(requestData);
      interceptedRequest.end();
    });
  }

  /**
   * Create a server streaming call
   */
  public createServerStreamingCall<TRequest, TResponse>(
    service: grpc.ServiceDefinition<TRequest, TResponse>,
    method: grpc.MethodDefinition<TRequest, TResponse>,
    requestData: TRequest
  ): {
    call: grpc.ServerStreamingCall<TRequest, TResponse>;
    promise: Promise<grpc.Response<TResponse>>;
  } {
    const call = this.createServerStreamingCall(service, method);

    // Apply request interceptors
    let interceptedRequest = call;
    for (const interceptor of this.requestInterceptors) {
      if (interceptor.onRequest) {
        interceptedRequest = interceptor.onRequest(interceptedRequest);
      }
    }

    const promise = new Promise<grpc.Response<TResponse>>((resolve, reject) => {
      call.onMessage((message: TResponse) => {
        resolve({ message });
      });

      call.onError((error: Error) => {
        reject(error);
      });
    });

    return { call, promise };
  }

  /**
   * Delay helper for retry logic
   */
  private delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  /**
   * Close the client and clean up resources
   */
  public close(): void {
    // Clean up interceptors
    this.requestInterceptors = [];
    this.responseInterceptors = [];
  }
}

/**
 * Create a default gRPC client instance
 */
export function createGrpcClient(config?: GrpcClientConfig): GrpcClient {
  return new GrpcClient(config || {});
}

/**
 * Default client instance
 */
export const defaultClient = createGrpcClient();

/**
 * Helper function to create headers
 */
export function createMetadata(headers: Record<string, string>): grpc.Metadata {
  const metadata = new grpc.Metadata();
  Object.entries(headers).forEach(([key, value]) => {
    metadata.set(key, value);
  });
  return metadata;
}

/**
 * Helper function to get auth token from localStorage/sessionStorage
 */
export function getAuthToken(): string | null {
  if (typeof window === "undefined") {
    return null;
  }

  // Try to get token from various storage locations
  const token = localStorage.getItem("authToken") ||
                sessionStorage.getItem("authToken");

  return token;
}

/**
 * Helper function to create authenticated metadata
 */
export function createAuthMetadata(): grpc.Metadata {
  const token = getAuthToken();
  const metadata = new grpc.Metadata();

  if (token) {
    metadata.set("authorization", `Bearer ${token}`);
  }

  return metadata;
}

/**
 * Error codes mapping
 */
export const GRPC_ERROR_CODES: Record<grpc.StatusCode, string> = {
  [grpc.StatusCode.OK]: "OK",
  [grpc.StatusCode.CANCELLED]: "Cancelled",
  [grpc.StatusCode.UNKNOWN]: "Unknown",
  [grpc.StatusCode.INVALID_ARGUMENT]: "Invalid Argument",
  [grpc.StatusCode.DEADLINE_EXCEEDED]: "Deadline Exceeded",
  [grpc.StatusCode.NOT_FOUND]: "Not Found",
  [grpc.StatusCode.ALREADY_EXISTS]: "Already Exists",
  [grpc.StatusCode.PERMISSION_DENIED]: "Permission Denied",
  [grpc.StatusCode.RESOURCE_EXHAUSTED]: "Resource Exhausted",
  [grpc.StatusCode.FAILED_PRECONDITION]: "Failed Precondition",
  [grpc.StatusCode.ABORTED]: "Aborted",
  [grpc.StatusCode.OUT_OF_RANGE]: "Out of Range",
  [grpc.StatusCode.UNIMPLEMENTED]: "Unimplemented",
  [grpc.StatusCode.INTERNAL]: "Internal",
  [grpc.StatusCode.UNAVAILABLE]: "Unavailable",
  [grpc.StatusCode.DATA_LOSS]: "Data Loss",
  [grpc.StatusCode.UNAUTHENTICATED]: "Unauthenticated",
};

/**
 * Get error message from gRPC error
 */
export function getGrpcErrorMessage(error: any): string {
  if (error?.code && GRPC_ERROR_CODES[error.code]) {
    return GRPC_ERROR_CODES[error.code];
  }
  return error?.message || "Unknown error";
}
