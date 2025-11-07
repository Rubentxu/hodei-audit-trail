/**
 * React Query Integration
 *
 * Provides custom hooks for data fetching using React Query with gRPC and REST APIs.
 */

import { useQuery, useMutation, useQueryClient, UseQueryOptions, UseMutationOptions } from "@tanstack/react-query";
import { apiClients } from "./client";
import { mockApis } from "./mock";
import { QueryEventsRequest, QueryEventsResponse, Event, AnalyticsQuery, AnalyticsResult, ComplianceReport, GenerateReportRequest, AuthRequest, AuthResponse } from "../../grpc/types";
import { defaultCache } from "./cache";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * React Query configuration
 */
export const queryClientConfig = {
  defaultOptions: {
    queries: {
      retry: 3,
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
      refetchOnWindowFocus: false,
      refetchOnReconnect: true,
    },
    mutations: {
      retry: 1,
    },
  },
};

/**
 * Query Keys
 */
export const queryKeys = {
  // Event queries
  events: {
    all: (tenantId: string) => ["events", tenantId] as const,
    list: (tenantId: string, params?: any) => [...queryKeys.events.all(tenantId), "list", params] as const,
    detail: (id: string) => ["events", "detail", id] as const,
  },
  // Analytics queries
  analytics: {
    all: (tenantId: string) => ["analytics", tenantId] as const,
    query: (hash: string) => ["analytics", "query", hash] as const,
    saved: (tenantId: string) => [...queryKeys.analytics.all(tenantId), "saved"] as const,
  },
  // Compliance queries
  compliance: {
    all: (tenantId: string) => ["compliance", tenantId] as const,
    reports: (tenantId: string, filters?: any) => [...queryKeys.compliance.all(tenantId), "reports", filters] as const,
    report: (id: string) => ["compliance", "report", id] as const,
    keys: (tenantId: string) => [...queryKeys.compliance.all(tenantId), "keys"] as const,
  },
  // Auth queries
  auth: {
    user: (userId: string) => ["auth", "user", userId] as const,
    token: () => ["auth", "token"] as const,
  },
} as const;

// =============================================================================
// Event Hooks
// =============================================================================

/**
 * Hook to query events
 */
export function useEvents(
  params: QueryEventsRequest,
  options?: UseQueryOptions<QueryEventsResponse>
) {
  return useQuery({
    queryKey: queryKeys.events.list(params.tenantId, params),
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.event.queryEvents(params);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch events");
        }
        return response.data!;
      }

      const response = await apiClients.event.queryEvents(params);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch events");
      }
      return response.data!;
    },
    ...options,
  });
}

/**
 * Hook to get a single event
 */
export function useEvent(
  id: string,
  options?: UseQueryOptions<Event>
) {
  return useQuery({
    queryKey: queryKeys.events.detail(id),
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.event.getEvent(id);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch event");
        }
        return response.data!;
      }

      const response = await apiClients.event.getEvent(id);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch event");
      }
      return response.data!;
    },
    enabled: !!id,
    ...options,
  });
}

/**
 * Hook to export events
 */
export function useExportEvents() {
  return useMutation({
    mutationFn: async (params: QueryEventsRequest & { format: "csv" | "json" | "pdf" }) => {
      if (USE_MOCK_API) {
        // Mock export - return success
        return { downloadUrl: "/api/events/export/mock" };
      }

      const response = await apiClients.event.exportEvents(params);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to export events");
      }
      return response.data!;
    },
  });
}

// =============================================================================
// Analytics Hooks
// =============================================================================

/**
 * Hook to run analytics query
 */
export function useAnalyticsQuery(
  query: AnalyticsQuery | null,
  options?: UseQueryOptions<AnalyticsResult>
) {
  return useQuery({
    queryKey: queryKeys.analytics.query(JSON.stringify(query)),
    queryFn: async () => {
      if (!query) {
        throw new Error("Query is required");
      }

      if (USE_MOCK_API) {
        const response = await mockApis.analytics.runQuery(query);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to run analytics query");
        }
        return response.data!;
      }

      const response = await apiClients.analytics.runQuery(query);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to run analytics query");
      }
      return response.data!;
    },
    enabled: !!query,
    ...options,
  });
}

