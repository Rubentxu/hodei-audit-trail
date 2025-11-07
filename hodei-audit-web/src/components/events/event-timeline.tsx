'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  CheckCircle,
  XCircle,
  AlertCircle,
  Clock,
  User,
  Activity,
  Filter
} from 'lucide-react';

type Event = {
  id: string;
  timestamp: string;
  user: string;
  action: string;
  resource: string;
  status: 'success' | 'failure' | 'warning';
  source: string;
  details?: string;
};

interface EventTimelineProps {
  events: Event[];
}

export function EventTimeline({ events }: EventTimelineProps) {
  const [timeFilter, setTimeFilter] = useState<'1h' | '24h' | '7d' | 'all'>('24h');

  const getTimeFilterHours = () => {
    switch (timeFilter) {
      case '1h': return 1;
      case '24h': return 24;
      case '7d': return 168;
      case 'all': return Infinity;
    }
  };

  const filteredEvents = events.filter(event => {
    if (timeFilter === 'all') return true;

    const eventTime = new Date(event.timestamp).getTime();
    const now = Date.now();
    const hours = getTimeFilterHours();
    const cutoff = now - (hours * 60 * 60 * 1000);

    return eventTime >= cutoff;
  });

  // Group events by hour
  const groupedEvents = filteredEvents.reduce((acc, event) => {
    const hour = event.timestamp.split(' ')[1].split(':')[0];
    const dateKey = event.timestamp.split(' ')[0];
    const key = `${dateKey} ${hour}:00`;

    if (!acc[key]) {
      acc[key] = [];
    }
    acc[key].push(event);

    return acc;
  }, {} as Record<string, Event[]>);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'success':
        return 'text-green-600';
      case 'failure':
        return 'text-red-600';
      case 'warning':
        return 'text-yellow-600';
      default:
        return 'text-gray-600';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="h-4 w-4" />;
      case 'failure':
        return <XCircle className="h-4 w-4" />;
      case 'warning':
        return <AlertCircle className="h-4 w-4" />;
      default:
        return null;
    }
  };

  const timeRanges = [
    { value: '1h', label: 'Last Hour' },
    { value: '24h', label: 'Last 24 Hours' },
    { value: '7d', label: 'Last 7 Days' },
    { value: 'all', label: 'All Time' },
  ] as const;

  return (
    <div className="space-y-6">
      {/* Time Range Filter */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold">Event Timeline</h2>
        <div className="flex space-x-2">
          {timeRanges.map(range => (
            <Button
              key={range.value}
              variant={timeFilter === range.value ? 'default' : 'outline'}
              size="sm"
              onClick={() => setTimeFilter(range.value)}
            >
              {range.label}
            </Button>
          ))}
        </div>
      </div>

      {/* Timeline View */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Clock className="h-5 w-5" />
            <span>Activity Timeline</span>
            <Badge variant="secondary">
              {filteredEvents.length} events
            </Badge>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-8">
            {Object.entries(groupedEvents).length === 0 ? (
              <div className="text-center py-12 text-gray-500">
                <Activity className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>No events found for the selected time range</p>
              </div>
            ) : (
              Object.entries(groupedEvents)
                .sort(([a], [b]) => b.localeCompare(a))
                .map(([timeSlot, timeEvents]) => (
                  <div key={timeSlot} className="relative">
                    {/* Time marker */}
                    <div className="flex items-center space-x-4 mb-4">
                      <div className="flex items-center space-x-2">
                        <Clock className="h-4 w-4 text-gray-500" />
                        <span className="font-semibold text-sm">
                          {timeSlot}
                        </span>
                        <Badge variant="outline" className="text-xs">
                          {timeEvents.length} {timeEvents.length === 1 ? 'event' : 'events'}
                        </Badge>
                      </div>
                    </div>

                    {/* Events in this time slot */}
                    <div className="ml-6 space-y-3 border-l-2 border-gray-200 pl-6">
                      {timeEvents.map((event) => (
                        <div
                          key={event.id}
                          className="relative flex items-start space-x-3 p-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                        >
                          {/* Timeline dot */}
                          <div className="absolute -left-[25px] mt-1">
                            <div className={`h-3 w-3 rounded-full border-2 border-white ${getStatusColor(event.status).replace('text-', 'bg-')}`} />
                          </div>

                          {/* Event content */}
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center justify-between">
                              <div className="flex items-center space-x-2">
                                <span className={getStatusColor(event.status)}>
                                  {getStatusIcon(event.status)}
                                </span>
                                <code className="text-sm font-semibold">
                                  {event.action}
                                </code>
                              </div>
                              <span className="text-xs text-gray-500 font-mono">
                                {event.timestamp.split(' ')[1]}
                              </span>
                            </div>

                            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 truncate">
                              {event.resource}
                            </p>

                            <div className="flex items-center space-x-4 mt-2 text-xs text-gray-500">
                              <div className="flex items-center space-x-1">
                                <User className="h-3 w-3" />
                                <span>{event.user.split('@')[0]}</span>
                              </div>
                              <span>•</span>
                              <span>{event.source}</span>
                              {event.details && (
                                <>
                                  <span>•</span>
                                  <span className="max-w-[300px] truncate">
                                    {event.details}
                                  </span>
                                </>
                              )}
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                ))
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
