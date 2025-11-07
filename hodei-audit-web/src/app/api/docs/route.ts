/**
 * API Documentation Endpoints
 *
 * Serve OpenAPI documentation and specifications.
 */

import { NextRequest, NextResponse } from "next/server";
import { apiDocumentation, generateMarkdownDocs } from "@/lib/api/docs";

/**
 * GET /api/docs/openapi.json
 * Get OpenAPI specification
 */
export async function GET(request: NextRequest) {
  try {
    const url = new URL(request.url);
    const format = url.searchParams.get("format") || "json";

    if (format === "json") {
      return NextResponse.json(apiDocumentation, {
        headers: {
          "Content-Type": "application/json",
          "Cache-Control": "public, max-age=3600",
        },
      });
    } else if (format === "markdown" || format === "md") {
      const markdown = generateMarkdownDocs();
      return new Response(markdown, {
        headers: {
          "Content-Type": "text/markdown",
          "Cache-Control": "public, max-age=3600",
        },
      });
    } else {
      return NextResponse.json(
        {
          success: false,
          error: {
            code: "INVALID_FORMAT",
            message: "Format must be 'json' or 'markdown'",
          },
        },
        { status: 400 }
      );
    }
  } catch (error) {
    console.error("[Docs] Error generating documentation:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "DOCS_ERROR",
          message: "Failed to generate documentation",
        },
      },
      { status: 500 }
    );
  }
}

/**
 * POST /api/docs/validate
 * Validate OpenAPI specification
 */
export async function POST(request: NextRequest) {
  try {
    // In a real implementation, validate the OpenAPI spec
    const isValid = true;

    return NextResponse.json({
      success: true,
      data: {
        valid: isValid,
        errors: [],
        warnings: [],
      },
    });
  } catch (error) {
    console.error("[Docs] Validation error:", error);
    return NextResponse.json(
      {
        success: false,
        error: {
          code: "VALIDATION_ERROR",
          message: "Failed to validate specification",
        },
      },
      { status: 500 }
    );
  }
}
