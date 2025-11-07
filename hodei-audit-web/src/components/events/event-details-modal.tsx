'use client';

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import {
  Calendar,
  User,
  Activity,
  Globe,
  Hash,
  FileText,
  CheckCircle,
  XCircle,
  AlertCircle
} from 'lucide-react';

type EventDetails = {
  id: string;
  timestamp: string;
  user: string;
  action: string;
  resource: string;
  status: 'success' | 'failure' | 'warning';
  source: string;
  details?: string;
  ip?: string;
  userAgent?: string;
  duration?: number;
  bytes?: number;
};

interface EventDetailsModalProps {
  event: EventDetails | null;
  isOpen: boolean;
  onClose: () => void;
}

export function EventDetailsModal({ event, isOpen, onClose }: EventDetailsModalProps) {
  if (!event) return null;

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
        return <CheckCircle className="h-5 w-5 text-green-600" />;
      case 'failure':
        return <XCircle className="h-5 w-5 text-red-600" />;
      case 'warning':
        return <AlertCircle className="h-5 w-5 text-yellow-600" />;
      default:
        return null;
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <DialogTitle className="text-xl">Event Details</DialogTitle>
            <Badge className={getStatusColor(event.status)}>
              {event.status}
            </Badge>
          </div>
          <DialogDescription>
            View detailed information about this audit event
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Event Summary */}
          <div className="flex items-center space-x-3">
            {getStatusIcon(event.status)}
            <div>
              <h3 className="font-semibold text-lg">{event.action}</h3>
              <p className="text-sm text-gray-600">{event.resource}</p>
            </div>
          </div>

          <Separator />

          {/* Event Information Grid */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <Hash className="h-4 w-4 mr-2" />
                Event ID
              </p>
              <code className="text-sm font-mono bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                {event.id}
              </code>
            </div>

            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <Calendar className="h-4 w-4 mr-2" />
                Timestamp
              </p>
              <p className="text-sm font-mono">{event.timestamp}</p>
            </div>

            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <User className="h-4 w-4 mr-2" />
                User
              </p>
              <p className="text-sm">{event.user}</p>
            </div>

            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <Globe className="h-4 w-4 mr-2" />
                Source
              </p>
              <p className="text-sm">{event.source}</p>
            </div>

            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <Activity className="h-4 w-4 mr-2" />
                Action
              </p>
              <code className="text-sm bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                {event.action}
              </code>
            </div>

            <div className="space-y-1">
              <p className="text-sm text-gray-500 flex items-center">
                <FileText className="h-4 w-4 mr-2" />
                Status
              </p>
              <Badge className={getStatusColor(event.status)}>
                {event.status}
              </Badge>
            </div>

            {event.ip && (
              <div className="space-y-1">
                <p className="text-sm text-gray-500">IP Address</p>
                <p className="text-sm font-mono">{event.ip}</p>
              </div>
            )}

            {event.duration && (
              <div className="space-y-1">
                <p className="text-sm text-gray-500">Duration</p>
                <p className="text-sm">{event.duration}ms</p>
              </div>
            )}

            {event.bytes && (
              <div className="space-y-1">
                <p className="text-sm text-gray-500">Data Size</p>
                <p className="text-sm">{event.bytes.toLocaleString()} bytes</p>
              </div>
            )}
          </div>

          {event.details && (
            <>
              <Separator />
              <div className="space-y-2">
                <p className="text-sm text-gray-500">Description</p>
                <p className="text-sm">{event.details}</p>
              </div>
            </>
          )}

          {event.userAgent && (
            <>
              <Separator />
              <div className="space-y-2">
                <p className="text-sm text-gray-500">User Agent</p>
                <code className="text-xs bg-gray-100 dark:bg-gray-800 p-2 rounded block overflow-x-auto">
                  {event.userAgent}
                </code>
              </div>
            </>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
