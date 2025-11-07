'use client';

import { TrendingUp, Activity } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function EventCountWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data - in real app, fetch from API
  const eventCount = 12543;
  const previousCount = 11234;
  const change = ((eventCount - previousCount) / previousCount) * 100;

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-4">
        <div>
          <p className="text-sm text-gray-600 dark:text-gray-400">Total Events</p>
          <p className="text-4xl font-bold text-gray-900 dark:text-white">
            {eventCount.toLocaleString()}
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <div className={`flex items-center space-x-1 ${change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
            <TrendingUp className="h-4 w-4" />
            <span className="text-sm font-medium">
              {change >= 0 ? '+' : ''}{change.toFixed(1)}%
            </span>
          </div>
          <span className="text-sm text-gray-500">vs last period</span>
        </div>
        <div className="pt-2 border-t">
          <p className="text-xs text-gray-500">Last updated: Just now</p>
        </div>
      </div>
    </BaseWidget>
  );
}
