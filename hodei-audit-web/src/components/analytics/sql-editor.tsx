'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Textarea } from '@/components/ui/textarea';
import {
  Play,
  Save,
  RotateCcw,
  Copy,
  FileText,
  History,
  Lightbulb
} from 'lucide-react';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';

interface SQLEditorProps {
  onExecute?: (query: string) => void;
  onSave?: (name: string, query: string) => void;
}

const SAMPLE_QUERIES = [
  {
    name: 'Count Events by Status',
    query: `SELECT status, COUNT(*) as event_count
FROM events
GROUP BY status
ORDER BY event_count DESC;`
  },
  {
    name: 'Users with Most Events',
    query: `SELECT user, COUNT(*) as event_count
FROM events
WHERE timestamp >= NOW() - INTERVAL '7 days'
GROUP BY user
ORDER BY event_count DESC
LIMIT 10;`
  },
  {
    name: 'Critical Events This Week',
    query: `SELECT *
FROM events
WHERE status = 'critical'
  AND timestamp >= DATE_TRUNC('week', NOW())
ORDER BY timestamp DESC;`
  },
  {
    name: 'Average Events Per Hour',
    query: `SELECT
  DATE_TRUNC('hour', timestamp) as hour,
  COUNT(*) as event_count
FROM events
WHERE timestamp >= NOW() - INTERVAL '24 hours'
GROUP BY hour
ORDER BY hour;`
  },
];

export function SQLEditor({ onExecute, onSave }: SQLEditorProps) {
  const [query, setQuery] = useState('');
  const [isExecuting, setIsExecuting] = useState(false);
  const [savedQueries, setSavedQueries] = useState<any[]>([]);
  const [queryHistory, setQueryHistory] = useState<string[]>([]);

  const handleExecute = async () => {
    if (!query.trim()) return;

    setIsExecuting(true);

    // Add to history
    setQueryHistory(prev => [query, ...prev.slice(0, 9)]);

    // Simulate query execution
    await new Promise(resolve => setTimeout(resolve, 1000));

    if (onExecute) {
      onExecute(query);
    }

    setIsExecuting(false);
  };

  const handleLoadSample = (sampleQuery: string) => {
    setQuery(sampleQuery);
  };

  const handleSave = () => {
    if (query.trim()) {
      const name = prompt('Enter a name for this query:');
      if (name && onSave) {
        onSave(name, query);
        setSavedQueries(prev => [{ name, query }, ...prev]);
      }
    }
  };

  const handleCopy = () => {
    if (query.trim()) {
      navigator.clipboard.writeText(query);
    }
  };

  const handleFormat = () => {
    // Simple SQL formatting - in a real app, use a SQL formatter
    const formatted = query
      .replace(/\bSELECT\b/gi, 'SELECT')
      .replace(/\bFROM\b/gi, '\nFROM')
      .replace(/\bWHERE\b/gi, '\nWHERE')
      .replace(/\bGROUP BY\b/gi, '\nGROUP BY')
      .replace(/\bORDER BY\b/gi, '\nORDER BY')
      .replace(/\bLIMIT\b/gi, '\nLIMIT');

    setQuery(formatted);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">SQL Query Editor</h2>
          <p className="text-gray-600 mt-1">
            Write custom SQL queries to analyze your data
          </p>
        </div>
        <div className="flex space-x-2">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="outline" onClick={handleFormat}>
                  <FileText className="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Format SQL</TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button variant="outline" onClick={handleCopy} disabled={!query.trim()}>
                  <Copy className="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Copy Query</TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <Button variant="outline" onClick={handleSave} disabled={!query.trim()}>
            <Save className="h-4 w-4 mr-2" />
            Save
          </Button>
          <Button onClick={handleExecute} disabled={!query.trim() || isExecuting}>
            <Play className="h-4 w-4 mr-2" />
            {isExecuting ? 'Executing...' : 'Execute'}
          </Button>
        </div>
      </div>

      {/* Query Editor */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>Query</CardTitle>
            <div className="flex space-x-2">
              <Badge variant="outline" className="text-xs">
                {query.split('\n').length} lines
              </Badge>
              <Badge variant="outline" className="text-xs">
                {query.length} chars
              </Badge>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <Textarea
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="SELECT * FROM events WHERE..."
            className="font-mono text-sm min-h-[300px]"
            spellCheck={false}
          />
          <div className="mt-4 flex items-center justify-between text-sm text-gray-500">
            <div className="flex items-center space-x-2">
              <Lightbulb className="h-4 w-4" />
              <span>Tip: Use Ctrl+Enter to execute query</span>
            </div>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setQuery('')}
            >
              <RotateCcw className="h-4 w-4 mr-2" />
              Clear
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Sample Queries */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Sample Queries</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            {SAMPLE_QUERIES.map((sample, index) => (
              <div
                key={index}
                className="flex items-center justify-between p-3 border rounded hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
                onClick={() => handleLoadSample(sample.query)}
              >
                <div>
                  <p className="font-medium">{sample.name}</p>
                  <p className="text-sm text-gray-500 font-mono">
                    {sample.query.split('\n')[0]}...
                  </p>
                </div>
                <Button variant="ghost" size="sm">
                  Load
                </Button>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Saved Queries */}
      {savedQueries.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Saved Queries</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {savedQueries.map((saved, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between p-3 border rounded"
                >
                  <div>
                    <p className="font-medium">{saved.name}</p>
                    <p className="text-sm text-gray-500 font-mono">
                      {saved.query.split('\n')[0]}...
                    </p>
                  </div>
                  <div className="flex space-x-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setQuery(saved.query)}
                    >
                      Load
                    </Button>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleLoadSample(saved.query)}
                    >
                      Execute
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Query History */}
      {queryHistory.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg flex items-center">
              <History className="h-5 w-5 mr-2" />
              Recent Queries
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {queryHistory.map((histQuery, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between p-2 border rounded hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
                  onClick={() => setQuery(histQuery)}
                >
                  <code className="text-sm font-mono truncate">
                    {histQuery.split('\n')[0]}
                  </code>
                  <Button variant="ghost" size="sm">
                    Load
                  </Button>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
