/**
 * Compliance API Endpoints
 *
 * Provides compliance report generation and management.
 */

import { NextRequest, NextResponse } from "next/server";
import { withAuth, AuthContext } from "@/lib/middleware/auth";
import { mockApis } from "@/lib/api/mock";
import { createApiClients } from "@/lib/api/client";
import { createComplianceServiceClient } from "@/lib/grpc/factory";
import { GenerateReportRequest, ComplianceReport } from "@/lib/grpc/types";

// Feature flag to use mock APIs in development
const USE_MOCK_API = process.env.NEXT_PUBLIC_USE_MOCK_API === "true";

/**
 * Create compliance service client
 */
const complianceClient = USE_MOCK_API ? null : createComplianceServiceClient();

/**
 * GET /api/compliance/reports
 * Get compliance reports with filtering
 */
async function handleGetReports(
  request: NextRequest,
  context: AuthContext
): Promise<NextResponse> {
  try {
    const searchParams = request.nextUrl.searchParams;
    const type = searchParams.get("type");
    const status = searchParams.get("status");
    const page = parseInt(searchParams.get("page") || "1");
    const pageSize = parseInt(searchParams.get("pageSize") || "10");

    // In a real implementation, apply filters and pagination
    // For now, return all reports

    let result: ComplianceReport[];
    if (USE_MOCK_API) {
      const response = await mockApis.compliance.getReports();
      result = response.data!;
    } else {
      const apiClients = createApiClients(complianceClient!);
      const response = await apiClients.compliance.getReports({ tenantId: context.tenantId! });
      result = response.data!;
    }

    // Apply filters
    if (type) {
      result = result.filter((r) => r.type === type);
    }
    if (status) {
      result = result.filter((r) => r.status === status);
    }

    // Apply pagination
    const total = result.length;
    const start = (page - 1) * pageSize;
    const end = start + pageSize;
    const paginated = result.slice(start, end);

    return NextResponse.json({
      success: true,
      data: paginated,
      meta: {
        total,
        page,
        pageSize,
        hasMore: end < total,
        requestId: `req_${Date.now()}`,
        timestamp: new Date().toISOString(),
      },
    });
  } catch (error) {
    console.error("[Compliance] Get reports error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "COMPLIANCE_ERROR",
          message: "Failed to fetch reports",
        },
      },
      { status: 500 }
    );
  }
}

/**
 * POST /api/compliance/reports
 * Generate a new compliance report
 */
async function handleGenerateReport(
  request: NextRequest,
  context: AuthContext
): Promise<NextResponse> {
  try {
    const body = await request.json();
    const { name, type, format, timeRange, sections, template, recipients } = body;

    if (!name || !type || !format || !timeRange) {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "VALIDATION_ERROR",
            message: "name, type, format, and timeRange are required",
          },
        },
        { status: 400 }
      );
    }

    const reportRequest: GenerateReportRequest = {
      tenantId: context.tenantId!,
      name,
      type: type as ComplianceReport["type"],
      format: format as ComplianceReport["format"],
      timeRange,
      sections,
      template,
      recipients,
    };

    let report: ComplianceReport;
    if (USE_MOCK_API) {
      const response = await mockApis.compliance.generateReport(reportRequest);
      report = response.data!;
    } else {
      const apiClients = createApiClients(complianceClient!);
      const response = await apiClients.compliance.generateReport(reportRequest);
      report = response.data!;
    }

    // Log report generation
    console.log(`[Compliance] Report generated: ${report.id} by user ${context.user?.id}`);

    return NextResponse.json({
      success: true,
      data: report,
      meta: {
        requestId: `req_${Date.now()}`,
        timestamp: new Date().toISOString(),
      },
    });
  } catch (error) {
    console.error("[Compliance] Generate report error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "COMPLIANCE_ERROR",
          message: "Failed to generate report",
        },
      },
      { status: 500 }
    );
  }
}

/**
 * GET /api/compliance/keys
 * Get cryptographic keys
 */
async function handleGetKeys(
  request: NextRequest,
  context: AuthContext
): Promise<NextResponse> {
  try {
    let result;
    if (USE_MOCK_API) {
      const response = await mockApis.compliance.getKeys(context.tenantId!);
      result = response.data!;
    } else {
      const apiClients = createApiClients(complianceClient!);
      const response = await apiClients.compliance.getKeys({ tenantId: context.tenantId! });
      result = response.data!;
    }

    return NextResponse.json({
      success: true,
      data: result,
      meta: {
        requestId: `req_${Date.now()}`,
        timestamp: new Date().toISOString(),
      },
    });
  } catch (error) {
    console.error("[Compliance] Get keys error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "COMPLIANCE_ERROR",
          message: "Failed to fetch keys",
        },
      },
      { status: 500 }
    );
  }
}

export const GET = withAuth(handleGetReports, ["view:compliance"]);
export const POST = withAuth(handleGenerateReport, ["view:compliance", "generate:reports"]);

export const GET_KEYS = withAuth(handleGetKeys, ["view:compliance", "manage:compliance"]);
