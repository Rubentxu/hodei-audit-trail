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
import { Shield, FileText, Key, Settings } from "lucide-react";
import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { ReportsList } from "@/components/compliance/reports-list";

export default function CompliancePage() {
  const { data: session, status } = useSession();
  const router = useRouter();
  const [activeTab, setActiveTab] = useState("reports");

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
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="reports" className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            Reports
          </TabsTrigger>
          <TabsTrigger value="digests" className="flex items-center gap-2">
            <Shield className="h-4 w-4" />
            Digests
          </TabsTrigger>
          <TabsTrigger value="keys" className="flex items-center gap-2">
            <Key className="h-4 w-4" />
            Keys
          </TabsTrigger>
          <TabsTrigger value="settings" className="flex items-center gap-2">
            <Settings className="h-4 w-4" />
            Settings
          </TabsTrigger>
        </TabsList>

        <TabsContent value="reports" className="space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-2xl font-semibold">Compliance Reports</h2>
            {hasPermission("generate:reports") && (
              <Button>
                <FileText className="h-4 w-4 mr-2" />
                Generate Report
              </Button>
            )}
          </div>

          <ReportsList />
        </TabsContent>

        <TabsContent value="digests" className="space-y-4">
          <h2 className="text-2xl font-semibold">Digest Chain</h2>
          <Card>
            <CardHeader>
              <CardTitle>Data Integrity Verification</CardTitle>
              <CardDescription>
                Verify the integrity of your audit trail data using
                cryptographic hash chains
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-center py-8 text-gray-500">
                <Shield className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>Digest chain view will be implemented here</p>
                <p className="text-sm mt-2">Story 06.05 - Digest chain view</p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="keys" className="space-y-4">
          <h2 className="text-2xl font-semibold">Key Management</h2>
          <Card>
            <CardHeader>
              <CardTitle>Cryptographic Keys</CardTitle>
              <CardDescription>
                Manage encryption keys and rotation schedules
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-center py-8 text-gray-500">
                <Key className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>Key management interface will be implemented here</p>
                <p className="text-sm mt-2">
                  Story 06.07 - Key management section
                </p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="settings" className="space-y-4">
          <h2 className="text-2xl font-semibold">Compliance Settings</h2>
          <Card>
            <CardHeader>
              <CardTitle>Configuration</CardTitle>
              <CardDescription>
                Configure retention policies, encryption, and notification
                settings
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="text-center py-8 text-gray-500">
                <Settings className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>Compliance settings will be implemented here</p>
                <p className="text-sm mt-2">
                  Story 06.09 - Compliance settings
                </p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
