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
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Plus,
  Trash2,
  Play,
  Save,
  Download,
  ArrowRight,
  BarChart3
} from 'lucide-react';

export interface QueryFilter {
  id: string;
  field: string;
  operator: string;
  value: string;
  logicalOperator: 'AND' | 'OR';
}

export interface QueryGroup {
  id: string;
  field: string;
  aggregator: string;
}

export interface QuerySort {
  id: string;
  field: string;
  direction: 'asc' | 'desc';
}

interface QueryBuilderProps {
  onRun?: (query: any) => void;
  onSave?: (name: string) => void;
}

const AVAILABLE_FIELDS = [
  { value: 'timestamp', label: 'Timestamp', type: 'date' },
  { value: 'user', label: 'User', type: 'string' },
  { value: 'action', label: 'Action', type: 'string' },
  { value: 'resource', label: 'Resource', type: 'string' },
  { value: 'status', label: 'Status', type: 'string' },
  { value: 'source', label: 'Source', type: 'string' },
];

const OPERATORS = [
  { value: 'equals', label: 'Equals' },
  { value: 'not_equals', label: 'Not Equals' },
  { value: 'contains', label: 'Contains' },
  { value: 'not_contains', label: 'Not Contains' },
  { value: 'starts_with', label: 'Starts With' },
  { value: 'ends_with', label: 'Ends With' },
  { value: 'greater_than', label: 'Greater Than' },
  { value: 'less_than', label: 'Less Than' },
  { value: 'is_empty', label: 'Is Empty' },
  { value: 'is_not_empty', label: 'Is Not Empty' },
];

const AGGREGATORS = [
  { value: 'count', label: 'Count' },
  { value: 'sum', label: 'Sum' },
  { value: 'avg', label: 'Average' },
  { value: 'min', label: 'Minimum' },
  { value: 'max', label: 'Maximum' },
  { value: 'group_by', label: 'Group By' },
];

