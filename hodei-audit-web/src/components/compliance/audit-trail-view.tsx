'use client';

import { useState, useMemo } from 'react';
import { Button } from '@/components/ui/button';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import {
  Dialog,
  DialogContent,
  DialogDescription,
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
import {
  FileText,
  Search,
  Filter,
  Download,
  ChevronLeft,
  ChevronRight,
  MoreVertical,
  Calendar,
  User,
  Shield,
  Lock,
  Eye,
  AlertCircle,
  CheckCircle,
  Info,
} from 'lucide-react';

type ActionType =
  | 'key_generated'
  | 'key_rotated'
  | 'key_revoked'
  | 'report_generated'
  | 'report_downloaded'
  | 'digest_verified'
  | 'digest_failed'
  | 'settings_changed'
  | 'user_login'
  | 'user_logout'
  | 'permission_changed'
  | 'data_exported'
  | 'backup_created'
  | 'security_alert';

type AuditResult = 'success' | 'failure' | 'warning';

interface AuditLogEntry {
  id: string;
  timestamp: string;
  user: string;
  action: ActionType;
  resource: string;
  ipAddress: string;
  userAgent: string;
  result: AuditResult;
  details: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  complianceRelated: boolean;
  metadata?: Record<string, any>;
}

const ITEMS_PER_PAGE = 20;

const mockAuditLogs: AuditLogEntry[] = [
  {
    id: 'audit-001',
    timestamp: '2024-11-07 14:30:15',
    user: 'admin@acme.com',
    action: 'key_rotated',
    resource: 'key-001 (Primary Signing Key)',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Key rotated successfully as per scheduled rotation policy',
    severity: 'medium',
    complianceRelated: true,
  },
  {
    id: 'audit-002',
    timestamp: '2024-11-07 14:25:33',
    user: 'admin@acme.com',
    action: 'report_generated',
    resource: 'SOC 2 Compliance Report Q4 2024',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Report generated in PDF format with all sections',
    severity: 'low',
    complianceRelated: true,
  },
  {
    id: 'audit-003',
    timestamp: '2024-11-07 14:22:45',
    user: 'auditor@acme.com',
    action: 'digest_verified',
    resource: 'digest-2024-001',
    ipAddress: '192.168.1.105',
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36',
    result: 'success',
    details: 'Digest verified successfully. Hash matches expected value.',
    severity: 'medium',
    complianceRelated: true,
  },
  {
    id: 'audit-004',
    timestamp: '2024-11-07 14:20:12',
    user: 'admin@acme.com',
    action: 'settings_changed',
    resource: 'Compliance Settings',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Updated key rotation period from 90 to 85 days',
    severity: 'high',
    complianceRelated: true,
  },
  {
    id: 'audit-005',
    timestamp: '2024-11-07 14:15:08',
    user: 'admin@acme.com',
    action: 'user_login',
    resource: 'Authentication',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Successful login via password authentication',
    severity: 'low',
    complianceRelated: true,
  },
  {
    id: 'audit-006',
    timestamp: '2024-11-07 13:45:22',
    user: 'admin@acme.com',
    action: 'data_exported',
    resource: 'Audit Logs',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Exported 160 audit log entries to CSV file',
    severity: 'medium',
    complianceRelated: true,
  },
  {
    id: 'audit-007',
    timestamp: '2024-11-07 13:30:45',
    user: 'system',
    action: 'backup_created',
    resource: 'Database Backup',
    ipAddress: '10.0.0.5',
    userAgent: 'Automated Backup Service',
    result: 'success',
    details: 'Daily backup completed successfully. Size: 2.4 GB',
    severity: 'low',
    complianceRelated: true,
  },
  {
    id: 'audit-008',
    timestamp: '2024-11-07 13:20:11',
    user: 'analyst@acme.com',
    action: 'report_downloaded',
    resource: 'PCI-DSS Report Q3 2024',
    ipAddress: '192.168.1.110',
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36',
    result: 'success',
    details: 'Report downloaded in JSON format',
    severity: 'low',
    complianceRelated: true,
  },
  {
    id: 'audit-009',
    timestamp: '2024-11-07 12:55:33',
    user: 'admin@acme.com',
    action: 'key_generated',
    resource: 'key-006 (New Encryption Key)',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'New AES-256 key generated and stored in HSM',
    severity: 'high',
    complianceRelated: true,
  },
  {
    id: 'audit-010',
    timestamp: '2024-11-07 12:45:09',
    user: 'auditor@acme.com',
    action: 'digest_failed',
    resource: 'digest-2024-005',
    ipAddress: '192.168.1.105',
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36',
    result: 'failure',
    details: 'Digest verification failed. Hash mismatch detected.',
    severity: 'critical',
    complianceRelated: true,
  },
  {
    id: 'audit-011',
    timestamp: '2024-11-07 11:30:22',
    user: 'admin@acme.com',
    action: 'security_alert',
    resource: 'Failed Login Attempts',
    ipAddress: '203.0.113.45',
    userAgent: 'curl/7.68.0',
    result: 'warning',
    details: '5 failed login attempts from suspicious IP address',
    severity: 'high',
    complianceRelated: true,
  },
  {
    id: 'audit-012',
    timestamp: '2024-11-07 11:15:44',
    user: 'admin@acme.com',
    action: 'permission_changed',
    resource: 'User: analyst@acme.com',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Granted permission: generate:reports',
    severity: 'medium',
    complianceRelated: true,
  },
  {
    id: 'audit-013',
    timestamp: '2024-11-07 10:30:15',
    user: 'auditor@acme.com',
    action: 'user_logout',
    resource: 'Authentication',
    ipAddress: '192.168.1.105',
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36',
    result: 'success',
    details: 'User logged out successfully',
    severity: 'low',
    complianceRelated: true,
  },
  {
    id: 'audit-014',
    timestamp: '2024-11-07 10:00:00',
    user: 'system',
    action: 'key_revoked',
    resource: 'key-004 (Expired API Key)',
    ipAddress: '10.0.0.5',
    userAgent: 'Automated Key Management Service',
    result: 'success',
    details: 'Key automatically revoked due to expiration',
    severity: 'medium',
    complianceRelated: true,
  },
  {
    id: 'audit-015',
    timestamp: '2024-11-07 09:45:12',
    user: 'admin@acme.com',
    action: 'report_generated',
    resource: 'GDPR Compliance Report 2024',
    ipAddress: '192.168.1.100',
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
    result: 'success',
    details: 'Report generated in PDF format with data processing sections',
    severity: 'low',
    complianceRelated: true,
  },
];

const ACTION_TYPES: ActionType[] = [
  'key_generated',
  'key_rotated',
  'key_revoked',
  'report_generated',
  'report_downloaded',
  'digest_verified',
  'digest_failed',
  'settings_changed',
  'user_login',
  'user_logout',
  'permission_changed',
  'data_exported',
  'backup_created',
  'security_alert',
];

export function AuditTrailView() {
  const [logs, setLogs] = useState<AuditLogEntry[]>(mockAuditLogs);
  const [searchQuery, setSearchQuery] = useState('');
  const [actionFilter, setActionFilter] = useState<ActionType | 'all'>('all');
  const [userFilter, setUserFilter] = useState('all');
  const [resultFilter, setResultFilter] = useState<AuditResult | 'all'>('all');
  const [dateFilter, setDateFilter] = useState('all');
  const [currentPage, setCurrentPage] = useState(1);
  const [selectedLog, setSelectedLog] = useState<AuditLogEntry | null>(null);
  const [isDetailsOpen, setIsDetailsOpen] = useState(false);

  const uniqueUsers = Array.from(new Set(logs.map((log) => log.user)));

  const filteredLogs = useMemo(() => {
    return logs.filter((log) => {
      const matchesSearch =
        log.user.toLowerCase().includes(searchQuery.toLowerCase()) ||
        log.action.toLowerCase().includes(searchQuery.toLowerCase()) ||
        log.resource.toLowerCase().includes(searchQuery.toLowerCase()) ||
        log.details.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesAction = actionFilter === 'all' || log.action === actionFilter;
      const matchesUser = userFilter === 'all' || log.user === userFilter;
      const matchesResult = resultFilter === 'all' || log.result === resultFilter;

      let matchesDate = true;
      if (dateFilter !== 'all') {
        const logDate = new Date(log.timestamp);
        const now = new Date();
        if (dateFilter === '1d') {
          const oneDayAgo = new Date(now.getTime() - 24 * 60 * 60 * 1000);
          matchesDate = logDate >= oneDayAgo;
        } else if (dateFilter === '7d') {
          const sevenDaysAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
          matchesDate = logDate >= sevenDaysAgo;
        } else if (dateFilter === '30d') {
          const thirtyDaysAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
          matchesDate = logDate >= thirtyDaysAgo;
        }
      }

      return matchesSearch && matchesAction && matchesUser && matchesResult && matchesDate;
    });
  }, [logs, searchQuery, actionFilter, userFilter, resultFilter, dateFilter]);

  const totalPages = Math.ceil(filteredLogs.length / ITEMS_PER_PAGE);
  const startIndex = (currentPage - 1) * ITEMS_PER_PAGE;
  const paginatedLogs = filteredLogs.slice(startIndex, startIndex + ITEMS_PER_PAGE);

  const getActionIcon = (action: ActionType) => {
    if (action.includes('key')) return <Lock className="h-4 w-4" />;
    if (action.includes('report')) return <FileText className="h-4 w-4" />;
    if (action.includes('digest')) return <Shield className="h-4 w-4" />;
    if (action.includes('login') || action.includes('logout')) return <User className="h-4 w-4" />;
    if (action.includes('alert')) return <AlertCircle className="h-4 w-4" />;
    return <Info className="h-4 w-4" />;
  };

  const getResultBadge = (result: AuditResult, severity: string) => {
    if (result === 'success') {
      return (
        <Badge variant="outline" className="text-green-600">
          <CheckCircle className="h-3 w-3 mr-1" />
          Success
        </Badge>
      );
    } else if (result === 'failure') {
      return (
        <Badge variant="destructive">
          <AlertCircle className="h-3 w-3 mr-1" />
          Failure
        </Badge>
      );
    } else {
      return (
        <Badge variant="outline" className="text-yellow-600">
          <AlertCircle className="h-3 w-3 mr-1" />
          Warning
        </Badge>
      );
    }
  };

  const getSeverityBadge = (severity: string) => {
    const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
      'low': 'secondary',
      'medium': 'default',
      'high': 'outline',
      'critical': 'destructive',
    };
    return <Badge variant={variants[severity]}>{severity}</Badge>;
  };

  const handleExport = () => {
    const csvContent = [
      ['Timestamp', 'User', 'Action', 'Resource', 'IP Address', 'Result', 'Severity', 'Details'],
      ...filteredLogs.map((log) => [
        log.timestamp,
        log.user,
        log.action,
        log.resource,
        log.ipAddress,
        log.result,
        log.severity,
        log.details,
      ]),
    ].map((row) => row.join(',')).join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `audit-logs-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Audit Trail</h3>
          <p className="text-sm text-gray-600">
            Complete log of all compliance actions and system events
          </p>
        </div>
        <Button onClick={handleExport}>
          <Download className="h-4 w-4 mr-2" />
          Export Logs
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
            <div className="space-y-2">
              <Label className="text-sm font-medium">Search</Label>
              <div className="relative">
                <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
                <Input
                  placeholder="Search logs..."
                  value={searchQuery}
                  onChange={(e) => {
                    setSearchQuery(e.target.value);
                    setCurrentPage(1);
                  }}
                  className="pl-9"
                />
              </div>
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium">Action</Label>
              <Select
                value={actionFilter}
                onValueChange={(value) => {
                  setActionFilter(value as ActionType | 'all');
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All actions" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All actions</SelectItem>
                  {ACTION_TYPES.map((action) => (
                    <SelectItem key={action} value={action}>
                      {action.replace(/_/g, ' ')}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium">User</Label>
              <Select
                value={userFilter}
                onValueChange={(value) => {
                  setUserFilter(value);
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All users" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All users</SelectItem>
                  {uniqueUsers.map((user) => (
                    <SelectItem key={user} value={user}>
                      {user}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium">Result</Label>
              <Select
                value={resultFilter}
                onValueChange={(value) => {
                  setResultFilter(value as AuditResult | 'all');
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All results" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All results</SelectItem>
                  <SelectItem value="success">Success</SelectItem>
                  <SelectItem value="failure">Failure</SelectItem>
                  <SelectItem value="warning">Warning</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium">Date Range</Label>
              <Select
                value={dateFilter}
                onValueChange={(value) => {
                  setDateFilter(value);
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All dates" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All dates</SelectItem>
                  <SelectItem value="1d">Last 24 hours</SelectItem>
                  <SelectItem value="7d">Last 7 days</SelectItem>
                  <SelectItem value="30d">Last 30 days</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent className="p-0">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Timestamp</TableHead>
                <TableHead>User</TableHead>
                <TableHead>Action</TableHead>
                <TableHead>Resource</TableHead>
                <TableHead>IP Address</TableHead>
                <TableHead>Result</TableHead>
                <TableHead>Severity</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {paginatedLogs.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={8} className="text-center py-8 text-gray-500">
                    No audit logs found
                  </TableCell>
                </TableRow>
              ) : (
                paginatedLogs.map((log) => (
                  <TableRow key={log.id}>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <Calendar className="h-3 w-3 text-gray-400" />
                        {log.timestamp}
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <User className="h-3 w-3 text-gray-400" />
                        {log.user}
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        {getActionIcon(log.action)}
                        <span className="text-sm">{log.action.replace(/_/g, ' ')}</span>
                      </div>
                    </TableCell>
                    <TableCell className="max-w-xs truncate">{log.resource}</TableCell>
                    <TableCell className="font-mono text-xs">{log.ipAddress}</TableCell>
                    <TableCell>{getResultBadge(log.result, log.severity)}</TableCell>
                    <TableCell>{getSeverityBadge(log.severity)}</TableCell>
                    <TableCell className="text-right">
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="ghost" size="sm">
                            <MoreVertical className="h-4 w-4" />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end">
                          <DropdownMenuLabel>Actions</DropdownMenuLabel>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem
                            onClick={() => {
                              setSelectedLog(log);
                              setIsDetailsOpen(true);
                            }}
                          >
                            <Eye className="h-4 w-4 mr-2" />
                            View Details
                          </DropdownMenuItem>
                          <DropdownMenuItem onClick={() => navigator.clipboard.writeText(log.details)}>
                            <FileText className="h-4 w-4 mr-2" />
                            Copy Details
                          </DropdownMenuItem>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {totalPages > 1 && (
        <div className="flex items-center justify-between">
          <p className="text-sm text-gray-600">
            Showing {startIndex + 1} to {Math.min(startIndex + ITEMS_PER_PAGE, filteredLogs.length)} of{' '}
            {filteredLogs.length} logs
          </p>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCurrentPage(currentPage - 1)}
              disabled={currentPage === 1}
            >
              <ChevronLeft className="h-4 w-4" />
              Previous
            </Button>
            <div className="flex items-center gap-1">
              {Array.from({ length: totalPages }, (_, i) => i + 1).map((page) => (
                <Button
                  key={page}
                  variant={currentPage === page ? 'default' : 'outline'}
                  size="sm"
                  onClick={() => setCurrentPage(page)}
                  className="w-8"
                >
                  {page}
                </Button>
              ))}
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setCurrentPage(currentPage + 1)}
              disabled={currentPage === totalPages}
            >
              Next
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      )}

      <Dialog open={isDetailsOpen} onOpenChange={setIsDetailsOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>Audit Log Details</DialogTitle>
            <DialogDescription>
              {selectedLog?.timestamp} - {selectedLog?.action.replace(/_/g, ' ')}
            </DialogDescription>
          </DialogHeader>
          {selectedLog && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label className="text-sm font-medium">User</Label>
                  <p className="text-sm">{selectedLog.user}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Action</Label>
                  <p className="text-sm">{selectedLog.action.replace(/_/g, ' ')}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Resource</Label>
                  <p className="text-sm">{selectedLog.resource}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">IP Address</Label>
                  <p className="text-sm font-mono">{selectedLog.ipAddress}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Result</Label>
                  <div className="mt-1">{getResultBadge(selectedLog.result, selectedLog.severity)}</div>
                </div>
                <div>
                  <Label className="text-sm font-medium">Severity</Label>
                  <div className="mt-1">{getSeverityBadge(selectedLog.severity)}</div>
                </div>
              </div>
              <div>
                <Label className="text-sm font-medium">Details</Label>
                <p className="text-sm mt-1 p-3 bg-gray-100 rounded">{selectedLog.details}</p>
              </div>
              <div>
                <Label className="text-sm font-medium">User Agent</Label>
                <p className="text-xs mt-1 p-3 bg-gray-100 rounded break-all">
                  {selectedLog.userAgent}
                </p>
              </div>
              {selectedLog.complianceRelated && (
                <div className="p-3 bg-blue-50 border border-blue-200 rounded">
                  <div className="flex items-center gap-2">
                    <Shield className="h-4 w-4 text-blue-600" />
                    <span className="text-sm font-medium text-blue-900">
                      Compliance Related Event
                    </span>
                  </div>
                </div>
              )}
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
