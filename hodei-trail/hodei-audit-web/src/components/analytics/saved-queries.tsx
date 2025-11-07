'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
  Save,
  Search,
  Clock,
  Trash2,
  MoreVertical,
  Star,
  Copy,
  Share2,
  Play
} from 'lucide-react';

export interface SavedQuery {
  id: string;
  name: string;
  description: string;
  type: 'visual' | 'sql';
  query: any;
  createdAt: string;
  updatedAt: string;
  isFavorite: boolean;
  tags: string[];
  shared: boolean;
}

const mockSavedQueries: SavedQuery[] = [
  {
    id: 'q1',
    name: 'Critical Events This Week',
    description: 'All critical events from the past 7 days with status breakdown',
    type: 'visual',
    query: { filters: [], groups: [], sorts: [] },
    createdAt: '2024-11-01',
    updatedAt: '2024-11-05',
    isFavorite: true,
    tags: ['critical', 'events', 'weekly'],
    shared: false,
  },
  {
    id: 'q2',
    name: 'User Activity Analysis',
    description: 'Analysis of user activity with top performers',
    type: 'sql',
    query: 'SELECT user, COUNT(*) as events FROM events GROUP BY user ORDER BY events DESC LIMIT 10',
    createdAt: '2024-10-28',
    updatedAt: '2024-11-03',
    isFavorite: false,
    tags: ['users', 'activity', 'analysis'],
    shared: true,
  },
  {
    id: 'q3',
    name: 'Failed Login Attempts',
    description: 'Track failed login attempts by source and time',
    type: 'visual',
    query: { filters: [{ field: 'status', operator: 'equals', value: 'failure' }] },
    createdAt: '2024-10-25',
    updatedAt: '2024-10-30',
    isFavorite: true,
    tags: ['security', 'login', 'failures'],
    shared: false,
  },
];

interface SavedQueriesManagerProps {
  onLoadQuery: (query: SavedQuery) => void;
  onDeleteQuery: (id: string) => void;
}

