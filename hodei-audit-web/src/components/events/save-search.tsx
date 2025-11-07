'use client';

import { useState } from 'react';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Badge } from '@/components/ui/badge';
import {
  Save,
  Search as SearchIcon,
  Star,
  Trash2,
  MoreVertical,
  Clock
} from 'lucide-react';
import { FilterOptions } from './advanced-filter-panel';

export interface SavedSearch {
  id: string;
  name: string;
  description?: string;
  filters: FilterOptions;
  searchQuery: string;
  createdAt: string;
  isFavorite?: boolean;
}

interface SaveSearchDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (name: string, description?: string) => void;
  currentFilters: FilterOptions;
  currentSearchQuery: string;
}

export function SaveSearchDialog({
  isOpen,
  onClose,
  onSave,
  currentFilters,
  currentSearchQuery,
}: SaveSearchDialogProps) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');

  const handleSave = () => {
    if (name.trim()) {
      onSave(name, description);
      setName('');
      setDescription('');
      onClose();
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Save Search</DialogTitle>
          <DialogDescription>
            Save your current search filters for quick access later
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-4">
          <div className="space-y-2">
            <Label htmlFor="search-name">Name *</Label>
            <Input
              id="search-name"
              placeholder="e.g., Last Week's Critical Events"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="search-description">Description (optional)</Label>
            <Textarea
              id="search-description"
              placeholder="Add a description for this search..."
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button onClick={handleSave} disabled={!name.trim()}>
            <Save className="h-4 w-4 mr-2" />
            Save Search
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

// Mock saved searches - in a real app, this would come from an API
const mockSavedSearches: SavedSearch[] = [
  {
    id: 'search-1',
    name: 'Critical Events This Week',
    description: 'All critical events from the past 7 days',
    filters: {
      dateRange: { start: new Date('2024-11-01'), end: new Date('2024-11-07') },
      status: ['critical'],
      actions: [],
      users: [],
      sources: [],
    },
    searchQuery: 'critical',
    createdAt: '2024-11-05',
    isFavorite: true,
  },
  {
    id: 'search-2',
    name: 'Failed Logins',
    description: 'All failed login attempts',
    filters: {
      dateRange: { start: null, end: null },
      status: ['failure'],
      actions: ['LOGIN'],
      users: [],
      sources: [],
    },
    searchQuery: 'login failure',
    createdAt: '2024-11-04',
    isFavorite: false,
  },
];

interface SavedSearchesListProps {
  onLoadSearch: (search: SavedSearch) => void;
  onDeleteSearch: (id: string) => void;
}

export function SavedSearchesList({ onLoadSearch, onDeleteSearch }: SavedSearchesListProps) {
  const [savedSearches, setSavedSearches] = useState<SavedSearch[]>(mockSavedSearches);

  const handleDelete = (id: string) => {
    setSavedSearches(prev => prev.filter(s => s.id !== id));
    onDeleteSearch(id);
  };

  const toggleFavorite = (id: string) => {
    setSavedSearches(prev =>
      prev.map(s =>
        s.id === id ? { ...s, isFavorite: !s.isFavorite } : s
      )
    );
  };

  const formatFilters = (filters: FilterOptions) => {
    const parts: string[] = [];
    if (filters.status.length > 0) {
      parts.push(`Status: ${filters.status.join(', ')}`);
    }
    if (filters.actions.length > 0) {
      parts.push(`Actions: ${filters.actions.join(', ')}`);
    }
    if (filters.users.length > 0) {
      parts.push(`${filters.users.length} user(s)`);
    }
    return parts.join(' • ');
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold">Saved Searches</h3>
        <Badge variant="secondary">
          {savedSearches.length} saved
        </Badge>
      </div>

      <div className="space-y-2">
        {savedSearches.map((search) => (
          <Card
            key={search.id}
            className="hover:border-blue-500 cursor-pointer transition-colors"
            onClick={() => onLoadSearch(search)}
          >
            <CardHeader className="pb-3">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-2">
                    <CardTitle className="text-base">{search.name}</CardTitle>
                    {search.isFavorite && (
                      <Star className="h-4 w-4 text-yellow-500 fill-yellow-500" />
                    )}
                  </div>
                  {search.description && (
                    <CardDescription className="mt-1">
                      {search.description}
                    </CardDescription>
                  )}
                </div>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild onClick={(e) => e.stopPropagation()}>
                    <Button variant="ghost" size="icon" className="h-8 w-8">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem onClick={(e) => {
                      e.stopPropagation();
                      toggleFavorite(search.id);
                    }}>
                      <Star className="h-4 w-4 mr-2" />
                      {search.isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDelete(search.id);
                      }}
                      className="text-red-600"
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </CardHeader>
            <CardContent className="pt-0">
              <div className="space-y-2">
                {search.searchQuery && (
                  <div className="flex items-center text-sm text-gray-600">
                    <SearchIcon className="h-4 w-4 mr-2" />
                    <code className="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                      {search.searchQuery}
                    </code>
                  </div>
                )}
                <p className="text-xs text-gray-500">
                  {formatFilters(search.filters)}
                </p>
                <div className="flex items-center text-xs text-gray-500">
                  <Clock className="h-3 w-3 mr-1" />
                  Saved {search.createdAt}
                </div>
              </div>
            </CardContent>
          </Card>
        ))}

        {savedSearches.length === 0 && (
          <div className="text-center py-8 text-gray-500">
            <SearchIcon className="h-8 w-8 mx-auto mb-2 opacity-50" />
            <p>No saved searches yet</p>
            <p className="text-sm">Save your current search to access it quickly later</p>
          </div>
        )}
      </div>
    </div>
  );
}
