import { useSession } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

/**
 * Hook for protecting client-side routes
 * Redirects to login if not authenticated
 */
export function useAuth(requiredTenant: boolean = true) {
  const { data: session, status } = useSession();
  const router = useRouter();

  useEffect(() => {
    // If still loading session, wait
    if (status === 'loading') return;

    // If not authenticated, redirect to login
    if (status === 'unauthenticated') {
      router.push('/auth/login');
      return;
    }

    // If tenant is required but not selected, redirect to tenant selection
    if (requiredTenant && status === 'authenticated' && !session?.user?.tenantId) {
      router.push('/auth/tenant-select');
      return;
    }
  }, [session, status, router, requiredTenant]);

  return {
    session,
    status,
    isLoading: status === 'loading',
    isAuthenticated: status === 'authenticated',
  };
}

/**
 * Hook for checking if user has specific role
 */
export function useRequireRole(requiredRole: string | string[]) {
  const { session } = useAuth();
  const userRole = session?.user?.role;

  if (Array.isArray(requiredRole)) {
    return requiredRole.includes(userRole || '');
  }

  return userRole === requiredRole;
}
