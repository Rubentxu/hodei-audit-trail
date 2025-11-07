'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { RefreshCw, MoreVertical } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { WidgetProps } from './types';

interface BaseWidgetProps extends WidgetProps {
  children: React.ReactNode;
}

export function BaseWidget({
  id,
  title,
  onRefresh,
  isRefreshing,
  children,
}: BaseWidgetProps) {
  return (
    <Card>
      <CardHeader className="pb-2 flex flex-row items-center justify-between space-y-0">
        <CardTitle className="text-lg">{title}</CardTitle>
        <div className="flex items-center space-x-2">
          {onRefresh && (
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={onRefresh}
              disabled={isRefreshing}
            >
              <RefreshCw className={`h-4 w-4 ${isRefreshing ? 'animate-spin' : ''}`} />
            </Button>
          )}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon" className="h-8 w-8">
                <MoreVertical className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem>View Details</DropdownMenuItem>
              <DropdownMenuItem>Export</DropdownMenuItem>
              <DropdownMenuItem>Refresh</DropdownMenuItem>
              <DropdownMenuItem className="text-red-600">Remove</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </CardHeader>
      <CardContent>{children}</CardContent>
    </Card>
  );
}
