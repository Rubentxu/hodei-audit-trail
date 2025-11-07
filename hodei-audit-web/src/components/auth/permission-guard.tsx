"use client";

import React from "react";
import { Permission } from "@/lib/auth/permissions";
import {
  usePermission,
  useAnyPermission,
  useAllPermissions,
} from "@/hooks/use-permissions";

interface PermissionGuardProps {
  permission?: Permission;
  permissions?: Permission[];
  requireAll?: boolean; // If true, require all permissions; if false, require any
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

/**
 * Component that conditionally renders children based on user permissions
 *
 * @param permission - Single permission to check
 * @param permissions - Array of permissions to check
 * @param requireAll - If true, user must have ALL permissions; if false, user must have ANY permission
 * @param children - Content to render if permission check passes
 * @param fallback - Content to render if permission check fails
 */
export function PermissionGuard({
  permission,
  permissions = [],
  requireAll = false,
  children,
  fallback = null,
}: PermissionGuardProps) {
  // Call all hooks unconditionally to satisfy Rules of Hooks
  // We always call all three hooks in the same order, every render
  const permissionResult = usePermission(permission || "");
  const allPermissionsResult = useAllPermissions(permissions);
  const anyPermissionsResult = useAnyPermission(permissions);

  // Use appropriate result based on the input parameters
  const hasPermission = permission
    ? permissionResult
    : requireAll
      ? allPermissionsResult
      : anyPermissionsResult;

  if (hasPermission) {
    return <>{children}</>;
  }

  return <>{fallback}</>;
}

/**
 * HOC for protecting components with permissions
 */
export function withPermissionGuard<P extends object>(
  Component: React.ComponentType<P>,
  options: {
    permission?: Permission;
    permissions?: Permission[];
    requireAll?: boolean;
    fallback?: React.ReactNode;
  },
) {
  return function ProtectedComponent(props: P) {
    const {
      permission,
      permissions = [],
      requireAll = false,
      fallback,
    } = options;

    return (
      <PermissionGuard
        permission={permission}
        permissions={permissions}
        requireAll={requireAll}
        fallback={fallback}
      >
        <Component {...props} />
      </PermissionGuard>
    );
  };
}