export function SavedQueriesManager({ onLoadQuery, onDeleteQuery }: SavedQueriesManagerProps) {
  const [savedQueries, setSavedQueries] = useState<SavedQuery[]>(mockSavedQueries);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedQuery, setSelectedQuery] = useState<SavedQuery | null>(null);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);

  const filteredQueries = savedQueries.filter(query =>
    query.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    query.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
    query.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  const handleDelete = (query: SavedQuery) => {
    setSelectedQuery(query);
    setShowDeleteDialog(true);
  };

  const confirmDelete = () => {
    if (selectedQuery) {
      setSavedQueries(prev => prev.filter(q => q.id !== selectedQuery.id));
      onDeleteQuery(selectedQuery.id);
      setShowDeleteDialog(false);
      setSelectedQuery(null);
    }
  };

  const toggleFavorite = (id: string) => {
    setSavedQueries(prev =>
      prev.map(q => q.id === id ? { ...q, isFavorite: !q.isFavorite } : q)
    );
  };

  const handleShare = (query: SavedQuery) => {
    // In a real app, this would generate a shareable link
    const shareUrl = `${window.location.origin}/analytics/shared/${query.id}`;
    navigator.clipboard.writeText(shareUrl);
    alert('Share link copied to clipboard!');
  };

  return (
    <>
      <div className="space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-bold">Saved Queries</h2>
            <p className="text-gray-600 mt-1">
              Manage and organize your saved queries
            </p>
          </div>
          <Badge variant="secondary">
            {savedQueries.length} saved
          </Badge>
        </div>

        {/* Search */}
        <div className="relative">
          <Search className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
          <Input
            placeholder="Search queries by name, description, or tags..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10"
          />
        </div>

        {/* Filters */}
        <div className="flex space-x-2">
          <Button variant="outline" size="sm">
            <Star className="h-4 w-4 mr-2" />
            Favorites
          </Button>
          <Button variant="outline" size="sm">
            <Clock className="h-4 w-4 mr-2" />
            Recent
          </Button>
          <Button variant="outline" size="sm">
            <Share2 className="h-4 w-4 mr-2" />
            Shared
          </Button>
        </div>

        {/* Queries List */}
        <div className="space-y-4">
          {filteredQueries.map((query) => (
            <Card key={query.id} className="hover:border-blue-500 transition-colors">
              <CardHeader className="pb-3">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-2">
                      <CardTitle className="text-lg">{query.name}</CardTitle>
                      {query.isFavorite && (
                        <Star className="h-4 w-4 text-yellow-500 fill-yellow-500" />
                      )}
                      {query.shared && (
                        <Badge variant="outline" className="text-xs">
                          Shared
                        </Badge>
                      )}
                      <Badge variant={query.type === 'visual' ? 'default' : 'secondary'}>
                        {query.type}
                      </Badge>
                    </div>
                    <p className="text-sm text-gray-600 mt-1">
                      {query.description}
                    </p>
                  </div>
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="icon" className="h-8 w-8">
                        <MoreVertical className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem onClick={() => toggleFavorite(query.id)}>
                        <Star className="h-4 w-4 mr-2" />
                        {query.isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => onLoadQuery(query)}>
                        <Play className="h-4 w-4 mr-2" />
                        Run Query
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => handleShare(query)}>
                        <Share2 className="h-4 w-4 mr-2" />
                        Share
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => {
                        navigator.clipboard.writeText(JSON.stringify(query.query, null, 2));
                        alert('Query copied to clipboard!');
                      }}>
                        <Copy className="h-4 w-4 mr-2" />
                        Copy Query
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        onClick={() => handleDelete(query)}
                        className="text-red-600"
                      >
                        <Trash2 className="h-4 w-4 mr-2" />
                        Delete
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <div className="flex flex-wrap gap-1">
                    {query.tags.map((tag) => (
                      <Badge key={tag} variant="outline" className="text-xs">
                        {tag}
                      </Badge>
                    ))}
                  </div>
                  <div className="flex items-center justify-between text-xs text-gray-500">
                    <span>Created: {query.createdAt}</span>
                    <span>Updated: {query.updatedAt}</span>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}

          {filteredQueries.length === 0 && (
            <Card>
              <CardContent className="pt-6">
                <div className="text-center py-12 text-gray-500">
                  <Search className="h-12 w-12 mx-auto mb-4 opacity-50" />
                  <p>No saved queries found</p>
                  <p className="text-sm mt-1">Try adjusting your search or create a new query</p>
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      </div>

      {/* Delete Confirmation Dialog */}
      <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Query</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete "{selectedQuery?.name}"? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowDeleteDialog(false)}>
              Cancel
            </Button>
            <Button variant="destructive" onClick={confirmDelete}>
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}

export function QueryHistory() {
  const [history, setHistory] = useState<any[]>([
    { query: 'SELECT * FROM events WHERE status = "critical"', timestamp: '2024-11-07 10:45' },
    { query: 'SELECT user, COUNT(*) FROM events GROUP BY user', timestamp: '2024-11-07 10:30' },
    { query: 'SELECT * FROM events WHERE timestamp >= NOW() - INTERVAL 7 days', timestamp: '2024-11-07 09:15' },
  ]);

  const clearHistory = () => {
    setHistory([]);
  };

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center">
            <Clock className="h-5 w-5 mr-2" />
            Query History
          </CardTitle>
          <Button
            variant="ghost"
            size="sm"
            onClick={clearHistory}
            disabled={history.length === 0}
          >
            Clear
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        {history.length === 0 ? (
          <p className="text-gray-500 text-center py-8">No query history</p>
        ) : (
          <div className="space-y-2">
            {history.map((item, index) => (
              <div
                key={index}
                className="p-3 border rounded hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer"
                onClick={() => {
                  navigator.clipboard.writeText(item.query);
                  alert('Query copied to clipboard!');
                }}
              >
                <code className="text-sm">{item.query}</code>
                <p className="text-xs text-gray-500 mt-1">{item.timestamp}</p>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
