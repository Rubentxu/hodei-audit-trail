'use client';

import { useState } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { DashboardLayout } from '@/components/layout';
import { QueryBuilder } from '@/components/analytics/query-builder';
import { SQLEditor } from '@/components/analytics/sql-editor';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import {
  BarChart3,
  LineChart,
  PieChart,
  AreaChart,
  TrendingUp,
  Download
} from 'lucide-react';

export default function AnalyticsPage() {
  const [activeTab, setActiveTab] = useState('visual');
  const [queryResult, setQueryResult] = useState<any>(null);

  const handleRunQuery = (query: any) => {
    console.log('Running query:', query);
    // Simulate query result
    setQueryResult({
      rows: [
        { status: 'success', count: 12543 },
        { status: 'failure', count: 23 },
        { status: 'warning', count: 67 },
      ],
      totalRows: 3,
    });
  };

  const handleExecuteSQL = (sql: string) => {
    console.log('Executing SQL:', sql);
    // Simulate SQL execution
    setQueryResult({
      rows: [
        { user: 'john.doe', events: 1245 },
        { user: 'jane.smith', events: 987 },
        { user: 'bob.johnson', events: 743 },
      ],
      totalRows: 3,
    });
  };

  const handleSaveQuery = (name: string) => {
    console.log('Saving query:', name);
  };

  const handleSaveSQL = (name: string, query: string) => {
    console.log('Saving SQL:', name, query);
  };

  return (
    <DashboardLayout>
      <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Analytics & Query Builder</h1>
        <p className="text-gray-600 dark:text-gray-400 mt-2">
          Build custom queries and analyze your audit data
        </p>
      </div>

      {/* Query Builder and SQL Editor Tabs */}
      <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
        <TabsList className="grid w-full max-w-md grid-cols-2">
          <TabsTrigger value="visual">
            <BarChart3 className="h-4 w-4 mr-2" />
            Visual Query Builder
          </TabsTrigger>
          <TabsTrigger value="sql">
            <LineChart className="h-4 w-4 mr-2" />
            SQL Editor
          </TabsTrigger>
        </TabsList>

        <TabsContent value="visual" className="space-y-6">
          <QueryBuilder onRun={handleRunQuery} onSave={handleSaveQuery} />
        </TabsContent>

        <TabsContent value="sql" className="space-y-6">
          <SQLEditor onExecute={handleExecuteSQL} onSave={handleSaveSQL} />
        </TabsContent>
      </Tabs>

      {/* Query Results */}
      {queryResult && (
        <div className="mt-8 space-y-6">
          <div className="flex items-center justify-between">
            <h2 className="text-2xl font-bold">Query Results</h2>
            <div className="flex space-x-2">
              <Badge variant="secondary">
                {queryResult.totalRows} rows
              </Badge>
              <Button variant="outline" size="sm">
                <Download className="h-4 w-4 mr-2" />
                Export
              </Button>
            </div>
          </div>

          {/* Results Table */}
          <Card>
            <CardHeader>
              <CardTitle>Data</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="rounded-md border">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b">
                      {queryResult.rows.length > 0 &&
                        Object.keys(queryResult.rows[0]).map((key) => (
                          <th key={key} className="text-left p-3 font-semibold">
                            {key}
                          </th>
                        ))}
                    </tr>
                  </thead>
                  <tbody>
                    {queryResult.rows.map((row: any, index: number) => (
                      <tr key={index} className="border-b hover:bg-gray-50">
                        {Object.values(row).map((value: any, i) => (
                          <td key={i} className="p-3">
                            {value}
                          </td>
                        ))}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </CardContent>
          </Card>

          {/* Visualization Options */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <Card className="cursor-pointer hover:border-blue-500 transition-colors">
              <CardContent className="pt-6">
                <div className="flex flex-col items-center space-y-2">
                  <BarChart3 className="h-8 w-8 text-blue-600" />
                  <p className="font-medium">Bar Chart</p>
                </div>
              </CardContent>
            </Card>
            <Card className="cursor-pointer hover:border-blue-500 transition-colors">
              <CardContent className="pt-6">
                <div className="flex flex-col items-center space-y-2">
                  <LineChart className="h-8 w-8 text-green-600" />
                  <p className="font-medium">Line Chart</p>
                </div>
              </CardContent>
            </Card>
            <Card className="cursor-pointer hover:border-blue-500 transition-colors">
              <CardContent className="pt-6">
                <div className="flex flex-col items-center space-y-2">
                  <PieChart className="h-8 w-8 text-purple-600" />
                  <p className="font-medium">Pie Chart</p>
                </div>
              </CardContent>
            </Card>
            <Card className="cursor-pointer hover:border-blue-500 transition-colors">
              <CardContent className="pt-6">
                <div className="flex flex-col items-center space-y-2">
                  <AreaChart className="h-8 w-8 text-orange-600" />
                  <p className="font-medium">Area Chart</p>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      )}
      </div>
    </DashboardLayout>
  );
}
