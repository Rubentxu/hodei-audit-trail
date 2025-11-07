'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Switch } from '@/components/ui/switch';
import {
  Settings,
  Save,
  RotateCcw,
  Shield,
  Clock,
  Bell,
  Lock,
  Database,
  Mail,
  AlertTriangle,
  CheckCircle,
  FileText,
  KeyRound,
  Server,
} from 'lucide-react';

interface ComplianceSettings {
  retention: {
    periodDays: number;
    storageTier: string;
    autoArchive: boolean;
    archiveAfterDays: number;
  };
  encryption: {
    enabled: boolean;
    algorithm: string;
    keyRotationDays: number;
    hsmRequired: boolean;
  };
  digitalSignature: {
    enabled: boolean;
    algorithm: string;
    timestampServer: string;
    ocspRequired: boolean;
  };
  compliance: {
    mode: 'strict' | 'standard' | 'relaxed';
    auditRetentionDays: number;
    immutableStorage: boolean;
    appendOnlyMode: boolean;
  };
  notifications: {
    enabled: boolean;
    emailRecipients: string;
    keyExpirationDays: number;
    digestFailureAlerts: boolean;
    reportGenerationAlerts: boolean;
    securityAlerts: boolean;
  };
  system: {
    logLevel: string;
    maxLogSize: number;
    compressionEnabled: boolean;
    backupFrequency: string;
  };
}

const defaultSettings: ComplianceSettings = {
  retention: {
    periodDays: 2555, // 7 years
    storageTier: 'standard',
    autoArchive: true,
    archiveAfterDays: 90,
  },
  encryption: {
    enabled: true,
    algorithm: 'AES-256-GCM',
    keyRotationDays: 90,
    hsmRequired: true,
  },
  digitalSignature: {
    enabled: true,
    algorithm: 'RSA-4096',
    timestampServer: 'https://tsa.example.com',
    ocspRequired: true,
  },
  compliance: {
    mode: 'standard',
    auditRetentionDays: 2555,
    immutableStorage: true,
    appendOnlyMode: true,
  },
  notifications: {
    enabled: true,
    emailRecipients: 'admin@acme.com, auditor@acme.com',
    keyExpirationDays: 30,
    digestFailureAlerts: true,
    reportGenerationAlerts: true,
    securityAlerts: true,
  },
  system: {
    logLevel: 'info',
    maxLogSize: 100,
    compressionEnabled: true,
    backupFrequency: 'daily',
  },
};

