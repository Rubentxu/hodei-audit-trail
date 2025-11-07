'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Progress } from '@/components/ui/progress';
import {
  Download,
  FileText,
  Table,
  FileSpreadsheet,
  Loader2
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

type ExportFormat = 'csv' | 'json' | 'pdf';
type ExportScope = 'current' | 'filtered' | 'all';

interface ExportDataProps {
  events: Event[];
  filteredEvents: Event[];
  onExport?: (format: ExportFormat, scope: ExportScope) => void;
}

export function ExportData({ events, filteredEvents, onExport }: ExportDataProps) {
  const [isExporting, setIsExporting] = useState(false);
  const [exportProgress, setExportProgress] = useState(0);
  const [exportFormat, setExportFormat] = useState<ExportFormat>('csv');
  const [exportScope, setExportScope] = useState<ExportScope>('filtered');

  const handleExport = async (format: ExportFormat, scope: ExportScope) => {
    setIsExporting(true);
    setExportFormat(format);
    setExportScope(scope);
    setExportProgress(0);

    try {
      // Simulate export progress
      for (let i = 0; i <= 100; i += 10) {
        setExportProgress(i);
        await new Promise(resolve => setTimeout(resolve, 100));
      }

      // Get the data to export
      const dataToExport = scope === 'current' ?
        filteredEvents.slice(0, 100) :
        scope === 'filtered' ?
        filteredEvents :
        events;

      // Format the data
      let exportContent = '';
      let filename = '';
      let mimeType = '';

      switch (format) {
        case 'csv':
          exportContent = convertToCSV(dataToExport);
          filename = `events-${new Date().toISOString().split('T')[0]}.csv`;
          mimeType = 'text/csv';
          break;
        case 'json':
          exportContent = JSON.stringify(dataToExport, null, 2);
          filename = `events-${new Date().toISOString().split('T')[0]}.json`;
          mimeType = 'application/json';
          break;
        case 'pdf':
          // In a real app, you'd use a library like jsPDF
          exportContent = convertToText(dataToExport);
          filename = `events-${new Date().toISOString().split('T')[0]}.txt`;
          mimeType = 'text/plain';
          break;
      }

      // Create download
      const blob = new Blob([exportContent], { type: mimeType });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      a.click();
      URL.revokeObjectURL(url);

      if (onExport) {
        onExport(format, scope);
      }
    } catch (error) {
      console.error('Export failed:', error);
    } finally {
      setIsExporting(false);
      setExportProgress(0);
    }
  };

  const convertToCSV = (events: Event[]) => {
    const headers = ['ID', 'Timestamp', 'User', 'Action', 'Resource', 'Status', 'Source', 'Details'];
    const rows = events.map(event => [
      event.id,
      event.timestamp,
      event.user,
      event.action,
      event.resource,
      event.status,
      event.source,
      event.details || ''
    ]);
    return [headers, ...rows].map(row => row.map(cell => `"${cell}"`).join(',')).join('\n');
  };

  const convertToText = (events: Event[]) => {
    return events.map(event => `
Event ID: ${event.id}
Timestamp: ${event.timestamp}
User: ${event.user}
Action: ${event.action}
Resource: ${event.resource}
Status: ${event.status}
Source: ${event.source}
Details: ${event.details || 'N/A'}
`).join('\n---\n\n');
  };

  const getEventCount = (scope: ExportScope) => {
    switch (scope) {
      case 'current':
        return Math.min(filteredEvents.length, 100);
      case 'filtered':
        return filteredEvents.length;
      case 'all':
        return events.length;
    }
  };

  if (isExporting) {
    return (
      <div className="space-y-4">
        <div className="flex items-center space-x-2">
          <Loader2 className="h-4 w-4 animate-spin" />
          <span className="text-sm font-medium">Exporting {exportFormat.toUpperCase()}...</span>
        </div>
        <Progress value={exportProgress} className="w-full" />
        <p className="text-xs text-gray-500">
          {exportProgress}% complete
        </p>
      </div>
    );
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline">
          <Download className="h-4 w-4 mr-2" />
          Export
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-64">
        <div className="px-2 py-1.5 text-sm font-semibold">
          Export Events
        </div>
        <DropdownMenuSeparator />

        {/* CSV Export */}
        <DropdownMenuItem onClick={() => handleExport('csv', 'current')}>
          <FileSpreadsheet className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">CSV - Current Page</p>
            <p className="text-xs text-gray-500">
              {getEventCount('current')} events
            </p>
          </div>
        </DropdownMenuItem>

        <DropdownMenuItem onClick={() => handleExport('csv', 'filtered')}>
          <FileSpreadsheet className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">CSV - Filtered Results</p>
            <p className="text-xs text-gray-500">
              {getEventCount('filtered')} events
            </p>
          </div>
        </DropdownMenuItem>

        <DropdownMenuItem onClick={() => handleExport('csv', 'all')}>
          <FileSpreadsheet className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">CSV - All Events</p>
            <p className="text-xs text-gray-500">
              {getEventCount('all')} events
            </p>
          </div>
        </DropdownMenuItem>

        <DropdownMenuSeparator />

        {/* JSON Export */}
        <DropdownMenuItem onClick={() => handleExport('json', 'filtered')}>
          <FileText className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">JSON - Filtered</p>
            <p className="text-xs text-gray-500">
              {getEventCount('filtered')} events
            </p>
          </div>
        </DropdownMenuItem>

        <DropdownMenuItem onClick={() => handleExport('json', 'all')}>
          <FileText className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">JSON - All Events</p>
            <p className="text-xs text-gray-500">
              {getEventCount('all')} events
            </p>
          </div>
        </DropdownMenuItem>

        <DropdownMenuSeparator />

        {/* PDF Export */}
        <DropdownMenuItem onClick={() => handleExport('pdf', 'current')}>
          <Table className="h-4 w-4 mr-2" />
          <div className="flex-1">
            <p className="font-medium">PDF - Current Page</p>
            <p className="text-xs text-gray-500">
              {getEventCount('current')} events
            </p>
          </div>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
