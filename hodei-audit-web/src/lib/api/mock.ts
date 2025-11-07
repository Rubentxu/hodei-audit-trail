/**
 * Mock API Service for Development
 *
 * Provides mock implementations of API endpoints for development and testing
 * without requiring a backend server.
 */

import { ApiResponse, ApiClient } from "./client";
import { QueryEventsRequest, QueryEventsResponse, Event, AnalyticsQuery, AnalyticsResult, ComplianceReport, GenerateReportRequest, CryptographicKey, AuthRequest, AuthResponse } from "../../grpc/types";
import { defaultCache } from "./cache";

/**
 * Generate mock events
 */
function generateMockEvents(count: number = 100): Event[] {
  const events: Event[] = [];
  const actions = ["create", "update", "delete", "view", "login", "logout", "export", "import"];
  const resources = ["user", "document", "report", "dashboard", "settings", "tenant"];
  const statuses: Event["status"][] = ["success", "failure", "error"];

  for (let i = 0; i < count; i++) {
    events.push({
      id: `evt-${i + 1}`,
      timestamp: new Date(Date.now() - Math.random() * 7 * 24 * 60 * 60 * 1000).toISOString(),
      tenantId: "tenant-1",
      userId: `user-${Math.floor(Math.random() * 10) + 1}`,
      userName: `User ${Math.floor(Math.random() * 10) + 1}`,
      action: actions[Math.floor(Math.random() * actions.length)],
      resource: resources[Math.floor(Math.random() * resources.length)],
      resourceId: `res-${Math.floor(Math.random() * 1000) + 1}`,
      sourceIp: `192.168.1.${Math.floor(Math.random() * 255) + 1}`,
      userAgent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
      status: statuses[Math.floor(Math.random() * statuses.length)],
      details: {
        field1: `value-${i}`,
        field2: Math.random() > 0.5 ? "yes" : "no",
      },
      metadata: {
        source: "web",
        environment: "production",
      },
    });
  }

  return events;
}

/**
 * Mock Event API
 */
export class MockEventApi {
  private events: Event[] = generateMockEvents(500);

  public async queryEvents(params: QueryEventsRequest): Promise<ApiResponse<QueryEventsResponse>> {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 200));

    // Apply filters
    let filtered = [...this.events];

    // Time range filter
    if (params.timeRange) {
      const start = new Date(params.timeRange.start).getTime();
      const end = new Date(params.timeRange.end).getTime();
      filtered = filtered.filter(e => {
        const ts = new Date(e.timestamp).getTime();
        return ts >= start && ts <= end;
      });
    }

    // Apply additional filters
    if (params.filters) {
      params.filters.forEach(filter => {
        filtered = filtered.filter(event => {
          const value = (event as any)[filter.field];
          switch (filter.operator) {
            case "eq":
              return value === filter.value;
            case "ne":
              return value !== filter.value;
            case "contains":
              return String(value).toLowerCase().includes(String(filter.value).toLowerCase());
            default:
              return true;
          }
        });
      });
    }

    // Apply sorting
    if (params.sorting) {
      const { field, direction } = params.sorting;
      filtered.sort((a, b) => {
        const aVal = (a as any)[field];
        const bVal = (b as any)[field];
        return direction === "asc"
          ? aVal > bVal ? 1 : -1
          : aVal < bVal ? 1 : -1;
      });
    }

    // Apply pagination
    const total = filtered.length;
    const page = params.pagination?.page || 1;
    const pageSize = params.pagination?.pageSize || 50;
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    const paginatedEvents = filtered.slice(start, end);

    return {
      success: true,
      data: {
        events: paginatedEvents,
        total,
        page,
        pageSize,
        hasMore: end < total,
      },
      meta: {
        requestId: `req_${Date.now()}`,
        timestamp: new Date().toISOString(),
      },
    };
  }

  public async getEvent(id: string): Promise<ApiResponse<Event>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    const event = this.events.find(e => e.id === id);

    if (!event) {
      return {
        success: false,
        error: {
          code: "NOT_FOUND",
          message: "Event not found",
        },
      };
    }

    return {
      success: true,
      data: event,
    };
  }
}

/**
 * Mock Analytics API
 */
export class MockAnalyticsApi {
  public async runQuery(query: AnalyticsQuery): Promise<ApiResponse<AnalyticsResult>> {
    await new Promise(resolve => setTimeout(resolve, 500));

    // Generate mock analytics data
    const data: Record<string, any>[] = [];
    const pointCount = 20;

    for (let i = 0; i < pointCount; i++) {
      data.push({
        timestamp: new Date(Date.now() - i * 24 * 60 * 60 * 1000).toISOString(),
        count: Math.floor(Math.random() * 1000) + 100,
        success: Math.floor(Math.random() * 800) + 100,
        failure: Math.floor(Math.random() * 50),
        unique_users: Math.floor(Math.random() * 100) + 10,
      });
    }

    return {
      success: true,
      data: {
        data,
        metadata: {
          queryTime: 500,
          rowCount: data.length,
          fields: ["timestamp", "count", "success", "failure", "unique_users"],
        },
      },
    };
  }

  public async getSavedQueries(): Promise<ApiResponse<any[]>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
      data: [
        {
          id: "q1",
          name: "Daily Event Count",
          query: { type: "count", field: "timestamp" },
          createdAt: new Date().toISOString(),
        },
        {
          id: "q2",
          name: "User Activity",
          query: { type: "groupBy", field: "userId" },
          createdAt: new Date().toISOString(),
        },
      ],
    };
  }

  public async saveQuery(query: any): Promise<ApiResponse<any>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
      data: {
        id: `q_${Date.now()}`,
        ...query,
        createdAt: new Date().toISOString(),
      },
    };
  }

  public async deleteSavedQuery(id: string): Promise<ApiResponse<void>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
    };
  }
}

