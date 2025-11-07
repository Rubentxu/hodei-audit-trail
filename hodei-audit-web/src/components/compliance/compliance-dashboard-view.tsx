"use client";

import { useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import {
  Shield,
  AlertTriangle,
  CheckCircle,
  Clock,
  Key,
  FileText,
  TrendingUp,
  TrendingDown,
  Activity,
  Calendar,
  RefreshCw,
  Eye,
  Download,
  MoreVertical,
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

interface ComplianceMetric {
  id: string;
  title: string;
  value: string | number;
  change: number;
  trend: "up" | "down" | "stable";
  status: "good" | "warning" | "critical";
  icon: React.ReactNode;
}

interface ComplianceAlert {
  id: string;
  type: "key" | "digest" | "report" | "security";
  severity: "low" | "medium" | "high" | "critical";
  message: string;
  timestamp: string;
  actionRequired: boolean;
}

const mockMetrics: ComplianceMetric[] = [
  {
    id: "compliance-score",
    title: "Overall Compliance Score",
    value: "94%",
    change: 3,
    trend: "up",
    status: "good",
    icon: <Shield className="h-4 w-4" />,
  },
  {
    id: "keys-expiring",
    title: "Keys Expiring (30 days)",
    value: 2,
    change: 0,
    trend: "stable",
    status: "warning",
    icon: <Key className="h-4 w-4" />,
  },
  {
    id: "digest-verified",
    title: "Digests Verified",
    value: "100%",
    change: 0,
    trend: "stable",
    status: "good",
    icon: <CheckCircle className="h-4 w-4" />,
  },
  {
    id: "reports-pending",
    title: "Reports Pending",
    value: 1,
    change: 1,
    trend: "up",
    status: "warning",
    icon: <FileText className="h-4 w-4" />,
  },
];

const mockAlerts: ComplianceAlert[] = [
  {
    id: "alert-001",
    type: "key",
    severity: "medium",
    message: 'Key "Primary Signing Key" expires in 25 days',
    timestamp: "2024-11-07 14:30:00",
    actionRequired: true,
  },
  {
    id: "alert-002",
    type: "report",
    severity: "low",
    message: "SOC 2 report generation completed successfully",
    timestamp: "2024-11-07 11:15:00",
    actionRequired: false,
  },
  {
    id: "alert-003",
    type: "digest",
    severity: "critical",
    message: "Digest verification failed for period 2024-11-22 to 2024-11-28",
    timestamp: "2024-11-07 09:45:00",
    actionRequired: true,
  },
  {
    id: "alert-004",
    type: "security",
    severity: "high",
    message: "Multiple failed login attempts detected",
    timestamp: "2024-11-07 08:20:00",
    actionRequired: true,
  },
];

const auditActivityData = [
  { day: "Mon", events: 420 },
  { day: "Tue", events: 380 },
  { day: "Wed", events: 510 },
  { day: "Thu", events: 460 },
  { day: "Fri", events: 390 },
  { day: "Sat", events: 290 },
  { day: "Sun", events: 240 },
];

const complianceChecklist = [
  { id: 1, item: "All keys rotated within policy", status: "complete" },
  { id: 2, item: "Digest chain verified", status: "complete" },
  { id: 3, item: "SOC 2 report generated", status: "pending" },
  { id: 4, item: "Encryption enabled for all data", status: "complete" },
  { id: 5, item: "Audit logs immutable", status: "complete" },
  { id: 6, item: "Backup verification completed", status: "failed" },
];

export function ComplianceDashboardView() {
  const [refreshing, setRefreshing] = useState(false);

  const handleRefresh = async () => {
    setRefreshing(true);
    setTimeout(() => {
      setRefreshing(false);
    }, 1000);
  };

  const getStatusColor = (status: "good" | "warning" | "critical") => {
    switch (status) {
      case "good":
        return "text-green-600";
      case "warning":
        return "text-yellow-600";
      case "critical":
        return "text-red-600";
    }
  };

  const getAlertIcon = (type: ComplianceAlert["type"]) => {
    switch (type) {
      case "key":
        return <Key className="h-4 w-4" />;
      case "digest":
        return <Shield className="h-4 w-4" />;
      case "report":
        return <FileText className="h-4 w-4" />;
      case "security":
        return <AlertTriangle className="h-4 w-4" />;
    }
  };

  const getSeverityColor = (severity: ComplianceAlert["severity"]) => {
    switch (severity) {
      case "low":
        return "bg-blue-100 text-blue-800";
      case "medium":
        return "bg-yellow-100 text-yellow-800";
      case "high":
        return "bg-orange-100 text-orange-800";
      case "critical":
        return "bg-red-100 text-red-800";
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Compliance Dashboard</h3>
          <p className="text-sm text-gray-600">
            Real-time overview of compliance status and metrics
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleRefresh}
            disabled={refreshing}
          >
            <RefreshCw
              className={`h-4 w-4 mr-2 ${refreshing ? "animate-spin" : ""}`}
            />
            Refresh
          </Button>
          <Button size="sm">
            <Eye className="h-4 w-4 mr-2" />
            Full Dashboard
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {mockMetrics.map((metric) => (
          <Card key={metric.id}>
            <CardHeader className="pb-2">
              <div className="flex items-center justify-between">
                <CardTitle className="text-sm font-medium text-gray-600">
                  {metric.title}
                </CardTitle>
                <div
                  className={`p-2 rounded-lg bg-gray-100 ${getStatusColor(metric.status)}`}
                >
                  {metric.icon}
                </div>
              </div>
            </CardHeader>
            <CardContent>
              <div className="flex items-end justify-between">
                <div>
                  <div className="text-2xl font-bold">{metric.value}</div>
                  <div className="flex items-center gap-1 text-xs text-gray-600 mt-1">
                    {metric.trend === "up" ? (
                      <TrendingUp className="h-3 w-3 text-green-600" />
                    ) : metric.trend === "down" ? (
                      <TrendingDown className="h-3 w-3 text-red-600" />
                    ) : (
                      <Activity className="h-3 w-3 text-gray-600" />
                    )}
                    <span>
                      {metric.change > 0 ? "+" : ""}
                      {metric.change}%
                    </span>
                  </div>
                </div>
                {metric.status !== "good" && (
                  <Badge
                    variant="outline"
                    className={getStatusColor(metric.status)}
                  >
                    {metric.status}
                  </Badge>
                )}
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card>
          <CardHeader>
            <CardTitle>Compliance Score Trend</CardTitle>
            <CardDescription>
              Monthly compliance score over time
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm">Jun 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={87} className="h-2" />
                  <span className="text-sm font-medium">87%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Jul 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={89} className="h-2" />
                  <span className="text-sm font-medium">89%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Aug 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={91} className="h-2" />
                  <span className="text-sm font-medium">91%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Sep 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={90} className="h-2" />
                  <span className="text-sm font-medium">90%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Oct 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={92} className="h-2" />
                  <span className="text-sm font-medium">92%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Nov 2024</span>
                <div className="flex items-center gap-2 flex-1 mx-4">
                  <Progress value={94} className="h-2" />
                  <span className="text-sm font-medium">94%</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Report Status Distribution</CardTitle>
            <CardDescription>
              Current status of all compliance reports
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 rounded-full bg-green-500"></div>
                  <span className="text-sm">Verified</span>
                </div>
                <div className="flex items-center gap-2">
                  <Progress value={90} className="w-32 h-2" />
                  <span className="text-sm font-medium">145</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
                  <span className="text-sm">Pending</span>
                </div>
                <div className="flex items-center gap-2">
                  <Progress value={7} className="w-32 h-2" />
                  <span className="text-sm font-medium">12</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="w-3 h-3 rounded-full bg-red-500"></div>
                  <span className="text-sm">Failed</span>
                </div>
                <div className="flex items-center gap-2">
                  <Progress value={2} className="w-32 h-2" />
                  <span className="text-sm font-medium">3</span>
                </div>
              </div>
              <Separator />
              <div className="text-center">
                <p className="text-2xl font-bold">160</p>
                <p className="text-sm text-gray-600">Total Reports</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card>
          <CardHeader>
            <CardTitle>Recent Alerts</CardTitle>
            <CardDescription>
              Latest compliance-related alerts and notifications
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {mockAlerts.map((alert) => (
                <div
                  key={alert.id}
                  className="flex items-start gap-3 p-3 border rounded-lg"
                >
                  <div className="mt-0.5">{getAlertIcon(alert.type)}</div>
                  <div className="flex-1 space-y-1">
                    <div className="flex items-center gap-2">
                      <Badge className={getSeverityColor(alert.severity)}>
                        {alert.severity}
                      </Badge>
                      {alert.actionRequired && (
                        <Badge variant="outline">Action Required</Badge>
                      )}
                    </div>
                    <p className="text-sm">{alert.message}</p>
                    <p className="text-xs text-gray-500">{alert.timestamp}</p>
                  </div>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="sm">
                        <MoreVertical className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem>View Details</DropdownMenuItem>
                      <DropdownMenuItem>Mark as Read</DropdownMenuItem>
                      {alert.actionRequired && (
                        <>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem>Take Action</DropdownMenuItem>
                        </>
                      )}
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Compliance Checklist</CardTitle>
            <CardDescription>
              Key compliance items and their status
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {complianceChecklist.map((item) => (
                <div key={item.id} className="flex items-center gap-3">
                  {item.status === "complete" ? (
                    <CheckCircle className="h-5 w-5 text-green-600" />
                  ) : item.status === "failed" ? (
                    <AlertTriangle className="h-5 w-5 text-red-600" />
                  ) : (
                    <Clock className="h-5 w-5 text-yellow-600" />
                  )}
                  <div className="flex-1">
                    <p className="text-sm">{item.item}</p>
                    <Badge
                      variant="outline"
                      className={
                        item.status === "complete"
                          ? "text-green-600"
                          : item.status === "failed"
                            ? "text-red-600"
                            : "text-yellow-600"
                      }
                    >
                      {item.status}
                    </Badge>
                  </div>
                </div>
              ))}
            </div>
            <Separator className="my-4" />
            <div className="text-center">
              <Button variant="outline" size="sm">
                <Calendar className="h-4 w-4 mr-2" />
                View Full Checklist
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Audit Activity</CardTitle>
          <CardDescription>
            Number of audit events per day (last 7 days)
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {auditActivityData.map((item) => (
              <div key={item.day} className="flex items-center gap-3">
                <span className="text-sm w-12">{item.day}</span>
                <div className="flex-1">
                  <Progress value={(item.events / 510) * 100} className="h-2" />
                </div>
                <span className="text-sm font-medium w-16 text-right">
                  {item.events}
                </span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card>
          <CardHeader>
            <CardTitle>Upcoming Audits</CardTitle>
            <CardDescription>
              Scheduled compliance audits and reviews
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              <div className="p-3 border rounded-lg">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm font-medium">SOC 2 Type II Audit</p>
                    <p className="text-xs text-gray-600">
                      External auditor visit
                    </p>
                  </div>
                  <Badge variant="outline">Jan 15, 2025</Badge>
                </div>
              </div>
              <div className="p-3 border rounded-lg">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm font-medium">
                      PCI-DSS Quarterly Review
                    </p>
                    <p className="text-xs text-gray-600">
                      Internal compliance check
                    </p>
                  </div>
                  <Badge variant="outline">Dec 20, 2024</Badge>
                </div>
              </div>
              <div className="p-3 border rounded-lg">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm font-medium">
                      GDPR Data Protection Review
                    </p>
                    <p className="text-xs text-gray-600">Annual assessment</p>
                  </div>
                  <Badge variant="outline">Feb 01, 2025</Badge>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Quick Actions</CardTitle>
            <CardDescription>Common compliance tasks</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-3">
              <Button variant="outline" className="h-auto flex-col gap-2 py-4">
                <FileText className="h-5 w-5" />
                <span>Generate Report</span>
              </Button>
              <Button variant="outline" className="h-auto flex-col gap-2 py-4">
                <Shield className="h-5 w-5" />
                <span>Verify Digests</span>
              </Button>
              <Button variant="outline" className="h-auto flex-col gap-2 py-4">
                <Key className="h-5 w-5" />
                <span>Rotate Keys</span>
              </Button>
              <Button variant="outline" className="h-auto flex-col gap-2 py-4">
                <Download className="h-5 w-5" />
                <span>Export Data</span>
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
