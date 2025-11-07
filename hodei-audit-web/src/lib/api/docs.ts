/**
 * API Documentation Configuration
 *
 * OpenAPI/Swagger documentation for all API endpoints.
 */

export const apiDocumentation = {
  openapi: "3.0.0",
  info: {
    title: "Hodei Audit API",
    version: "1.0.0",
    description: "Comprehensive audit trail API with real-time updates",
    contact: {
      name: "API Support",
      email: "support@hodei-audit.com",
    },
    license: {
      name: "MIT",
      url: "https://opensource.org/licenses/MIT",
    },
  },
  servers: [
    {
      url: process.env.NEXT_PUBLIC_API_BASE_URL || "http://localhost:3000/api",
      description: "Development server",
    },
    {
      url: "https://api.hodei-audit.com/v1",
      description: "Production server",
    },
  ],
  components: {
    securitySchemes: {
      bearerAuth: {
        type: "http",
        scheme: "bearer",
        bearerFormat: "JWT",
        description: "JWT token for authentication",
      },
      apiKey: {
        type: "apiKey",
        in: "header",
        name: "x-api-key",
        description: "API key for service-to-service authentication",
      },
    },
    schemas: {
      Error: {
        type: "object",
        properties: {
          success: {
            type: "boolean",
            example: false,
          },
          error: {
            type: "object",
            properties: {
              code: {
                type: "string",
                example: "VALIDATION_ERROR",
              },
              message: {
                type: "string",
                example: "Invalid input parameters",
              },
              details: {
                type: "object",
                nullable: true,
              },
            },
          },
        },
      },
      Event: {
        type: "object",
        properties: {
          id: { type: "string", example: "evt-123" },
          timestamp: { type: "string", format: "date-time" },
          tenantId: { type: "string" },
          userId: { type: "string", nullable: true },
          userName: { type: "string", nullable: true },
          action: { type: "string" },
          resource: { type: "string" },
          resourceId: { type: "string", nullable: true },
          sourceIp: { type: "string" },
          userAgent: { type: "string", nullable: true },
          status: { type: "string", enum: ["success", "failure", "error"] },
          details: { type: "object", nullable: true },
          metadata: { type: "object", nullable: true },
        },
      },
      ComplianceReport: {
        type: "object",
        properties: {
          id: { type: "string" },
          name: { type: "string" },
          type: { type: "string", enum: ["SOC2", "PCI_DSS", "GDPR", "HIPAA", "ISO27001"] },
          status: { type: "string", enum: ["generating", "ready", "failed", "expired"] },
          format: { type: "string", enum: ["pdf", "json", "csv"] },
          tenantId: { type: "string" },
          createdAt: { type: "string", format: "date-time" },
          generatedAt: { type: "string", format: "date-time", nullable: true },
          expiresAt: { type: "string", format: "date-time", nullable: true },
        },
      },
    },
  },
  security: [
    { bearerAuth: [] },
  ],
  tags: [
    { name: "Events", description: "Event management endpoints" },
    { name: "Analytics", description: "Analytics and reporting endpoints" },
    { name: "Compliance", description: "Compliance report endpoints" },
    { name: "Auth", description: "Authentication endpoints" },
    { name: "SSE", description: "Server-Sent Events endpoints" },
  ],
  paths: {
    "/events": {
      get: {
        tags: ["Events"],
        summary: "Query events",
        description: "Retrieve events with filtering, pagination, and sorting",
        parameters: [
          { name: "tenantId", in: "query", schema: { type: "string" } },
          { name: "page", in: "query", schema: { type: "integer", default: 1 } },
          { name: "pageSize", in: "query", schema: { type: "integer", default: 50 } },
          { name: "sortBy", in: "query", schema: { type: "string", default: "timestamp" } },
          { name: "sortOrder", in: "query", schema: { type: "string", enum: ["asc", "desc"], default: "desc" } },
          { name: "startDate", in: "query", schema: { type: "string", format: "date-time" } },
          { name: "endDate", in: "query", schema: { type: "string", format: "date-time" } },
        ],
        responses: {
          200: {
            description: "Success",
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    success: { type: "boolean", example: true },
                    data: {
                      type: "object",
                      properties: {
                        events: { type: "array", items: { $ref: "#/components/schemas/Event" } },
                        total: { type: "integer" },
                        page: { type: "integer" },
                        pageSize: { type: "integer" },
                        hasMore: { type: "boolean" },
                      },
                    },
                  },
                },
              },
            },
          },
          400: { $ref: "#/components/responses/BadRequest" },
          401: { $ref: "#/components/responses/Unauthorized" },
          429: { $ref: "#/components/responses/RateLimited" },
          500: { $ref: "#/components/responses/InternalError" },
        },
      },
    },
    "/events/{id}": {
      get: {
        tags: ["Events"],
        summary: "Get event by ID",
        parameters: [
          { name: "id", in: "path", required: true, schema: { type: "string" } },
        ],
        responses: {
          200: { description: "Success" },
          404: { $ref: "#/components/responses/NotFound" },
        },
      },
    },
    "/analytics/query": {
      post: {
        tags: ["Analytics"],
        summary: "Run analytics query",
        requestBody: {
          required: true,
          content: {
            "application/json": {
              schema: {
                type: "object",
                properties: {
                  query: { type: "object" },
                },
              },
            },
          },
        },
        responses: {
          200: { description: "Success" },
          400: { $ref: "#/components/responses/BadRequest" },
        },
      },
    },
    "/compliance/reports": {
      get: {
        tags: ["Compliance"],
        summary: "Get compliance reports",
        parameters: [
          { name: "type", in: "query", schema: { type: "string" } },
          { name: "status", in: "query", schema: { type: "string" } },
          { name: "page", in: "query", schema: { type: "integer", default: 1 } },
        ],
        responses: {
          200: { description: "Success" },
        },
      },
      post: {
        tags: ["Compliance"],
        summary: "Generate compliance report",
        requestBody: {
          required: true,
          content: {
            "application/json": {
              schema: {
                type: "object",
                required: ["name", "type", "format", "timeRange"],
                properties: {
                  name: { type: "string" },
                  type: { type: "string", enum: ["SOC2", "PCI_DSS", "GDPR", "HIPAA", "ISO27001"] },
                  format: { type: "string", enum: ["pdf", "json", "csv"] },
                  timeRange: {
                    type: "object",
                    properties: {
                      start: { type: "string", format: "date-time" },
                      end: { type: "string", format: "date-time" },
                    },
                  },
                },
              },
            },
          },
        },
        responses: {
          201: { description: "Report generation started" },
        },
      },
    },
    "/sse/stream": {
      get: {
        tags: ["SSE"],
        summary: "Server-Sent Events stream",
        description: "Establish SSE connection for real-time updates",
        parameters: [
          { name: "tenantId", in: "query", schema: { type: "string" } },
          { name: "eventTypes", in: "query", schema: { type: "string" } },
        ],
        responses: {
          200: {
            description: "SSE stream established",
            content: {
              "text/event-stream": {
                schema: { type: "string" },
              },
            },
          },
        },
      },
    },
  },
  responses: {
    BadRequest: {
      description: "Bad request",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
          example: {
            success: false,
            error: {
              code: "VALIDATION_ERROR",
              message: "Invalid request parameters",
            },
          },
        },
      },
    },
    Unauthorized: {
      description: "Unauthorized",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
        },
      },
    },
    Forbidden: {
      description: "Forbidden",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
        },
      },
    },
    NotFound: {
      description: "Not found",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
        },
      },
    },
    RateLimited: {
      description: "Rate limit exceeded",
      headers: {
        "Retry-After": {
          description: "Seconds to wait before retrying",
          schema: { type: "integer" },
        },
      },
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
        },
      },
    },
    InternalError: {
      description: "Internal server error",
      content: {
        "application/json": {
          schema: { $ref: "#/components/schemas/Error" },
        },
      },
    },
  },
};

