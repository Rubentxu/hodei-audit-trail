"use client"

import { DashboardLayout } from "@/components/layout"
import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { CheckCircle, ArrowRight, Palette, Code, Smartphone } from "lucide-react"

export default function SidebarDemo() {
  const [currentPage, setCurrentPage] = useState("dashboard")

  const features = [
    {
      icon: <Palette className="h-5 w-5" />,
      title: "Modern Design",
      description: "Beautiful gradient logo with smooth animations",
    },
    {
      icon: <Code className="h-5 w-5" />,
      title: "Easy to Use",
      description: "Simple API with DashboardLayout wrapper",
    },
    {
      icon: <Smartphone className="h-5 w-5" />,
      title: "Responsive",
      description: "Works perfectly on all screen sizes",
    },
    {
      icon: <CheckCircle className="h-5 w-5" />,
      title: "Active State",
      description: "Automatically highlights current page",
    },
  ]

  const codeExample = `import { DashboardLayout } from "@/components/layout"

export default function MyPage() {
  return (
    <DashboardLayout>
      <div>
        <h1>Dashboard</h1>
        {/* Your content */}
      </div>
    </DashboardLayout>
  )
}`

  return (
    <DashboardLayout>
      <div className="space-y-6">
        {/* Header */}
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Sidebar Demo</h1>
          <p className="text-muted-foreground">
            Explore the features of the new Hodei Audit sidebar
          </p>
        </div>

        {/* Feature Grid */}
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          {features.map((feature, index) => (
            <Card key={index}>
              <CardHeader className="pb-2">
                <div className="mb-2 text-blue-600">
                  {feature.icon}
                </div>
                <CardTitle className="text-lg">{feature.title}</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  {feature.description}
                </p>
              </CardContent>
            </Card>
          ))}
        </div>

        {/* Code Example */}
        <Card>
          <CardHeader>
            <CardTitle>Quick Start</CardTitle>
            <CardDescription>
              Get started with the sidebar in just a few lines of code
            </CardDescription>
          </CardHeader>
          <CardContent>
            <pre className="bg-gray-100 dark:bg-gray-800 p-4 rounded-lg overflow-x-auto text-sm">
              <code>{codeExample}</code>
            </pre>
          </CardContent>
        </Card>

        {/* Feature Showcase */}
        <Card>
          <CardHeader>
            <CardTitle>Key Features</CardTitle>
            <CardDescription>
              Everything you need for a modern navigation
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-start space-x-3">
              <CheckCircle className="h-5 w-5 text-green-500 mt-0.5" />
              <div>
                <p className="font-medium">Collapsible Design</p>
                <p className="text-sm text-muted-foreground">
                  Click the arrow to collapse or expand the sidebar
                </p>
              </div>
            </div>
            <div className="flex items-start space-x-3">
              <CheckCircle className="h-5 w-5 text-green-500 mt-0.5" />
              <div>
                <p className="font-medium">Active Page Highlighting</p>
                <p className="text-sm text-muted-foreground">
                  Current page is automatically highlighted with gradient
                </p>
              </div>
            </div>
            <div className="flex items-start space-x-3">
              <CheckCircle className="h-5 w-5 text-green-500 mt-0.5" />
              <div>
                <p className="font-medium">Smooth Animations</p>
                <p className="text-sm text-muted-foreground">
                  300ms transitions for all interactions
                </p>
              </div>
            </div>
            <div className="flex items-start space-x-3">
              <CheckCircle className="h-5 w-5 text-green-500 mt-0.5" />
              <div>
                <p className="font-medium">Hodei Audit Branding</p>
                <p className="text-sm text-muted-foreground">
                  Beautiful gradient logo with application name
                </p>
              </div>
            </div>
            <div className="flex items-start space-x-3">
              <CheckCircle className="h-5 w-5 text-green-500 mt-0.5" />
              <div>
                <p className="font-medium">Responsive Design</p>
                <p className="text-sm text-muted-foreground">
                  Works on desktop, tablet, and mobile devices
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Navigation Example */}
        <Card>
          <CardHeader>
            <CardTitle>Navigation Items</CardTitle>
            <CardDescription>
              The sidebar includes these navigation options
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {[
                { name: "Dashboard", path: "/dashboard", active: currentPage === "dashboard" },
                { name: "Events", path: "/events", active: currentPage === "events" },
                { name: "Analytics", path: "/analytics", active: currentPage === "analytics" },
                { name: "Compliance", path: "/compliance", active: currentPage === "compliance" },
                { name: "Settings", path: "/settings", active: currentPage === "settings" },
              ].map((item) => (
                <button
                  key={item.path}
                  onClick={() => setCurrentPage(item.name.toLowerCase())}
                  className={`w-full flex items-center justify-between px-4 py-3 rounded-lg transition-all ${
                    item.active
                      ? "bg-gradient-to-r from-blue-600 to-purple-600 text-white"
                      : "hover:bg-gray-100 dark:hover:bg-gray-800"
                  }`}
                >
                  <span className="font-medium">{item.name}</span>
                  {item.active && <ArrowRight className="h-4 w-4" />}
                </button>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Documentation Link */}
        <Card className="bg-gradient-to-r from-blue-600 to-purple-600 text-white">
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-xl font-bold mb-2">View Documentation</h3>
                <p className="text-blue-100">
                  Learn more about customization and advanced features
                </p>
              </div>
              <Button variant="secondary" className="bg-white text-blue-600 hover:bg-blue-50">
                Read Docs
                <ArrowRight className="ml-2 h-4 w-4" />
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </DashboardLayout>
  )
}
