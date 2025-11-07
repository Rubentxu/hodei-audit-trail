import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface TenantState {
  selectedTenantId: string | null;
  setSelectedTenant: (tenantId: string) => void;
  getSelectedTenant: () => string | null;
}

export const useTenantStore = create<TenantState>()(
  persist(
    (set, get) => ({
      selectedTenantId: null,
      setSelectedTenant: (tenantId: string) => {
        set({ selectedTenantId: tenantId });
        // Store in localStorage for easy access
        if (typeof window !== 'undefined') {
          localStorage.setItem('x-tenant-id', tenantId);
        }
      },
      getSelectedTenant: () => {
        const state = get();
        if (state.selectedTenantId) {
          return state.selectedTenantId;
        }
        // Fallback to localStorage
        if (typeof window !== 'undefined') {
          const stored = localStorage.getItem('x-tenant-id');
          if (stored) {
            set({ selectedTenantId: stored });
            return stored;
          }
        }
        return null;
      },
    }),
    {
      name: 'tenant-storage',
    }
  )
);
