/**
 * TypeScript Types from Proto Files
 *
 * This file contains TypeScript type definitions generated from the proto files.
 * In a real implementation, these would be generated using protoc with the
 * grpc-web plugin.
 */

import { grpc } from "@improbable-eng/grpc-web";

// =============================================================================
// Event Service Types
// =============================================================================

export interface Event {
  id: string;
  timestamp: string;
  tenantId: string;
  userId?: string;
  userName?: string;
  action: string;
  resource: string;
  resourceId?: string;
  sourceIp: string;
  userAgent?: string;
  status: "success" | "failure" | "error";
  details?: Record<string, any>;
  metadata?: Record<string, string>;
}

export interface QueryEventsRequest {
  tenantId: string;
  filters?: EventFilter[];
  pagination?: PaginationRequest;
  sorting?: SortingRequest;
  timeRange?: TimeRange;
}

export interface EventFilter {
  field: string;
  operator: "eq" | "ne" | "gt" | "gte" | "lt" | "lte" | "in" | "nin" | "contains" | "regex";
  value: string | number | boolean | string[];
}

export interface PaginationRequest {
  page: number;
  pageSize: number;
}

export interface SortingRequest {
  field: string;
  direction: "asc" | "desc";
}

export interface TimeRange {
  start: string;
  end: string;
}

export interface QueryEventsResponse {
  events: Event[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}

// =============================================================================
// Analytics Service Types
// =============================================================================

export interface AnalyticsQuery {
  tenantId: string;
  query: string;
  parameters?: Record<string, any>;
  timeRange?: TimeRange;
  groupBy?: string[];
  aggregations?: Aggregation[];
}

export interface Aggregation {
  function: "count" | "sum" | "avg" | "min" | "max" | "distinct";
  field: string;
  alias?: string;
}

export interface RunAnalyticsRequest {
  query: AnalyticsQuery;
}

export interface AnalyticsResult {
  data: Record<string, any>[];
  metadata: {
    queryTime: number;
    rowCount: number;
    fields: string[];
  };
}

// =============================================================================
// Compliance Service Types
// =============================================================================

export interface ComplianceReport {
  id: string;
  name: string;
  type: "SOC2" | "PCI_DSS" | "GDPR" | "HIPAA" | "ISO27001";
  status: "generating" | "ready" | "failed" | "expired";
  format: "pdf" | "json" | "csv";
  tenantId: string;
  createdAt: string;
  generatedAt?: string;
  expiresAt?: string;
  metadata?: Record<string, any>;
}

export interface GenerateReportRequest {
  tenantId: string;
  name: string;
  type: ComplianceReport["type"];
  format: ComplianceReport["format"];
  timeRange: TimeRange;
  sections?: string[];
  template?: string;
  recipients?: string[];
}

export interface DigestEntry {
  id: string;
  period: string;
  eventsCount: number;
  hash: string;
  previousHash?: string;
  nextHash?: string;
  status: "verified" | "pending" | "failed";
  createdAt: string;
}

export interface VerifyDigestRequest {
  tenantId: string;
  digestId: string;
}

export interface CryptographicKey {
  id: string;
  name: string;
  algorithm: string;
  status: "active" | "expired" | "revoked" | "pending";
  createdAt: string;
  expiresAt: string;
  fingerprint: string;
}

// =============================================================================
// Auth Service Types
// =============================================================================

export interface AuthRequest {
  tenantId: string;
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  refreshToken: string;
  expiresAt: string;
  user: {
    id: string;
    username: string;
    email: string;
    role: "admin" | "auditor" | "analyst" | "viewer";
  };
}

export interface ValidateTokenRequest {
  token: string;
}

export interface ValidateTokenResponse {
  valid: boolean;
  user?: AuthResponse["user"];
  tenantId?: string;
}

// =============================================================================
// Common Types
// =============================================================================

export interface Status {
  code: number;
  message: string;
  details?: string;
}

export interface Error {
  code: string;
  message: string;
  details?: Record<string, any>;
}

export interface HealthCheckRequest {
  service: string;
}

export interface HealthCheckResponse {
  status: "healthy" | "unhealthy" | "degraded";
  version: string;
  uptime: number;
}

// =============================================================================
// Service Definitions
// =============================================================================

// Event Service Definition
export interface EventService {
  QueryEvents(
    request: QueryEventsRequest,
    metadata?: grpc.Metadata
  ): Promise<QueryEventsResponse>;
}

// Analytics Service Definition
export interface AnalyticsService {
  RunAnalytics(
    request: RunAnalyticsRequest,
    metadata?: grpc.Metadata
  ): Promise<AnalyticsResult>;
}

// Compliance Service Definition
export interface ComplianceService {
  GenerateReport(
    request: GenerateReportRequest,
    metadata?: grpc.Metadata
  ): Promise<ComplianceReport>;

  VerifyDigest(
    request: VerifyDigestRequest,
    metadata?: grpc.Metadata
  ): Promise<DigestEntry>;

  GetKeys(
    request: { tenantId: string },
    metadata?: grpc.Metadata
  ): Promise<CryptographicKey[]>;
}

// Auth Service Definition
export interface AuthService {
  Login(
    request: AuthRequest,
    metadata?: grpc.Metadata
  ): Promise<AuthResponse>;

  ValidateToken(
    request: ValidateTokenRequest,
    metadata?: grpc.Metadata
  ): Promise<ValidateTokenResponse>;
}

// Health Service Definition
export interface HealthService {
  Check(
    request: HealthCheckRequest,
    metadata?: grpc.Metadata
  ): Promise<HealthCheckResponse>;
}

// =============================================================================
// Message Types for gRPC
// =============================================================================

export type EventMessage = Event & grpc.Message<EventMessage>;
export type QueryEventsRequestMessage = QueryEventsRequest & grpc.Message<QueryEventsRequestMessage>;
export type QueryEventsResponseMessage = QueryEventsResponse & grpc.Message<QueryEventsResponseMessage>;
export type AuthRequestMessage = AuthRequest & grpc.Message<AuthRequestMessage>;
export type AuthResponseMessage = AuthResponse & grpc.Message<AuthResponseMessage>;
