'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { X, Filter, Calendar } from 'lucide-react';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';

export interface FilterOptions {
  dateRange: {
    start: Date | null;
    end: Date | null;
  };
  status: string[];
  actions: string[];
  users: string[];
  sources: string[];
  minEvents?: number;
}

interface AdvancedFilterPanelProps {
  filters: FilterOptions;
  onFiltersChange: (filters: FilterOptions) => void;
  onApply: () => void;
  onReset: () => void;
  availableActions: string[];
  availableUsers: string[];
  availableSources: string[];
}

export function AdvancedFilterPanel({
  filters,
  onFiltersChange,
  onApply,
  onReset,
  availableActions,
  availableUsers,
  availableSources,
}: AdvancedFilterPanelProps) {
  const [isOpen, setIsOpen] = useState(false);

  const handleStatusToggle = (status: string) => {
    const newStatus = filters.status.includes(status)
      ? filters.status.filter(s => s !== status)
      : [...filters.status, status];
    onFiltersChange({ ...filters, status: newStatus });
  };

  const handleActionToggle = (action: string) => {
    const newActions = filters.actions.includes(action)
      ? filters.actions.filter(a => a !== action)
      : [...filters.actions, action];
    onFiltersChange({ ...filters, actions: newActions });
  };

  const handleUserToggle = (user: string) => {
    const newUsers = filters.users.includes(user)
      ? filters.users.filter(u => u !== user)
      : [...filters.users, user];
    onFiltersChange({ ...filters, users: newUsers });
  };

  const handleSourceToggle = (source: string) => {
    const newSources = filters.sources.includes(source)
      ? filters.sources.filter(s => s !== source)
      : [...filters.sources, source];
    onFiltersChange({ ...filters, sources: newSources });
  };

  const handleApply = () => {
    onApply();
    setIsOpen(false);
  };

  const handleReset = () => {
    onReset();
    setIsOpen(false);
  };

  const activeFiltersCount =
    (filters.status.length > 0 ? 1 : 0) +
    (filters.actions.length > 0 ? 1 : 0) +
    (filters.users.length > 0 ? 1 : 0) +
    (filters.sources.length > 0 ? 1 : 0) +
    (filters.dateRange.start || filters.dateRange.end ? 1 : 0);

  return (
    <Popover open={isOpen} onOpenChange={setIsOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline" className="relative">
          <Filter className="h-4 w-4 mr-2" />
          Advanced Filters
          {activeFiltersCount > 0 && (
            <Badge className="ml-2 h-5 w-5 rounded-full p-0 text-xs">
              {activeFiltersCount}
            </Badge>
          )}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-96 p-0" align="start">
        <Card className="border-0 shadow-none">
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">Advanced Filters</CardTitle>
              {activeFiltersCount > 0 && (
                <Button variant="ghost" size="sm" onClick={handleReset}>
                  <X className="h-4 w-4 mr-1" />
                  Clear All
                </Button>
              )}
            </div>
          </CardHeader>
          <CardContent className="space-y-6 max-h-[600px] overflow-y-auto">
            {/* Date Range */}
            <div className="space-y-2">
              <Label className="flex items-center space-x-2">
                <Calendar className="h-4 w-4" />
                <span>Date Range</span>
              </Label>
              <div className="grid grid-cols-2 gap-2">
                <div>
                  <Label htmlFor="start-date" className="text-xs text-gray-500">
                    From
                  </Label>
                  <Input
                    id="start-date"
                    type="date"
                    value={filters.dateRange.start?.toISOString().split('T')[0] || ''}
                    onChange={(e) =>
                      onFiltersChange({
                        ...filters,
                        dateRange: {
                          ...filters.dateRange,
                          start: e.target.value ? new Date(e.target.value) : null,
                        },
                      })
                    }
                  />
                </div>
                <div>
                  <Label htmlFor="end-date" className="text-xs text-gray-500">
                    To
                  </Label>
                  <Input
                    id="end-date"
                    type="date"
                    value={filters.dateRange.end?.toISOString().split('T')[0] || ''}
                    onChange={(e) =>
                      onFiltersChange({
                        ...filters,
                        dateRange: {
                          ...filters.dateRange,
                          end: e.target.value ? new Date(e.target.value) : null,
                        },
                      })
                    }
                  />
                </div>
              </div>
            </div>

            {/* Status Filter */}
            <div className="space-y-2">
              <Label>Status</Label>
              <div className="flex flex-wrap gap-2">
                {['success', 'failure', 'warning'].map((status) => (
                  <Badge
                    key={status}
                    variant={filters.status.includes(status) ? 'default' : 'outline'}
                    className="cursor-pointer"
                    onClick={() => handleStatusToggle(status)}
                  >
                    {status}
                  </Badge>
                ))}
              </div>
            </div>

            {/* Actions Filter */}
            <div className="space-y-2">
              <Label>Actions</Label>
              <div className="flex flex-wrap gap-2 max-h-32 overflow-y-auto border rounded p-2">
                {availableActions.map((action) => (
                  <Badge
                    key={action}
                    variant={filters.actions.includes(action) ? 'default' : 'outline'}
                    className="cursor-pointer"
                    onClick={() => handleActionToggle(action)}
                  >
                    {action}
                  </Badge>
                ))}
              </div>
            </div>

            {/* Users Filter */}
            <div className="space-y-2">
              <Label>Users</Label>
              <div className="flex flex-wrap gap-2 max-h-32 overflow-y-auto border rounded p-2">
                {availableUsers.map((user) => (
                  <Badge
                    key={user}
                    variant={filters.users.includes(user) ? 'default' : 'outline'}
                    className="cursor-pointer"
                    onClick={() => handleUserToggle(user)}
                  >
                    {user.split('@')[0]}
                  </Badge>
                ))}
              </div>
            </div>

            {/* Sources Filter */}
            <div className="space-y-2">
              <Label>Sources</Label>
              <div className="flex flex-wrap gap-2">
                {availableSources.map((source) => (
                  <Badge
                    key={source}
                    variant={filters.sources.includes(source) ? 'default' : 'outline'}
                    className="cursor-pointer"
                    onClick={() => handleSourceToggle(source)}
                  >
                    {source}
                  </Badge>
                ))}
              </div>
            </div>
          </CardContent>
          <div className="p-4 border-t flex justify-end space-x-2">
            <Button variant="outline" onClick={() => setIsOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleApply}>
              Apply Filters
            </Button>
          </div>
        </Card>
      </PopoverContent>
    </Popover>
  );
}
