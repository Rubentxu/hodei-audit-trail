import { ReactNode } from 'react';

export interface WidgetProps {
  id: string;
  title: string;
  onRefresh?: () => void;
  isRefreshing?: boolean;
  data?: any;
}

export interface WidgetConfig {
  id: string;
  type: string;
  title: string;
  description?: string;
  position: {
    x: number;
    y: number;
    w: number;
    h: number;
  };
  refreshInterval?: number; // in seconds
  settings?: Record<string, any>;
}

export interface WidgetRegistry {
  [key: string]: {
    component: React.ComponentType<WidgetProps>;
    defaultSize: { w: number; h: number };
    displayName: string;
    description?: string;
  };
}
