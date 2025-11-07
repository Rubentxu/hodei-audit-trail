import { getServerSession } from 'next-auth';
import { redirect } from 'next/navigation';
import { authOptions } from '@/lib/auth/config';
import { useAuth } from '@/hooks/use-auth';

export default async function DashboardPage({
  searchParams,
}: {
  searchParams: { callbackUrl?: string };
}) {
  const session = await getServerSession(authOptions);

  if (!session) {
    redirect('/auth/login?callbackUrl=/dashboard');
  }

  // Note: In App Router, we can't use client-side hooks directly in server components
  // The middleware will handle the tenant check

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">Dashboard</h1>
        <p className="text-gray-600 mt-2">
          Welcome to your audit trail dashboard
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Quick Stats Cards */}
        <div className="bg-white p-6 rounded-lg shadow-md border">
          <h3 className="text-lg font-semibold text-gray-900 mb-2">
            Total Events
          </h3>
          <p className="text-3xl font-bold text-blue-600">0</p>
          <p className="text-sm text-gray-500 mt-1">Last 30 days</p>
        </div>

        <div className="bg-white p-6 rounded-lg shadow-md border">
          <h3 className="text-lg font-semibold text-gray-900 mb-2">
            Critical Events
          </h3>
          <p className="text-3xl font-bold text-red-600">0</p>
          <p className="text-sm text-gray-500 mt-1">Requiring attention</p>
        </div>

        <div className="bg-white p-6 rounded-lg shadow-md border">
          <h3 className="text-lg font-semibold text-gray-900 mb-2">
            Compliance Score
          </h3>
          <p className="text-3xl font-bold text-green-600">100%</p>
          <p className="text-sm text-gray-500 mt-1">Based on recent audits</p>
        </div>
      </div>

      <div className="mt-8">
        <h2 className="text-2xl font-bold text-gray-900 mb-4">
          Recent Activity
        </h2>
        <div className="bg-white rounded-lg shadow-md border p-6">
          <p className="text-gray-600 text-center py-8">
            No recent activity to display
          </p>
        </div>
      </div>
    </div>
  );
}
