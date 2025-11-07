'use client';

import { useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  BarChart3,
  LineChart,
  PieChart,
  AreaChart,
  Download,
  Maximize2
} from 'lucide-react';

type ChartData = {
  labels: string[];
  datasets: {
    label: string;
    data: number[];
    color?: string;
  }[];
};

type ChartType = 'bar' | 'line' | 'pie' | 'area';

interface ChartVisualizationProps {
  type: ChartType;
  data: ChartData;
  title?: string;
  height?: number;
}

export function ChartVisualization({
  type,
  data,
  title,
  height = 300
}: ChartVisualizationProps) {
  const maxValue = useMemo(() => {
    return Math.max(...data.datasets.flatMap(d => d.data));
  }, [data]);

  const colors = [
    'bg-blue-500',
    'bg-green-500',
    'bg-purple-500',
    'bg-orange-500',
    'bg-red-500',
    'bg-yellow-500',
    'bg-indigo-500',
    'bg-pink-500',
  ];

  const renderBarChart = () => (
    <div className="space-y-4">
      {data.labels.map((label, i) => {
        const value = data.datasets[0]?.data[i] || 0;
        const percentage = (value / maxValue) * 100;

        return (
          <div key={i} className="space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">{label}</span>
              <Badge variant="secondary">{value}</Badge>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-6 relative">
              <div
                className={`h-6 rounded-full ${colors[i % colors.length]}`}
                style={{ width: `${percentage}%` }}
              />
              <span className="absolute inset-0 flex items-center justify-center text-xs font-medium text-white">
                {percentage.toFixed(1)}%
              </span>
            </div>
          </div>
        );
      })}
    </div>
  );

  const renderLineChart = () => (
    <div className="relative" style={{ height }}>
      <svg width="100%" height="100%" className="overflow-visible">
        {/* Grid lines */}
        {[0, 0.25, 0.5, 0.75, 1].map((ratio) => (
          <line
            key={ratio}
            x1="0"
            y1={height - ratio * (height - 40)}
            x2="100%"
            y2={height - ratio * (height - 40)}
            stroke="currentColor"
            strokeWidth="1"
            className="text-gray-200"
          />
        ))}

        {/* Data line */}
        <polyline
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          className="text-blue-500"
          points={data.datasets[0]?.data.map((value, i) => {
            const x = (i / (data.labels.length - 1)) * (height - 40);
            const y = height - (value / maxValue) * (height - 40) - 20;
            return `${x},${y}`;
          }).join(' ')}
        />

        {/* Data points */}
        {data.datasets[0]?.data.map((value, i) => {
          const x = (i / (data.labels.length - 1)) * (height - 40);
          const y = height - (value / maxValue) * (height - 40) - 20;
          return (
            <g key={i}>
              <circle
                cx={x}
                cy={y}
                r="4"
                fill="currentColor"
                className="text-blue-500"
              />
              <text
                x={x}
                y={height - 5}
                textAnchor="middle"
                className="text-xs fill-gray-600"
              >
                {data.labels[i]}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );

  const renderPieChart = () => {
    const total = data.datasets[0]?.data.reduce((a, b) => a + b, 0) || 0;
    let cumulativePercentage = 0;

    return (
      <div className="relative">
        <svg width="100%" height={height} viewBox="0 0 200 200">
          {data.labels.map((label, i) => {
            const value = data.datasets[0]?.data[i] || 0;
            const percentage = (value / total) * 100;
            const startAngle = (cumulativePercentage / 100) * 2 * Math.PI;
            const endAngle = ((cumulativePercentage + percentage) / 100) * 2 * Math.PI;

            const x1 = 100 + 60 * Math.cos(startAngle);
            const y1 = 100 + 60 * Math.sin(startAngle);
            const x2 = 100 + 60 * Math.cos(endAngle);
            const y2 = 100 + 60 * Math.sin(endAngle);

            const largeArcFlag = percentage > 50 ? 1 : 0;

            const pathData = `M 100 100 L ${x1} ${y1} A 60 60 0 ${largeArcFlag} 1 ${x2} ${y2} Z`;

            cumulativePercentage += percentage;

            return (
              <g key={i}>
                <path
                  d={pathData}
                  fill={colors[i % colors.length].replace('bg-', '').replace('-500', '')}
                  stroke="white"
                  strokeWidth="2"
                />
                <text
                  x={100 + 40 * Math.cos(startAngle + (endAngle - startAngle) / 2)}
                  y={100 + 40 * Math.sin(startAngle + (endAngle - startAngle) / 2)}
                  textAnchor="middle"
                  className="text-xs fill-white font-medium"
                >
                  {percentage.toFixed(0)}%
                </text>
              </g>
            );
          })}
        </svg>
        <div className="mt-4 space-y-2">
          {data.labels.map((label, i) => {
            const value = data.datasets[0]?.data[i] || 0;
            const percentage = ((value / (total || 1)) * 100).toFixed(1);
            return (
              <div key={i} className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <div className={`w-3 h-3 rounded ${colors[i % colors.length]}`} />
                  <span className="text-sm">{label}</span>
                </div>
                <Badge variant="secondary">{value} ({percentage}%)</Badge>
              </div>
            );
          })}
        </div>
      </div>
    );
  };

  const renderAreaChart = () => (
    <div className="relative" style={{ height }}>
      <svg width="100%" height="100%" className="overflow-visible">
        {/* Grid lines */}
        {[0, 0.25, 0.5, 0.75, 1].map((ratio) => (
          <line
            key={ratio}
            x1="0"
            y1={height - ratio * (height - 40)}
            x2="100%"
            y2={height - ratio * (height - 40)}
            stroke="currentColor"
            strokeWidth="1"
            className="text-gray-200"
          />
        ))}

        {/* Area fill */}
        <polygon
          fill="currentColor"
          className="text-blue-500 opacity-20"
          points={[
            `0,${height}`,
            ...data.datasets[0]?.data.map((value, i) => {
              const x = (i / (data.labels.length - 1)) * (height - 40);
              const y = height - (value / maxValue) * (height - 40) - 20;
              return `${x},${y}`;
            }) || [],
            `${height - 40},${height}`,
          ].join(' ')}
        />

        {/* Data line */}
        <polyline
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          className="text-blue-500"
          points={data.datasets[0]?.data.map((value, i) => {
            const x = (i / (data.labels.length - 1)) * (height - 40);
            const y = height - (value / maxValue) * (height - 40) - 20;
            return `${x},${y}`;
          }).join(' ')}
        />

        {/* Data points */}
        {data.datasets[0]?.data.map((value, i) => {
          const x = (i / (data.labels.length - 1)) * (height - 40);
          const y = height - (value / maxValue) * (height - 40) - 20;
          return (
            <g key={i}>
              <circle
                cx={x}
                cy={y}
                r="4"
                fill="currentColor"
                className="text-blue-500"
              />
              <text
                x={x}
                y={height - 5}
                textAnchor="middle"
                className="text-xs fill-gray-600"
              >
                {data.labels[i]}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );

  const getChartIcon = () => {
    switch (type) {
      case 'bar':
        return <BarChart3 className="h-5 w-5" />;
      case 'line':
        return <LineChart className="h-5 w-5" />;
      case 'pie':
        return <PieChart className="h-5 w-5" />;
      case 'area':
        return <AreaChart className="h-5 w-5" />;
    }
  };

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center space-x-2">
            {getChartIcon()}
            <span>{title || `${type.charAt(0).toUpperCase() + type.slice(1)} Chart`}</span>
          </CardTitle>
          <div className="flex space-x-2">
            <Button variant="ghost" size="icon">
              <Maximize2 className="h-4 w-4" />
            </Button>
            <Button variant="ghost" size="icon">
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent>
        {type === 'bar' && renderBarChart()}
        {type === 'line' && renderLineChart()}
        {type === 'pie' && renderPieChart()}
        {type === 'area' && renderAreaChart()}
      </CardContent>
    </Card>
  );
}

// Helper function to convert query results to chart data
export function convertToChartData(
  rows: any[],
  xField: string,
  yField: string
): ChartData {
  return {
    labels: rows.map(row => String(row[xField])),
    datasets: [
      {
        label: yField,
        data: rows.map(row => Number(row[yField])),
      },
    ],
  };
}
