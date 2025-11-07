'use client';

import { useState, useMemo } from 'react';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
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
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Calendar, ChevronLeft, ChevronRight, Download, Eye, Filter, MoreVertical, Search, Trash2 } from 'lucide-react';

type ReportType = 'SOC 2' | 'PCI-DSS' | 'GDPR' | 'HIPAA' | 'ISO 27001';
type ReportStatus = 'Generating' | 'Ready' | 'Failed' | 'Expired';

interface ComplianceReport {
  id: string;
  name: string;
  type: ReportType;
  period: string;
  generatedDate: string;
  status: ReportStatus;
  size: string;
  generatedBy: string;
}

const mockReports: ComplianceReport[] = [
  {
    id: 'rpt-001',
    name: 'Q4 2024 SOC 2 Compliance',
    type: 'SOC 2',
    period: '2024 Q4',
    generatedDate: '2024-11-07 10:45:23',
    status: 'Ready',
    size: '2.4 MB',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-002',
    name: 'PCI-DSS Q3 2024',
    type: 'PCI-DSS',
    period: '2024 Q3',
    generatedDate: '2024-10-15 14:22:11',
    status: 'Ready',
    size: '1.8 MB',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-003',
    name: 'GDPR Compliance - EU Operations',
    type: 'GDPR',
    period: '2024',
    generatedDate: '2024-11-01 09:30:45',
    status: 'Ready',
    size: '3.2 MB',
    generatedBy: 'auditor@acme.com',
  },
  {
    id: 'rpt-004',
    name: 'HIPAA Security Rule Report',
    type: 'HIPAA',
    period: '2024 Q4',
    generatedDate: '2024-11-05 16:15:33',
    status: 'Generating',
    size: '-',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-005',
    name: 'ISO 27001 Annual Review',
    type: 'ISO 27001',
    period: '2024',
    generatedDate: '2024-09-20 11:45:12',
    status: 'Expired',
    size: '4.1 MB',
    generatedBy: 'auditor@acme.com',
  },
  {
    id: 'rpt-006',
    name: 'SOC 2 Q3 2024',
    type: 'SOC 2',
    period: '2024 Q3',
    generatedDate: '2024-10-01 08:20:55',
    status: 'Ready',
    size: '2.1 MB',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-007',
    name: 'PCI-DSS Q2 2024',
    type: 'PCI-DSS',
    period: '2024 Q2',
    generatedDate: '2024-07-15 13:10:22',
    status: 'Ready',
    size: '1.6 MB',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-008',
    name: 'GDPR Data Processing Activities',
    type: 'GDPR',
    period: '2024 H2',
    generatedDate: '2024-11-06 12:30:18',
    status: 'Failed',
    size: '-',
    generatedBy: 'auditor@acme.com',
  },
  {
    id: 'rpt-009',
    name: 'HIPAA Annual Risk Assessment',
    type: 'HIPAA',
    period: '2023',
    generatedDate: '2024-01-15 10:00:00',
    status: 'Expired',
    size: '5.3 MB',
    generatedBy: 'admin@acme.com',
  },
  {
    id: 'rpt-010',
    name: 'ISO 27001 Q4 2024',
    type: 'ISO 27001',
    period: '2024 Q4',
    generatedDate: '2024-11-07 11:00:00',
    status: 'Ready',
    size: '2.8 MB',
    generatedBy: 'auditor@acme.com',
  },
];

const REPORT_TYPES: ReportType[] = ['SOC 2', 'PCI-DSS', 'GDPR', 'HIPAA', 'ISO 27001'];
const REPORT_STATUSES: ReportStatus[] = ['Generating', 'Ready', 'Failed', 'Expired'];

const ITEMS_PER_PAGE = 10;

