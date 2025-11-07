'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Calendar,
  TrendingUp,
  Users,
  Activity,
  Clock,
  BarChart3
} from 'lucide-react';

const TIME_BUCKETS = [
  { value: 'minute', label: 'Minute' },
  { value: 'hour', label: 'Hour' },
  { value: 'day', label: 'Day' },
  { value: 'week', label: 'Week' },
  { value: 'month', label: 'Month' },
  { value: 'quarter', label: 'Quarter' },
  { value: 'year', label: 'Year' },
];

const AGGREGATION_FUNCTIONS = [
  {
    value: 'count',
    label: 'Count',
    description: 'Count the number of events',
    icon: <BarChart3 className="h-4 w-4" />
  },
  {
    value: 'sum',
    label: 'Sum',
    description: 'Sum of numeric values',
    icon: <TrendingUp className="h-4 w-4" />
  },
  {
    value: 'avg',
    label: 'Average',
    description: 'Average of numeric values',
    icon: <Activity className="h-4 w-4" />
  },
  {
    value: 'min',
    label: 'Minimum',
    description: 'Minimum value',
    icon: <TrendingUp className="h-4 w-4 rotate-180" />
  },
  {
    value: 'max',
    label: 'Maximum',
    description: 'Maximum value',
    icon: <TrendingUp className="h-4 w-4" />
  },
  {
    value: 'count_distinct',
    label: 'Count Distinct',
    description: 'Count unique values',
    icon: <Users className="h-4 w-4" />
  },
];

interface AggregationPanelProps {
  onApply: (config: AggregationConfig) => void;
}

export interface AggregationConfig {
  timeBucket?: string;
  aggregations: {
    field: string;
    function: string;
    alias?: string;
  }[];
  groupBy: string[];
}

