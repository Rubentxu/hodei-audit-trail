import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { getToken } from "next-auth/jwt";
import { Permission, ROLE_PERMISSIONS } from "@/lib/auth/permissions";

// Define which paths require which permissions
interface RoutePermission {
  pattern: RegExp;
  permission: Permission;
}

// Define protected routes with their required permissions
const PROTECTED_ROUTES: RoutePermission[] = [
  { pattern: /^\/admin(?:\/.*)?$/, permission: "manage:tenants" as Permission },
  {
    pattern: /^\/settings(?:\/.*)?$/,
    permission: "manage:settings" as Permission,
  },
  { pattern: /^\/profile(?:\/.*)?$/, permission: "view:users" as Permission },
  {
    pattern: /^\/dashboard(?:\/.*)?$/,
    permission: "view:events" as Permission,
  },
  { pattern: /^\/events(?:\/.*)?$/, permission: "view:events" as Permission },
  {
    pattern: /^\/analytics(?:\/.*)?$/,
    permission: "view:analytics" as Permission,
  },
  {
    pattern: /^\/compliance(?:\/.*)?$/,
    permission: "view:compliance" as Permission,
  },
];

// Basic protected paths (require authentication)
const BASIC_PROTECTED_PATHS = ["/dashboard", "/profile", "/settings", "/admin"];

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Check if the path requires any protection
  const requiresAuth = PROTECTED_ROUTES.some((route) =>
    route.pattern.test(pathname),
  );
  const isBasicProtected = BASIC_PROTECTED_PATHS.some((path) =>
    pathname.startsWith(path),
  );

  if (!requiresAuth && !isBasicProtected) {
    return NextResponse.next();
  }

  // Get the token from the request
  const token = await getToken({
    req: request,
    secret: process.env.NEXTAUTH_SECRET,
  });

  // If no token exists, redirect to login
  if (!token) {
    const loginUrl = new URL("/auth/login", request.url);
    loginUrl.searchParams.set("callbackUrl", pathname);
    return NextResponse.redirect(loginUrl);
  }

  // Check if tenant is selected (required for multi-tenancy)
  const tenantId = request.headers.get("x-tenant-id") || token.tenantId;

  if (!tenantId && pathname !== "/auth/tenant-select") {
    const tenantSelectUrl = new URL("/auth/tenant-select", request.url);
    tenantSelectUrl.searchParams.set("callbackUrl", pathname);
    return NextResponse.redirect(tenantSelectUrl);
  }

  // Check permissions for routes that require them
  const matchingRoute = PROTECTED_ROUTES.find((route) =>
    route.pattern.test(pathname),
  );
  if (matchingRoute) {
    const userRole = token.role as string;
    const requiredPermission = matchingRoute.permission;

    const userPermissions =
      ROLE_PERMISSIONS[userRole as keyof typeof ROLE_PERMISSIONS] || [];

    if (!userPermissions.includes(requiredPermission)) {
      // User doesn't have permission, redirect to unauthorized page or dashboard
      const unauthorizedUrl = new URL("/unauthorized", request.url);
      unauthorizedUrl.searchParams.set("callbackUrl", pathname);
      return NextResponse.redirect(unauthorizedUrl);
    }
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - api/auth (NextAuth.js API routes)
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * - public files (public folder)
     */
    "/((?!api/auth|_next/static|_next/image|favicon.ico|.*\\..*).*)",
  ],
};
