"use client";

import { useState, useEffect } from "react";
import { getServerSession } from "next-auth";
import { useSession } from "next-auth/react";
import { redirect } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Plus, RefreshCw, LayoutGrid, Settings } from "lucide-react";
import { useAuth } from "@/hooks/use-auth";

interface WidgetConfig {
  id: string;
  type: string;
  title: string;
  position: { x: number; y: number; w: number; h: number };
}

const defaultWidgets: WidgetConfig[] = [
  {
    id: "event-count",
    type: "event-count",
    title: "Event Count",
    position: { x: 0, y: 0, w: 1, h: 1 },
  },
  {
    id: "critical-events",
    type: "critical-events",
    title: "Critical Events",
    position: { x: 1, y: 0, w: 1, h: 1 },
  },
  {
    id: "compliance-score",
    type: "compliance-score",
    title: "Compliance Score",
    position: { x: 2, y: 0, w: 1, h: 1 },
  },
  {
    id: "time-series",
    type: "time-series",
    title: "Events Over Time",
    position: { x: 0, y: 1, w: 2, h: 1 },
  },
  {
    id: "top-users",
    type: "top-users",
    title: "Top Users",
    position: { x: 2, y: 1, w: 1, h: 1 },
  },
];

export default function DashboardPage() {
  const { status } = useSession();
  const [widgets, setWidgets] = useState<WidgetConfig[]>(defaultWidgets);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);

  // Redirect to login if not authenticated
  if (status === "unauthenticated") {
    redirect("/auth/login?callbackUrl=/dashboard");
  }

  const handleRefresh = async () => {
    setIsRefreshing(true);
    // Simulate data refresh
    setTimeout(() => {
      setIsRefreshing(false);
    }, 1000);
  };

  const handleAddWidget = () => {
    // TODO: Open widget selection modal
    console.log("Add widget");
  };

  const handleCustomizeDashboard = () => {
    // TODO: Open dashboard customization
    console.log("Customize dashboard");
  };

  // Auto-refresh every 30 seconds
  useEffect(() => {
    if (!autoRefresh) return;

    const interval = setInterval(() => {
      handleRefresh();
    }, 30000);

    return () => clearInterval(interval);
  }, [autoRefresh]);

  if (status === "loading") {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <RefreshCw className="h-8 w-8 animate-spin mx-auto" />
          <p className="mt-2 text-gray-600">Loading dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Dashboard
          </h1>
          <p className="text-gray-600 dark:text-gray-400 mt-2">
            Real-time audit trail overview
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleRefresh}
            disabled={isRefreshing}
          >
            <RefreshCw
              className={`h-4 w-4 mr-2 ${isRefreshing ? "animate-spin" : ""}`}
            />
            Refresh
          </Button>
          <Button variant="outline" size="sm" onClick={handleAddWidget}>
            <Plus className="h-4 w-4 mr-2" />
            Add Widget
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleCustomizeDashboard}
          >
            <LayoutGrid className="h-4 w-4 mr-2" />
            Customize
          </Button>
        </div>
      </div>

      {/* Auto-refresh indicator */}
      <div className="mb-4 flex items-center justify-between">
        <div className="flex items-center space-x-2">
          <div
            className={`h-2 w-2 rounded-full ${autoRefresh ? "bg-green-500" : "bg-gray-400"}`}
          />
          <span className="text-sm text-gray-600">
            {autoRefresh ? "Auto-refresh: ON" : "Auto-refresh: OFF"}
          </span>
        </div>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setAutoRefresh(!autoRefresh)}
        >
          <Settings className="h-4 w-4 mr-2" />
          {autoRefresh ? "Disable" : "Enable"} Auto-refresh
        </Button>
      </div>

      {/* Widget Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {widgets.map((widget) => (
          <Card key={widget.id} className="col-span-1">
            <CardHeader className="pb-2">
              <CardTitle className="text-lg">{widget.title}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-center h-40 text-gray-400">
                <div className="text-center">
                  <LayoutGrid className="h-8 w-8 mx-auto mb-2" />
                  <p className="text-sm">{widget.type} widget</p>
                  <p className="text-xs text-gray-500 mt-1">
                    Widget placeholder
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