export function ReportsList() {
  const [reports, setReports] = useState<ComplianceReport[]>(mockReports);
  const [searchQuery, setSearchQuery] = useState('');
  const [typeFilter, setTypeFilter] = useState<ReportType | 'all'>('all');
  const [statusFilter, setStatusFilter] = useState<ReportStatus | 'all'>('all');
  const [dateFilter, setDateFilter] = useState('all');
  const [currentPage, setCurrentPage] = useState(1);
  const [sortField, setSortField] = useState<keyof ComplianceReport>('generatedDate');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('desc');

  const handleSort = (field: keyof ComplianceReport) => {
    if (sortField === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('asc');
    }
  };

  const filteredAndSortedReports = useMemo(() => {
    let filtered = reports.filter((report) => {
      const matchesSearch = report.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                           report.generatedBy.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesType = typeFilter === 'all' || report.type === typeFilter;
      const matchesStatus = statusFilter === 'all' || report.status === statusFilter;

      // Date filter logic
      const reportDate = new Date(report.generatedDate);
      const now = new Date();
      let matchesDate = true;

      if (dateFilter === '7d') {
        const sevenDaysAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
        matchesDate = reportDate >= sevenDaysAgo;
      } else if (dateFilter === '30d') {
        const thirtyDaysAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
        matchesDate = reportDate >= thirtyDaysAgo;
      } else if (dateFilter === '90d') {
        const ninetyDaysAgo = new Date(now.getTime() - 90 * 24 * 60 * 60 * 1000);
        matchesDate = reportDate >= ninetyDaysAgo;
      }

      return matchesSearch && matchesType && matchesStatus && matchesDate;
    });

    return filtered.sort((a, b) => {
      const aValue = a[sortField];
      const bValue = b[sortField];

      if (aValue < bValue) return sortDirection === 'asc' ? -1 : 1;
      if (aValue > bValue) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  }, [reports, searchQuery, typeFilter, statusFilter, dateFilter, sortField, sortDirection]);

  const totalPages = Math.ceil(filteredAndSortedReports.length / ITEMS_PER_PAGE);
  const startIndex = (currentPage - 1) * ITEMS_PER_PAGE;
  const paginatedReports = filteredAndSortedReports.slice(startIndex, startIndex + ITEMS_PER_PAGE);

  const getStatusBadge = (status: ReportStatus) => {
    const variants: Record<ReportStatus, 'default' | 'success' | 'destructive' | 'secondary'> = {
      'Generating': 'secondary',
      'Ready': 'success',
      'Failed': 'destructive',
      'Expired': 'default',
    };

    const colors: Record<ReportStatus, string> = {
      'Generating': 'bg-blue-100 text-blue-800',
      'Ready': 'bg-green-100 text-green-800',
      'Failed': 'bg-red-100 text-red-800',
      'Expired': 'bg-gray-100 text-gray-800',
    };

    return (
      <Badge className={colors[status]}>
        {status}
      </Badge>
    );
  };

  const handleExportList = () => {
    const csvContent = [
      ['Report Name', 'Type', 'Period', 'Generated Date', 'Status', 'Size', 'Generated By'],
      ...filteredAndSortedReports.map(r => [
        r.name,
        r.type,
        r.period,
        r.generatedDate,
        r.status,
        r.size,
        r.generatedBy,
      ]),
    ].map(row => row.join(',')).join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `compliance-reports-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
  };

  const handleDelete = (id: string) => {
    setReports(reports.filter(r => r.id !== id));
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">Report List</h3>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={handleExportList}>
            <Download className="h-4 w-4 mr-2" />
            Export List
          </Button>
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Search</label>
              <div className="relative">
                <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
                <Input
                  placeholder="Search reports..."
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
              <label className="text-sm font-medium">Report Type</label>
              <Select
                value={typeFilter}
                onValueChange={(value) => {
                  setTypeFilter(value as ReportType | 'all');
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All types" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All types</SelectItem>
                  {REPORT_TYPES.map((type) => (
                    <SelectItem key={type} value={type}>
                      {type}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Status</label>
              <Select
                value={statusFilter}
                onValueChange={(value) => {
                  setStatusFilter(value as ReportStatus | 'all');
                  setCurrentPage(1);
                }}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All statuses" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All statuses</SelectItem>
                  {REPORT_STATUSES.map((status) => (
                    <SelectItem key={status} value={status}>
                      {status}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Date Range</label>
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
                  <SelectItem value="7d">Last 7 days</SelectItem>
                  <SelectItem value="30d">Last 30 days</SelectItem>
                  <SelectItem value="90d">Last 90 days</SelectItem>
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
                <TableHead
                  className="cursor-pointer hover:bg-gray-50"
                  onClick={() => handleSort('name')}
                >
                  Report Name
                  {sortField === 'name' && (
                    <span className="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                  )}
                </TableHead>
                <TableHead
                  className="cursor-pointer hover:bg-gray-50"
                  onClick={() => handleSort('type')}
                >
                  Type
                  {sortField === 'type' && (
                    <span className="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                  )}
                </TableHead>
                <TableHead
                  className="cursor-pointer hover:bg-gray-50"
                  onClick={() => handleSort('period')}
                >
                  Period
                  {sortField === 'period' && (
                    <span className="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                  )}
                </TableHead>
                <TableHead
                  className="cursor-pointer hover:bg-gray-50"
                  onClick={() => handleSort('generatedDate')}
                >
                  Generated Date
                  {sortField === 'generatedDate' && (
                    <span className="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                  )}
                </TableHead>
                <TableHead
                  className="cursor-pointer hover:bg-gray-50"
                  onClick={() => handleSort('status')}
                >
                  Status
                  {sortField === 'status' && (
                    <span className="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                  )}
                </TableHead>
                <TableHead>Size</TableHead>
                <TableHead>Generated By</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {paginatedReports.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={8} className="text-center py-8 text-gray-500">
                    No reports found
                  </TableCell>
                </TableRow>
              ) : (
                paginatedReports.map((report) => (
                  <TableRow key={report.id}>
                    <TableCell className="font-medium">{report.name}</TableCell>
                    <TableCell>
                      <Badge variant="outline">{report.type}</Badge>
                    </TableCell>
                    <TableCell>{report.period}</TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <Calendar className="h-4 w-4 text-gray-400" />
                        {report.generatedDate}
                      </div>
                    </TableCell>
                    <TableCell>{getStatusBadge(report.status)}</TableCell>
                    <TableCell>{report.size}</TableCell>
                    <TableCell>{report.generatedBy}</TableCell>
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
                          <DropdownMenuItem>
                            <Eye className="h-4 w-4 mr-2" />
                            View
                          </DropdownMenuItem>
                          <DropdownMenuItem>
                            <Download className="h-4 w-4 mr-2" />
                            Download
                          </DropdownMenuItem>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem
                            className="text-red-600"
                            onClick={() => handleDelete(report.id)}
                          >
                            <Trash2 className="h-4 w-4 mr-2" />
                            Delete
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
            Showing {startIndex + 1} to {Math.min(startIndex + ITEMS_PER_PAGE, filteredAndSortedReports.length)} of{' '}
            {filteredAndSortedReports.length} reports
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
    </div>
  );
}
