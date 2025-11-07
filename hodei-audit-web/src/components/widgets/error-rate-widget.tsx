'use client';

import { AlertCircle, XCircle, Activity } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function ErrorRateWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data
  const errorRate = 2.3;
  const totalRequests = 15420;
  const failedRequests = 354;
  const status = 'Normal';

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm text-gray-600 dark:text-gray-400">Error Rate</p>
            <p className="text-4xl font-bold text-gray-900 dark:text-white">{errorRate}%</p>
          </div>
          <div className="h-12 w-12 rounded-full bg-orange-100 flex items-center justify-center">
            <AlertCircle className="h-6 w-6 text-orange-600" />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <p className="text-xs text-gray-500">Total Requests</p>
            <p className="text-lg font-semibold">{totalRequests.toLocaleString()}</p>
          </div>
          <div>
            <p className="text-xs text-gray-500">Failed</p>
            <p className="text-lg font-semibold text-red-600">{failedRequests.toLocaleString()}</p>
          </div>
        </div>

        <div className="pt-2 border-t">
          <div className="flex items-center justify-between">
            <span className="text-xs text-gray-500">Status</span>
            <span className="text-xs font-medium text-green-600">{status}</span>
          </div>
        </div>
      </div>
    </BaseWidget>
  );
}
