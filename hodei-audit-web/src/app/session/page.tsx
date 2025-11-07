import { getServerSession } from 'next-auth';
import { redirect } from 'next/navigation';
import { authOptions } from '@/lib/auth/config';
import { SessionManager } from '@/components/auth/session-manager';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

export default async function SessionPage() {
  const session = await getServerSession(authOptions);

  if (!session) {
    redirect('/auth/login?callbackUrl=/session');
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Session</h1>
        <p className="text-gray-600 dark:text-gray-400 mt-2">
          Monitor and manage your current session
        </p>
      </div>

      <div className="max-w-2xl">
        <SessionManager />
      </div>
    </div>
  );
}
