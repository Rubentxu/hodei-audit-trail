import { getServerSession } from 'next-auth';
import { redirect } from 'next/navigation';
import { authOptions } from '@/lib/auth/config';
import { Permission } from '@/lib/auth/permissions';
import { hasPermission } from '@/lib/auth/permissions';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { Building, Users, Shield, Plus, MoreHorizontal } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export default async function AdminTenantsPage() {
  const session = await getServerSession(authOptions);

  if (!session) {
    redirect('/auth/login?callbackUrl=/admin/tenants');
  }

  const userRole = session.user?.role;
  if (!userRole || !hasPermission(userRole as any, Permission.MANAGE_TENANTS)) {
    redirect('/unauthorized?callbackUrl=/admin/tenants');
  }

  // Mock tenant data - in a real app, this would come from an API
  const tenants = [
    {
      id: 'tenant-1',
      name: 'Acme Corp',
      description: 'Primary tenant for Acme Corporation',
      userCount: 42,
      status: 'active',
      createdAt: '2024-01-15T10:30:00Z',
      plan: 'Enterprise',
    },
    {
      id: 'tenant-2',
      name: 'Globex Corporation',
      description: 'Globex enterprise tenant',
      userCount: 28,
      status: 'active',
      createdAt: '2024-02-20T14:15:00Z',
      plan: 'Professional',
    },
    {
      id: 'tenant-3',
      name: 'Soylent Corp',
      description: 'Soylent company tenant',
      userCount: 15,
      status: 'active',
      createdAt: '2024-03-10T09:45:00Z',
      plan: 'Standard',
    },
    {
      id: 'tenant-4',
      name: 'Initech',
      description: 'Initech technology tenant',
      userCount: 0,
      status: 'suspended',
      createdAt: '2024-04-05T16:20:00Z',
      plan: 'Trial',
    },
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100';
      case 'suspended':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-100';
      case 'trial':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-100';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100';
    }
  };

  const getPlanColor = (plan: string) => {
    switch (plan) {
      case 'Enterprise':
        return 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-100';
      case 'Professional':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-100';
      case 'Standard':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100';
      case 'Trial':
        return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100';
    }
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Tenant Management</h1>
          <p className="text-gray-600 dark:text-gray-400 mt-2">
            Manage tenants in the system
          </p>
        </div>
        <Button>
          <Plus className="mr-2 h-4 w-4" />
          Create Tenant
        </Button>
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">Total Tenants</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{tenants.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">Active</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">
              {tenants.filter(t => t.status === 'active').length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">Suspended</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600">
              {tenants.filter(t => t.status === 'suspended').length}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">Total Users</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {tenants.reduce((sum, t) => sum + t.userCount, 0)}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Tenants List */}
      <Card>
        <CardHeader>
          <CardTitle>All Tenants</CardTitle>
          <CardDescription>
            A list of all tenants in the system
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {tenants.map((tenant) => (
            <div
              key={tenant.id}
              className="flex items-center justify-between p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
            >
              <div className="flex-1">
                <div className="flex items-center space-x-3">
                  <Building className="h-5 w-5 text-gray-500" />
                  <div>
                    <h3 className="font-semibold text-lg">{tenant.name}</h3>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {tenant.description}
                    </p>
                  </div>
                </div>

                <div className="flex items-center space-x-4 mt-2">
                  <Badge className={getStatusColor(tenant.status)}>
                    {tenant.status}
                  </Badge>
                  <Badge className={getPlanColor(tenant.plan)}>
                    {tenant.plan}
                  </Badge>
                  <div className="flex items-center space-x-1 text-sm text-gray-600">
                    <Users className="h-4 w-4" />
                    <span>{tenant.userCount} users</span>
                  </div>
                  <div className="text-sm text-gray-500">
                    Created: {new Date(tenant.createdAt).toLocaleDateString()}
                  </div>
                </div>
              </div>

              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon">
                    <MoreHorizontal className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem>View Details</DropdownMenuItem>
                  <DropdownMenuItem>Edit Tenant</DropdownMenuItem>
                  <DropdownMenuItem>Manage Users</DropdownMenuItem>
                  <Separator />
                  <DropdownMenuItem className="text-red-600">
                    {tenant.status === 'active' ? 'Suspend' : 'Activate'}
                  </DropdownMenuItem>
                  <DropdownMenuItem className="text-red-600">
                    Delete Tenant
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
}