export function ComplianceSettingsView() {
  const [settings, setSettings] = useState<ComplianceSettings>(defaultSettings);
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);
  const [lastSaved, setLastSaved] = useState<Date | null>(null);

  const handleSettingChange = (category: keyof ComplianceSettings, key: string, value: any) => {
    setSettings((prev) => ({
      ...prev,
      [category]: {
        ...prev[category],
        [key]: value,
      },
    }));
    setHasChanges(true);
  };

  const handleSave = async () => {
    setIsSaving(true);
    setTimeout(() => {
      setIsSaving(false);
      setHasChanges(false);
      setLastSaved(new Date());
    }, 1000);
  };

  const handleReset = () => {
    setSettings(defaultSettings);
    setHasChanges(true);
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Compliance Settings</h3>
          <p className="text-sm text-gray-600">
            Configure retention policies, encryption, and notification settings
          </p>
        </div>
        <div className="flex items-center gap-2">
          {hasChanges && (
            <div className="flex items-center gap-2 text-sm text-orange-600">
              <AlertTriangle className="h-4 w-4" />
              <span>Unsaved changes</span>
            </div>
          )}
          {lastSaved && (
            <div className="text-sm text-gray-600">
              Last saved: {lastSaved.toLocaleTimeString()}
            </div>
          )}
          <Button variant="outline" onClick={handleReset}>
            <RotateCcw className="h-4 w-4 mr-2" />
            Reset
          </Button>
          <Button onClick={handleSave} disabled={!hasChanges || isSaving}>
            {isSaving ? (
              <div className="flex items-center gap-2">
                <div className="h-4 w-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                Saving...
              </div>
            ) : (
              <>
                <Save className="h-4 w-4 mr-2" />
                Save Changes
              </>
            )}
          </Button>
        </div>
      </div>

      <Tabs defaultValue="retention" className="space-y-4">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="retention" className="flex items-center gap-2">
            <Clock className="h-4 w-4" />
            Retention
          </TabsTrigger>
          <TabsTrigger value="encryption" className="flex items-center gap-2">
            <Lock className="h-4 w-4" />
            Encryption
          </TabsTrigger>
          <TabsTrigger value="compliance" className="flex items-center gap-2">
            <Shield className="h-4 w-4" />
            Compliance
          </TabsTrigger>
          <TabsTrigger value="notifications" className="flex items-center gap-2">
            <Bell className="h-4 w-4" />
            Notifications
          </TabsTrigger>
        </TabsList>

        <TabsContent value="retention" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Database className="h-5 w-5" />
                Data Retention
              </CardTitle>
              <CardDescription>
                Configure how long data is retained and archived
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="retention-period">
                  Retention Period (days)
                </Label>
                <Input
                  id="retention-period"
                  type="number"
                  value={settings.retention.periodDays}
                  onChange={(e) =>
                    handleSettingChange('retention', 'periodDays', parseInt(e.target.value))
                  }
                />
                <p className="text-xs text-gray-600">
                  How long to keep audit logs (default: 2555 days / 7 years)
                </p>
              </div>

              <div className="space-y-2">
                <Label htmlFor="storage-tier">Storage Tier</Label>
                <Select
                  value={settings.retention.storageTier}
                  onValueChange={(value) =>
                    handleSettingChange('retention', 'storageTier', value)
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="hot">Hot (Frequently Accessed)</SelectItem>
                    <SelectItem value="standard">Standard</SelectItem>
                    <SelectItem value="cold">Cold (Infrequent Access)</SelectItem>
                    <SelectItem value="archive">Archive (Long-term)</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="auto-archive"
                  checked={settings.retention.autoArchive}
                  onCheckedChange={(checked) =>
                    handleSettingChange('retention', 'autoArchive', checked)
                  }
                />
                <Label htmlFor="auto-archive">Enable automatic archiving</Label>
              </div>

              <div className="space-y-2">
                <Label htmlFor="archive-after">Archive After (days)</Label>
                <Input
                  id="archive-after"
                  type="number"
                  value={settings.retention.archiveAfterDays}
                  onChange={(e) =>
                    handleSettingChange('retention', 'archiveAfterDays', parseInt(e.target.value))
                  }
                  disabled={!settings.retention.autoArchive}
                />
                <p className="text-xs text-gray-600">
                  Move data to archive storage after this period
                </p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="encryption" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Lock className="h-5 w-5" />
                Encryption Settings
              </CardTitle>
              <CardDescription>
                Configure encryption and key management
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center space-x-2">
                <Switch
                  id="encryption-enabled"
                  checked={settings.encryption.enabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('encryption', 'enabled', checked)
                  }
                />
                <Label htmlFor="encryption-enabled">Enable encryption</Label>
              </div>

              <div className="space-y-2">
                <Label htmlFor="encryption-algo">Encryption Algorithm</Label>
                <Select
                  value={settings.encryption.algorithm}
                  onValueChange={(value) =>
                    handleSettingChange('encryption', 'algorithm', value)
                  }
                  disabled={!settings.encryption.enabled}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="AES-256-GCM">AES-256-GCM (Recommended)</SelectItem>
                    <SelectItem value="AES-256-CBC">AES-256-CBC</SelectItem>
                    <SelectItem value="ChaCha20-Poly1305">ChaCha20-Poly1305</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="key-rotation">Key Rotation (days)</Label>
                <Input
                  id="key-rotation"
                  type="number"
                  value={settings.encryption.keyRotationDays}
                  onChange={(e) =>
                    handleSettingChange('encryption', 'keyRotationDays', parseInt(e.target.value))
                  }
                  disabled={!settings.encryption.enabled}
                />
                <p className="text-xs text-gray-600">
                  Automatically rotate keys after this period
                </p>
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="hsm-required"
                  checked={settings.encryption.hsmRequired}
                  onCheckedChange={(checked) =>
                    handleSettingChange('encryption', 'hsmRequired', checked)
                  }
                  disabled={!settings.encryption.enabled}
                />
                <Label htmlFor="hsm-required">Require HSM (Hardware Security Module)</Label>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <KeyRound className="h-5 w-5" />
                Digital Signatures
              </CardTitle>
              <CardDescription>
                Configure digital signature settings
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center space-x-2">
                <Switch
                  id="signature-enabled"
                  checked={settings.digitalSignature.enabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('digitalSignature', 'enabled', checked)
                  }
                />
                <Label htmlFor="signature-enabled">Enable digital signatures</Label>
              </div>

              <div className="space-y-2">
                <Label htmlFor="signature-algo">Signature Algorithm</Label>
                <Select
                  value={settings.digitalSignature.algorithm}
                  onValueChange={(value) =>
                    handleSettingChange('digitalSignature', 'algorithm', value)
                  }
                  disabled={!settings.digitalSignature.enabled}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="RSA-4096">RSA-4096 (Recommended)</SelectItem>
                    <SelectItem value="RSA-2048">RSA-2048</SelectItem>
                    <SelectItem value="ECDSA-P384">ECDSA-P384</SelectItem>
                    <SelectItem value="ECDSA-P256">ECDSA-P256</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="timestamp-server">Timestamp Server URL</Label>
                <Input
                  id="timestamp-server"
                  value={settings.digitalSignature.timestampServer}
                  onChange={(e) =>
                    handleSettingChange('digitalSignature', 'timestampServer', e.target.value)
                  }
                  disabled={!settings.digitalSignature.enabled}
                />
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="ocsp-required"
                  checked={settings.digitalSignature.ocspRequired}
                  onCheckedChange={(checked) =>
                    handleSettingChange('digitalSignature', 'ocspRequired', checked)
                  }
                  disabled={!settings.digitalSignature.enabled}
                />
                <Label htmlFor="ocsp-required">Require OCSP validation</Label>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="compliance" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5" />
                Compliance Mode
              </CardTitle>
              <CardDescription>
                Select compliance strictness level
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="compliance-mode">Mode</Label>
                <Select
                  value={settings.compliance.mode}
                  onValueChange={(value) =>
                    handleSettingChange('compliance', 'mode', value)
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="strict">
                      <div>
                        <div className="font-medium">Strict</div>
                        <div className="text-xs text-gray-600">
                          Maximum security, all features enabled
                        </div>
                      </div>
                    </SelectItem>
                    <SelectItem value="standard">
                      <div>
                        <div className="font-medium">Standard (Recommended)</div>
                        <div className="text-xs text-gray-600">
                          Balanced security and usability
                        </div>
                      </div>
                    </SelectItem>
                    <SelectItem value="relaxed">
                      <div>
                        <div className="font-medium">Relaxed</div>
                        <div className="text-xs text-gray-600">
                          Minimum requirements for basic compliance
                        </div>
                      </div>
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label htmlFor="audit-retention">Audit Trail Retention (days)</Label>
                <Input
                  id="audit-retention"
                  type="number"
                  value={settings.compliance.auditRetentionDays}
                  onChange={(e) =>
                    handleSettingChange('compliance', 'auditRetentionDays', parseInt(e.target.value))
                  }
                />
                <p className="text-xs text-gray-600">
                  How long to keep compliance audit logs
                </p>
              </div>

              <div className="flex items-center space-x-2">
                <Switch
                  id="immutable-storage"
                  checked={settings.compliance.immutableStorage}
                  onCheckedChange={(checked) =>
                    handleSettingChange('compliance', 'immutableStorage', checked)
                  }
                />
                <Label htmlFor="immutable-storage">Enable immutable storage</Label>
              </div>

              <div className="flex items-center space-x-2">
                <Switch
                  id="append-only"
                  checked={settings.compliance.appendOnlyMode}
                  onCheckedChange={(checked) =>
                    handleSettingChange('compliance', 'appendOnlyMode', checked)
                  }
                />
                <Label htmlFor="append-only">Enable append-only mode</Label>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="notifications" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Bell className="h-5 w-5" />
                Notification Settings
              </CardTitle>
              <CardDescription>
                Configure alerts and notifications
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center space-x-2">
                <Switch
                  id="notifications-enabled"
                  checked={settings.notifications.enabled}
                  onCheckedChange={(checked) =>
                    handleSettingChange('notifications', 'enabled', checked)
                  }
                />
                <Label htmlFor="notifications-enabled">Enable notifications</Label>
              </div>

              <div className="space-y-2">
                <Label htmlFor="email-recipients">Email Recipients</Label>
                <Textarea
                  id="email-recipients"
                  value={settings.notifications.emailRecipients}
                  onChange={(e) =>
                    handleSettingChange('notifications', 'emailRecipients', e.target.value)
                  }
                  disabled={!settings.notifications.enabled}
                  placeholder="Enter email addresses separated by commas"
                  rows={3}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="key-expiration">
                  Key Expiration Alert (days before)
                </Label>
                <Input
                  id="key-expiration"
                  type="number"
                  value={settings.notifications.keyExpirationDays}
                  onChange={(e) =>
                    handleSettingChange('notifications', 'keyExpirationDays', parseInt(e.target.value))
                  }
                  disabled={!settings.notifications.enabled}
                />
                <p className="text-xs text-gray-600">
                  Alert when keys expire within this many days
                </p>
              </div>

              <div className="space-y-3">
                <Label>Alert Types</Label>
                <div className="space-y-2">
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="digest-failure"
                      checked={settings.notifications.digestFailureAlerts}
                      onCheckedChange={(checked) =>
                        handleSettingChange('notifications', 'digestFailureAlerts', checked)
                      }
                      disabled={!settings.notifications.enabled}
                    />
                    <Label htmlFor="digest-failure">Digest verification failures</Label>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="report-generation"
                      checked={settings.notifications.reportGenerationAlerts}
                      onCheckedChange={(checked) =>
                        handleSettingChange('notifications', 'reportGenerationAlerts', checked)
                      }
                      disabled={!settings.notifications.enabled}
                    />
                    <Label htmlFor="report-generation">Report generation complete</Label>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="security-alerts"
                      checked={settings.notifications.securityAlerts}
                      onCheckedChange={(checked) =>
                        handleSettingChange('notifications', 'securityAlerts', checked)
                      }
                      disabled={!settings.notifications.enabled}
                    />
                    <Label htmlFor="security-alerts">Security incidents</Label>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      <Card>
        <CardHeader>
          <CardTitle>System Information</CardTitle>
          <CardDescription>Additional system configuration</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label>Log Level</Label>
              <Select
                value={settings.system.logLevel}
                onValueChange={(value) => handleSettingChange('system', 'logLevel', value)}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="debug">Debug</SelectItem>
                  <SelectItem value="info">Info</SelectItem>
                  <SelectItem value="warn">Warning</SelectItem>
                  <SelectItem value="error">Error</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label htmlFor="max-log-size">Max Log Size (MB)</Label>
              <Input
                id="max-log-size"
                type="number"
                value={settings.system.maxLogSize}
                onChange={(e) =>
                  handleSettingChange('system', 'maxLogSize', parseInt(e.target.value))
                }
              />
            </div>

            <div className="space-y-2">
              <Label>Backup Frequency</Label>
              <Select
                value={settings.system.backupFrequency}
                onValueChange={(value) => handleSettingChange('system', 'backupFrequency', value)}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="hourly">Hourly</SelectItem>
                  <SelectItem value="daily">Daily</SelectItem>
                  <SelectItem value="weekly">Weekly</SelectItem>
                  <SelectItem value="monthly">Monthly</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="flex items-center space-x-2">
              <Switch
                id="compression"
                checked={settings.system.compressionEnabled}
                onCheckedChange={(checked) =>
                  handleSettingChange('system', 'compressionEnabled', checked)
                }
              />
              <Label htmlFor="compression">Enable log compression</Label>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
