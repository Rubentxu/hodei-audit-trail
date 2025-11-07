"use client";

import { useState } from "react";
import { format } from "date-fns";
import {
  Calendar,
  Clock,
  FileText,
  Mail,
  Plus,
  MoreHorizontal,
  Play,
  Pause,
  Edit,
  Trash2,
  History,
  Settings,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Switch } from "@/components/ui/switch";
import { Checkbox } from "@/components/ui/checkbox";
import { Avatar, AvatarFallback, AvatarInitials } from "@/components/ui/avatar";
import { Progress } from "@/components/ui/progress";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

type ScheduleStatus = "active" | "paused" | "failed" | "completed" | "disabled";
type ReportType = "SOC 2" | "PCI-DSS" | "GDPR" | "HIPAA" | "ISO 27001";
type ReportFormat = "PDF" | "JSON" | "CSV";
type Frequency = "daily" | "weekly" | "monthly" | "quarterly" | "custom";

interface ScheduledReport {
  id: string;
  name: string;
  reportType: ReportType;
  format: ReportFormat;
  frequency: Frequency;
  cronExpression?: string;
  time: string;
  recipients: string[];
  lastRun?: string;
  nextRun: string;
  status: ScheduleStatus;
  totalRuns: number;
  successCount: number;
  failureCount: number;
  createdBy: string;
  createdAt: string;
  retryAttempts: number;
  enabled: boolean;
}

interface ScheduleHistoryEntry {
  id: string;
  scheduleId: string;
  runAt: string;
  status: "success" | "failure" | "timeout";
  duration: string;
  reportUrl?: string;
  errorMessage?: string;
  triggeredBy: string;
}

const mockScheduledReports: ScheduledReport[] = [
  {
    id: "sched-001",
    name: "SOC 2 Weekly Compliance",
    reportType: "SOC 2",
    format: "PDF",
    frequency: "weekly",
    time: "09:00",
    recipients: ["compliance@company.com", "auditor@company.com"],
    lastRun: "2025-11-01T09:00:00Z",
    nextRun: "2025-11-08T09:00:00Z",
    status: "active",
    totalRuns: 8,
    successCount: 7,
    failureCount: 0,
    createdBy: "John Doe",
    createdAt: "2025-09-01T10:00:00Z",
    retryAttempts: 2,
    enabled: true,
  },
  {
    id: "sched-002",
    name: "PCI-DSS Monthly Security",
    reportType: "PCI-DSS",
    format: "PDF",
    frequency: "monthly",
    time: "08:00",
    recipients: ["security@company.com"],
    lastRun: "2025-10-01T08:00:00Z",
    nextRun: "2025-12-01T08:00:00Z",
    status: "active",
    totalRuns: 3,
    successCount: 3,
    failureCount: 0,
    createdBy: "Jane Smith",
    createdAt: "2025-08-15T12:00:00Z",
    retryAttempts: 1,
    enabled: true,
  },
  {
    id: "sched-003",
    name: "GDPR Daily Data Audit",
    reportType: "GDPR",
    format: "JSON",
    frequency: "daily",
    time: "23:30",
    recipients: ["dpo@company.com"],
    lastRun: "2025-11-06T23:30:00Z",
    nextRun: "2025-11-07T23:30:00Z",
    status: "active",
    totalRuns: 30,
    successCount: 28,
    failureCount: 2,
    createdBy: "Bob Wilson",
    createdAt: "2025-08-01T09:00:00Z",
    retryAttempts: 3,
    enabled: true,
  },
  {
    id: "sched-004",
    name: "HIPAA Quarterly Review",
    reportType: "HIPAA",
    format: "PDF",
    frequency: "quarterly",
    time: "10:00",
    recipients: ["compliance@company.com", "legal@company.com"],
    status: "paused",
    totalRuns: 2,
    successCount: 2,
    failureCount: 0,
    createdBy: "Alice Johnson",
    createdAt: "2025-07-01T11:00:00Z",
    retryAttempts: 1,
    enabled: false,
  },
];

