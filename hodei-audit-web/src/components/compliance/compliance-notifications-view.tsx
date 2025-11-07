"use client";

import { useState } from "react";
import { format } from "date-fns";
import {
  Bell,
  BellRing,
  Check,
  CheckCheck,
  Clock,
  Key,
  Shield,
  AlertTriangle,
  FileText,
  Settings,
  Trash2,
  Filter,
  Search,
  Mail,
  Smartphone,
  Monitor,
  User,
  X,
  Archive,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
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
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Separator } from "@/components/ui/separator";

type NotificationType =
  | "key_expiration"
  | "digest_verification_failure"
  | "report_generated"
  | "schedule_failure"
  | "security_alert"
  | "compliance_check"
  | "system_update";

type NotificationStatus = "unread" | "read" | "archived";
type ChannelType = "email" | "in_app" | "push";
type Severity = "low" | "medium" | "high" | "critical";

interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  status: NotificationStatus;
  severity: Severity;
  channel: ChannelType;
  createdAt: string;
  readAt?: string;
  actionUrl?: string;
  metadata?: Record<string, any>;
  userId: string;
}

interface NotificationPreferences {
  userId: string;
  email: boolean;
  inApp: boolean;
  push: boolean;
  keyExpiration: boolean;
  digestVerification: boolean;
  reportGenerated: boolean;
  scheduleFailure: boolean;
  securityAlerts: boolean;
  quietHours: {
    enabled: boolean;
    start: string;
    end: string;
  };
  frequency: "immediate" | "hourly" | "daily" | "weekly";
}

const mockNotifications: Notification[] = [
  {
    id: "notif-001",
    type: "key_expiration",
    title: "Cryptographic Key Expiring",
    message: "AES-256 key 'production-ssl-cert' will expire in 7 days",
    status: "unread",
    severity: "high",
    channel: "in_app",
    createdAt: "2025-11-07T10:30:00Z",
    userId: "user-1",
    metadata: { keyName: "production-ssl-cert", daysUntilExpiration: 7 },
  },
  {
    id: "notif-002",
    type: "report_generated",
    title: "SOC 2 Report Generated",
    message: "Monthly SOC 2 compliance report has been generated successfully",
    status: "unread",
    severity: "medium",
    channel: "email",
    createdAt: "2025-11-07T09:00:00Z",
    readAt: undefined,
    userId: "user-1",
    actionUrl: "/compliance/reports/soc2-2025-11-01",
    metadata: { reportType: "SOC 2", reportId: "rpt-001" },
  },
  {
    id: "notif-003",
    type: "digest_verification_failure",
    title: "Digest Chain Verification Failed",
    message:
      "Unable to verify digest chain for period 2025-11-01. Hash mismatch detected.",
    status: "unread",
    severity: "critical",
    channel: "in_app",
    createdAt: "2025-11-07T08:15:00Z",
    userId: "user-1",
    metadata: { period: "2025-11-01", error: "hash_mismatch" },
  },
  {
    id: "notif-004",
    type: "schedule_failure",
    title: "Scheduled Report Failed",
    message: "Daily GDPR audit report generation failed: Connection timeout",
    status: "read",
    severity: "medium",
    channel: "email",
    createdAt: "2025-11-06T23:30:00Z",
    readAt: "2025-11-07T08:00:00Z",
    userId: "user-1",
    metadata: { scheduleId: "sched-003", error: "connection_timeout" },
  },
  {
    id: "notif-005",
    type: "security_alert",
    title: "Unusual Access Pattern Detected",
    message: "Multiple failed login attempts detected from IP 192.168.1.100",
    status: "unread",
    severity: "high",
    channel: "push",
    createdAt: "2025-11-06T14:22:00Z",
    userId: "user-1",
    metadata: { ipAddress: "192.168.1.100", attempts: 5 },
  },
  {
    id: "notif-006",
    type: "compliance_check",
    title: "Weekly Compliance Check Complete",
    message: "All compliance checks passed. 98% compliance score maintained.",
    status: "read",
    severity: "low",
    channel: "in_app",
    createdAt: "2025-11-06T12:00:00Z",
    readAt: "2025-11-06T12:30:00Z",
    userId: "user-1",
    metadata: { complianceScore: 98 },
  },
  {
    id: "notif-007",
    type: "key_expiration",
    title: "PGP Key Expiring",
    message: "PGP signing key 'internal-docs' will expire in 30 days",
    status: "read",
    severity: "medium",
    channel: "email",
    createdAt: "2025-11-05T16:45:00Z",
    readAt: "2025-11-05T17:00:00Z",
    userId: "user-1",
    metadata: { keyName: "internal-docs", daysUntilExpiration: 30 },
  },
  {
    id: "notif-008",
    type: "system_update",
    title: "Compliance System Update",
    message:
      "System update v2.4.0 deployed successfully. New features available.",
    status: "read",
    severity: "low",
    channel: "in_app",
    createdAt: "2025-11-05T10:00:00Z",
    readAt: "2025-11-05T10:15:00Z",
    userId: "user-1",
    metadata: { version: "2.4.0" },
  },
];

