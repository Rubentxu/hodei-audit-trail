'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Download,
  FileText,
  File,
  Table,
  Mail,
  Clock,
  CheckCircle,
  AlertCircle,
  XCircle,
  MoreVertical,
  History,
  RefreshCw,
} from 'lucide-react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

type DownloadStatus = 'downloading' | 'completed' | 'failed' | 'expired' | 'pending';

interface DownloadHistoryEntry {
  id: string;
  reportId: string;
  reportName: string;
  format: 'PDF' | 'JSON' | 'CSV';
  size: string;
  downloadedAt: string;
  status: DownloadStatus;
  downloadUrl?: string;
  expiresAt?: string;
}

interface ReportDownloadManagerProps {
  reportId: string;
  reportName: string;
  availableFormats: Array<'PDF' | 'JSON' | 'CSV'>;
  onDownload: (format: 'PDF' | 'JSON' | 'CSV') => void;
}

const mockDownloadHistory: DownloadHistoryEntry[] = [
  {
    id: 'dl-001',
    reportId: 'rpt-001',
    reportName: 'Q4 2024 SOC 2 Compliance',
    format: 'PDF',
    size: '2.4 MB',
    downloadedAt: '2024-11-07 14:30:00',
    status: 'completed',
    downloadUrl: '/downloads/report-001.pdf',
    expiresAt: '2024-11-14 14:30:00',
  },
  {
    id: 'dl-002',
    reportId: 'rpt-002',
    reportName: 'PCI-DSS Q3 2024',
    format: 'JSON',
    size: '1.8 MB',
    downloadedAt: '2024-11-07 12:15:00',
    status: 'completed',
    downloadUrl: '/downloads/report-002.json',
    expiresAt: '2024-11-14 12:15:00',
  },
  {
    id: 'dl-003',
    reportId: 'rpt-003',
    reportName: 'GDPR Compliance - EU Operations',
    format: 'CSV',
    size: '3.2 MB',
    downloadedAt: '2024-11-07 11:00:00',
    status: 'failed',
  },
  {
    id: 'dl-004',
    reportId: 'rpt-004',
    reportName: 'HIPAA Security Rule Report',
    format: 'PDF',
    size: '2.1 MB',
    downloadedAt: '2024-11-06 16:45:00',
    status: 'expired',
  },
  {
    id: 'dl-005',
    reportId: 'rpt-006',
    reportName: 'SOC 2 Q3 2024',
    format: 'PDF',
    size: '2.0 MB',
    downloadedAt: '2024-11-07 15:00:00',
    status: 'downloading',
  },
];

const downloadProgressData: Record<string, number> = {
  'dl-005': 65,
};

