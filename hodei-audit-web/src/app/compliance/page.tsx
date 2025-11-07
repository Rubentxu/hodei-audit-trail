"use client";

import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Shield, FileText, Key, Settings, Bell } from "lucide-react";
import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { ReportsList } from "@/components/compliance/reports-list";
import { GenerateReportModal } from "@/components/compliance/generate-report-modal";
import { TemplatesList } from "@/components/compliance/templates-list";
import { DigestChainView } from "@/components/compliance/digest-chain-view";
import { KeyManagementView } from "@/components/compliance/key-management-view";
import { ComplianceSettingsView } from "@/components/compliance/compliance-settings-view";
import { ComplianceDashboardView } from "@/components/compliance/compliance-dashboard-view";
import { AuditTrailView } from "@/components/compliance/audit-trail-view";
import { ReportScheduler } from "@/components/compliance/report-scheduler";
import { ComplianceNotificationsView } from "@/components/compliance/compliance-notifications-view";

export default function CompliancePage() {
  const { data: session, status } = useSession();
  const router = useRouter();
  const [activeTab, setActiveTab] = useState("reports");
  const [isGenerateModalOpen, setIsGenerateModalOpen] = useState(false);

  const handleGenerateReport = (config: any) => {
    console.log("Generating report with config:", config);
  };

  if (status === "loading") {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 mx-auto"></div>
          <p className="mt-2 text-sm text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

  if (!session) {
    router.push("/auth/login");
    return null;
  }

  const hasPermission = (permission: string) => {
    const rolePermissions: Record<string, string[]> = {
      admin: ["view:compliance", "manage:compliance"],
      auditor: ["view:compliance", "generate:reports"],
      analyst: ["view:compliance"],
      viewer: ["view:compliance"],
    };
    return rolePermissions[session.user.role]?.includes(permission) || false;
  };

  if (!hasPermission("view:compliance")) {
    return (
      <div className="flex items-center justify-center min-h-[400px]">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5" />
              Access Denied
            </CardTitle>
            <CardDescription>
              You do not have permission to view compliance reports.
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">
            Compliance & Reporting
          </h1>
          <p className="text-gray-600 mt-1">
            Generate and manage compliance reports, verify data integrity, and
            configure compliance settings.
          </p>
        </div>
        <Shield className="h-8 w-8 text-blue-600" />
      </div>

      <Tabs
        value={activeTab}
        onValueChange={setActiveTab}
        className="space-y-4"
      >
        <TabsList className="grid w-full grid-cols-8">
          <TabsTrigger value="dashboard" className="flex items-center gap-2">
            <Shield className="h-4 w-4" />
            Dashboard
          </TabsTrigger>
          <TabsTrigger value="reports" className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            Reports
          </TabsTrigger>
          <TabsTrigger value="schedule" className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            Schedule
          </TabsTrigger>
          <TabsTrigger value="digests" className="flex items-center gap-2">
            <Shield className="h-4 w-4" />
            Digests
          </TabsTrigger>
          <TabsTrigger value="keys" className="flex items-center gap-2">
            <Key className="h-4 w-4" />
            Keys
          </TabsTrigger>
          <TabsTrigger value="audit" className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            Audit Trail
          </TabsTrigger>
          <TabsTrigger
            value="notifications"
            className="flex items-center gap-2"
          >
            <Bell className="h-4 w-4" />
            Notifications
          </TabsTrigger>
          <TabsTrigger value="settings" className="flex items-center gap-2">
            <Settings className="h-4 w-4" />
            Settings
          </TabsTrigger>
        </TabsList>

        <TabsContent value="dashboard" className="space-y-4">
          <ComplianceDashboardView />
        </TabsContent>

        <TabsContent value="reports" className="space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-2xl font-semibold">Compliance Reports</h2>
            {hasPermission("generate:reports") && (
              <Button onClick={() => setIsGenerateModalOpen(true)}>
                <FileText className="h-4 w-4 mr-2" />
                Generate Report
              </Button>
            )}
          </div>

          <div className="space-y-6">
            <ReportsList />
            <TemplatesList />
          </div>
        </TabsContent>

        <TabsContent value="schedule" className="space-y-4">
          <ReportScheduler />
        </TabsContent>

        <TabsContent value="digests" className="space-y-4">
          <DigestChainView />
        </TabsContent>

        <TabsContent value="keys" className="space-y-4">
          <KeyManagementView />
        </TabsContent>

        <TabsContent value="audit" className="space-y-4">
          <AuditTrailView />
        </TabsContent>

        <TabsContent value="notifications" className="space-y-4">
          <ComplianceNotificationsView />
        </TabsContent>

        <TabsContent value="settings" className="space-y-4">
          <ComplianceSettingsView />
        </TabsContent>
      </Tabs>

      <GenerateReportModal
        open={isGenerateModalOpen}
        onOpenChange={setIsGenerateModalOpen}
        onGenerate={handleGenerateReport}
      />
    </div>
  );
}