/**
 * Generate markdown documentation
 */
export function generateMarkdownDocs(): string {
  const doc = apiDocumentation;

  let markdown = `# ${doc.info.title}\n\n`;
  markdown += `**Version:** ${doc.info.version}\n\n`;
  markdown += `${doc.info.description}\n\n`;

  // Authentication section
  markdown += `## Authentication\n\n`;
  markdown += `The API supports two authentication methods:\n\n`;
  markdown += `1. **Bearer Token (JWT)** - For user authentication\n`;
  markdown += `   \\`\\`\\`\n`;
  markdown += `   Authorization: Bearer <token>\n`;
  markdown += `   \\`\\`\\`\n\n`;
  markdown += `2. **API Key** - For service-to-service authentication\n`;
  markdown += `   \\`\\`\\`\n`;
  markdown += `   x-api-key: <api-key>\n`;
  markdown += `   \\`\\`\\`\n\n`;

  // Endpoints
  markdown += `## Endpoints\n\n`;

  Object.entries(doc.paths).forEach(([path, methods]) => {
    const pathKey = path.replace("/", "").replace(/-/g, " ");
    markdown += `### ${pathKey}\n\n`;

    Object.entries(methods as any).forEach(([method, details]: [string, any]) => {
      markdown += `#### ${method.toUpperCase()} ${path}\n\n`;
      markdown += `${details.summary}\n\n`;

      if (details.parameters) {
        markdown += `**Parameters:**\n\n`;
        details.parameters.forEach((param: any) => {
          markdown += `- \`${param.name}\` (${param.in}) - ${param.schema?.type || param.schema?.$ref || 'object'}\n`;
        });
        markdown += `\n`;
      }

      if (details.responses) {
        markdown += `**Responses:**\n\n`;
        Object.entries(details.responses).forEach(([code, response]: [string, any]) => {
          const description = response.description || '';
          markdown += `- \`${code}\` - ${description}\n`;
        });
        markdown += `\n`;
      }
    });
  });

  // Error codes
  markdown += `## Error Codes\n\n`;
  markdown += `| Code | Description |\n`;
  markdown += `|------|-------------|\n`;
  markdown += `| UNAUTHENTICATED | Authentication required |\n`;
  markdown += `| FORBIDDEN | Insufficient permissions |\n`;
  markdown += `| VALIDATION_ERROR | Invalid request parameters |\n`;
  markdown += `| NOT_FOUND | Resource not found |\n`;
  markdown += `| RATE_LIMIT_EXCEEDED | Too many requests |\n`;
  markdown += `| INTERNAL_ERROR | Server error |\n\n`;

  return markdown;
}
