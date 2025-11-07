'use client';

import { LineChart, BarChart3 } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function TimeSeriesWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data - in real app, fetch from API
  const data = [
    { label: 'Mon', value: 45 },
    { label: 'Tue', value: 62 },
    { label: 'Wed', value: 54 },
    { label: 'Thu', value: 78 },
    { label: 'Fri', value: 65 },
    { label: 'Sat', value: 43 },
    { label: 'Sun', value: 38 },
  ];

  const maxValue = Math.max(...data.map(d => d.value));

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <p className="text-sm text-gray-600 dark:text-gray-400">Events (Last 7 days)</p>
          <LineChart className="h-4 w-4 text-gray-400" />
        </div>
        <div className="space-y-2">
          {data.map((item, index) => (
            <div key={index} className="flex items-center space-x-2">
              <p className="text-xs w-8 text-gray-500">{item.label}</p>
              <div className="flex-1 h-6 bg-gray-100 rounded relative overflow-hidden">
                <div
                  className="h-full bg-blue-600 rounded"
                  style={{ width: `${(item.value / maxValue) * 100}%` }}
                />
              </div>
              <p className="text-xs w-8 text-right text-gray-600">{item.value}</p>
            </div>
          ))}
        </div>
        <div className="pt-2 border-t">
          <p className="text-xs text-gray-500">Showing daily event counts</p>
        </div>
      </div>
    </BaseWidget>
  );
}