const mockPreferences: NotificationPreferences = {
  userId: "user-1",
  email: true,
  inApp: true,
  push: true,
  keyExpiration: true,
  digestVerification: true,
  reportGenerated: true,
  scheduleFailure: true,
  securityAlerts: true,
  quietHours: {
    enabled: true,
    start: "22:00",
    end: "08:00",
  },
  frequency: "immediate",
};

export function ComplianceNotificationsView() {
  const [notifications, setNotifications] =
    useState<Notification[]>(mockNotifications);
  const [preferences, setPreferences] =
    useState<NotificationPreferences>(mockPreferences);
  const [activeTab, setActiveTab] = useState("notifications");
  const [isPreferenceModalOpen, setIsPreferenceModalOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [typeFilter, setTypeFilter] = useState<NotificationType | "all">("all");
  const [statusFilter, setStatusFilter] = useState<NotificationStatus | "all">(
    "all",
  );
  const [severityFilter, setSeverityFilter] = useState<Severity | "all">("all");

  const getSeverityColor = (severity: Severity) => {
    switch (severity) {
      case "critical":
        return "bg-red-500/10 text-red-700 dark:text-red-400";
      case "high":
        return "bg-orange-500/10 text-orange-700 dark:text-orange-400";
      case "medium":
        return "bg-yellow-500/10 text-yellow-700 dark:text-yellow-400";
      case "low":
        return "bg-blue-500/10 text-blue-700 dark:text-blue-400";
      default:
        return "bg-gray-500/10 text-gray-700 dark:text-gray-400";
    }
  };

  const getTypeIcon = (type: NotificationType) => {
    switch (type) {
      case "key_expiration":
        return <Key className="h-4 w-4" />;
      case "digest_verification_failure":
        return <Shield className="h-4 w-4" />;
      case "report_generated":
        return <FileText className="h-4 w-4" />;
      case "schedule_failure":
        return <AlertTriangle className="h-4 w-4" />;
      case "security_alert":
        return <Shield className="h-4 w-4" />;
      case "compliance_check":
        return <Check className="h-4 w-4" />;
      case "system_update":
        return <Settings className="h-4 w-4" />;
      default:
        return <Bell className="h-4 w-4" />;
    }
  };

  const getChannelIcon = (channel: ChannelType) => {
    switch (channel) {
      case "email":
        return <Mail className="h-3 w-3" />;
      case "push":
        return <Smartphone className="h-3 w-3" />;
      case "in_app":
        return <Monitor className="h-3 w-3" />;
      default:
        return <Bell className="h-3 w-3" />;
    }
  };

  const filteredNotifications = notifications.filter((notification) => {
    const matchesSearch =
      notification.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      notification.message.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesType =
      typeFilter === "all" || notification.type === typeFilter;
    const matchesStatus =
      statusFilter === "all" || notification.status === statusFilter;
    const matchesSeverity =
      severityFilter === "all" || notification.severity === severityFilter;
    return matchesSearch && matchesType && matchesStatus && matchesSeverity;
  });

  const unreadCount = notifications.filter((n) => n.status === "unread").length;

  const markAsRead = (notificationId: string) => {
    setNotifications((prev) =>
      prev.map((notification) =>
        notification.id === notificationId
          ? {
              ...notification,
              status: "read",
              readAt: new Date().toISOString(),
            }
          : notification,
      ),
    );
  };

  const markAsUnread = (notificationId: string) => {
    setNotifications((prev) =>
      prev.map((notification) =>
        notification.id === notificationId
          ? { ...notification, status: "unread", readAt: undefined }
          : notification,
      ),
    );
  };

  const archiveNotification = (notificationId: string) => {
    setNotifications((prev) =>
      prev.map((notification) =>
        notification.id === notificationId
          ? { ...notification, status: "archived" as NotificationStatus }
          : notification,
      ),
    );
  };

  const deleteNotification = (notificationId: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== notificationId));
  };

  const markAllAsRead = () => {
    setNotifications((prev) =>
      prev.map((notification) => ({
        ...notification,
        status: "read" as NotificationStatus,
        readAt: new Date().toISOString(),
      })),
    );
  };

  const deleteAllRead = () => {
    setNotifications((prev) => prev.filter((n) => n.status !== "read"));
  };

  const getTimeAgo = (dateString: string) => {
    const now = new Date();
    const date = new Date(dateString);
    const diffInMs = now.getTime() - date.getTime();
    const diffInHours = Math.floor(diffInMs / (1000 * 60 * 60));
    const diffInDays = Math.floor(diffInHours / 24);

    if (diffInDays > 0) {
      return `${diffInDays} day${diffInDays > 1 ? "s" : ""} ago`;
    } else if (diffInHours > 0) {
      return `${diffInHours} hour${diffInHours > 1 ? "s" : ""} ago`;
    } else {
      return "Just now";
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">
            Compliance Notifications
          </h1>
          <p className="text-muted-foreground">
            Stay informed about compliance events and system status
          </p>
        </div>
        <div className="flex items-center gap-2">
          {unreadCount > 0 && (
            <Button variant="outline" onClick={markAllAsRead}>
              <CheckCheck className="mr-2 h-4 w-4" />
              Mark All Read
            </Button>
          )}
          <Button
            variant="outline"
            onClick={() => setIsPreferenceModalOpen(true)}
          >
            <Settings className="mr-2 h-4 w-4" />
            Preferences
          </Button>
        </div>
      </div>

      <Tabs value={activeTab} onValueChange={setActiveTab}>
        <TabsList>
          <TabsTrigger
            value="notifications"
            className="flex items-center gap-2"
          >
            <Bell className="h-4 w-4" />
            Notifications
            {unreadCount > 0 && (
              <Badge className="ml-2 bg-red-500 text-white">
                {unreadCount}
              </Badge>
            )}
          </TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="notifications" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Recent Notifications</CardTitle>
              <CardDescription>
                {unreadCount} unread notification{unreadCount !== 1 ? "s" : ""}
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex gap-4">
                <div className="flex-1">
                  <div className="relative">
                    <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                    <Input
                      placeholder="Search notifications..."
                      value={searchQuery}
                      onChange={(e) => setSearchQuery(e.target.value)}
                      className="pl-10"
                    />
                  </div>
                </div>
                <Select
                  value={typeFilter}
                  onValueChange={(value) => setTypeFilter(value as any)}
                >
                  <SelectTrigger className="w-[180px]">
                    <SelectValue placeholder="Filter by type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">All Types</SelectItem>
                    <SelectItem value="key_expiration">
                      Key Expiration
                    </SelectItem>
                    <SelectItem value="digest_verification_failure">
                      Digest Failure
                    </SelectItem>
                    <SelectItem value="report_generated">
                      Report Generated
                    </SelectItem>
                    <SelectItem value="schedule_failure">
                      Schedule Failure
                    </SelectItem>
                    <SelectItem value="security_alert">
                      Security Alert
                    </SelectItem>
                    <SelectItem value="compliance_check">
                      Compliance Check
                    </SelectItem>
                    <SelectItem value="system_update">System Update</SelectItem>
                  </SelectContent>
                </Select>
                <Select
                  value={statusFilter}
                  onValueChange={(value) => setStatusFilter(value as any)}
                >
                  <SelectTrigger className="w-[150px]">
                    <SelectValue placeholder="Filter by status" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">All Statuses</SelectItem>
                    <SelectItem value="unread">Unread</SelectItem>
                    <SelectItem value="read">Read</SelectItem>
                    <SelectItem value="archived">Archived</SelectItem>
                  </SelectContent>
                </Select>
                <Select
                  value={severityFilter}
                  onValueChange={(value) => setSeverityFilter(value as any)}
                >
                  <SelectTrigger className="w-[150px]">
                    <SelectValue placeholder="Filter by severity" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">All Severities</SelectItem>
                    <SelectItem value="critical">Critical</SelectItem>
                    <SelectItem value="high">High</SelectItem>
                    <SelectItem value="medium">Medium</SelectItem>
                    <SelectItem value="low">Low</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {filteredNotifications.length === 0 ? (
                <div className="text-center py-12">
                  <Bell className="mx-auto h-12 w-12 text-muted-foreground" />
                  <h3 className="mt-4 text-lg font-medium">
                    No notifications found
                  </h3>
                  <p className="text-sm text-muted-foreground">
                    {searchQuery ||
                    typeFilter !== "all" ||
                    statusFilter !== "all"
                      ? "Try adjusting your filters"
                      : "You're all caught up!"}
                  </p>
                </div>
              ) : (
                <div className="space-y-2">
                  {filteredNotifications.map((notification) => (
                    <Card
                      key={notification.id}
                      className={`p-4 ${
                        notification.status === "unread"
                          ? "border-l-4 border-l-blue-500 bg-muted/50"
                          : ""
                      }`}
                    >
                      <div className="flex items-start gap-4">
                        <div className="flex-shrink-0 mt-1">
                          {notification.status === "unread" ? (
                            <BellRing className="h-5 w-5 text-blue-600" />
                          ) : (
                            <Bell className="h-5 w-5 text-muted-foreground" />
                          )}
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-start justify-between gap-2">
                            <div className="flex-1">
                              <div className="flex items-center gap-2">
                                {getTypeIcon(notification.type)}
                                <h4 className="font-medium">
                                  {notification.title}
                                </h4>
                              </div>
                              <p className="text-sm text-muted-foreground mt-1">
                                {notification.message}
                              </p>
                              <div className="flex items-center gap-2 mt-2">
                                <Badge
                                  className={getSeverityColor(
                                    notification.severity,
                                  )}
                                >
                                  {notification.severity}
                                </Badge>
                                <Badge
                                  variant="outline"
                                  className="flex items-center gap-1"
                                >
                                  {getChannelIcon(notification.channel)}
                                  {notification.channel}
                                </Badge>
                                <span className="text-xs text-muted-foreground">
                                  {getTimeAgo(notification.createdAt)}
                                </span>
                              </div>
                            </div>
                            <DropdownMenu>
                              <DropdownMenuTrigger asChild>
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  className="h-8 w-8"
                                >
                                  <X className="h-4 w-4" />
                                </Button>
                              </DropdownMenuTrigger>
                              <DropdownMenuContent align="end">
                                <DropdownMenuLabel>Actions</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                {notification.status === "unread" ? (
                                  <DropdownMenuItem
                                    onClick={() => markAsRead(notification.id)}
                                  >
                                    <Check className="mr-2 h-4 w-4" />
                                    Mark as Read
                                  </DropdownMenuItem>
                                ) : (
                                  <DropdownMenuItem
                                    onClick={() =>
                                      markAsUnread(notification.id)
                                    }
                                  >
                                    <Bell className="mr-2 h-4 w-4" />
                                    Mark as Unread
                                  </DropdownMenuItem>
                                )}
                                <DropdownMenuItem
                                  onClick={() =>
                                    archiveNotification(notification.id)
                                  }
                                >
                                  <Archive className="mr-2 h-4 w-4" />
                                  Archive
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem
                                  className="text-destructive"
                                  onClick={() =>
                                    deleteNotification(notification.id)
                                  }
                                >
                                  <Trash2 className="mr-2 h-4 w-4" />
                                  Delete
                                </DropdownMenuItem>
                              </DropdownMenuContent>
                            </DropdownMenu>
                          </div>
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="history" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Notification History</CardTitle>
              <CardDescription>
                View and manage all past notifications
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Type</TableHead>
                    <TableHead>Title</TableHead>
                    <TableHead>Severity</TableHead>
                    <TableHead>Channel</TableHead>
                    <TableHead>Status</TableHead>
                    <TableHead>Date</TableHead>
                    <TableHead className="w-[50px]"></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {notifications.map((notification) => (
                    <TableRow key={notification.id}>
                      <TableCell>
                        <div className="flex items-center gap-2">
                          {getTypeIcon(notification.type)}
                          <span className="capitalize">
                            {notification.type.replace(/_/g, " ")}
                          </span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="font-medium">{notification.title}</div>
                        <div className="text-sm text-muted-foreground">
                          {notification.message}
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge
                          className={getSeverityColor(notification.severity)}
                        >
                          {notification.severity}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <Badge
                          variant="outline"
                          className="flex items-center gap-1"
                        >
                          {getChannelIcon(notification.channel)}
                          {notification.channel}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <Badge
                          className={
                            notification.status === "unread"
                              ? "bg-blue-500/10 text-blue-700 dark:text-blue-400"
                              : notification.status === "read"
                                ? "bg-green-500/10 text-green-700 dark:text-green-400"
                                : "bg-gray-500/10 text-gray-700 dark:text-gray-400"
                          }
                        >
                          {notification.status}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <div className="text-sm">
                          {format(
                            new Date(notification.createdAt),
                            "MMM dd, yyyy HH:mm",
                          )}
                        </div>
                      </TableCell>
                      <TableCell>
                        <DropdownMenu>
                          <DropdownMenuTrigger asChild>
                            <Button variant="ghost" size="icon">
                              <X className="h-4 w-4" />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent align="end">
                            <DropdownMenuItem
                              onClick={() =>
                                deleteNotification(notification.id)
                              }
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

        <TabsContent value="settings" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Notification Settings</CardTitle>
              <CardDescription>
                Configure how you receive compliance notifications
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-6">
              <div>
                <h4 className="font-medium mb-3">Delivery Channels</h4>
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Mail className="h-4 w-4" />
                      <Label>Email Notifications</Label>
                    </div>
                    <Switch
                      checked={preferences.email}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Monitor className="h-4 w-4" />
                      <Label>In-App Notifications</Label>
                    </div>
                    <Switch
                      checked={preferences.inApp}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Smartphone className="h-4 w-4" />
                      <Label>Push Notifications</Label>
                    </div>
                    <Switch
                      checked={preferences.push}
                      onCheckedChange={() => {}}
                    />
                  </div>
                </div>
              </div>

              <Separator />

              <div>
                <h4 className="font-medium mb-3">Notification Types</h4>
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <Label>Key Expiration Alerts</Label>
                    <Switch
                      checked={preferences.keyExpiration}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Digest Verification Failures</Label>
                    <Switch
                      checked={preferences.digestVerification}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Report Generated</Label>
                    <Switch
                      checked={preferences.reportGenerated}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Schedule Failures</Label>
                    <Switch
                      checked={preferences.scheduleFailure}
                      onCheckedChange={() => {}}
                    />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label>Security Alerts</Label>
                    <Switch
                      checked={preferences.securityAlerts}
                      onCheckedChange={() => {}}
                    />
                  </div>
                </div>
              </div>

              <Separator />

              <div>
                <h4 className="font-medium mb-3">Delivery Frequency</h4>
                <Select value={preferences.frequency} onValueChange={() => {}}>
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="immediate">Immediate</SelectItem>
                    <SelectItem value="hourly">Hourly Digest</SelectItem>
                    <SelectItem value="daily">Daily Digest</SelectItem>
                    <SelectItem value="weekly">Weekly Digest</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator />

              <div>
                <h4 className="font-medium mb-3">Quiet Hours</h4>
                <div className="flex items-center justify-between mb-3">
                  <Label>Enable Quiet Hours</Label>
                  <Switch
                    checked={preferences.quietHours.enabled}
                    onCheckedChange={() => {}}
                  />
                </div>
                {preferences.quietHours.enabled && (
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <Label htmlFor="quietStart">Start Time</Label>
                      <Input
                        id="quietStart"
                        type="time"
                        value={preferences.quietHours.start}
                        onChange={() => {}}
                      />
                    </div>
                    <div>
                      <Label htmlFor="quietEnd">End Time</Label>
                      <Input
                        id="quietEnd"
                        type="time"
                        value={preferences.quietHours.end}
                        onChange={() => {}}
                      />
                    </div>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      <NotificationPreferencesModal
        open={isPreferenceModalOpen}
        onOpenChange={setIsPreferenceModalOpen}
        preferences={preferences}
        onSave={(updatedPrefs) => {
          setPreferences(updatedPrefs);
          setIsPreferenceModalOpen(false);
        }}
      />
    </div>
  );
}

function NotificationPreferencesModal({
  open,
  onOpenChange,
  preferences,
  onSave,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  preferences: NotificationPreferences;
  onSave: (preferences: NotificationPreferences) => void;
}) {
  const [formData, setFormData] = useState(preferences);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <DialogContent className="max-w-2xl">
      <DialogHeader>
        <DialogTitle>Notification Preferences</DialogTitle>
        <DialogDescription>
          Configure how you receive compliance notifications
        </DialogDescription>
      </DialogHeader>
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="space-y-4">
          <div>
            <h4 className="font-medium mb-3">Delivery Channels</h4>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Mail className="h-4 w-4" />
                  <Label>Email Notifications</Label>
                </div>
                <Switch
                  checked={formData.email}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, email: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Monitor className="h-4 w-4" />
                  <Label>In-App Notifications</Label>
                </div>
                <Switch
                  checked={formData.inApp}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, inApp: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Smartphone className="h-4 w-4" />
                  <Label>Push Notifications</Label>
                </div>
                <Switch
                  checked={formData.push}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, push: checked })
                  }
                />
              </div>
            </div>
          </div>

          <Separator />

          <div>
            <h4 className="font-medium mb-3">Notification Types</h4>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <Label>Key Expiration Alerts</Label>
                <Switch
                  checked={formData.keyExpiration}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, keyExpiration: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <Label>Digest Verification Failures</Label>
                <Switch
                  checked={formData.digestVerification}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, digestVerification: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <Label>Report Generated</Label>
                <Switch
                  checked={formData.reportGenerated}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, reportGenerated: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <Label>Schedule Failures</Label>
                <Switch
                  checked={formData.scheduleFailure}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, scheduleFailure: checked })
                  }
                />
              </div>
              <div className="flex items-center justify-between">
                <Label>Security Alerts</Label>
                <Switch
                  checked={formData.securityAlerts}
                  onCheckedChange={(checked) =>
                    setFormData({ ...formData, securityAlerts: checked })
                  }
                />
              </div>
            </div>
          </div>

          <Separator />

          <div>
            <h4 className="font-medium mb-3">Delivery Frequency</h4>
            <Select
              value={formData.frequency}
              onValueChange={(value) =>
                setFormData({ ...formData, frequency: value as any })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="immediate">Immediate</SelectItem>
                <SelectItem value="hourly">Hourly Digest</SelectItem>
                <SelectItem value="daily">Daily Digest</SelectItem>
                <SelectItem value="weekly">Weekly Digest</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <Separator />

          <div>
            <h4 className="font-medium mb-3">Quiet Hours</h4>
            <div className="flex items-center justify-between mb-3">
              <Label>Enable Quiet Hours</Label>
              <Switch
                checked={formData.quietHours.enabled}
                onCheckedChange={(checked) =>
                  setFormData({
                    ...formData,
                    quietHours: { ...formData.quietHours, enabled: checked },
                  })
                }
              />
            </div>
            {formData.quietHours.enabled && (
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label htmlFor="quietStart">Start Time</Label>
                  <Input
                    id="quietStart"
                    type="time"
                    value={formData.quietHours.start}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        quietHours: {
                          ...formData.quietHours,
                          start: e.target.value,
                        },
                      })
                    }
                  />
                </div>
                <div>
                  <Label htmlFor="quietEnd">End Time</Label>
                  <Input
                    id="quietEnd"
                    type="time"
                    value={formData.quietHours.end}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        quietHours: {
                          ...formData.quietHours,
                          end: e.target.value,
                        },
                      })
                    }
                  />
                </div>
              </div>
            )}
          </div>
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit">Save Preferences</Button>
        </DialogFooter>
      </form>
    </DialogContent>
  );
}