/**
 * Mock Compliance API
 */
export class MockComplianceApi {
  private reports: ComplianceReport[] = [
    {
      id: "rpt-001",
      name: "SOC 2 Monthly Report",
      type: "SOC2",
      status: "ready",
      format: "pdf",
      tenantId: "tenant-1",
      createdAt: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
      generatedAt: new Date(Date.now() - 6 * 24 * 60 * 60 * 1000).toISOString(),
    },
    {
      id: "rpt-002",
      name: "PCI-DSS Weekly Report",
      type: "PCI_DSS",
      status: "generating",
      format: "pdf",
      tenantId: "tenant-1",
      createdAt: new Date().toISOString(),
    },
  ];

  public async generateReport(params: GenerateReportRequest): Promise<ApiResponse<ComplianceReport>> {
    await new Promise(resolve => setTimeout(resolve, 1000));

    const report: ComplianceReport = {
      id: `rpt-${Date.now()}`,
      name: params.name,
      type: params.type,
      status: "generating",
      format: params.format,
      tenantId: params.tenantId,
      createdAt: new Date().toISOString(),
    };

    this.reports.push(report);

    // Simulate report generation completing
    setTimeout(() => {
      const idx = this.reports.findIndex(r => r.id === report.id);
      if (idx >= 0) {
        this.reports[idx].status = "ready";
        this.reports[idx].generatedAt = new Date().toISOString();
      }
    }, 5000);

    return {
      success: true,
      data: report,
    };
  }

  public async getReports(): Promise<ApiResponse<ComplianceReport[]>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
      data: this.reports,
    };
  }

  public async getReport(id: string): Promise<ApiResponse<ComplianceReport>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    const report = this.reports.find(r => r.id === id);

    if (!report) {
      return {
        success: false,
        error: {
          code: "NOT_FOUND",
          message: "Report not found",
        },
      };
    }

    return {
      success: true,
      data: report,
    };
  }

  public async downloadReport(id: string): Promise<ApiResponse<{ downloadUrl: string }>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
      data: {
        downloadUrl: `/api/compliance/reports/${id}/download`,
      },
    };
  }

  public async deleteReport(id: string): Promise<ApiResponse<void>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    this.reports = this.reports.filter(r => r.id !== id);

    return {
      success: true,
    };
  }

  public async getKeys(tenantId: string): Promise<ApiResponse<CryptographicKey[]>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    const keys: CryptographicKey[] = [
      {
        id: "key-001",
        name: "production-ssl-cert",
        algorithm: "RSA-2048",
        status: "active",
        createdAt: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000).toISOString(),
        expiresAt: new Date(Date.now() + 275 * 24 * 60 * 60 * 1000).toISOString(),
        fingerprint: "AA:BB:CC:DD:EE:FF",
      },
      {
        id: "key-002",
        name: "internal-docs-pgp",
        algorithm: "PGP",
        status: "active",
        createdAt: new Date(Date.now() - 60 * 24 * 60 * 60 * 1000).toISOString(),
        expiresAt: new Date(Date.now() + 305 * 24 * 60 * 60 * 1000).toISOString(),
        fingerprint: "11:22:33:44:55:66",
      },
    ];

    return {
      success: true,
      data: keys,
    };
  }

  public async rotateKey(id: string): Promise<ApiResponse<CryptographicKey>> {
    await new Promise(resolve => setTimeout(resolve, 1000));

    const newKey: CryptographicKey = {
      id: `key-${Date.now()}`,
      name: `rotated-${id}`,
      algorithm: "RSA-2048",
      status: "active",
      createdAt: new Date().toISOString(),
      expiresAt: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString(),
      fingerprint: "FF:EE:DD:CC:BB:AA",
    };

    return {
      success: true,
      data: newKey,
    };
  }
}

/**
 * Mock Auth API
 */
export class MockAuthApi {
  public async login(params: AuthRequest): Promise<ApiResponse<AuthResponse>> {
    await new Promise(resolve => setTimeout(resolve, 500));

    // Simple mock authentication
    if (params.username === "admin" && params.password === "password") {
      return {
        success: true,
        data: {
          token: `token_${Date.now()}`,
          refreshToken: `refresh_${Date.now()}`,
          expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
          user: {
            id: "user-1",
            username: "admin",
            email: "admin@example.com",
            role: "admin",
          },
        },
      };
    }

    return {
      success: false,
      error: {
        code: "INVALID_CREDENTIALS",
        message: "Invalid username or password",
      },
    };
  }

  public async logout(): Promise<ApiResponse<void>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
    };
  }

  public async refreshToken(refreshToken: string): Promise<ApiResponse<AuthResponse>> {
    await new Promise(resolve => setTimeout(resolve, 200));

    return {
      success: true,
      data: {
        token: `token_${Date.now()}`,
        refreshToken: `refresh_${Date.now()}`,
        expiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
        user: {
          id: "user-1",
          username: "admin",
          email: "admin@example.com",
          role: "admin",
        },
      },
    };
  }

  public async validateToken(token: string): Promise<ApiResponse<{ valid: boolean }>> {
    await new Promise(resolve => setTimeout(resolve, 100));

    return {
      success: true,
      data: {
        valid: token.startsWith("token_"),
      },
    };
  }
}

/**
 * Create mock API instances
 */
export function createMockApis() {
  return {
    event: new MockEventApi(),
    analytics: new MockAnalyticsApi(),
    compliance: new MockComplianceApi(),
    auth: new MockAuthApi(),
  };
}

/**
 * Mock API instance
 */
export const mockApis = createMockApis();
