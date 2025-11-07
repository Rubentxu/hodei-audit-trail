/**
 * API Client - REST-like abstraction over gRPC
 *
 * This module provides a familiar REST-like API interface that wraps gRPC calls,
 * making it easier to use in the application.
 */

import { GrpcClient } from "../grpc/client";
import { Event, QueryEventsRequest, QueryEventsResponse, AnalyticsQuery, AnalyticsResult, ComplianceReport, GenerateReportRequest, CryptographicKey, AuthRequest, AuthResponse } from "../grpc/types";

/**
 * API Client Configuration
 */
export interface ApiClientConfig {
  baseUrl: string;
  apiKey?: string;
  timeout?: number;
  withCredentials?: boolean;
}

/**
 * Standard API Response wrapper
 */
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: ApiError;
  meta?: {
    total?: number;
    page?: number;
    pageSize?: number;
    hasMore?: boolean;
    requestId?: string;
    timestamp?: string;
  };
}

/**
 * API Error structure
 */
export interface ApiError {
  code: string;
  message: string;
  details?: any;
  fieldErrors?: Record<string, string[]>;
}

/**
 * API Request interceptor
 */
export type RequestInterceptor = (config: RequestConfig) => RequestConfig;

/**
 * API Response interceptor
 */
export type ResponseInterceptor<T> = (response: ApiResponse<T>) => ApiResponse<T>;

/**
 * Request configuration
 */
export interface RequestConfig {
  url: string;
  method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
  headers?: Record<string, string>;
  body?: any;
  timeout?: number;
  retry?: number;
}

/**
 * Base API Client
 */
export class ApiClient {
  private config: Required<ApiClientConfig>;
  private grpcClient: GrpcClient;
  private requestInterceptors: RequestInterceptor[] = [];
  private responseInterceptors: ResponseInterceptor<any>[] = [];
  private version: string = "v1";

  constructor(config: ApiClientConfig, grpcClient: GrpcClient) {
    this.config = {
      baseUrl: config.baseUrl.replace(/\/$/, ""), // Remove trailing slash
      apiKey: config.apiKey,
      timeout: config.timeout || 10000,
      withCredentials: config.withCredentials !== false,
    };
    this.grpcClient = grpcClient;
  }

  /**
   * Get request
   */
  public async get<T>(url: string, params?: Record<string, any>): Promise<ApiResponse<T>> {
    const queryString = params ? "?" + new URLSearchParams(params as any).toString() : "";
    return this.request<T>({
      url: `${this.version}${url}${queryString}`,
      method: "GET",
    });
  }

  /**
   * POST request
   */
  public async post<T>(url: string, data?: any): Promise<ApiResponse<T>> {
    return this.request<T>({
      url: `${this.version}${url}`,
      method: "POST",
      body: data,
    });
  }

  /**
   * PUT request
   */
  public async put<T>(url: string, data?: any): Promise<ApiResponse<T>> {
    return this.request<T>({
      url: `${this.version}${url}`,
      method: "PUT",
      body: data,
    });
  }

  /**
   * PATCH request
   */
  public async patch<T>(url: string, data?: any): Promise<ApiResponse<T>> {
    return this.request<T>({
      url: `${this.version}${url}`,
      method: "PATCH",
      body: data,
    });
  }

  /**
   * DELETE request
   */
  public async delete<T>(url: string): Promise<ApiResponse<T>> {
    return this.request<T>({
      url: `${this.version}${url}`,
      method: "DELETE",
    });
  }

  /**
   * Generic request method
   */
  private async request<T>(config: RequestConfig): Promise<ApiResponse<T>> {
    // Apply request interceptors
    let interceptedConfig = { ...config };
    for (const interceptor of this.requestInterceptors) {
      interceptedConfig = interceptor(interceptedConfig);
    }

    try {
      // In a real implementation, this would translate REST to gRPC
      // For now, we'll simulate with mock data
      const response = await this.performRequest<T>(interceptedConfig);

      // Apply response interceptors
      for (const interceptor of this.responseInterceptors) {
        return interceptor(response);
      }

      return response;
    } catch (error) {
      return this.handleError<T>(error);
    }
  }

