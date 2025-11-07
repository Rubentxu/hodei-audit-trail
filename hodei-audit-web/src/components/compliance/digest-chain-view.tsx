"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Progress } from "@/components/ui/progress";
import { Label } from "@/components/ui/label";
import {
  Shield,
  CheckCircle,
  XCircle,
  AlertTriangle,
  Download,
  Eye,
  MoreVertical,
  Lock,
  Link as LinkIcon,
  FileText,
  Copy,
  RefreshCw,
  GitBranch,
} from "lucide-react";

type DigestStatus = "Verified" | "Unverified" | "Invalid" | "Pending";

interface DigestEntry {
  id: string;
  period: string;
  eventsCount: number;
  hash: string;
  previousHash?: string;
  nextHash?: string;
  status: DigestStatus;
  createdAt: string;
  verifiedAt?: string;
  verifiedBy?: string;
  fileSize: string;
  chainPosition: number;
  algorithm: string;
}

const mockDigests: DigestEntry[] = [
  {
    id: "digest-2024-001",
    period: "2024-11-01 to 2024-11-07",
    eventsCount: 12543,
    hash: "a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4",
    previousHash: "0".repeat(64),
    nextHash: "b4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5",
    status: "Verified",
    createdAt: "2024-11-07 23:59:59",
    verifiedAt: "2024-11-08 00:05:12",
    verifiedBy: "admin@acme.com",
    fileSize: "2.4 MB",
    chainPosition: 1,
    algorithm: "SHA-256",
  },
  {
    id: "digest-2024-002",
    period: "2024-11-08 to 2024-11-14",
    eventsCount: 11876,
    hash: "b4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d8e",
    previousHash:
      "a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4",
    nextHash:
      "c5f7e9d0a1b3c4e5d6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0",
    status: "Verified",
    createdAt: "2024-11-14 23:59:59",
    verifiedAt: "2024-11-15 00:05:45",
    verifiedBy: "admin@acme.com",
    fileSize: "2.2 MB",
    chainPosition: 2,
    algorithm: "SHA-256",
  },
  {
    id: "digest-2024-003",
    period: "2024-11-15 to 2024-11-21",
    eventsCount: 13201,
    hash: "c5f7e9d0a1b3c4e5d6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f",
    previousHash:
      "b4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d7e8c9b2a1f4e6d8c9b0a3f5d8e",
    nextHash:
      "d6e8f0a1b2c3d4e5f6a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2",
    status: "Verified",
    createdAt: "2024-11-21 23:59:59",
    verifiedAt: "2024-11-22 00:04:32",
    verifiedBy: "auditor@acme.com",
    fileSize: "2.5 MB",
    chainPosition: 3,
    algorithm: "SHA-256",
  },
  {
    id: "digest-2024-004",
    period: "2024-11-22 to 2024-11-28",
    eventsCount: 12789,
    hash: "d6e8f0a1b2c3d4e5f6a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3",
    previousHash:
      "c5f7e9d0a1b3c4e5d6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f",
    nextHash:
      "e7f9a1b2c3d4e5f6a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3",
    status: "Pending",
    createdAt: "2024-11-28 23:59:59",
    fileSize: "2.3 MB",
    chainPosition: 4,
    algorithm: "SHA-256",
  },
  {
    id: "digest-2024-005",
    period: "2024-11-29 to 2024-12-05",
    eventsCount: 0,
    hash: "e7f9a1b2c3d4e5f6a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
    previousHash:
      "d6e8f0a1b2c3d4e5f6a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3",
    nextHash: undefined,
    status: "Unverified",
    createdAt: "2024-12-05 23:59:59",
    fileSize: "0 bytes",
    chainPosition: 5,
    algorithm: "SHA-256",
  },
];