export function ReportDownloadManager({
  reportId,
  reportName,
  availableFormats,
  onDownload,
}: ReportDownloadManagerProps) {
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadFormat, setDownloadFormat] = useState<'PDF' | 'JSON' | 'CSV' | null>(null);
  const [downloadHistory, setDownloadHistory] = useState<DownloadHistoryEntry[]>(mockDownloadHistory);
  const [isHistoryOpen, setIsHistoryOpen] = useState(false);
  const [isEmailOpen, setIsEmailOpen] = useState(false);
  const [emailRecipients, setEmailRecipients] = useState('');

  const handleDownload = async (format: 'PDF' | 'JSON' | 'CSV') => {
    setIsDownloading(true);
    setDownloadFormat(format);

    const downloadId = `dl-${Date.now()}`;
    const newEntry: DownloadHistoryEntry = {
      id: downloadId,
      reportId,
      reportName,
      format,
      size: '2.4 MB',
      downloadedAt: new Date().toISOString().replace('T', ' ').substring(0, 19),
      status: 'downloading',
    };

    setDownloadHistory((prev) => [newEntry, ...prev]);

    setTimeout(() => {
      setDownloadHistory((prev) =>
        prev.map((entry) =>
          entry.id === downloadId
            ? {
                ...entry,
                status: 'completed',
                downloadUrl: `/downloads/${reportId}.${format.toLowerCase()}`,
                expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000)
                  .toISOString()
                  .replace('T', ' ')
                  .substring(0, 19),
              }
            : entry
        )
      );
      setIsDownloading(false);
      setDownloadFormat(null);
      onDownload(format);
    }, 2000);
  };

  const handleRetryDownload = (entry: DownloadHistoryEntry) => {
    setDownloadHistory((prev) =>
      prev.map((e) =>
        e.id === entry.id
          ? {
              ...e,
              status: 'downloading',
              downloadedAt: new Date().toISOString().replace('T', ' ').substring(0, 19),
            }
          : e
      )
    );

    setTimeout(() => {
      setDownloadHistory((prev) =>
        prev.map((e) =>
          e.id === entry.id
            ? {
                ...e,
                status: 'completed',
                downloadUrl: `/downloads/${entry.reportId}.${entry.format.toLowerCase()}`,
                expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000)
                  .toISOString()
                  .replace('T', ' ')
                  .substring(0, 19),
              }
            : e
        )
      );
    }, 1500);
  };

  const handleEmailDownload = (entry: DownloadHistoryEntry) => {
    setIsEmailOpen(true);
    setEmailRecipients('');
  };

  const sendEmailLink = () => {
    setIsEmailOpen(false);
  };

  const getStatusBadge = (status: DownloadStatus) => {
    switch (status) {
      case 'downloading':
        return (
          <Badge variant="outline" className="text-blue-600">
            <div className="h-2 w-2 rounded-full bg-blue-600 mr-1 animate-pulse" />
            Downloading
          </Badge>
        );
      case 'completed':
        return (
          <Badge variant="outline" className="text-green-600">
            <CheckCircle className="h-3 w-3 mr-1" />
            Completed
          </Badge>
        );
      case 'failed':
        return (
          <Badge variant="destructive">
            <XCircle className="h-3 w-3 mr-1" />
            Failed
          </Badge>
        );
      case 'expired':
        return (
          <Badge variant="outline" className="text-gray-600">
            <Clock className="h-3 w-3 mr-1" />
            Expired
          </Badge>
        );
      case 'pending':
        return (
          <Badge variant="outline" className="text-yellow-600">
            <AlertCircle className="h-3 w-3 mr-1" />
            Pending
          </Badge>
        );
    }
  };

  const getFormatIcon = (format: string) => {
    switch (format) {
      case 'PDF':
        return <FileText className="h-4 w-4" />;
      case 'JSON':
        return <File className="h-4 w-4" />;
      case 'CSV':
        return <Table className="h-4 w-4" />;
      default:
        return <File className="h-4 w-4" />;
    }
  };

  const completedDownloads = downloadHistory.filter((d) => d.status === 'completed');

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="sm">
            <Download className="h-4 w-4 mr-2" />
            Download
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>Download Options</DropdownMenuLabel>
          <DropdownMenuSeparator />
          {availableFormats.map((format) => (
            <DropdownMenuItem
              key={format}
              onClick={() => handleDownload(format)}
              disabled={isDownloading}
            >
              {getFormatIcon(format)}
              <span className="ml-2">Download as {format}</span>
            </DropdownMenuItem>
          ))}
          <DropdownMenuSeparator />
          <DropdownMenuItem onClick={() => setIsHistoryOpen(true)}>
            <History className="h-4 w-4 mr-2" />
            Download History
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {isDownloading && downloadFormat && (
        <div className="mt-2 p-3 bg-blue-50 border border-blue-200 rounded">
          <div className="flex items-center gap-2 mb-2">
            <Download className="h-4 w-4 text-blue-600" />
            <span className="text-sm font-medium text-blue-900">
              Downloading {reportName} as {downloadFormat}
            </span>
          </div>
          <Progress value={downloadProgressData['dl-005'] || 45} className="h-2" />
          <p className="text-xs text-blue-700 mt-1">
            Please wait while we prepare your download...
          </p>
        </div>
      )}

      <Dialog open={isHistoryOpen} onOpenChange={setIsHistoryOpen}>
        <DialogContent className="max-w-4xl">
          <DialogHeader>
            <DialogTitle>Download History</DialogTitle>
            <DialogDescription>
              View and manage your report downloads
            </DialogDescription>
          </DialogHeader>
          <Tabs defaultValue="all" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="all">All Downloads</TabsTrigger>
              <TabsTrigger value="completed">Completed</TabsTrigger>
              <TabsTrigger value="failed">Failed</TabsTrigger>
            </TabsList>
            <TabsContent value="all" className="space-y-4">
              <div className="max-h-96 overflow-y-auto space-y-2">
                {downloadHistory.map((entry) => (
                  <div
                    key={entry.id}
                    className="flex items-center justify-between p-3 border rounded-lg"
                  >
                    <div className="flex items-center gap-3 flex-1">
                      {getFormatIcon(entry.format)}
                      <div className="flex-1">
                        <p className="text-sm font-medium">{entry.reportName}</p>
                        <div className="flex items-center gap-2 text-xs text-gray-600">
                          <span>{entry.format}</span>
                          <span>•</span>
                          <span>{entry.size}</span>
                          <span>•</span>
                          <span>{entry.downloadedAt}</span>
                        </div>
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      {getStatusBadge(entry.status)}
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="ghost" size="sm">
                            <MoreVertical className="h-4 w-4" />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end">
                          {entry.status === 'completed' && entry.downloadUrl && (
                            <>
                              <DropdownMenuItem
                                onClick={() => window.open(entry.downloadUrl, '_blank')}
                              >
                                <Download className="h-4 w-4 mr-2" />
                                Download Again
                              </DropdownMenuItem>
                              <DropdownMenuItem onClick={() => handleEmailDownload(entry)}>
                                <Mail className="h-4 w-4 mr-2" />
                                Email Link
                              </DropdownMenuItem>
                            </>
                          )}
                          {entry.status === 'failed' && (
                            <DropdownMenuItem
                              onClick={() => handleRetryDownload(entry)}
                            >
                              <RefreshCw className="h-4 w-4 mr-2" />
                              Retry Download
                            </DropdownMenuItem>
                          )}
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </div>
                  </div>
                ))}
              </div>
            </TabsContent>
            <TabsContent value="completed" className="space-y-4">
              <div className="max-h-96 overflow-y-auto space-y-2">
                {completedDownloads.map((entry) => (
                  <div
                    key={entry.id}
                    className="flex items-center justify-between p-3 border rounded-lg"
                  >
                    <div className="flex items-center gap-3 flex-1">
                      {getFormatIcon(entry.format)}
                      <div className="flex-1">
                        <p className="text-sm font-medium">{entry.reportName}</p>
                        <div className="flex items-center gap-2 text-xs text-gray-600">
                          <span>{entry.format}</span>
                          <span>•</span>
                          <span>{entry.size}</span>
                          <span>•</span>
                          <span>{entry.downloadedAt}</span>
                        </div>
                        {entry.expiresAt && (
                          <p className="text-xs text-orange-600 mt-1">
                            Expires: {entry.expiresAt}
                          </p>
                        )}
                      </div>
                    </div>
                    <div className="flex items-center gap-3">
                      {getStatusBadge(entry.status)}
                      {entry.downloadUrl && (
                        <Button
                          size="sm"
                          onClick={() => window.open(entry.downloadUrl, '_blank')}
                        >
                          <Download className="h-4 w-4 mr-2" />
                          Download
                        </Button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </TabsContent>
            <TabsContent value="failed" className="space-y-4">
              <div className="max-h-96 overflow-y-auto space-y-2">
                {downloadHistory
                  .filter((entry) => entry.status === 'failed')
                  .map((entry) => (
                    <div
                      key={entry.id}
                      className="flex items-center justify-between p-3 border rounded-lg"
                    >
                      <div className="flex items-center gap-3 flex-1">
                        {getFormatIcon(entry.format)}
                        <div className="flex-1">
                          <p className="text-sm font-medium">{entry.reportName}</p>
                          <div className="flex items-center gap-2 text-xs text-gray-600">
                            <span>{entry.format}</span>
                            <span>•</span>
                            <span>{entry.size}</span>
                            <span>•</span>
                            <span>{entry.downloadedAt}</span>
                          </div>
                        </div>
                      </div>
                      <div className="flex items-center gap-3">
                        {getStatusBadge(entry.status)}
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => handleRetryDownload(entry)}
                        >
                          <RefreshCw className="h-4 w-4 mr-2" />
                          Retry
                        </Button>
                      </div>
                    </div>
                  ))}
              </div>
            </TabsContent>
          </Tabs>
        </DialogContent>
      </Dialog>

      <Dialog open={isEmailOpen} onOpenChange={setIsEmailOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Email Download Link</DialogTitle>
            <DialogDescription>
              Send the download link to an email address
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Email Recipients</label>
              <input
                type="email"
                className="w-full px-3 py-2 border rounded-md"
                placeholder="Enter email addresses separated by commas"
                value={emailRecipients}
                onChange={(e) => setEmailRecipients(e.target.value)}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsEmailOpen(false)}>
              Cancel
            </Button>
            <Button onClick={sendEmailLink}>
              <Mail className="h-4 w-4 mr-2" />
              Send Email
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