const mockScheduleHistory: ScheduleHistoryEntry[] = [
  {
    id: "hist-001",
    scheduleId: "sched-001",
    runAt: "2025-11-01T09:00:00Z",
    status: "success",
    duration: "2.3s",
    reportUrl: "/reports/soc2-2025-11-01.pdf",
    triggeredBy: "system",
  },
  {
    id: "hist-002",
    scheduleId: "sched-002",
    runAt: "2025-10-01T08:00:00Z",
    status: "success",
    duration: "3.1s",
    reportUrl: "/reports/pci-dss-2025-10-01.pdf",
    triggeredBy: "system",
  },
  {
    id: "hist-003",
    scheduleId: "sched-003",
    runAt: "2025-11-06T23:30:00Z",
    status: "failure",
    duration: "45s",
    errorMessage: "Connection timeout to audit service",
    triggeredBy: "system",
  },
  {
    id: "hist-004",
    scheduleId: "sched-001",
    runAt: "2025-10-25T09:00:00Z",
    status: "success",
    duration: "2.1s",
    reportUrl: "/reports/soc2-2025-10-25.pdf",
    triggeredBy: "system",
  },
];

export function ReportScheduler() {
  const [schedules, setSchedules] = useState<ScheduledReport[]>(mockScheduledReports);
  const [activeTab, setActiveTab] = useState("schedules");
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [selectedSchedule, setSelectedSchedule] = useState<ScheduledReport | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [statusFilter, setStatusFilter] = useState<ScheduleStatus | "all">("all");
  const [bulkSelection, setBulkSelection] = useState<string[]>([]);

  const getStatusColor = (status: ScheduleStatus) => {
    switch (status) {
      case "active":
        return "bg-green-500/10 text-green-700 dark:text-green-400";
      case "paused":
        return "bg-yellow-500/10 text-yellow-700 dark:text-yellow-400";
      case "failed":
        return "bg-red-500/10 text-red-700 dark:text-red-400";
      case "completed":
        return "bg-blue-500/10 text-blue-700 dark:text-blue-400";
      case "disabled":
        return "bg-gray-500/10 text-gray-700 dark:text-gray-400";
      default:
        return "bg-gray-500/10 text-gray-700 dark:text-gray-400";
    }
  };

  const filteredSchedules = schedules.filter((schedule) => {
    const matchesSearch = schedule.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      schedule.reportType.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesStatus = statusFilter === "all" || schedule.status === statusFilter;
    return matchesSearch && matchesStatus;
  });

  const toggleSchedule = (scheduleId: string) => {
    setSchedules((prev) =>
      prev.map((schedule) => {
        if (schedule.id === scheduleId) {
          return {
            ...schedule,
            enabled: !schedule.enabled,
            status: !schedule.enabled ? "active" : "disabled",
          };
        }
        return schedule;
      })
    );
  };

  const deleteSchedule = (scheduleId: string) => {
    setSchedules((prev) => prev.filter((s) => s.id !== scheduleId));
  };

  const toggleBulkSelection = (scheduleId: string) => {
    setBulkSelection((prev) =>
      prev.includes(scheduleId)
        ? prev.filter((id) => id !== scheduleId)
        : [...prev, scheduleId]
    );
  };

  const selectAllSchedules = () => {
    if (bulkSelection.length === filteredSchedules.length) {
      setBulkSelection([]);
    } else {
      setBulkSelection(filteredSchedules.map((s) => s.id));
    }
  };

  const handleBulkEnable = () => {
    setSchedules((prev) =>
      prev.map((schedule) =>
        bulkSelection.includes(schedule.id)
          ? { ...schedule, enabled: true, status: "active" as ScheduleStatus }
          : schedule
      )
    );
    setBulkSelection([]);
  };

  const handleBulkDisable = () => {
    setSchedules((prev) =>
      prev.map((schedule) =>
        bulkSelection.includes(schedule.id)
          ? { ...schedule, enabled: false, status: "disabled" as ScheduleStatus }
          : schedule
      )
    );
    setBulkSelection([]);
  };

  const handleBulkDelete = () => {
    setSchedules((prev) => prev.filter((s) => !bulkSelection.includes(s.id)));
    setBulkSelection([]);
  };

  const openEditModal = (schedule: ScheduledReport) => {
    setSelectedSchedule(schedule);
    setIsEditModalOpen(true);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Report Scheduling</h1>
          <p className="text-muted-foreground">
            Automate compliance report generation and delivery
          </p>
        </div>
        <Dialog open={isCreateModalOpen} onOpenChange={setIsCreateModalOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="mr-2 h-4 w-4" />
              Create Schedule
            </Button>
          </DialogTrigger>
          <CreateScheduleModal
            onClose={() => setIsCreateModalOpen(false)}
            onSubmit={(data) => {
              console.log("Creating schedule:", data);
              setIsCreateModalOpen(false);
            }}
          />
        </Dialog>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger value="schedules">
            <Calendar className="mr-2 h-4 w-4" />
            Scheduled Reports ({schedules.length})
          </TabsTrigger>
          <TabsTrigger value="history">
            <History className="mr-2 h-4 w-4" />
            Execution History ({mockScheduleHistory.length})
          </TabsTrigger>
        </TabsList>

        <TabsContent value="schedules" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Scheduled Reports</CardTitle>
              <CardDescription>
                Manage automated report generation schedules
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {bulkSelection.length > 0 && (
                <div className="flex items-center gap-2 p-3 bg-muted rounded-lg">
                  <span className="text-sm font-medium">
                    {bulkSelection.length} schedule(s) selected
                  </span>
                  <div className="ml-auto flex gap-2">
                    <Button size="sm" variant="outline" onClick={handleBulkEnable}>
                      <Play className="mr-1 h-3 w-3" />
                      Enable
                    </Button>
                    <Button size="sm" variant="outline" onClick={handleBulkDisable}>
                      <Pause className="mr-1 h-3 w-3" />
                      Disable
                    </Button>
                    <Button size="sm" variant="destructive" onClick={handleBulkDelete}>
                      <Trash2 className="mr-1 h-3 w-3" />
                      Delete
                    </Button>
                  </div>
                </div>
              )}

              <div className="flex gap-4">
                <div className="flex-1">
                  <Input
                    placeholder="Search schedules..."
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                  />
                </div>
                <Select value={statusFilter} onValueChange={(value) => setStatusFilter(value as any)}>
                  <SelectTrigger className="w-[180px]">
                    <SelectValue placeholder="Filter by status" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">All Statuses</SelectItem>
                    <SelectItem value="active">Active</SelectItem>
                    <SelectItem value="paused">Paused</SelectItem>
                    <SelectItem value="failed">Failed</SelectItem>
                    <SelectItem value="completed">Completed</SelectItem>
                    <SelectItem value="disabled">Disabled</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="w-12">
                      <Checkbox
                        checked={bulkSelection.length === filteredSchedules.length}
                        onCheckedChange={selectAllSchedules}
                      />
                    </TableHead>
                    <TableHead>Schedule</TableHead>
                    <TableHead>Frequency</TableHead>
                    <TableHead>Format</TableHead>
                    <TableHead>Next Run</TableHead>
                    <TableHead>Status</TableHead>
                    <TableHead>Success Rate</TableHead>
                    <TableHead className="w-[50px]"></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filteredSchedules.map((schedule) => (
                    <TableRow key={schedule.id}>
                      <TableCell>
                        <Checkbox
                          checked={bulkSelection.includes(schedule.id)}
                          onCheckedChange={() => toggleBulkSelection(schedule.id)}
                        />
                      </TableCell>
                      <TableCell>
                        <div className="flex flex-col">
                          <span className="font-medium">{schedule.name}</span>
                          <span className="text-sm text-muted-foreground">
                            {schedule.reportType} • {schedule.recipients.length} recipient(s)
                          </span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-1">
                          <Clock className="h-3 w-3" />
                          {schedule.frequency === "custom" ? (
                            <span className="text-sm">{schedule.cronExpression}</span>
                          ) : (
                            <span className="text-sm capitalize">{schedule.frequency}</span>
                          )}
                        </div>
                        <div className="text-xs text-muted-foreground">at {schedule.time}</div>
                      </TableCell>
                      <TableCell>
                        <Badge variant="outline">{schedule.format}</Badge>
                      </TableCell>
                      <TableCell>
                        <div className="text-sm">
                          {format(new Date(schedule.nextRun), "MMM dd, yyyy")}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {format(new Date(schedule.nextRun), "HH:mm")}
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-2">
                          <Switch
                            checked={schedule.enabled}
                            onCheckedChange={() => toggleSchedule(schedule.id)}
                          />
                          <Badge className={getStatusColor(schedule.status)}>
                            {schedule.status}
                          </Badge>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-2">
                          <Progress
                            value={
                              schedule.totalRuns > 0
                                ? (schedule.successCount / schedule.totalRuns) * 100
                                : 0
                            }
                            className="w-16"
                          />
                          <span className="text-sm">
                            {schedule.totalRuns > 0
                              ? Math.round((schedule.successCount / schedule.totalRuns) * 100)
                              : 0}
                            %
                          </span>
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {schedule.successCount}/{schedule.totalRuns} runs
                        </div>
                      </TableCell>
                      <TableCell>
                        <DropdownMenu>
                          <DropdownMenuTrigger asChild>
                            <Button variant="ghost" size="icon">
                              <MoreHorizontal className="h-4 w-4" />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent align="end">
                            <DropdownMenuLabel>Actions</DropdownMenuLabel>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem onClick={() => openEditModal(schedule)}>
                              <Edit className="mr-2 h-4 w-4" />
                              Edit
                            </DropdownMenuItem>
                            <DropdownMenuItem onClick={() => toggleSchedule(schedule.id)}>
                              {schedule.enabled ? (
                                <>
                                  <Pause className="mr-2 h-4 w-4" />
                                  Disable
                                </>
                              ) : (
                                <>
                                  <Play className="mr-2 h-4 w-4" />
                                  Enable
                                </>
                              )}
                            </DropdownMenuItem>
                            <DropdownMenuSeparator />
                            <DropdownMenuItem
                              className="text-destructive"
                              onClick={() => deleteSchedule(schedule.id)}
                            >
                              <Trash2 className="mr-2 h-4 w-4" />
                              Delete
                            </DropdownMenuItem>
                          </DropdownMenuContent>
                        </DropdownMenu>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="history" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Execution History</CardTitle>
              <CardDescription>
                View historical data of scheduled report executions
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Schedule</TableHead>
                    <TableHead>Run At</TableHead>
                    <TableHead>Status</TableHead>
                    <TableHead>Duration</TableHead>
                    <TableHead>Triggered By</TableHead>
                    <TableHead>Report</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {mockScheduleHistory.map((entry) => {
                    const schedule = schedules.find((s) => s.id === entry.scheduleId);
                    return (
                      <TableRow key={entry.id}>
                        <TableCell>
                          <div className="font-medium">{schedule?.name || "Unknown"}</div>
                        </TableCell>
                        <TableCell>
                          <div className="text-sm">
                            {format(new Date(entry.runAt), "MMM dd, yyyy HH:mm")}
                          </div>
                        </TableCell>
                        <TableCell>
                          <Badge
                            className={
                              entry.status === "success"
                                ? "bg-green-500/10 text-green-700 dark:text-green-400"
                                : "bg-red-500/10 text-red-700 dark:text-red-400"
                            }
                          >
                            {entry.status}
                          </Badge>
                        </TableCell>
                        <TableCell>
                          <span className="text-sm">{entry.duration}</span>
                          {entry.errorMessage && (
                            <div className="text-xs text-destructive">
                              {entry.errorMessage}
                            </div>
                          )}
                        </TableCell>
                        <TableCell>
                          <div className="flex items-center gap-2">
                            <Avatar className="h-6 w-6">
                              <AvatarFallback className="text-xs">
                                {entry.triggeredBy.substring(0, 2).toUpperCase()}
                              </AvatarFallback>
                            </Avatar>
                            <span className="text-sm capitalize">{entry.triggeredBy}</span>
                          </div>
                        </TableCell>
                        <TableCell>
                          {entry.reportUrl ? (
                            <Button variant="link" size="sm" className="h-auto p-0">
                              <FileText className="mr-1 h-3 w-3" />
                              View Report
                            </Button>
                          ) : (
                            <span className="text-sm text-muted-foreground">N/A</span>
                          )}
                        </TableCell>
                      </TableRow>
                    );
                  })}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      {selectedSchedule && (
        <EditScheduleModal
          open={isEditModalOpen}
          onOpenChange={setIsEditModalOpen}
          schedule={selectedSchedule}
          onClose={() => {
            setSelectedSchedule(null);
            setIsEditModalOpen(false);
          }}
          onSubmit={(data) => {
            setSchedules((prev) =>
              prev.map((s) => (s.id === selectedSchedule.id ? { ...s, ...data } : s))
            );
            setSelectedSchedule(null);
            setIsEditModalOpen(false);
          }}
        />
      )}
    </div>
  );
}

function CreateScheduleModal({
  onClose,
  onSubmit,
}: {
  onClose: () => void;
  onSubmit: (data: any) => void;
}) {
  const [formData, setFormData] = useState({
    name: "",
    reportType: "SOC 2" as ReportType,
    format: "PDF" as ReportFormat,
    frequency: "weekly" as Frequency,
    cronExpression: "",
    time: "09:00",
    recipients: "",
    retryAttempts: 2,
    enabled: true,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({
      ...formData,
      recipients: formData.recipients
        .split(",")
        .map((email) => email.trim())
        .filter(Boolean),
    });
  };

  return (
    <DialogContent className="max-w-2xl">
      <DialogHeader>
        <DialogTitle>Create New Schedule</DialogTitle>
        <DialogDescription>
          Set up automated report generation and delivery
        </DialogDescription>
      </DialogHeader>
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="name">Schedule Name</Label>
            <Input
              id="name"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="reportType">Report Type</Label>
            <Select
              value={formData.reportType}
              onValueChange={(value) =>
                setFormData({ ...formData, reportType: value as ReportType })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="SOC 2">SOC 2</SelectItem>
                <SelectItem value="PCI-DSS">PCI-DSS</SelectItem>
                <SelectItem value="GDPR">GDPR</SelectItem>
                <SelectItem value="HIPAA">HIPAA</SelectItem>
                <SelectItem value="ISO 27001">ISO 27001</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="format">Output Format</Label>
            <Select
              value={formData.format}
              onValueChange={(value) =>
                setFormData({ ...formData, format: value as ReportFormat })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="PDF">PDF</SelectItem>
                <SelectItem value="JSON">JSON</SelectItem>
                <SelectItem value="CSV">CSV</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="frequency">Frequency</Label>
            <Select
              value={formData.frequency}
              onValueChange={(value) =>
                setFormData({ ...formData, frequency: value as Frequency })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="daily">Daily</SelectItem>
                <SelectItem value="weekly">Weekly</SelectItem>
                <SelectItem value="monthly">Monthly</SelectItem>
                <SelectItem value="quarterly">Quarterly</SelectItem>
                <SelectItem value="custom">Custom (Cron)</SelectItem>
              </SelectContent>
            </Select>
          </div>
          {formData.frequency === "custom" && (
            <div className="col-span-2 space-y-2">
              <Label htmlFor="cronExpression">Cron Expression</Label>
              <Input
                id="cronExpression"
                placeholder="0 9 * * 1"
                value={formData.cronExpression}
                onChange={(e) =>
                  setFormData({ ...formData, cronExpression: e.target.value })
                }
              />
              <p className="text-xs text-muted-foreground">
                Example: "0 9 * * 1" = Every Monday at 09:00
              </p>
            </div>
          )}
          <div className="space-y-2">
            <Label htmlFor="time">Time</Label>
            <Input
              id="time"
              type="time"
              value={formData.time}
              onChange={(e) => setFormData({ ...formData, time: e.target.value })}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="retryAttempts">Retry Attempts</Label>
            <Input
              id="retryAttempts"
              type="number"
              min="0"
              max="5"
              value={formData.retryAttempts}
              onChange={(e) =>
                setFormData({ ...formData, retryAttempts: parseInt(e.target.value) })
              }
            />
          </div>
        </div>

        <div className="space-y-2">
          <Label htmlFor="recipients">Email Recipients</Label>
          <Textarea
            id="recipients"
            placeholder="email1@company.com, email2@company.com"
            value={formData.recipients}
            onChange={(e) => setFormData({ ...formData, recipients: e.target.value })}
            rows={3}
          />
        </div>

        <div className="flex items-center space-x-2">
          <Switch
            id="enabled"
            checked={formData.enabled}
            onCheckedChange={(checked) =>
              setFormData({ ...formData, enabled: checked })
            }
          />
          <Label htmlFor="enabled">Enable schedule immediately</Label>
        </div>

        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit">Create Schedule</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  );
}

function EditScheduleModal({
  open,
  onClose,
  schedule,
  onSubmit,
}: {
  open: boolean;
  onClose: () => void;
  schedule: ScheduledReport;
  onSubmit: (data: any) => void;
}) {
  const [formData, setFormData] = useState({
    name: schedule.name,
    reportType: schedule.reportType,
    format: schedule.format,
    frequency: schedule.frequency,
    cronExpression: schedule.cronExpression || "",
    time: schedule.time,
    recipients: schedule.recipients.join(", "),
    retryAttempts: schedule.retryAttempts,
    enabled: schedule.enabled,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({
      ...formData,
      recipients: formData.recipients
        .split(",")
        .map((email) => email.trim())
        .filter(Boolean),
    });
  };

  return (
    <DialogContent className="max-w-2xl">
      <DialogHeader>
        <DialogTitle>Edit Schedule</DialogTitle>
        <DialogDescription>Modify schedule settings and options</DialogDescription>
      </DialogHeader>
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="name">Schedule Name</Label>
            <Input
              id="name"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="reportType">Report Type</Label>
            <Select
              value={formData.reportType}
              onValueChange={(value) =>
                setFormData({ ...formData, reportType: value as ReportType })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="SOC 2">SOC 2</SelectItem>
                <SelectItem value="PCI-DSS">PCI-DSS</SelectItem>
                <SelectItem value="GDPR">GDPR</SelectItem>
                <SelectItem value="HIPAA">HIPAA</SelectItem>
                <SelectItem value="ISO 27001">ISO 27001</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="format">Output Format</Label>
            <Select
              value={formData.format}
              onValueChange={(value) =>
                setFormData({ ...formData, format: value as ReportFormat })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="PDF">PDF</SelectItem>
                <SelectItem value="JSON">JSON</SelectItem>
                <SelectItem value="CSV">CSV</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="frequency">Frequency</Label>
            <Select
              value={formData.frequency}
              onValueChange={(value) =>
                setFormData({ ...formData, frequency: value as Frequency })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="daily">Daily</SelectItem>
                <SelectItem value="weekly">Weekly</SelectItem>
                <SelectItem value="monthly">Monthly</SelectItem>
                <SelectItem value="quarterly">Quarterly</SelectItem>
                <SelectItem value="custom">Custom (Cron)</SelectItem>
              </SelectContent>
            </Select>
          </div>
          {formData.frequency === "custom" && (
            <div className="col-span-2 space-y-2">
              <Label htmlFor="cronExpression">Cron Expression</Label>
              <Input
                id="cronExpression"
                value={formData.cronExpression}
                onChange={(e) =>
                  setFormData({ ...formData, cronExpression: e.target.value })
                }
              />
            </div>
          )}
          <div className="space-y-2">
            <Label htmlFor="time">Time</Label>
            <Input
              id="time"
              type="time"
              value={formData.time}
              onChange={(e) => setFormData({ ...formData, time: e.target.value })}
              required
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="retryAttempts">Retry Attempts</Label>
            <Input
              id="retryAttempts"
              type="number"
              min="0"
              max="5"
              value={formData.retryAttempts}
              onChange={(e) =>
                setFormData({ ...formData, retryAttempts: parseInt(e.target.value) })
              }
            />
          </div>
        </div>

        <div className="space-y-2">
          <Label htmlFor="recipients">Email Recipients</Label>
          <Textarea
            id="recipients"
            value={formData.recipients}
            onChange={(e) => setFormData({ ...formData, recipients: e.target.value })}
            rows={3}
          />
        </div>

        <div className="flex items-center space-x-2">
          <Switch
            id="enabled"
            checked={formData.enabled}
            onCheckedChange={(checked) =>
              setFormData({ ...formData, enabled: checked })
            }
          />
          <Label htmlFor="enabled">Enable schedule</Label>
        </div>

        <DialogFooter>
          <Button type="button" variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button type="submit">Save Changes</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  );
}