/**
 * Hook to get saved queries
 */
export function useSavedQueries(
  options?: UseQueryOptions<any[]>
) {
  return useQuery({
    queryKey: queryKeys.analytics.saved("tenant-1"), // In real app, use actual tenant
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.analytics.getSavedQueries();
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch saved queries");
        }
        return response.data!;
      }

      const response = await apiClients.analytics.getSavedQueries();
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch saved queries");
      }
      return response.data!;
    },
    staleTime: 2 * 60 * 1000, // 2 minutes
    ...options,
  });
}

/**
 * Hook to save a query
 */
export function useSaveQuery() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: { name: string; query: AnalyticsQuery; isPublic: boolean }) => {
      if (USE_MOCK_API) {
        const response = await mockApis.analytics.saveQuery(params);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to save query");
        }
        return response.data!;
      }

      const response = await apiClients.analytics.saveQuery(params);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to save query");
      }
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.analytics.saved("tenant-1") });
    },
  });
}

/**
 * Hook to delete a saved query
 */
export function useDeleteSavedQuery() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (id: string) => {
      if (USE_MOCK_API) {
        const response = await mockApis.analytics.deleteSavedQuery(id);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to delete query");
        }
        return;
      }

      const response = await apiClients.analytics.deleteSavedQuery(id);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to delete query");
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.analytics.saved("tenant-1") });
    },
  });
}

// =============================================================================
// Compliance Hooks
// =============================================================================

/**
 * Hook to get compliance reports
 */
export function useComplianceReports(
  filters?: Record<string, any>,
  options?: UseQueryOptions<ComplianceReport[]>
) {
  return useQuery({
    queryKey: queryKeys.compliance.reports("tenant-1", filters), // In real app, use actual tenant
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.getReports();
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch reports");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.getReports(filters);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch reports");
      }
      return response.data!;
    },
    staleTime: 1 * 60 * 1000, // 1 minute
    ...options,
  });
}

/**
 * Hook to get a single compliance report
 */
export function useComplianceReport(
  id: string,
  options?: UseQueryOptions<ComplianceReport>
) {
  return useQuery({
    queryKey: queryKeys.compliance.report(id),
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.getReport(id);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch report");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.getReport(id);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch report");
      }
      return response.data!;
    },
    enabled: !!id,
    ...options,
  });
}

/**
 * Hook to generate a compliance report
 */
export function useGenerateReport() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: GenerateReportRequest) => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.generateReport(params);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to generate report");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.generateReport(params);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to generate report");
      }
      return response.data!;
    },
    onSuccess: (data) => {
      // Invalidate reports list
      queryClient.invalidateQueries({ queryKey: queryKeys.compliance.reports("tenant-1") });
      // Add the new report to cache
      queryClient.setQueryData(queryKeys.compliance.report(data.id), data);
    },
  });
}

/**
 * Hook to download a report
 */
export function useDownloadReport() {
  return useMutation({
    mutationFn: async (id: string) => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.downloadReport(id);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to download report");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.downloadReport(id);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to download report");
      }
      return response.data!;
    },
  });
}

/**
 * Hook to get cryptographic keys
 */
export function useComplianceKeys(
  options?: UseQueryOptions<CryptographicKey[]>
) {
  return useQuery({
    queryKey: queryKeys.compliance.keys("tenant-1"), // In real app, use actual tenant
    queryFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.getKeys("tenant-1");
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to fetch keys");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.getKeys("tenant-1");
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to fetch keys");
      }
      return response.data!;
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
    ...options,
  });
}

/**
 * Hook to rotate a key
 */