export function QueryBuilder({ onRun, onSave }: QueryBuilderProps) {
  const [filters, setFilters] = useState<QueryFilter[]>([]);
  const [groups, setGroups] = useState<QueryGroup[]>([]);
  const [sorts, setSorts] = useState<QuerySort[]>([]);
  const [limit, setLimit] = useState<string>('100');
  const [isRunning, setIsRunning] = useState(false);
  const [queryName, setQueryName] = useState('');

  const addFilter = () => {
    const newFilter: QueryFilter = {
      id: `filter-${Date.now()}`,
      field: '',
      operator: 'equals',
      value: '',
      logicalOperator: filters.length > 0 ? 'AND' : 'AND',
    };
    setFilters([...filters, newFilter]);
  };

  const removeFilter = (id: string) => {
    setFilters(filters.filter(f => f.id !== id));
  };

  const updateFilter = (id: string, updates: Partial<QueryFilter>) => {
    setFilters(filters.map(f => f.id === id ? { ...f, ...updates } : f));
  };

  const addGroup = () => {
    const newGroup: QueryGroup = {
      id: `group-${Date.now()}`,
      field: '',
      aggregator: 'count',
    };
    setGroups([...groups, newGroup]);
  };

  const removeGroup = (id: string) => {
    setGroups(groups.filter(g => g.id !== id));
  };

  const updateGroup = (id: string, updates: Partial<QueryGroup>) => {
    setGroups(groups.map(g => g.id === id ? { ...g, ...updates } : g));
  };

  const addSort = () => {
    const newSort: QuerySort = {
      id: `sort-${Date.now()}`,
      field: '',
      direction: 'desc',
    };
    setSorts([...sorts, newSort]);
  };

  const removeSort = (id: string) => {
    setSorts(sorts.filter(s => s.id !== id));
  };

  const updateSort = (id: string, updates: Partial<QuerySort>) => {
    setSorts(sorts.map(s => s.id === id ? { ...s, ...updates } : s));
  };

  const handleRun = async () => {
    setIsRunning(true);

    // Simulate query execution
    await new Promise(resolve => setTimeout(resolve, 1000));

    if (onRun) {
      onRun({
        filters,
        groups,
        sorts,
        limit: parseInt(limit),
      });
    }

    setIsRunning(false);
  };

  const handleSave = () => {
    if (queryName.trim() && onSave) {
      onSave(queryName);
      setQueryName('');
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">Query Builder</h2>
          <p className="text-gray-600 mt-1">
            Build custom queries to analyze your audit data
          </p>
        </div>
        <div className="flex space-x-2">
          <Button variant="outline" onClick={handleSave} disabled={!queryName.trim()}>
            <Save className="h-4 w-4 mr-2" />
            Save Query
          </Button>
          <Button onClick={handleRun} disabled={isRunning}>
            <Play className="h-4 w-4 mr-2" />
            {isRunning ? 'Running...' : 'Run Query'}
          </Button>
        </div>
      </div>

      {/* Query Name */}
      <Card>
        <CardContent className="pt-6">
          <div className="space-y-2">
            <Label htmlFor="query-name">Query Name</Label>
            <Input
              id="query-name"
              placeholder="e.g., Critical Events This Week"
              value={queryName}
              onChange={(e) => setQueryName(e.target.value)}
              className="max-w-md"
            />
          </div>
        </CardContent>
      </Card>

      {/* Filters Section */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-lg">Filters</CardTitle>
            <Button variant="outline" size="sm" onClick={addFilter}>
              <Plus className="h-4 w-4 mr-2" />
              Add Filter
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {filters.length === 0 ? (
            <p className="text-gray-500 text-center py-4">No filters added</p>
          ) : (
            <div className="space-y-3">
              {filters.map((filter, index) => (
                <div key={filter.id} className="flex items-center space-x-2">
                  {index > 0 && (
                    <Badge variant="outline">{filter.logicalOperator}</Badge>
                  )}
                  <Select
                    value={filter.field}
                    onValueChange={(value) => updateFilter(filter.id, { field: value })}
                  >
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Field" />
                    </SelectTrigger>
                    <SelectContent>
                      {AVAILABLE_FIELDS.map(field => (
                        <SelectItem key={field.value} value={field.value}>
                          {field.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Select
                    value={filter.operator}
                    onValueChange={(value) => updateFilter(filter.id, { operator: value })}
                  >
                    <SelectTrigger className="w-[160px]">
                      <SelectValue placeholder="Operator" />
                    </SelectTrigger>
                    <SelectContent>
                      {OPERATORS.map(op => (
                        <SelectItem key={op.value} value={op.value}>
                          {op.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Input
                    placeholder="Value"
                    value={filter.value}
                    onChange={(e) => updateFilter(filter.id, { value: e.target.value })}
                    className="flex-1"
                  />
                  {index > 0 && (
                    <Select
                      value={filter.logicalOperator}
                      onValueChange={(value: 'AND' | 'OR') => updateFilter(filter.id, { logicalOperator: value })}
                    >
                      <SelectTrigger className="w-[100px]">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="AND">AND</SelectItem>
                        <SelectItem value="OR">OR</SelectItem>
                      </SelectContent>
                    </Select>
                  )}
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => removeFilter(filter.id)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Groups & Aggregations */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-lg">Group & Aggregate</CardTitle>
            <Button variant="outline" size="sm" onClick={addGroup}>
              <Plus className="h-4 w-4 mr-2" />
              Add Group
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {groups.length === 0 ? (
            <p className="text-gray-500 text-center py-4">No groups added</p>
          ) : (
            <div className="space-y-3">
              {groups.map((group) => (
                <div key={group.id} className="flex items-center space-x-2">
                  <Select
                    value={group.field}
                    onValueChange={(value) => updateGroup(group.id, { field: value })}
                  >
                    <SelectTrigger className="w-[200px]">
                      <SelectValue placeholder="Field" />
                    </SelectTrigger>
                    <SelectContent>
                      {AVAILABLE_FIELDS.map(field => (
                        <SelectItem key={field.value} value={field.value}>
                          {field.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Select
                    value={group.aggregator}
                    onValueChange={(value) => updateGroup(group.id, { aggregator: value })}
                  >
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Aggregator" />
                    </SelectTrigger>
                    <SelectContent>
                      {AGGREGATORS.map(agg => (
                        <SelectItem key={agg.value} value={agg.value}>
                          {agg.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => removeGroup(group.id)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Sort */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-lg">Sort</CardTitle>
            <Button variant="outline" size="sm" onClick={addSort}>
              <Plus className="h-4 w-4 mr-2" />
              Add Sort
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {sorts.length === 0 ? (
            <p className="text-gray-500 text-center py-4">No sort options added</p>
          ) : (
            <div className="space-y-3">
              {sorts.map((sort) => (
                <div key={sort.id} className="flex items-center space-x-2">
                  <Select
                    value={sort.field}
                    onValueChange={(value) => updateSort(sort.id, { field: value })}
                  >
                    <SelectTrigger className="w-[200px]">
                      <SelectValue placeholder="Field" />
                    </SelectTrigger>
                    <SelectContent>
                      {AVAILABLE_FIELDS.map(field => (
                        <SelectItem key={field.value} value={field.value}>
                          {field.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Select
                    value={sort.direction}
                    onValueChange={(value: 'asc' | 'desc') => updateSort(sort.id, { direction: value })}
                  >
                    <SelectTrigger className="w-[140px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="asc">Ascending</SelectItem>
                      <SelectItem value="desc">Descending</SelectItem>
                    </SelectContent>
                  </Select>
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => removeSort(sort.id)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Limit */}
      <Card>
        <CardContent className="pt-6">
          <div className="space-y-2">
            <Label htmlFor="limit">Limit Results</Label>
            <Input
              id="limit"
              type="number"
              placeholder="100"
              value={limit}
              onChange={(e) => setLimit(e.target.value)}
              className="max-w-md"
            />
          </div>
        </CardContent>
      </Card>

      {/* Query Preview */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Query Preview</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="bg-gray-100 dark:bg-gray-800 p-4 rounded font-mono text-sm">
            <p>SELECT *</p>
            {groups.length > 0 && (
              <p>
                {', '}
                {groups.map(g => g.aggregator).join(', ')}
              </p>
            )}
            {filters.length > 0 && (
              <p>
                WHERE{' '}
                {filters.map((f, i) => (
                  <span key={f.id}>
                    {i > 0 && `${f.logicalOperator} `}
                    {f.field} {f.operator} '{f.value}'
                  </span>
                )).reduce((prev, curr) => <>{prev} {curr}</>)}
              </p>
            )}
            {groups.length > 0 && (
              <p>
                GROUP BY {groups.map(g => g.field).join(', ')}
              </p>
            )}
            {sorts.length > 0 && (
              <p>
                ORDER BY {sorts.map(s => `${s.field} ${s.direction}`).join(', ')}
              </p>
            )}
            {limit && <p>LIMIT {limit}</p>}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
