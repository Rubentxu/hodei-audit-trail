'use client';

import { useState, useEffect, useRef } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { CheckCircle, XCircle, AlertCircle, Calendar, User } from 'lucide-react';

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

interface VirtualEventListProps {
  events: Event[];
  pageSize?: number;
}

export function VirtualEventList({ events, pageSize = 100 }: VirtualEventListProps) {
  const [visibleRange, setVisibleRange] = useState({ start: 0, end: 50 });
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleScroll = () => {
      if (!containerRef.current) return;

      const { scrollTop, scrollHeight, clientHeight } = containerRef.current;
      const scrollPercentage = scrollTop / (scrollHeight - clientHeight);
      const start = Math.floor(scrollPercentage * (events.length - pageSize));
      const end = Math.min(start + pageSize, events.length);

      setVisibleRange({ start, end });
    };

    const container = containerRef.current;
    if (container) {
      container.addEventListener('scroll', handleScroll);
      handleScroll(); // Initial calculation
    }

    return () => {
      if (container) {
        container.removeEventListener('scroll', handleScroll);
      }
    };
  }, [events.length, pageSize]);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'success':
        return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100';
      case 'failure':
        return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-100';
      case 'warning':
        return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-100';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-100';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="h-4 w-4 text-green-600" />;
      case 'failure':
        return <XCircle className="h-4 w-4 text-red-600" />;
      case 'warning':
        return <AlertCircle className="h-4 w-4 text-yellow-600" />;
      default:
        return null;
    }
  };

  const visibleEvents = events.slice(visibleRange.start, visibleRange.end);
  const totalHeight = events.length * 60; // Assuming 60px per item

  return (
    <Card>
      <CardContent className="p-0">
        <div
          ref={containerRef}
          className="h-[600px] overflow-auto relative"
          style={{ scrollBehavior: 'smooth' }}
        >
          <div style={{ height: `${totalHeight}px`, position: 'relative' }}>
            <div
              style={{
                transform: `translateY(${visibleRange.start * 60}px)`,
              }}
              className="absolute w-full"
            >
              {visibleEvents.map((event, index) => (
                <div
                  key={event.id}
                  className="flex items-center justify-between p-4 border-b hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                  style={{ height: '60px' }}
                >
                  <div className="flex items-center space-x-4 flex-1">
                    {getStatusIcon(event.status)}
                    <div className="flex-1 min-w-0">
                      <p className="text-sm font-medium truncate">
                        {event.action}
                      </p>
                      <p className="text-xs text-gray-500 truncate">
                        {event.resource}
                      </p>
                    </div>
                  </div>

                  <div className="flex items-center space-x-4">
                    <div className="hidden md:block text-right">
                      <p className="text-xs text-gray-500 flex items-center">
                        <User className="h-3 w-3 mr-1" />
                        {event.user.split('@')[0]}
                      </p>
                      <p className="text-xs text-gray-500 flex items-center">
                        <Calendar className="h-3 w-3 mr-1" />
                        {event.timestamp.split(' ')[1]}
                      </p>
                    </div>
                    <Badge className={getStatusColor(event.status)}>
                      {event.status}
                    </Badge>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        <div className="p-4 border-t bg-gray-50 dark:bg-gray-900">
          <p className="text-sm text-gray-600">
            Showing {visibleRange.start + 1} to {visibleRange.end} of {events.length} events
            {events.length > pageSize && (
              <span className="ml-2 text-xs">
                (Virtualized for performance)
              </span>
            )}
          </p>
        </div>
      </CardContent>
    </Card>
  );
}