export function AggregationPanel({ onApply }: AggregationPanelProps) {
  const [timeBucket, setTimeBucket] = useState<string>('');
  const [aggregations, setAggregations] = useState<AggregationConfig['aggregations']>([]);
  const [groupBy, setGroupBy] = useState<string[]>([]);

  const addAggregation = () => {
    setAggregations([
      ...aggregations,
      { field: '', function: 'count', alias: '' }
    ]);
  };

  const removeAggregation = (index: number) => {
    setAggregations(aggregations.filter((_, i) => i !== index));
  };

  const updateAggregation = (index: number, updates: Partial<AggregationConfig['aggregations'][0]>) => {
    setAggregations(aggregations.map((agg, i) => i === index ? { ...agg, ...updates } : agg));
  };

  const toggleGroupBy = (field: string) => {
    setGroupBy(groupBy.includes(field)
      ? groupBy.filter(f => f !== field)
      : [...groupBy, field]
    );
  };

  const handleApply = () => {
    const config: AggregationConfig = {
      timeBucket: timeBucket || undefined,
      aggregations,
      groupBy,
    };
    onApply(config);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">Aggregation & Time Bucketing</CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Time Bucketing */}
        <div className="space-y-2">
          <label className="text-sm font-medium flex items-center">
            <Calendar className="h-4 w-4 mr-2" />
            Time Bucket
          </label>
          <Select value={timeBucket} onValueChange={setTimeBucket}>
            <SelectTrigger>
              <SelectValue placeholder="Select time bucket (optional)" />
            </SelectTrigger>
            <SelectContent>
              {TIME_BUCKETS.map(bucket => (
                <SelectItem key={bucket.value} value={bucket.value}>
                  {bucket.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {timeBucket && (
            <p className="text-xs text-gray-500">
              Events will be grouped by {timeBucket}
            </p>
          )}
        </div>

        {/* Group By */}
        <div className="space-y-2">
          <label className="text-sm font-medium flex items-center">
            <Users className="h-4 w-4 mr-2" />
            Group By
          </label>
          <div className="flex flex-wrap gap-2">
            {['user', 'status', 'action', 'source', 'resource'].map(field => (
              <Badge
                key={field}
                variant={groupBy.includes(field) ? 'default' : 'outline'}
                className="cursor-pointer"
                onClick={() => toggleGroupBy(field)}
              >
                {field}
              </Badge>
            ))}
          </div>
          {groupBy.length > 0 && (
            <p className="text-xs text-gray-500">
              Grouping by: {groupBy.join(', ')}
            </p>
          )}
        </div>

        {/* Aggregations */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium flex items-center">
              <Activity className="h-4 w-4 mr-2" />
              Aggregations
            </label>
            <Button variant="outline" size="sm" onClick={addAggregation}>
              Add Aggregation
            </Button>
          </div>

          {aggregations.map((agg, index) => (
            <div key={index} className="flex items-center space-x-2 p-3 border rounded">
              <Select
                value={agg.function}
                onValueChange={(value) => updateAggregation(index, { function: value })}
              >
                <SelectTrigger className="w-[140px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {AGGREGATION_FUNCTIONS.map(func => (
                    <SelectItem key={func.value} value={func.value}>
                      <div className="flex items-center space-x-2">
                        {func.icon}
                        <span>{func.label}</span>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>

              <Select
                value={agg.field}
                onValueChange={(value) => updateAggregation(index, { field: value })}
              >
                <SelectTrigger className="w-[160px]">
                  <SelectValue placeholder="Field" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="*">* (all fields)</SelectItem>
                  <SelectItem value="id">id</SelectItem>
                  <SelectItem value="timestamp">timestamp</SelectItem>
                  <SelectItem value="user">user</SelectItem>
                  <SelectItem value="action">action</SelectItem>
                  <SelectItem value="status">status</SelectItem>
                </SelectContent>
              </Select>

              <input
                type="text"
                placeholder="Alias (optional)"
                value={agg.alias || ''}
                onChange={(e) => updateAggregation(index, { alias: e.target.value })}
                className="flex-1 px-3 py-1 border rounded text-sm"
              />

              <Button
                variant="ghost"
                size="icon"
                onClick={() => removeAggregation(index)}
              >
                ×
              </Button>
            </div>
          ))}

          {aggregations.length === 0 && (
            <p className="text-sm text-gray-500 text-center py-4">
              No aggregations added. Add at least one to proceed.
            </p>
          )}
        </div>

        <Button
          onClick={handleApply}
          className="w-full"
          disabled={aggregations.length === 0}
        >
          Apply Aggregation
        </Button>
      </CardContent>
    </Card>
  );
}

export function AggregationPreview({ config }: { config: AggregationConfig | null }) {
  if (!config) {
    return (
      <Card>
        <CardContent className="pt-6">
          <p className="text-gray-500 text-center">
            Configure aggregation to see preview
          </p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">Aggregation Preview</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <h4 className="text-sm font-semibold">Configuration</h4>
          <div className="bg-gray-100 dark:bg-gray-800 p-4 rounded font-mono text-sm">
            {config.timeBucket && <p>Time Bucket: {config.timeBucket}</p>}
            {config.groupBy.length > 0 && (
              <p>Group By: {config.groupBy.join(', ')}</p>
            )}
            <p>Aggregations:</p>
            <ul className="ml-4">
              {config.aggregations.map((agg, index) => (
                <li key={index}>
                  {agg.function}({agg.field}){agg.alias ? ` AS ${agg.alias}` : ''}
                </li>
              ))}
            </ul>
          </div>
        </div>

        <div className="space-y-2">
          <h4 className="text-sm font-semibold">Example Result</h4>
          <div className="rounded border">
            <table className="w-full text-sm">
              <thead className="bg-gray-50">
                <tr>
                  {config.groupBy.map(field => (
                    <th key={field} className="text-left p-2 font-semibold">
                      {field}
                    </th>
                  ))}
                  {config.aggregations.map((agg, index) => (
                    <th key={index} className="text-left p-2 font-semibold">
                      {agg.alias || `${agg.function}(${agg.field})`}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                <tr className="border-t">
                  {config.groupBy.map(field => (
                    <td key={field} className="p-2">
                      {field === 'user' ? 'john.doe' : field === 'status' ? 'success' : 'example'}
                    </td>
                  ))}
                  {config.aggregations.map((agg, index) => (
                    <td key={index} className="p-2">
                      {Math.floor(Math.random() * 1000)}
                    </td>
                  ))}
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
