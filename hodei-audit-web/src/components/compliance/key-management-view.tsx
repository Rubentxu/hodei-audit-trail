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
import { Badge } from '@/components/ui/badge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
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
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Key,
  KeyRound,
  Download,
  Eye,
  MoreVertical,
  Calendar,
  User,
  AlertTriangle,
  CheckCircle,
  Clock,
  Shield,
  RefreshCw,
  Lock,
  Copy,
} from 'lucide-react';

type KeyStatus = 'Active' | 'Inactive' | 'Expired' | 'Revoked' | 'Compromised';
type KeyAlgorithm = 'RSA-2048' | 'RSA-4096' | 'ECDSA-P256' | 'ECDSA-P384' | 'AES-256';

interface CryptographicKey {
  id: string;
  name: string;
  algorithm: KeyAlgorithm;
  status: KeyStatus;
  createdAt: string;
  expiresAt: string;
  lastUsed?: string;
  fingerprint: string;
  publicKey: string;
  rotationHistory: KeyRotation[];
  createdBy: string;
  purpose: string;
  keySize: string;
  keyUsage: string[];
  isHSM: boolean;
}

interface KeyRotation {
  id: string;
  rotatedAt: string;
  rotatedBy: string;
  reason: string;
  oldKeyId: string;
  newKeyId: string;
}

const mockKeys: CryptographicKey[] = [
  {
    id: 'key-001',
    name: 'Primary Signing Key',
    algorithm: 'RSA-4096',
    status: 'Active',
    createdAt: '2024-01-15 10:30:00',
    expiresAt: '2025-01-15 10:30:00',
    lastUsed: '2024-11-07 14:22:33',
    fingerprint: 'SHA256:1234:ABCD:5678:EFGH:9012:IJKL:3456:MNOP',
    publicKey: 'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...',
    rotationHistory: [
      {
        id: 'rot-001',
        rotatedAt: '2024-01-15 10:30:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Initial key generation',
        oldKeyId: '',
        newKeyId: 'key-001',
      },
      {
        id: 'rot-002',
        rotatedAt: '2024-06-15 09:15:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Scheduled rotation',
        oldKeyId: 'key-001',
        newKeyId: 'key-001',
      },
    ],
    createdBy: 'admin@acme.com',
    purpose: 'Digital signatures',
    keySize: '4096 bits',
    keyUsage: ['Sign', 'Verify'],
    isHSM: true,
  },
  {
    id: 'key-002',
    name: 'Encryption Key - Customer Data',
    algorithm: 'AES-256',
    status: 'Active',
    createdAt: '2024-03-20 11:45:00',
    expiresAt: '2025-03-20 11:45:00',
    lastUsed: '2024-11-07 16:45:12',
    fingerprint: 'SHA256:ABCD:1234:EFGH:5678:IJKL:9012:MNOP:3456',
    publicKey: 'N/A (Symmetric Key)',
    rotationHistory: [
      {
        id: 'rot-003',
        rotatedAt: '2024-03-20 11:45:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Initial key generation',
        oldKeyId: '',
        newKeyId: 'key-002',
      },
    ],
    createdBy: 'admin@acme.com',
    purpose: 'Data encryption at rest',
    keySize: '256 bits',
    keyUsage: ['Encrypt', 'Decrypt'],
    isHSM: true,
  },
  {
    id: 'key-003',
    name: 'Backup Verification Key',
    algorithm: 'ECDSA-P256',
    status: 'Inactive',
    createdAt: '2024-05-10 08:00:00',
    expiresAt: '2024-11-10 08:00:00',
    lastUsed: '2024-11-05 12:30:00',
    fingerprint: 'SHA256:EFGH:5678:IJKL:9012:MNOP:3456:QRST:UVWX',
    publicKey: 'MIIB...',
    rotationHistory: [
      {
        id: 'rot-004',
        rotatedAt: '2024-05-10 08:00:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Initial key generation',
        oldKeyId: '',
        newKeyId: 'key-003',
      },
    ],
    createdBy: 'admin@acme.com',
    purpose: 'Backup verification',
    keySize: '256 bits',
    keyUsage: ['Sign'],
    isHSM: false,
  },
  {
    id: 'key-004',
    name: 'API Authentication',
    algorithm: 'RSA-2048',
    status: 'Expired',
    createdAt: '2023-10-01 10:00:00',
    expiresAt: '2024-10-01 10:00:00',
    lastUsed: '2024-09-28 15:20:00',
    fingerprint: 'SHA256:IJKL:9012:MNOP:3456:QRST:UVWX:YZAB:CDEF',
    publicKey: 'MIIB...',
    rotationHistory: [
      {
        id: 'rot-005',
        rotatedAt: '2023-10-01 10:00:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Initial key generation',
        oldKeyId: '',
        newKeyId: 'key-004',
      },
    ],
    createdBy: 'admin@acme.com',
    purpose: 'API authentication',
    keySize: '2048 bits',
    keyUsage: ['Authenticate'],
    isHSM: false,
  },
  {
    id: 'key-005',
    name: 'Emergency Access Key',
    algorithm: 'ECDSA-P384',
    status: 'Compromised',
    createdAt: '2024-02-01 14:00:00',
    expiresAt: '2025-02-01 14:00:00',
    lastUsed: '2024-10-15 09:30:00',
    fingerprint: 'SHA256:MNOP:3456:QRST:UVWX:YZAB:CDEF:1234:ABCD',
    publicKey: 'MIIB...',
    rotationHistory: [
      {
        id: 'rot-006',
        rotatedAt: '2024-02-01 14:00:00',
        rotatedBy: 'admin@acme.com',
        reason: 'Initial key generation',
        oldKeyId: '',
        newKeyId: 'key-005',
      },
    ],
    createdBy: 'admin@acme.com',
    purpose: 'Emergency access',
    keySize: '384 bits',
    keyUsage: ['Sign', 'Verify'],
    isHSM: true,
  },
];