  /**
   * Perform the actual request (gRPC integration)
   */
  private async performRequest<T>(config: RequestConfig): Promise<ApiResponse<T>> {
    // TODO: Translate REST to gRPC
    // This is where we would call the gRPC client
    // For demonstration, we'll return a mock success response

    return {
      success: true,
      data: {} as T,
      meta: {
        requestId: this.generateRequestId(),
        timestamp: new Date().toISOString(),
      },
    };
  }

  /**
   * Handle request errors
   */
  private handleError<T>(error: any): ApiResponse<T> {
    const apiError: ApiError = {
      code: error?.code || "UNKNOWN_ERROR",
      message: error?.message || "An unexpected error occurred",
      details: error?.details,
    };

    return {
      success: false,
      error: apiError,
      meta: {
        requestId: this.generateRequestId(),
        timestamp: new Date().toISOString(),
      },
    };
  }

  /**
   * Add request interceptor
   */
  public addRequestInterceptor(interceptor: RequestInterceptor): void {
    this.requestInterceptors.push(interceptor);
  }

  /**
   * Add response interceptor
   */
  public addResponseInterceptor<T>(interceptor: ResponseInterceptor<T>): void {
    this.responseInterceptors.push(interceptor as any);
  }

  /**
   * Set API version
   */
  public setVersion(version: string): void {
    this.version = version.replace(/^\//, "");
  }

  /**
   * Get the underlying gRPC client
   */
  public getGrpcClient(): GrpcClient {
    return this.grpcClient;
  }

  /**
   * Generate unique request ID
   */
  private generateRequestId(): string {
    return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }
}

/**
 * Event API Client
 */
export class EventApiClient {
  private apiClient: ApiClient;
  private grpcClient: GrpcClient;

  constructor(apiClient: ApiClient, grpcClient: GrpcClient) {
    this.apiClient = apiClient;
    this.grpcClient = grpcClient;
  }

  /**
   * Query events
   */
  public async queryEvents(params: QueryEventsRequest): Promise<ApiResponse<QueryEventsResponse>> {
    return this.apiClient.post<QueryEventsResponse>("/events/query", params);
  }

  /**
   * Get event by ID
   */
  public async getEvent(id: string): Promise<ApiResponse<Event>> {
    return this.apiClient.get<Event>(`/events/${id}`);
  }

  /**
   * Export events
   */
  public async exportEvents(params: QueryEventsRequest & { format: "csv" | "json" | "pdf" }): Promise<ApiResponse<{ downloadUrl: string }>> {
    return this.apiClient.post<{ downloadUrl: string }>("/events/export", params);
  }
}

/**
 * Analytics API Client
 */
export class AnalyticsApiClient {
  private apiClient: ApiClient;
  private grpcClient: GrpcClient;

  constructor(apiClient: ApiClient, grpcClient: GrpcClient) {
    this.apiClient = apiClient;
    this.grpcClient = grpcClient;
  }

  /**
   * Run analytics query
   */
  public async runQuery(query: AnalyticsQuery): Promise<ApiResponse<AnalyticsResult>> {
    return this.apiClient.post<AnalyticsResult>("/analytics/query", { query });
  }

  /**
   * Get saved queries
   */
  public async getSavedQueries(): Promise<ApiResponse<any[]>> {
    return this.apiClient.get<any[]>("/analytics/queries");
  }

  /**
   * Save query
   */
  public async saveQuery(query: { name: string; query: AnalyticsQuery; isPublic: boolean }): Promise<ApiResponse<any>> {
    return this.apiClient.post<any>("/analytics/queries", query);
  }

  /**
   * Delete saved query
   */
  public async deleteSavedQuery(id: string): Promise<ApiResponse<void>> {
    return this.apiClient.delete<void>(`/analytics/queries/${id}`);
  }
}

/**
 * Compliance API Client
 */
export class ComplianceApiClient {
  private apiClient: ApiClient;
  private grpcClient: GrpcClient;

