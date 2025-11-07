'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { TrendingUp, TrendingDown, Activity, AlertTriangle } from 'lucide-react';

interface StatItem {
  label: string;
  value: string | number;
  change?: number;
  trend?: 'up' | 'down' | 'neutral';
  icon?: React.ReactNode;
}

interface QuickStatsPanelProps {
  stats: StatItem[];
  className?: string;
}

export function QuickStatsPanel({ stats, className = '' }: QuickStatsPanelProps) {
  const getTrendColor = (trend?: 'up' | 'down' | 'neutral') => {
    switch (trend) {
      case 'up':
        return 'text-green-600';
      case 'down':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  };

  const getTrendIcon = (trend?: 'up' | 'down' | 'neutral') => {
    switch (trend) {
      case 'up':
        return <TrendingUp className="h-4 w-4" />;
      case 'down':
        return <TrendingDown className="h-4 w-4" />;
      default:
        return <Activity className="h-4 w-4" />;
    }
  };

  return (
    <div className={`grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 ${className}`}>
      {stats.map((stat, index) => (
        <Card key={index}>
          <CardHeader className="pb-2 flex flex-row items-center justify-between space-y-0">
            <CardTitle className="text-sm font-medium text-gray-600">
              {stat.label}
            </CardTitle>
            {stat.icon || getTrendIcon(stat.trend)}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stat.value}</div>
            {stat.change !== undefined && (
              <p className={`text-xs flex items-center space-x-1 ${getTrendColor(stat.trend)}`}>
                <span>
                  {stat.trend === 'up' ? '↑' : stat.trend === 'down' ? '↓' : '→'}
                </span>
                <span>{Math.abs(stat.change).toFixed(1)}%</span>
              </p>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  );
}

// Quick stats for dashboard
export function DashboardQuickStats() {
  const defaultStats: StatItem[] = [
    {
      label: 'Total Events',
      value: 12543,
      change: 11.6,
      trend: 'up',
    },
    {
      label: 'Critical Events',
      value: 23,
      change: 5.2,
      trend: 'down',
    },
    {
      label: 'Compliance Score',
      value: '98%',
      change: 2.1,
      trend: 'up',
    },
    {
      label: 'Error Rate',
      value: '2.3%',
      change: 0.8,
      trend: 'down',
    },
  ];

  return <QuickStatsPanel stats={defaultStats} />;
}
