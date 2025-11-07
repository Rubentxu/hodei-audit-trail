'use client';

import { Shield, CheckCircle } from 'lucide-react';
import { BaseWidget } from './base-widget';
import { WidgetProps } from './types';

export function ComplianceScoreWidget({ id, title, onRefresh, isRefreshing }: WidgetProps) {
  // Mock data
  const score = 98;
  const status = 'Excellent';

  return (
    <BaseWidget id={id} title={title} onRefresh={onRefresh} isRefreshing={isRefreshing}>
      <div className="space-y-4">
        <div className="relative">
          <div className="flex items-center justify-center">
            <svg className="w-32 h-32" viewBox="0 0 100 100">
              <circle
                cx="50"
                cy="50"
                r="40"
                stroke="currentColor"
                strokeWidth="10"
                fill="none"
                className="text-gray-200"
              />
              <circle
                cx="50"
                cy="50"
                r="40"
                stroke="currentColor"
                strokeWidth="10"
                fill="none"
                strokeLinecap="round"
                strokeDasharray={`${score * 2.51} 251`}
                className="text-green-600"
                style={{ transform: 'rotate(-90deg)', transformOrigin: '50% 50%' }}
              />
            </svg>
            <div className="absolute flex flex-col items-center">
              <p className="text-3xl font-bold text-gray-900">{score}%</p>
              <p className="text-xs text-gray-500">Score</p>
            </div>
          </div>
        </div>
        <div className="flex items-center justify-center space-x-2">
          <CheckCircle className="h-5 w-5 text-green-600" />
          <span className="text-sm font-medium text-green-600">{status}</span>
        </div>
        <div className="pt-2 border-t">
          <p className="text-xs text-gray-500 text-center">Based on recent audits</p>
        </div>
      </div>
    </BaseWidget>
  );
}
