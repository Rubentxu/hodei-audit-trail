"use client";

import { useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { useSession } from "next-auth/react";
import { TenantSelector } from "@/components/tenant/tenant-selector";
import { useTenantStore } from "@/lib/stores/tenant-store";

export default function TenantSelectPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { data: session } = useSession();
  const { setSelectedTenant } = useTenantStore();
  const callbackUrl = searchParams.get("callbackUrl") || "/";

  const handleTenantSelect = (tenantId: string) => {
    // Store selected tenant
    setSelectedTenant(tenantId);

    // Redirect to callback URL
    router.push(callbackUrl);
  };

  // If no session, the middleware will redirect to login
  // But we can also show a loading state here
  if (!session) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <p className="text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

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
            onTenantSelect={handleTenantSelect}
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
