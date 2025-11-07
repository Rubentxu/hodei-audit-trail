'use client';

import { useState } from 'react';
import { useSession } from 'next-auth/react';
import { ChevronDown } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useTenantStore } from '@/lib/stores/tenant-store';

export function TenantSelector() {
  const { data: session } = useSession();
  const [isOpen, setIsOpen] = useState(false);
  const { selectedTenantId, setSelectedTenant, getSelectedTenant } = useTenantStore();

  // Mock tenants - in a real app, this would come from an API
  const availableTenants = [
    { id: 'tenant-1', name: 'Acme Corp' },
    { id: 'tenant-2', name: 'Globex Corporation' },
    { id: 'tenant-3', name: 'Soylent Corp' },
  ];

  const currentTenant = availableTenants.find((t) => t.id === selectedTenantId) ||
    availableTenants.find((t) => t.id === session?.user?.tenant_id) ||
    availableTenants[0];

  const handleTenantSelect = (tenantId: string) => {
    setSelectedTenant(tenantId);
    setIsOpen(false);
  };

  if (!session) {
    return null;
  }

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className="w-48 justify-between">
          <span className="truncate">{currentTenant?.name}</span>
          <ChevronDown className="h-4 w-4 ml-2" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="w-48">
        {availableTenants.map((tenant) => (
          <DropdownMenuItem
            key={tenant.id}
            onClick={() => handleTenantSelect(tenant.id)}
            className={`cursor-pointer ${
              tenant.id === currentTenant?.id ? 'bg-accent' : ''
            }`}
          >
            <div className="flex flex-col">
              <span className="font-medium">{tenant.name}</span>
              <span className="text-xs text-muted-foreground">{tenant.id}</span>
            </div>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
