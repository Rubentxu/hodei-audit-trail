'use client';

import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Badge } from '@/components/ui/badge';
import {
  Share2,
  Link,
  Mail,
  Clock,
  CheckCircle,
  Calendar
} from 'lucide-react';

interface QuerySharingProps {
  query: any;
  onShare: (recipients: string[]) => void;
}

export function QuerySharing({ query, onShare }: QuerySharingProps) {
  const [showShareDialog, setShowShareDialog] = useState(false);
  const [shareLink, setShareLink] = useState('');
  const [emails, setEmails] = useState('');

  const generateShareLink = () => {
    const link = `${window.location.origin}/analytics/shared/${Date.now()}`;
    setShareLink(link);
    navigator.clipboard.writeText(link);
  };

  const handleEmailShare = () => {
    const recipientList = emails.split(',').map(e => e.trim()).filter(e => e);
    onShare(recipientList);
    setEmails('');
    setShowShareDialog(false);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center space-x-2">
          <Share2 className="h-5 w-5" />
          <span>Share Query</span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <Button onClick={generateShareLink} className="w-full">
          <Link className="h-4 w-4 mr-2" />
          Generate Share Link
        </Button>

        {shareLink && (
          <div className="p-3 bg-green-50 border border-green-200 rounded">
            <div className="flex items-center space-x-2 text-green-800">
              <CheckCircle className="h-4 w-4" />
              <span className="text-sm font-medium">Link copied to clipboard!</span>
            </div>
            <code className="text-xs mt-2 block">{shareLink}</code>
          </div>
        )}

        <Button
          variant="outline"
          onClick={() => setShowShareDialog(true)}
          className="w-full"
        >
          <Mail className="h-4 w-4 mr-2" />
          Share via Email
        </Button>

        <Dialog open={showShareDialog} onOpenChange={setShowShareDialog}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Share Query via Email</DialogTitle>
              <DialogDescription>
                Enter email addresses separated by commas
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="emails">Email Addresses</Label>
                <Input
                  id="emails"
                  placeholder="user1@example.com, user2@example.com"
                  value={emails}
                  onChange={(e) => setEmails(e.target.value)}
                />
              </div>
            </div>
            <DialogFooter>
              <Button variant="outline" onClick={() => setShowShareDialog(false)}>
                Cancel
              </Button>
              <Button onClick={handleEmailShare} disabled={!emails.trim()}>
                Share
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <div className="pt-4 border-t">
          <p className="text-xs text-gray-500">
            Shared queries are read-only and expire after 30 days
          </p>
        </div>
      </CardContent>
    </Card>
  );
}

interface ScheduledQuery {
  id: string;
  name: string;
  query: any;
  schedule: 'daily' | 'weekly' | 'monthly';
  time: string;
  recipients: string[];
  enabled: boolean;
  lastRun?: string;
  nextRun?: string;
}

const mockScheduledQueries: ScheduledQuery[] = [
  {
    id: '1',
    name: 'Daily Critical Events Report',
    query: { filters: [{ field: 'status', operator: 'equals', value: 'critical' }] },
    schedule: 'daily',
    time: '09:00',
    recipients: ['admin@acme.com'],
    enabled: true,
    lastRun: '2024-11-07 09:00',
    nextRun: '2024-11-08 09:00',
  },
];

interface QuerySchedulingProps {
  onSchedule: (config: any) => void;
}

export function QueryScheduling({ onSchedule }: QuerySchedulingProps) {
  const [scheduledQueries, setScheduledQueries] = useState<ScheduledQuery[]>(mockScheduledQueries);
  const [showScheduleDialog, setShowScheduleDialog] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    schedule: 'daily',
    time: '09:00',
    recipients: '',
  });

  const handleSchedule = () => {
    const config = {
      name: formData.name,
      schedule: formData.schedule,
      time: formData.time,
      recipients: formData.recipients.split(',').map(e => e.trim()).filter(e => e),
    };
    onSchedule(config);
    setShowScheduleDialog(false);
    setFormData({ name: '', schedule: 'daily', time: '09:00', recipients: '' });
  };

  const toggleSchedule = (id: string) => {
    setScheduledQueries(prev =>
      prev.map(sq => sq.id === id ? { ...sq, enabled: !sq.enabled } : sq)
    );
  };

  const deleteSchedule = (id: string) => {
    setScheduledQueries(prev => prev.filter(sq => sq.id !== id));
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center space-x-2">
                <Clock className="h-5 w-5" />
                <span>Scheduled Queries</span>
              </CardTitle>
              <p className="text-sm text-gray-600 mt-1">
                Automate query execution and reports
              </p>
            </div>
            <Button onClick={() => setShowScheduleDialog(true)}>
              <Calendar className="h-4 w-4 mr-2" />
              New Schedule
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {scheduledQueries.map((scheduled) => (
              <div key={scheduled.id} className="flex items-center justify-between p-4 border rounded">
                <div className="flex-1">
                  <div className="flex items-center space-x-2">
                    <h4 className="font-semibold">{scheduled.name}</h4>
                    <Badge variant={scheduled.enabled ? 'default' : 'secondary'}>
                      {scheduled.schedule}
                    </Badge>
                    <Badge variant="outline">{scheduled.time}</Badge>
                  </div>
                  <div className="mt-2 text-sm text-gray-600 space-y-1">
                    <p>Recipients: {scheduled.recipients.join(', ')}</p>
                    {scheduled.lastRun && (
                      <p>Last run: {scheduled.lastRun}</p>
                    )}
                    {scheduled.nextRun && (
                      <p>Next run: {scheduled.nextRun}</p>
                    )}
                  </div>
                </div>
                <div className="flex space-x-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => toggleSchedule(scheduled.id)}
                  >
                    {scheduled.enabled ? 'Disable' : 'Enable'}
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => deleteSchedule(scheduled.id)}
                    className="text-red-600"
                  >
                    Delete
                  </Button>
                </div>
              </div>
            ))}

            {scheduledQueries.length === 0 && (
              <p className="text-gray-500 text-center py-8">
                No scheduled queries. Create one to get started.
              </p>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Schedule Dialog */}
      <Dialog open={showScheduleDialog} onOpenChange={setShowScheduleDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Schedule Query</DialogTitle>
            <DialogDescription>
              Set up automatic query execution and email delivery
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="name">Schedule Name</Label>
              <Input
                id="name"
                placeholder="e.g., Weekly Security Report"
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              />
            </div>
            <div className="space-y-2">
              <Label>Frequency</Label>
              <Select
                value={formData.schedule}
                onValueChange={(value: any) => setFormData({ ...formData, schedule: value })}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="daily">Daily</SelectItem>
                  <SelectItem value="weekly">Weekly</SelectItem>
                  <SelectItem value="monthly">Monthly</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-2">
              <Label htmlFor="time">Time</Label>
              <Input
                id="time"
                type="time"
                value={formData.time}
                onChange={(e) => setFormData({ ...formData, time: e.target.value })}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="recipients">Recipients</Label>
              <Input
                id="recipients"
                placeholder="email1@example.com, email2@example.com"
                value={formData.recipients}
                onChange={(e) => setFormData({ ...formData, recipients: e.target.value })}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowScheduleDialog(false)}>
              Cancel
            </Button>
            <Button onClick={handleSchedule} disabled={!formData.name.trim()}>
              Schedule
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
