'use client';

import { useSession } from 'next-auth/react';
import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Session, RefreshCw, Clock, CheckCircle, XCircle } from 'lucide-react';

interface SessionInfo {
  expires: string;
  isActive: boolean;
  timeRemaining: string;
  lastActivity: string;
}

export function SessionManager() {
  const { data: session, status, update } = useSession();
  const [sessionInfo, setSessionInfo] = useState<SessionInfo | null>(null);
  const [isRefreshing, setIsRefreshing] = useState(false);

  useEffect(() => {
    if (session?.expires) {
      const expiresAt = new Date(session.expires);
      const now = new Date();
      const timeRemaining = expiresAt.getTime() - now.getTime();

      setSessionInfo({
        expires: session.expires,
        isActive: timeRemaining > 0,
        timeRemaining: formatTimeRemaining(timeRemaining),
        lastActivity: 'Just now',
      });

      // Update countdown every second
      const interval = setInterval(() => {
        const newTimeRemaining = expiresAt.getTime() - Date.now();
        if (newTimeRemaining > 0) {
          setSessionInfo(prev => prev ? {
            ...prev,
            timeRemaining: formatTimeRemaining(newTimeRemaining),
          } : null);
        } else {
          setSessionInfo(prev => prev ? {
            ...prev,
            isActive: false,
            timeRemaining: 'Expired',
          } : null);
        }
      }, 1000);

      return () => clearInterval(interval);
    }
  }, [session?.expires]);

  const handleRefreshSession = async () => {
    setIsRefreshing(true);
    try {
      await update();
    } catch (error) {
      console.error('Failed to refresh session:', error);
    } finally {
      setIsRefreshing(false);
    }
  };

  const formatTimeRemaining = (ms: number): string => {
    const minutes = Math.floor(ms / 60000);
    const seconds = Math.floor((ms % 60000) / 1000);
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  if (!session) {
    return (
      <Card>
        <CardContent className="pt-6">
          <p className="text-center text-gray-500">Not authenticated</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Session className="h-5 w-5" />
          <span>Session Management</span>
        </CardTitle>
        <CardDescription>
          Monitor and manage your current session
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Session Status */}
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">Status</span>
          <div className="flex items-center space-x-2">
            {sessionInfo?.isActive ? (
              <>
                <CheckCircle className="h-4 w-4 text-green-500" />
                <Badge variant="outline" className="text-green-700 border-green-200">
                  Active
                </Badge>
              </>
            ) : (
              <>
                <XCircle className="h-4 w-4 text-red-500" />
                <Badge variant="destructive">Expired</Badge>
              </>
            )}
          </div>
        </div>

        {/* Time Remaining */}
        {sessionInfo && (
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium flex items-center space-x-1">
              <Clock className="h-4 w-4" />
              <span>Time Remaining</span>
            </span>
            <Badge variant={sessionInfo.isActive ? "default" : "destructive"}>
              {sessionInfo.timeRemaining}
            </Badge>
          </div>
        )}

        {/* Expires At */}
        {session?.expires && (
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium">Expires At</span>
            <span className="text-sm text-gray-600">
              {new Date(session.expires).toLocaleString()}
            </span>
          </div>
        )}

        {/* Refresh Session Button */}
        <div className="pt-4 border-t">
          <Button
            onClick={handleRefreshSession}
            disabled={isRefreshing || !sessionInfo?.isActive}
            className="w-full"
            variant="outline"
          >
            {isRefreshing ? (
              <>
                <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                Refreshing...
              </>
            ) : (
              <>
                <RefreshCw className="mr-2 h-4 w-4" />
                Refresh Session
              </>
            )}
          </Button>
          <p className="text-xs text-gray-500 mt-2 text-center">
            Extend your session by 15 minutes
          </p>
        </div>

        {/* Warning for Low Time Remaining */}
        {sessionInfo && sessionInfo.isActive && parseInt(sessionInfo.timeRemaining) < 5 && (
          <Alert variant="destructive">
            <XCircle className="h-4 w-4" />
            <AlertDescription>
              Your session will expire in less than 5 minutes. Please save your work and refresh your session.
            </AlertDescription>
          </Alert>
        )}
      </CardContent>
    </Card>
  );
}
