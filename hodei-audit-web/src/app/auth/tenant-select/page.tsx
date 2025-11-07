import { getServerSession } from 'next-auth';
import { redirect } from 'next/navigation';
import { authOptions } from '@/lib/auth/config';
import TenantSelector from '@/components/tenant/tenant-selector';

export default async function TenantSelectPage({
  searchParams,
}: {
  searchParams: { callbackUrl?: string };
}) {
  const session = await getServerSession(authOptions);

  if (!session) {
    redirect('/auth/login');
  }

  const callbackUrl = searchParams.callbackUrl || '/';

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full space-y-8 p-8 bg-white rounded-lg shadow-md">
        <div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            Select a Tenant
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            You need to select a tenant to continue
          </p>
        </div>
        <div className="mt-8">
          <TenantSelector
            onTenantSelect={(tenantId) => {
              // Store selected tenant
              if (typeof window !== 'undefined') {
                localStorage.setItem('selectedTenant', tenantId);
                // Redirect to callback URL or home
                window.location.href = callbackUrl;
              }
            }}
            showLabel={false}
          />
        </div>
        <div className="mt-4 text-center">
          <p className="text-xs text-gray-500">
            After selecting a tenant, you will be redirected to your destination
          </p>
        </div>
      </div>
    </div>
  );
}
