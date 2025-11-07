'use client';

import { User, UserCheck } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function TopUsersWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data - top users by activity
  const users = [
    { name: 'john.doe', events: 1245, avatar: 'JD' },
    { name: 'jane.smith', events: 987, avatar: 'JS' },
    { name: 'bob.johnson', events: 743, avatar: 'BJ' },
    { name: 'alice.williams', events: 621, avatar: 'AW' },
    { name: 'charlie.brown', events: 534, avatar: 'CB' },
  ];

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-3">
        <p className="text-sm text-gray-600 dark:text-gray-400">Most Active Users</p>
        <div className="space-y-2">
          {users.map((user, index) => (
            <div key={index} className="flex items-center space-x-3 p-2 rounded hover:bg-gray-50 dark:hover:bg-gray-800">
              <div className="flex-shrink-0">
                <div className="h-8 w-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-xs font-semibold">
                  {user.avatar}
                </div>
              </div>
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900 truncate">{user.name}</p>
                <p className="text-xs text-gray-500">{user.events} events</p>
              </div>
              <div className="flex-shrink-0">
                <User className="h-4 w-4 text-gray-400" />
              </div>
            </div>
          ))}
        </div>
        <div className="pt-2 border-t">
          <p className="text-xs text-gray-500">Last 30 days</p>
        </div>
      </div>
    </BaseWidget>
  );
}
