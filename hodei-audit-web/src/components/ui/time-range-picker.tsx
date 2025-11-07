'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Calendar, ChevronDown } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export type TimeRange = {
  label: string;
  value: string;
  hours: number;
};

const TIME_RANGES: TimeRange[] = [
  { label: 'Last 15 minutes', value: '15m', hours: 0.25 },
  { label: 'Last 1 hour', value: '1h', hours: 1 },
  { label: 'Last 6 hours', value: '6h', hours: 6 },
  { label: 'Last 24 hours', value: '24h', hours: 24 },
  { label: 'Last 7 days', value: '7d', hours: 168 },
  { label: 'Last 30 days', value: '30d', hours: 720 },
  { label: 'Last 90 days', value: '90d', hours: 2160 },
];

interface TimeRangePickerProps {
  value: string;
  onChange: (value: string) => void;
  className?: string;
}

export function TimeRangePicker({ value, onChange, className = '' }: TimeRangePickerProps) {
  const [isOpen, setIsOpen] = useState(false);

  const selectedRange = TIME_RANGES.find(range => range.value === value) || TIME_RANGES[3]; // Default to 24h

  const handleSelect = (rangeValue: string) => {
    onChange(rangeValue);
    setIsOpen(false);
  };

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className={className}>
          <Calendar className="h-4 w-4 mr-2" />
          {selectedRange.label}
          <ChevronDown className="h-4 w-4 ml-2" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="w-48">
        {TIME_RANGES.map((range) => (
          <DropdownMenuItem
            key={range.value}
            onClick={() => handleSelect(range.value)}
            className={`cursor-pointer ${
              range.value === value ? 'bg-accent' : ''
            }`}
          >
            <div className="flex flex-col">
              <span className="font-medium">{range.label}</span>
              <span className="text-xs text-muted-foreground">
                {range.value}
              </span>
            </div>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