export function DigestChainView() {
  const [digests, setDigests] = useState<DigestEntry[]>(mockDigests);
  const [selectedDigest, setSelectedDigest] = useState<DigestEntry | null>(
    null,
  );
  const [isDetailsOpen, setIsDetailsOpen] = useState(false);
  const [isVerifyOpen, setIsVerifyOpen] = useState(false);
  const [isVerifyChainOpen, setIsVerifyChainOpen] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [verificationProgress, setVerificationProgress] = useState(0);
  const [verificationResult, setVerificationResult] = useState<string>("");

  const handleVerifyDigest = async (digest: DigestEntry) => {
    setSelectedDigest(digest);
    setIsVerifyOpen(true);
    setIsVerifying(true);
    setVerificationProgress(0);

    setTimeout(() => {
      setVerificationProgress(50);
    }, 500);

    setTimeout(() => {
      const mockResult = Math.random() > 0.1 ? "valid" : "invalid";
      setVerificationResult(mockResult);
      setVerificationProgress(100);
      setIsVerifying(false);

      setDigests(
        digests.map((d) =>
          d.id === digest.id
            ? {
                ...d,
                status: mockResult === "valid" ? "Verified" : "Invalid",
                verifiedAt: new Date()
                  .toISOString()
                  .replace("T", " ")
                  .substring(0, 19),
                verifiedBy: "admin@acme.com",
              }
            : d,
        ),
      );
    }, 1500);
  };

  const handleVerifyChain = async () => {
    setIsVerifyChainOpen(true);
    setIsVerifying(true);
    setVerificationProgress(0);

    for (let i = 0; i <= 100; i += 10) {
      setTimeout(() => {
        setVerificationProgress(i);
        if (i === 100) {
          setIsVerifying(false);
          setVerificationResult(
            "Chain integrity verified. All digests are valid.",
          );
        }
      }, i * 50);
    }
  };

  const getStatusBadge = (status: DigestStatus) => {
    const variants: Record<
      DigestStatus,
      "success" | "secondary" | "destructive" | "default"
    > = {
      Verified: "success",
      Unverified: "secondary",
      Invalid: "destructive",
      Pending: "default",
    };

    const icons: Record<DigestStatus, React.ReactNode> = {
      Verified: <CheckCircle className="h-3 w-3" />,
      Unverified: <AlertTriangle className="h-3 w-3" />,
      Invalid: <XCircle className="h-3 w-3" />,
      Pending: <Lock className="h-3 w-3" />,
    };

    return (
      <Badge variant={variants[status]} className="flex items-center gap-1">
        {icons[status]}
        {status}
      </Badge>
    );
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const chainIntegrity =
    digests.filter((d) => d.status === "Verified").length / digests.length;

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Digest Chain</h3>
          <p className="text-sm text-gray-600">
            Cryptographic hash chain for data integrity verification
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={handleVerifyChain}>
            <GitBranch className="h-4 w-4 mr-2" />
            Verify Entire Chain
          </Button>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Chain Integrity</CardTitle>
              <CardDescription>
                Overall status of the digest chain
              </CardDescription>
            </div>
            <div className="text-right">
              <div className="text-2xl font-bold">
                {Math.round(chainIntegrity * 100)}%
              </div>
              <div className="text-sm text-gray-600">Verified</div>
            </div>
          </div>
          <Progress value={chainIntegrity * 100} className="mt-2" />
        </CardHeader>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Digest List</CardTitle>
          <CardDescription>
            All digests in the chain with their verification status
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="mb-4 p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center gap-4 text-sm">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-green-500"></div>
                <span>Verified</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-gray-400"></div>
                <span>Pending/Unverified</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-red-500"></div>
                <span>Invalid</span>
              </div>
              <div className="flex items-center gap-2">
                <Lock className="h-4 w-4" />
                <span>Hash Algorithm: SHA-256</span>
              </div>
            </div>
          </div>

          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Digest ID</TableHead>
                <TableHead>Period</TableHead>
                <TableHead>Events</TableHead>
                <TableHead>Hash (SHA-256)</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Position</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {digests.map((digest) => (
                <TableRow key={digest.id}>
                  <TableCell className="font-mono text-sm">
                    {digest.id}
                  </TableCell>
                  <TableCell>{digest.period}</TableCell>
                  <TableCell>{digest.eventsCount.toLocaleString()}</TableCell>
                  <TableCell>
                    <div className="flex items-center gap-2">
                      <code className="text-xs font-mono bg-gray-100 px-2 py-1 rounded">
                        {digest.hash.substring(0, 16)}...
                      </code>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => copyToClipboard(digest.hash)}
                      >
                        <Copy className="h-3 w-3" />
                      </Button>
                    </div>
                  </TableCell>
                  <TableCell>{getStatusBadge(digest.status)}</TableCell>
                  <TableCell>
                    <div className="flex items-center gap-2">
                      {digest.previousHash && (
                        <LinkIcon className="h-3 w-3 text-gray-400" />
                      )}
                      <span className="text-sm">#{digest.chainPosition}</span>
                    </div>
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
                          onClick={() => {
                            setSelectedDigest(digest);
                            setIsDetailsOpen(true);
                          }}
                        >
                          <Eye className="h-4 w-4 mr-2" />
                          View Details
                        </DropdownMenuItem>
                        <DropdownMenuItem
                          onClick={() => handleVerifyDigest(digest)}
                          disabled={digest.status === "Verified"}
                        >
                          <Shield className="h-4 w-4 mr-2" />
                          Verify
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                          <Download className="h-4 w-4 mr-2" />
                          Download Digest
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      <Dialog open={isDetailsOpen} onOpenChange={setIsDetailsOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>Digest Details</DialogTitle>
            <DialogDescription>
              {selectedDigest?.id} - {selectedDigest?.period}
            </DialogDescription>
          </DialogHeader>
          {selectedDigest && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label className="text-sm font-medium">Status</Label>
                  <div className="mt-1">
                    {getStatusBadge(selectedDigest.status)}
                  </div>
                </div>
                <div>
                  <Label className="text-sm font-medium">Events Count</Label>
                  <p className="text-sm">
                    {selectedDigest.eventsCount.toLocaleString()}
                  </p>
                </div>
                <div>
                  <Label className="text-sm font-medium">File Size</Label>
                  <p className="text-sm">{selectedDigest.fileSize}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Algorithm</Label>
                  <p className="text-sm">{selectedDigest.algorithm}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Created At</Label>
                  <p className="text-sm">{selectedDigest.createdAt}</p>
                </div>
                {selectedDigest.verifiedAt && (
                  <div>
                    <Label className="text-sm font-medium">Verified At</Label>
                    <p className="text-sm">{selectedDigest.verifiedAt}</p>
                  </div>
                )}
              </div>
              <div>
                <Label className="text-sm font-medium">Hash (SHA-256)</Label>
                <div className="mt-1 p-3 bg-gray-100 rounded font-mono text-xs break-all">
                  {selectedDigest.hash}
                </div>
              </div>
              {selectedDigest.previousHash && (
                <div>
                  <Label className="text-sm font-medium">Previous Hash</Label>
                  <div className="mt-1 p-3 bg-gray-100 rounded font-mono text-xs break-all">
                    {selectedDigest.previousHash}
                  </div>
                </div>
              )}
              {selectedDigest.nextHash && (
                <div>
                  <Label className="text-sm font-medium">Next Hash</Label>
                  <div className="mt-1 p-3 bg-gray-100 rounded font-mono text-xs break-all">
                    {selectedDigest.nextHash}
                  </div>
                </div>
              )}
            </div>
          )}
        </DialogContent>
      </Dialog>

      <Dialog open={isVerifyOpen} onOpenChange={setIsVerifyOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Verify Digest</DialogTitle>
            <DialogDescription>
              Verifying integrity of digest: {selectedDigest?.id}
            </DialogDescription>
          </DialogHeader>
          {isVerifying ? (
            <div className="space-y-4 py-4">
              <div className="text-center">
                <RefreshCw className="h-8 w-8 animate-spin mx-auto text-blue-600" />
                <p className="mt-2 text-sm">Verifying digest integrity...</p>
              </div>
              <Progress value={verificationProgress} className="h-2" />
            </div>
          ) : (
            <div className="space-y-4">
              <div className="flex items-center gap-2">
                {verificationResult === "valid" ? (
                  <>
                    <CheckCircle className="h-8 w-8 text-green-600" />
                    <div>
                      <p className="font-semibold">Verification Successful</p>
                      <p className="text-sm text-gray-600">
                        The digest is valid and has not been tampered with.
                      </p>
                    </div>
                  </>
                ) : (
                  <>
                    <XCircle className="h-8 w-8 text-red-600" />
                    <div>
                      <p className="font-semibold">Verification Failed</p>
                      <p className="text-sm text-gray-600">
                        The digest has been modified or corrupted.
                      </p>
                    </div>
                  </>
                )}
              </div>
            </div>
          )}
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsVerifyOpen(false)}>
              Close
            </Button>
            <Button>
              <Download className="h-4 w-4 mr-2" />
              Export Verification Report
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Dialog open={isVerifyChainOpen} onOpenChange={setIsVerifyChainOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Verify Entire Chain</DialogTitle>
            <DialogDescription>
              Verifying integrity of the entire digest chain
            </DialogDescription>
          </DialogHeader>
          {isVerifying ? (
            <div className="space-y-4 py-4">
              <div className="text-center">
                <RefreshCw className="h-8 w-8 animate-spin mx-auto text-blue-600" />
                <p className="mt-2 text-sm">Verifying chain integrity...</p>
              </div>
              <Progress value={verificationProgress} className="h-2" />
              <p className="text-xs text-center text-gray-500">
                Checking hash relationships and chain continuity...
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              <div className="flex items-center gap-2">
                <CheckCircle className="h-8 w-8 text-green-600" />
                <div>
                  <p className="font-semibold">Chain Verification Complete</p>
                  <p className="text-sm text-gray-600">{verificationResult}</p>
                </div>
              </div>
            </div>
          )}
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setIsVerifyChainOpen(false)}
            >
              Close
            </Button>
            <Button>
              <Download className="h-4 w-4 mr-2" />
              Export Chain Report
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
