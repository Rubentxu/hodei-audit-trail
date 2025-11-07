import { UserRole } from '@/types/auth';

/**
 * Define all available permissions in the system
 */
export enum Permission {
  // Event permissions
  VIEW_EVENTS = 'view:events',
  CREATE_EVENTS = 'create:events',
  EDIT_EVENTS = 'edit:events',
  DELETE_EVENTS = 'delete:events',

  // Analytics permissions
  VIEW_ANALYTICS = 'view:analytics',
  CREATE_QUERIES = 'create:queries',
  EDIT_QUERIES = 'edit:queries',
  DELETE_QUERIES = 'delete:queries',

  // Compliance permissions
  VIEW_COMPLIANCE = 'view:compliance',
  GENERATE_REPORTS = 'generate:reports',
  MANAGE_COMPLIANCE = 'manage:compliance',

  // User management permissions
  VIEW_USERS = 'view:users',
  CREATE_USERS = 'create:users',
  EDIT_USERS = 'edit:users',
  DELETE_USERS = 'delete:users',

  // Tenant management permissions (admin only)
  VIEW_TENANTS = 'view:tenants',
  CREATE_TENANTS = 'create:tenants',
  EDIT_TENANTS = 'edit:tenants',
  DELETE_TENANTS = 'delete:tenants',
  MANAGE_TENANTS = 'manage:tenants',

  // System permissions
  VIEW_SYSTEM = 'view:system',
  MANAGE_SYSTEM = 'manage:system',
  MANAGE_SETTINGS = 'manage:settings',
}

/**
 * Define role-based permission mappings
 * Each role has a set of permissions
 */
export const ROLE_PERMISSIONS: Record<UserRole, Permission[]> = {
  admin: [
    // All permissions for admins
    Permission.VIEW_EVENTS,
    Permission.CREATE_EVENTS,
    Permission.EDIT_EVENTS,
    Permission.DELETE_EVENTS,
    Permission.VIEW_ANALYTICS,
    Permission.CREATE_QUERIES,
    Permission.EDIT_QUERIES,
    Permission.DELETE_QUERIES,
    Permission.VIEW_COMPLIANCE,
    Permission.GENERATE_REPORTS,
    Permission.MANAGE_COMPLIANCE,
    Permission.VIEW_USERS,
    Permission.CREATE_USERS,
    Permission.EDIT_USERS,
    Permission.DELETE_USERS,
    Permission.VIEW_TENANTS,
    Permission.CREATE_TENANTS,
    Permission.EDIT_TENANTS,
    Permission.DELETE_TENANTS,
    Permission.MANAGE_TENANTS,
    Permission.VIEW_SYSTEM,
    Permission.MANAGE_SYSTEM,
    Permission.MANAGE_SETTINGS,
  ],

  auditor: [
    // Read and compliance focused
    Permission.VIEW_EVENTS,
    Permission.VIEW_ANALYTICS,
    Permission.CREATE_QUERIES,
    Permission.EDIT_QUERIES,
    Permission.VIEW_COMPLIANCE,
    Permission.GENERATE_REPORTS,
  ],

  analyst: [
    // Data analysis focused
    Permission.VIEW_EVENTS,
    Permission.CREATE_EVENTS,
    Permission.VIEW_ANALYTICS,
    Permission.CREATE_QUERIES,
    Permission.EDIT_QUERIES,
    Permission.DELETE_QUERIES,
  ],

  viewer: [
    // Read-only access
    Permission.VIEW_EVENTS,
    Permission.VIEW_ANALYTICS,
  ],
};

/**
 * Check if a role has a specific permission
 */
export function hasPermission(role: UserRole, permission: Permission): boolean {
  const permissions = ROLE_PERMISSIONS[role] || [];
  return permissions.includes(permission);
}

/**
 * Check if a role has any of the specified permissions
 */
export function hasAnyPermission(
  role: UserRole,
  permissions: Permission[]
): boolean {
  return permissions.some(permission => hasPermission(role, permission));
}

/**
 * Check if a role has all of the specified permissions
 */
export function hasAllPermissions(
  role: UserRole,
  permissions: Permission[]
): boolean {
  return permissions.every(permission => hasPermission(role, permission));
}

/**
 * Get all permissions for a role
 */
export function getRolePermissions(role: UserRole): Permission[] {
  return [...(ROLE_PERMISSIONS[role] || [])];
}

/**
 * Check if a user can perform an action based on their role
 */
export function canUserPerform(
  role: UserRole,
  action: Permission,
  resource?: string
): boolean {
  // Add resource-based checks here if needed in the future
  return hasPermission(role, action);
}

/**
 * Get the highest role in a hierarchy
 * Order: admin > auditor > analyst > viewer
 */
export function getHighestRole(roles: UserRole[]): UserRole {
  const roleHierarchy: UserRole[] = ['admin', 'auditor', 'analyst', 'viewer'];

  for (const role of roleHierarchy) {
    if (roles.includes(role)) {
      return role;
    }
  }

  return 'viewer';
}

/**
 * Check if a role is higher than another in the hierarchy
 */
export function isRoleHigher(role1: UserRole, role2: UserRole): boolean {
  const roleHierarchy: UserRole[] = ['admin', 'auditor', 'analyst', 'viewer'];
  return roleHierarchy.indexOf(role1) < roleHierarchy.indexOf(role2);
}
