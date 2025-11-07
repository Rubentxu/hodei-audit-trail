/**
 * API Monitoring and Metrics Collection
 *
 * Collects and provides API performance metrics.
 */

export interface ApiMetrics {
  timestamp: number;
  endpoint: string;
  method: string;
  statusCode: number;
  responseTime: number;
  userId?: string;
  tenantId?: string;
  errorCode?: string;
}

export interface MetricsSummary {
  totalRequests: number;
  averageResponseTime: number;
  errorRate: number;
  requestsPerMinute: number;
  topEndpoints: { endpoint: string; count: number; avgTime: number }[];
  statusCodes: Record<string, number>;
  errorCodes: Record<string, number>;
}

/**
 * In-memory metrics store (use Redis in production)
 */
class MetricsStore {
  private metrics: ApiMetrics[] = [];
  private maxSize = 10000;

  /**
   * Record a metric
   */
  public record(metric: ApiMetrics): void {
    this.metrics.push(metric);

    // Keep only the most recent metrics
    if (this.metrics.length > this.maxSize) {
      this.metrics = this.metrics.slice(-this.maxSize);
    }
  }

  /**
   * Get metrics summary
   */
  public getSummary(timeWindowMs: number = 3600000): MetricsSummary {
    const cutoff = Date.now() - timeWindowMs;
    const recent = this.metrics.filter((m) => m.timestamp >= cutoff);

    const totalRequests = recent.length;
    const responseTimes = recent.map((m) => m.responseTime);
    const averageResponseTime = responseTimes.length > 0
      ? responseTimes.reduce((a, b) => a + b, 0) / responseTimes.length
      : 0;

    const errorCount = recent.filter((m) => m.statusCode >= 400).length;
    const errorRate = totalRequests > 0 ? (errorCount / totalRequests) * 100 : 0;

    const rpm = (totalRequests / (timeWindowMs / 60000)).toFixed(2);

    // Top endpoints
    const endpointMap = new Map<string, { count: number; totalTime: number }>();
    recent.forEach((m) => {
      const key = `${m.method} ${m.endpoint}`;
      const existing = endpointMap.get(key) || { count: 0, totalTime: 0 };
      existing.count++;
      existing.totalTime += m.responseTime;
      endpointMap.set(key, existing);
    });

    const topEndpoints = Array.from(endpointMap.entries())
      .map(([endpoint, { count, totalTime }]) => ({
        endpoint,
        count,
        avgTime: totalTime / count,
      }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);

    // Status codes
    const statusCodes: Record<string, number> = {};
    recent.forEach((m) => {
      const code = m.statusCode.toString();
      statusCodes[code] = (statusCodes[code] || 0) + 1;
    });

    // Error codes
    const errorCodes: Record<string, number> = {};
    recent.forEach((m) => {
      if (m.errorCode) {
        errorCodes[m.errorCode] = (errorCodes[m.errorCode] || 0) + 1;
      }
    });

    return {
      totalRequests,
      averageResponseTime,
      errorRate,
      requestsPerMinute: parseFloat(rpm),
      topEndpoints,
      statusCodes,
      errorCodes,
    };
  }

  /**
   * Get recent metrics
   */
  public getRecent(limit: number = 100): ApiMetrics[] {
    return this.metrics.slice(-limit);
  }

  /**
   * Clear all metrics
   */
  public clear(): void {
    this.metrics = [];
  }
}

// Global metrics store
export const metricsStore = new MetricsStore();

/**
 * Middleware to record API metrics
 */
export function withMetrics(
  handler: (request: NextRequest, context: any) => Promise<NextResponse>
) {
  return async (request: NextRequest, context: any) => {
    const startTime = Date.now();
    const url = new URL(request.url);

    try {
      const response = await handler(request, context);
      const endTime = Date.now();
      const responseTime = endTime - startTime;

      // Record metric
      metricsStore.record({
        timestamp: startTime,
        endpoint: url.pathname,
        method: request.method,
        statusCode: response.status,
        responseTime,
        userId: context.user?.id,
        tenantId: context.tenantId,
      });

      // Add performance headers
      response.headers.set("X-Response-Time", `${responseTime}ms`);
      response.headers.set("X-Timestamp", `${startTime}`);

      return response;
    } catch (error) {
      const endTime = Date.now();
      const responseTime = endTime - startTime;

      // Record error metric
      metricsStore.record({
        timestamp: startTime,
        endpoint: url.pathname,
        method: request.method,
        statusCode: 500,
        responseTime,
        userId: context.user?.id,
        tenantId: context.tenantId,
        errorCode: error instanceof Error ? error.message : "UNKNOWN_ERROR",
      });

      throw error;
    }
  };
}

/**
 * API endpoint to get metrics
 */
export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const timeWindow = parseInt(searchParams.get("timeWindow") || "3600000");
    const summary = metricsStore.getSummary(timeWindow);

    return NextResponse.json({
      success: true,
      data: summary,
      meta: {
        timeWindow,
        timestamp: Date.now(),
      },
    });
  } catch (error) {
    console.error("[Metrics] Get summary error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "METRICS_ERROR",
          message: "Failed to fetch metrics",
        },
      },
      { status: 500 }
    );
  }
}