export function KeyManagementView() {
  const [keys, setKeys] = useState<CryptographicKey[]>(mockKeys);
  const [selectedKey, setSelectedKey] = useState<CryptographicKey | null>(null);
  const [isDetailsOpen, setIsDetailsOpen] = useState(false);
  const [isRotationOpen, setIsRotationOpen] = useState(false);
  const [rotationReason, setRotationReason] = useState('');
  const [isRotating, setIsRotating] = useState(false);

  const handleViewDetails = (key: CryptographicKey) => {
    setSelectedKey(key);
    setIsDetailsOpen(true);
  };

  const handleRotateKey = (key: CryptographicKey) => {
    setSelectedKey(key);
    setIsRotationOpen(true);
  };

  const confirmRotation = async () => {
    if (!selectedKey) return;

    setIsRotating(true);

    setTimeout(() => {
      const newKeyId = `key-${String(parseInt(selectedKey.id.split('-')[1]) + 1).padStart(3, '0')}`;
      const newFingerprint = selectedKey.fingerprint.replace(
        /[A-F0-9]{4}:/,
        (match, offset) => {
          const newValue = (parseInt(match.replace(':', ''), 16) + 1).toString(16).toUpperCase();
          return newValue.padStart(4, '0') + ':';
        }
      );

      const newRotation: KeyRotation = {
        id: `rot-${String(selectedKey.rotationHistory.length + 1).padStart(3, '0')}`,
        rotatedAt: new Date().toISOString().replace('T', ' ').substring(0, 19),
        rotatedBy: 'admin@acme.com',
        reason: rotationReason || 'Manual rotation',
        oldKeyId: selectedKey.id,
        newKeyId: newKeyId,
      };

      setKeys(
        keys.map((k) =>
          k.id === selectedKey.id
            ? {
                ...k,
                rotationHistory: [...k.rotationHistory, newRotation],
              }
            : k
        )
      );

      setIsRotationOpen(false);
      setIsRotating(false);
      setRotationReason('');
      setSelectedKey(null);
    }, 1500);
  };

  const getStatusBadge = (status: KeyStatus) => {
    const variants: Record<KeyStatus, 'success' | 'secondary' | 'destructive' | 'default'> = {
      'Active': 'success',
      'Inactive': 'secondary',
      'Expired': 'destructive',
      'Revoked': 'destructive',
      'Compromised': 'destructive',
    };

    const icons: Record<KeyStatus, React.ReactNode> = {
      'Active': <CheckCircle className="h-3 w-3" />,
      'Inactive': <Clock className="h-3 w-3" />,
      'Expired': <AlertTriangle className="h-3 w-3" />,
      'Revoked': <Lock className="h-3 w-3" />,
      'Compromised': <AlertTriangle className="h-3 w-3" />,
    };

    return (
      <Badge variant={variants[status]} className="flex items-center gap-1">
        {icons[status]}
        {status}
      </Badge>
    );
  };

  const getDaysUntilExpiration = (expiresAt: string) => {
    const today = new Date();
    const expiration = new Date(expiresAt);
    const diffTime = expiration.getTime() - today.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    return diffDays;
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const activeKeys = keys.filter((k) => k.status === 'Active').length;
  const expiringKeys = keys.filter(
    (k) => k.status === 'Active' && getDaysUntilExpiration(k.expiresAt) <= 30
  ).length;

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Key Management</h3>
          <p className="text-sm text-gray-600">
            Manage cryptographic keys and rotation schedules
          </p>
        </div>
        <Button>
          <Key className="h-4 w-4 mr-2" />
          Generate New Key
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">
              Total Keys
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{keys.length}</div>
            <p className="text-xs text-gray-600 mt-1">
              {activeKeys} active
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">
              Expiring Soon
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">
              {expiringKeys}
            </div>
            <p className="text-xs text-gray-600 mt-1">
              Within 30 days
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-gray-600">
              HSM Protected
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {keys.filter((k) => k.isHSM).length}
            </div>
            <p className="text-xs text-gray-600 mt-1">
              Hardware security module
            </p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Key Inventory</CardTitle>
          <CardDescription>
            All cryptographic keys in the system
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Key Name</TableHead>
                <TableHead>Algorithm</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Created</TableHead>
                <TableHead>Expires</TableHead>
                <TableHead>Purpose</TableHead>
                <TableHead>HSM</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {keys.map((key) => {
                const daysUntilExpiration = getDaysUntilExpiration(key.expiresAt);
                return (
                  <TableRow key={key.id}>
                    <TableCell>
                      <div className="font-medium">{key.name}</div>
                      <div className="text-xs text-gray-500">{key.id}</div>
                    </TableCell>
                    <TableCell>
                      <div>{key.algorithm}</div>
                      <div className="text-xs text-gray-500">{key.keySize}</div>
                    </TableCell>
                    <TableCell>{getStatusBadge(key.status)}</TableCell>
                    <TableCell>
                      <div className="flex items-center gap-1 text-sm">
                        <Calendar className="h-3 w-3 text-gray-400" />
                        {key.createdAt.split(' ')[0]}
                      </div>
                    </TableCell>
                    <TableCell>
                      <div className="flex items-center gap-1 text-sm">
                        <Calendar className="h-3 w-3 text-gray-400" />
                        {key.expiresAt.split(' ')[0]}
                      </div>
                      {key.status === 'Active' && (
                        <div className="text-xs text-gray-500">
                          {daysUntilExpiration > 0
                            ? `${daysUntilExpiration} days left`
                            : 'Expired'}
                        </div>
                      )}
                    </TableCell>
                    <TableCell>{key.purpose}</TableCell>
                    <TableCell>
                      {key.isHSM ? (
                        <Badge variant="outline" className="text-green-600">
                          <Shield className="h-3 w-3 mr-1" />
                          HSM
                        </Badge>
                      ) : (
                        <Badge variant="outline">Software</Badge>
                      )}
                    </TableCell>
                    <TableCell className="text-right">
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="ghost" size="sm">
                            <MoreVertical className="h-4 w-4" />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end">
                          <DropdownMenuLabel>Actions</DropdownMenuLabel>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem
                            onClick={() => handleViewDetails(key)}
                          >
                            <Eye className="h-4 w-4 mr-2" />
                            View Details
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => copyToClipboard(key.fingerprint)}
                          >
                            <Copy className="h-4 w-4 mr-2" />
                            Copy Fingerprint
                          </DropdownMenuItem>
                          <DropdownMenuItem>
                            <Download className="h-4 w-4 mr-2" />
                            Export Public Key
                          </DropdownMenuItem>
                          {key.status === 'Active' && (
                            <>
                              <DropdownMenuSeparator />
                              <DropdownMenuItem
                                onClick={() => handleRotateKey(key)}
                              >
                                <RefreshCw className="h-4 w-4 mr-2" />
                                Rotate Key
                              </DropdownMenuItem>
                            </>
                          )}
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      <Dialog open={isDetailsOpen} onOpenChange={setIsDetailsOpen}>
        <DialogContent className="max-w-3xl">
          <DialogHeader>
            <DialogTitle>Key Details</DialogTitle>
            <DialogDescription>
              {selectedKey?.name} ({selectedKey?.id})
            </DialogDescription>
          </DialogHeader>
          {selectedKey && (
            <Tabs defaultValue="details" className="w-full">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="details">Key Details</TabsTrigger>
                <TabsTrigger value="history">Rotation History</TabsTrigger>
              </TabsList>
              <TabsContent value="details" className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <Label className="text-sm font-medium">Algorithm</Label>
                    <p className="text-sm">{selectedKey.algorithm}</p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Key Size</Label>
                    <p className="text-sm">{selectedKey.keySize}</p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Status</Label>
                    <div className="mt-1">{getStatusBadge(selectedKey.status)}</div>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">HSM Protected</Label>
                    <p className="text-sm">
                      {selectedKey.isHSM ? 'Yes' : 'No'}
                    </p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Created By</Label>
                    <p className="text-sm">{selectedKey.createdBy}</p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Purpose</Label>
                    <p className="text-sm">{selectedKey.purpose}</p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Created At</Label>
                    <p className="text-sm">{selectedKey.createdAt}</p>
                  </div>
                  <div>
                    <Label className="text-sm font-medium">Expires At</Label>
                    <p className="text-sm">{selectedKey.expiresAt}</p>
                  </div>
                  {selectedKey.lastUsed && (
                    <div>
                      <Label className="text-sm font-medium">Last Used</Label>
                      <p className="text-sm">{selectedKey.lastUsed}</p>
                    </div>
                  )}
                </div>
                <div>
                  <Label className="text-sm font-medium">Key Usage</Label>
                  <div className="mt-1 flex flex-wrap gap-2">
                    {selectedKey.keyUsage.map((usage) => (
                      <Badge key={usage} variant="secondary">
                        {usage}
                      </Badge>
                    ))}
                  </div>
                </div>
                <div>
                  <Label className="text-sm font-medium">Fingerprint (SHA-256)</Label>
                  <div className="mt-1 p-3 bg-gray-100 rounded font-mono text-xs break-all">
                    {selectedKey.fingerprint}
                  </div>
                </div>
                <div>
                  <Label className="text-sm font-medium">Public Key</Label>
                  <div className="mt-1 p-3 bg-gray-100 rounded font-mono text-xs break-all">
                    {selectedKey.publicKey.substring(0, 100)}...
                  </div>
                </div>
              </TabsContent>
              <TabsContent value="history" className="space-y-4">
                <div className="space-y-3">
                  {selectedKey.rotationHistory.map((rotation, index) => (
                    <div
                      key={rotation.id}
                      className="p-4 border rounded-lg"
                    >
                      <div className="flex items-start justify-between">
                        <div className="space-y-1">
                          <div className="flex items-center gap-2">
                            <Badge variant="outline">
                              Rotation #{index + 1}
                            </Badge>
                            <span className="text-sm font-medium">
                              {rotation.reason}
                            </span>
                          </div>
                          <div className="text-sm text-gray-600">
                            <div className="flex items-center gap-1">
                              <User className="h-3 w-3" />
                              {rotation.rotatedBy}
                            </div>
                            <div className="flex items-center gap-1">
                              <Calendar className="h-3 w-3" />
                              {rotation.rotatedAt}
                            </div>
                          </div>
                        </div>
                        <KeyRound className="h-5 w-5 text-blue-600" />
                      </div>
                      {rotation.oldKeyId && (
                        <div className="mt-2 text-xs text-gray-600">
                          <span className="font-medium">Old Key:</span> {rotation.oldKeyId} →{' '}
                          <span className="font-medium">New Key:</span> {rotation.newKeyId}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </TabsContent>
            </Tabs>
          )}
        </DialogContent>
      </Dialog>

      <Dialog open={isRotationOpen} onOpenChange={setIsRotationOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Rotate Key</DialogTitle>
            <DialogDescription>
              Rotate cryptographic key: {selectedKey?.name}
            </DialogDescription>
          </DialogHeader>
          {isRotating ? (
            <div className="space-y-4 py-4">
              <div className="text-center">
                <RefreshCw className="h-8 w-8 animate-spin mx-auto text-blue-600" />
                <p className="mt-2 text-sm">Rotating key...</p>
              </div>
              <p className="text-xs text-center text-gray-500">
                Generating new key and updating references
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                <div className="flex items-start gap-2">
                  <AlertTriangle className="h-5 w-5 text-yellow-600 mt-0.5" />
                  <div>
                    <p className="text-sm font-medium text-yellow-800">
                      Key Rotation Warning
                    </p>
                    <p className="text-sm text-yellow-700 mt-1">
                      Rotating this key will generate a new key pair. All systems using
                      this key will need to be updated with the new public key.
                    </p>
                  </div>
                </div>
              </div>
              <div className="space-y-2">
                <Label htmlFor="rotation-reason">Reason for Rotation *</Label>
                <Input
                  id="rotation-reason"
                  placeholder="e.g., Scheduled rotation, Security incident, Compromised key"
                  value={rotationReason}
                  onChange={(e) => setRotationReason(e.target.value)}
                />
              </div>
            </div>
          )}
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setIsRotationOpen(false)}
              disabled={isRotating}
            >
              Cancel
            </Button>
            <Button
              onClick={confirmRotation}
              disabled={isRotating || !rotationReason}
            >
              <RefreshCw className="h-4 w-4 mr-2" />
              Rotate Key
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
