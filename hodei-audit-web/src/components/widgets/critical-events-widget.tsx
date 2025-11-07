'use client';

import { AlertTriangle, AlertCircle } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function CriticalEventsWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data
  const criticalCount = 23;
  const warningCount = 67;

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm text-gray-600 dark:text-gray-400">Critical</p>
            <p className="text-3xl font-bold text-red-600">{criticalCount}</p>
          </div>
          <div className="h-12 w-12 rounded-full bg-red-100 flex items-center justify-center">
            <AlertTriangle className="h-6 w-6 text-red-600" />
          </div>
        </div>
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm text-gray-600 dark:text-gray-400">Warnings</p>
            <p className="text-2xl font-bold text-yellow-600">{warningCount}</p>
          </div>
          <div className="h-10 w-10 rounded-full bg-yellow-100 flex items-center justify-center">
            <AlertCircle className="h-5 w-5 text-yellow-600" />
          </div>
        </div>
        <div className="pt-2 border-t">
          <p className="text-xs text-gray-500">Requires attention</p>
        </div>
      </div>
    </BaseWidget>
  );
}
