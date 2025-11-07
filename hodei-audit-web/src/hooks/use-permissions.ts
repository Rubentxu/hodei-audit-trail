'use client';

import { useSession } from 'next-auth/react';
import { Permission, hasPermission, hasAnyPermission, hasAllPermissions } from '@/lib/auth/permissions';
import { UserRole } from '@/types/auth';

/**
 * Hook to check if the current user has a specific permission
 */
export function usePermission(permission: Permission): boolean {
  const { data: session } = useSession();
  const userRole = session?.user?.role as UserRole | undefined;

  if (!userRole) {
    return false;
  }

  return hasPermission(userRole, permission);
}

/**
 * Hook to check if the current user has any of the specified permissions
 */
export function useAnyPermission(permissions: Permission[]): boolean {
  const { data: session } = useSession();
  const userRole = session?.user?.role as UserRole | undefined;

  if (!userRole) {
    return false;
  }

  return hasAnyPermission(userRole, permissions);
}

/**
 * Hook to check if the current user has all of the specified permissions
 */
export function useAllPermissions(permissions: Permission[]): boolean {
  const { data: session } = useSession();
  const userRole = session?.user?.role as UserRole | undefined;

  if (!userRole) {
    return false;
  }

  return hasAllPermissions(userRole, permissions);
}

/**
 * Hook to get the current user's role
 */
export function useUserRole(): UserRole | null {
  const { data: session } = useSession();
  return (session?.user?.role as UserRole | undefined) || null;
}

/**
 * Hook to check if the current user is an admin
 */
export function useIsAdmin(): boolean {
  return usePermission('manage:tenants' as Permission);
}

/**
 * Hook to check if the current user is an auditor or higher
 */
export function useIsAuditorOrAbove(): boolean {
  const userRole = useUserRole();
  if (!userRole) return false;

  const roleHierarchy: UserRole[] = ['viewer', 'analyst', 'auditor', 'admin'];
  return roleHierarchy.indexOf(userRole) >= roleHierarchy.indexOf('auditor');
}

/**
 * Hook to check if the current user is an analyst or higher
 */
export function useIsAnalystOrAbove(): boolean {
  const userRole = useUserRole();
  if (!userRole) return false;

  const roleHierarchy: UserRole[] = ['viewer', 'analyst', 'auditor', 'admin'];
  return roleHierarchy.indexOf(userRole) >= roleHierarchy.indexOf('analyst');
}