export function useRotateKey() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (id: string) => {
      if (USE_MOCK_API) {
        const response = await mockApis.compliance.rotateKey(id);
        if (!response.success) {
          throw new Error(response.error?.message || "Failed to rotate key");
        }
        return response.data!;
      }

      const response = await apiClients.compliance.rotateKey(id);
      if (!response.success) {
        throw new Error(response.error?.message || "Failed to rotate key");
      }
      return response.data!;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.compliance.keys("tenant-1") });
    },
  });
}

// =============================================================================
// Auth Hooks
// =============================================================================

/**
 * Hook to login
 */
export function useLogin() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (params: AuthRequest) => {
      if (USE_MOCK_API) {
        const response = await mockApis.auth.login(params);
        if (!response.success) {
          throw new Error(response.error?.message || "Login failed");
        }
        return response.data!;
      }

      const response = await apiClients.auth.login(params);
      if (!response.success) {
        throw new Error(response.error?.message || "Login failed");
      }
      return response.data!;
    },
    onSuccess: (data) => {
      // Store token in cache or localStorage
      localStorage.setItem("authToken", data.token);
      localStorage.setItem("refreshToken", data.refreshToken);

      // Invalidate auth queries
      queryClient.invalidateQueries({ queryKey: queryKeys.auth.user(data.user.id) });
    },
  });
}

/**
 * Hook to logout
 */
export function useLogout() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      if (USE_MOCK_API) {
        const response = await mockApis.auth.logout();
        if (!response.success) {
          throw new Error(response.error?.message || "Logout failed");
        }
        return;
      }

      const response = await apiClients.auth.logout();
      if (!response.success) {
        throw new Error(response.error?.message || "Logout failed");
      }
    },
    onSuccess: () => {
      // Clear tokens
      localStorage.removeItem("authToken");
      localStorage.removeItem("refreshToken");

      // Clear all queries
      queryClient.clear();
    },
  });
}

/**
 * Hook to refresh token
 */
export function useRefreshToken() {
  return useMutation({
    mutationFn: async (refreshToken: string) => {
      if (USE_MOCK_API) {
        const response = await mockApis.auth.refreshToken(refreshToken);
        if (!response.success) {
          throw new Error(response.error?.message || "Token refresh failed");
        }
        return response.data!;
      }

      const response = await apiClients.auth.refreshToken(refreshToken);
      if (!response.success) {
        throw new Error(response.error?.message || "Token refresh failed");
      }
      return response.data!;
    },
    onSuccess: (data) => {
      localStorage.setItem("authToken", data.token);
      localStorage.setItem("refreshToken", data.refreshToken);
    },
  });
}

/**
 * Hook to validate token
 */
export function useValidateToken(options?: UseQueryOptions<{ valid: boolean }>) {
  return useQuery({
    queryKey: queryKeys.auth.token(),
    queryFn: async () => {
      const token = localStorage.getItem("authToken");
      if (!token) {
        return { valid: false };
      }

      if (USE_MOCK_API) {
        const response = await mockApis.auth.validateToken(token);
        if (!response.success) {
          throw new Error(response.error?.message || "Token validation failed");
        }
        return response.data!;
      }

      const response = await apiClients.auth.validateToken(token);
      if (!response.success) {
        throw new Error(response.error?.message || "Token validation failed");
      }
      return response.data!;
    },
    retry: false,
    refetchOnWindowFocus: true,
    staleTime: 0, // Always refetch
    ...options,
  });
}

// =============================================================================
// Utility Hooks
// =============================================================================

/**
 * Hook to invalidate all queries for a tenant
 */
export function useInvalidateTenantQueries() {
  const queryClient = useQueryClient();

  return (tenantId: string) => {
    queryClient.invalidateQueries({ queryKey: ["events", tenantId] });
    queryClient.invalidateQueries({ queryKey: ["analytics", tenantId] });
    queryClient.invalidateQueries({ queryKey: ["compliance", tenantId] });
  };
}

/**
 * Hook to refetch all active queries
 */
export function useRefetchQueries() {
  const queryClient = useQueryClient();

  return () => {
    queryClient.refetchQueries();
  };
}

/**
 * Hook to get query client
 */
export function useQueryClientInstance() {
  return useQueryClient();
}