  constructor(apiClient: ApiClient, grpcClient: GrpcClient) {
    this.apiClient = apiClient;
    this.grpcClient = grpcClient;
  }

  /**
   * Generate compliance report
   */
  public async generateReport(params: GenerateReportRequest): Promise<ApiResponse<ComplianceReport>> {
    return this.apiClient.post<ComplianceReport>("/compliance/reports", params);
  }

  /**
   * Get compliance reports
   */
  public async getReports(filters?: Record<string, any>): Promise<ApiResponse<ComplianceReport[]>> {
    return this.apiClient.get<ComplianceReport[]>("/compliance/reports", filters);
  }

  /**
   * Get report by ID
   */
  public async getReport(id: string): Promise<ApiResponse<ComplianceReport>> {
    return this.apiClient.get<ComplianceReport>(`/compliance/reports/${id}`);
  }

  /**
   * Download report
   */
  public async downloadReport(id: string): Promise<ApiResponse<{ downloadUrl: string }>> {
    return this.apiClient.post<{ downloadUrl: string }>(`/compliance/reports/${id}/download`);
  }

  /**
   * Delete report
   */
  public async deleteReport(id: string): Promise<ApiResponse<void>> {
    return this.apiClient.delete<void>(`/compliance/reports/${id}`);
  }

  /**
   * Get cryptographic keys
   */
  public async getKeys(tenantId: string): Promise<ApiResponse<CryptographicKey[]>> {
    return this.apiClient.get<CryptographicKey[]>(`/compliance/keys`, { tenantId });
  }

  /**
   * Rotate key
   */
  public async rotateKey(id: string): Promise<ApiResponse<CryptographicKey>> {
    return this.apiClient.post<CryptographicKey>(`/compliance/keys/${id}/rotate`);
  }
}

/**
 * Auth API Client
 */
export class AuthApiClient {
  private apiClient: ApiClient;
  private grpcClient: GrpcClient;

  constructor(apiClient: ApiClient, grpcClient: GrpcClient) {
    this.apiClient = apiClient;
    this.grpcClient = grpcClient;
  }

  /**
   * Login
   */
  public async login(params: AuthRequest): Promise<ApiResponse<AuthResponse>> {
    return this.apiClient.post<AuthResponse>("/auth/login", params);
  }

  /**
   * Logout
   */
  public async logout(): Promise<ApiResponse<void>> {
    return this.apiClient.post<void>("/auth/logout");
  }

  /**
   * Refresh token
   */
  public async refreshToken(refreshToken: string): Promise<ApiResponse<AuthResponse>> {
    return this.apiClient.post<AuthResponse>("/auth/refresh", { refreshToken });
  }

  /**
   * Validate token
   */
  public async validateToken(token: string): Promise<ApiResponse<{ valid: boolean }>> {
    return this.apiClient.post<{ valid: boolean }>("/auth/validate", { token });
  }
}

/**
 * Create API client instances
 */
export function createApiClients(grpcClient: GrpcClient) {
  const baseConfig: ApiClientConfig = {
    baseUrl: process.env.NEXT_PUBLIC_API_BASE_URL || "http://localhost:3000/api",
    timeout: 10000,
  };

  const apiClient = new ApiClient(baseConfig, grpcClient);
  const eventApi = new EventApiClient(apiClient, grpcClient);
  const analyticsApi = new AnalyticsApiClient(apiClient, grpcClient);
  const complianceApi = new ComplianceApiClient(apiClient, grpcClient);
  const authApi = new AuthApiClient(apiClient, grpcClient);

  return {
    apiClient,
    event: eventApi,
    analytics: analyticsApi,
    compliance: complianceApi,
    auth: authApi,
  };
}

/**
 * Default API clients instance
 */
export const apiClients = createApiClients(
  new GrpcClient({ host: process.env.NEXT_PUBLIC_GRPC_HOST || "http://localhost:8080" })
);
