'use client';

import { useState } from 'react';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Plus, Search, LayoutGrid, Trash2 } from 'lucide-react';
import { getAvailableWidgetTypes } from '@/components/widgets/registry';

interface WidgetItem {
  id: string;
  type: string;
  title: string;
}

interface WidgetManagementModalProps {
  isOpen: boolean;
  onClose: () => void;
  widgets: WidgetItem[];
  onAddWidget: (type: string) => void;
  onRemoveWidget: (id: string) => void;
}

export function WidgetManagementModal({
  isOpen,
  onClose,
  widgets,
  onAddWidget,
  onRemoveWidget,
}: WidgetManagementModalProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const availableWidgets = getAvailableWidgetTypes();

  const filteredWidgets = availableWidgets.filter((widget) =>
    widget.displayName.toLowerCase().includes(searchQuery.toLowerCase()) ||
    widget.description?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleAddWidget = (type: string) => {
    onAddWidget(type);
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[800px]">
        <DialogHeader>
          <DialogTitle>Manage Dashboard Widgets</DialogTitle>
          <DialogDescription>
            Add, remove, or customize your dashboard widgets
          </DialogDescription>
        </DialogHeader>

        <Tabs defaultValue="add" className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="add">Add Widgets</TabsTrigger>
            <TabsTrigger value="manage">Manage Widgets</TabsTrigger>
          </TabsList>

          <TabsContent value="add" className="space-y-4">
            <div className="relative">
              <Search className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
              <Input
                placeholder="Search widgets..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10"
              />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-96 overflow-y-auto">
              {filteredWidgets.map((widget) => (
                <div
                  key={widget.type}
                  className="border rounded-lg p-4 hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-950 transition-colors cursor-pointer"
                  onClick={() => handleAddWidget(widget.type)}
                >
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="font-semibold">{widget.displayName}</h3>
                      <p className="text-sm text-gray-600 mt-1">
                        {widget.description}
                      </p>
                    </div>
                    <Plus className="h-5 w-5 text-gray-400" />
                  </div>
                </div>
              ))}

              {filteredWidgets.length === 0 && (
                <div className="col-span-2 text-center py-8 text-gray-500">
                  No widgets found matching your search
                </div>
              )}
            </div>
          </TabsContent>

          <TabsContent value="manage" className="space-y-4">
            <div className="space-y-2 max-h-96 overflow-y-auto">
              {widgets.map((widget) => (
                <div
                  key={widget.id}
                  className="flex items-center justify-between p-3 border rounded-lg"
                >
                  <div className="flex items-center space-x-3">
                    <LayoutGrid className="h-5 w-5 text-gray-400" />
                    <div>
                      <p className="font-medium">{widget.title}</p>
                      <p className="text-sm text-gray-500">{widget.type}</p>
                    </div>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => onRemoveWidget(widget.id)}
                    className="text-red-600 hover:text-red-800"
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              ))}

              {widgets.length === 0 && (
                <div className="text-center py-8 text-gray-500">
                  No widgets on dashboard. Add some from the "Add Widgets" tab.
                </div>
              )}
            </div>
          </TabsContent>
        </Tabs>

        <div className="flex justify-end">
          <Button onClick={onClose}>Close</Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
