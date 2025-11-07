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
import { Textarea } from '@/components/ui/textarea';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';
import {
  FileText,
  Plus,
  Copy,
  Edit,
  Trash2,
  Eye,
  Share2,
  Search,
  MoreVertical,
  Calendar,
  User,
} from 'lucide-react';

type ReportType = 'SOC 2' | 'PCI-DSS' | 'GDPR' | 'HIPAA' | 'ISO 27001';

interface ReportTemplate {
  id: string;
  name: string;
  description: string;
  type: ReportType;
  isCustom: boolean;
  isDefault: boolean;
  createdBy: string;
  createdAt: string;
  updatedAt: string;
  sections: string[];
  format: 'PDF' | 'JSON' | 'CSV';
  version: string;
  isShared: boolean;
  usageCount: number;
}

const DEFAULT_TEMPLATES: ReportTemplate[] = [
  {
    id: 'tpl-soc2-standard',
    name: 'SOC 2 Standard',
    description: 'Complete SOC 2 Type II report with all trust service categories',
    type: 'SOC 2',
    isCustom: false,
    isDefault: true,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: [
      'Security',
      'Availability',
      'Processing Integrity',
      'Confidentiality',
      'Privacy',
    ],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 45,
  },
  {
    id: 'tpl-soc2-exec',
    name: 'SOC 2 Executive Summary',
    description: 'High-level executive summary for stakeholders',
    type: 'SOC 2',
    isCustom: false,
    isDefault: false,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: ['Security', 'Availability', 'Management Assertion'],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 23,
  },
  {
    id: 'tpl-pci-standard',
    name: 'PCI-DSS Complete',
    description: 'Full PCI-DSS compliance report covering all 12 requirements',
    type: 'PCI-DSS',
    isCustom: false,
    isDefault: true,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: [
      'Requirement 1: Install and maintain firewall configuration',
      'Requirement 2: Do not use vendor-supplied defaults',
      'Requirement 3: Protect stored cardholder data',
      'Requirement 4: Encrypt transmission',
      'Requirement 5: Protect against malware',
      'Requirement 6: Develop secure systems',
      'Requirement 7: Restrict access',
      'Requirement 8: Identify and authenticate',
      'Requirement 9: Restrict physical access',
      'Requirement 10: Track and monitor',
      'Requirement 11: Test security',
      'Requirement 12: Maintain security policy',
    ],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 38,
  },
  {
    id: 'tpl-gdpr-standard',
    name: 'GDPR Compliance',
    description: 'Complete GDPR compliance documentation',
    type: 'GDPR',
    isCustom: false,
    isDefault: true,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: [
      'Data Processing Activities',
      'Legal Basis for Processing',
      'Data Subject Rights',
      'Data Retention',
      'International Transfers',
      'Breach Notification',
    ],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 29,
  },
  {
    id: 'tpl-hipaa-standard',
    name: 'HIPAA Security Rule',
    description: 'HIPAA Security Rule compliance assessment',
    type: 'HIPAA',
    isCustom: false,
    isDefault: true,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: [
      'Administrative Safeguards',
      'Physical Safeguards',
      'Technical Safeguards',
      'Organizational Requirements',
    ],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 31,
  },
  {
    id: 'tpl-iso-standard',
    name: 'ISO 27001 Comprehensive',
    description: 'Complete ISO 27001 information security management',
    type: 'ISO 27001',
    isCustom: false,
    isDefault: true,
    createdBy: 'System',
    createdAt: '2024-01-01',
    updatedAt: '2024-01-01',
    sections: [
      'Information Security Policies',
      'Organization of Information Security',
      'Human Resource Security',
      'Asset Management',
      'Access Control',
      'Cryptography',
      'Physical Security',
      'Operations Security',
      'Communications Security',
      'System Acquisition',
      'Supplier Relationships',
      'Incident Management',
      'Business Continuity',
      'Compliance',
    ],
    format: 'PDF',
    version: '1.0',
    isShared: true,
    usageCount: 27,
  },
  {
    id: 'tpl-custom-001',
    name: 'Custom SOC 2 - Privacy Focus',
    description: 'Custom template focused on privacy controls',
    type: 'SOC 2',
    isCustom: true,
    isDefault: false,
    createdBy: 'admin@acme.com',
    createdAt: '2024-10-15',
    updatedAt: '2024-11-01',
    sections: ['Privacy', 'Confidentiality', 'Security'],
    format: 'PDF',
    version: '1.1',
    isShared: false,
    usageCount: 5,
  },
];

const REPORT_TYPES: ReportType[] = ['SOC 2', 'PCI-DSS', 'GDPR', 'HIPAA', 'ISO 27001'];

export function TemplatesList() {
  const [templates, setTemplates] = useState<ReportTemplate[]>(DEFAULT_TEMPLATES);
  const [searchQuery, setSearchQuery] = useState('');
  const [typeFilter, setTypeFilter] = useState<ReportType | 'all'>('all');
  const [selectedTemplate, setSelectedTemplate] = useState<ReportTemplate | null>(null);
  const [isPreviewOpen, setIsPreviewOpen] = useState(false);
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [isEditOpen, setIsEditOpen] = useState(false);
  const [isDeleteOpen, setIsDeleteOpen] = useState(false);
  const [isCloneOpen, setIsCloneOpen] = useState(false);
  const [isShareOpen, setIsShareOpen] = useState(false);

  const filteredTemplates = templates.filter((template) => {
    const matchesSearch =
      template.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      template.description.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesType = typeFilter === 'all' || template.type === typeFilter;
    return matchesSearch && matchesType;
  });

  const handlePreview = (template: ReportTemplate) => {
    setSelectedTemplate(template);
    setIsPreviewOpen(true);
  };

  const handleClone = (template: ReportTemplate) => {
    setSelectedTemplate(template);
    setIsCloneOpen(true);
  };

  const handleDelete = (template: ReportTemplate) => {
    setSelectedTemplate(template);
    setIsDeleteOpen(true);
  };

  const confirmDelete = () => {
    if (selectedTemplate) {
      setTemplates(templates.filter((t) => t.id !== selectedTemplate.id));
      setIsDeleteOpen(false);
      setSelectedTemplate(null);
    }
  };

  const getTypeBadgeColor = (type: ReportType) => {
    const colors: Record<ReportType, string> = {
      'SOC 2': 'bg-blue-100 text-blue-800',
      'PCI-DSS': 'bg-purple-100 text-purple-800',
      'GDPR': 'bg-green-100 text-green-800',
      'HIPAA': 'bg-red-100 text-red-800',
      'ISO 27001': 'bg-yellow-100 text-yellow-800',
    };
    return colors[type];
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">Report Templates</h3>
          <p className="text-sm text-gray-600">
            Manage and customize compliance report templates
          </p>
        </div>
        <Button onClick={() => setIsCreateOpen(true)}>
          <Plus className="h-4 w-4 mr-2" />
          Create Template
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Search</label>
              <div className="relative">
                <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
                <Input
                  placeholder="Search templates..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-9"
                />
              </div>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-medium">Report Type</label>
              <Select
                value={typeFilter}
                onValueChange={(value) =>
                  setTypeFilter(value as ReportType | 'all')
                }
              >
                <SelectTrigger>
                  <SelectValue placeholder="All types" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All types</SelectItem>
                  {REPORT_TYPES.map((type) => (
                    <SelectItem key={type} value={type}>
                      {type}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredTemplates.map((template) => (
          <Card key={template.id} className="hover:shadow-md transition-shadow">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div className="space-y-1 flex-1">
                  <div className="flex items-center gap-2">
                    <CardTitle className="text-base">{template.name}</CardTitle>
                    {template.isDefault && (
                      <Badge variant="secondary" className="text-xs">
                        Default
                      </Badge>
                    )}
                    {template.isCustom && (
                      <Badge variant="outline" className="text-xs">
                        Custom
                      </Badge>
                    )}
                  </div>
                  <CardDescription className="text-sm">
                    {template.description}
                  </CardDescription>
                </div>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="sm">
                      <MoreVertical className="h-4 w-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuLabel>Actions</DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem onClick={() => handlePreview(template)}>
                      <Eye className="h-4 w-4 mr-2" />
                      Preview
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => {
                        setSelectedTemplate(template);
                        setIsEditOpen(true);
                      }}
                    >
                      <Edit className="h-4 w-4 mr-2" />
                      Edit
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => handleClone(template)}>
                      <Copy className="h-4 w-4 mr-2" />
                      Clone
                    </DropdownMenuItem>
                    {!template.isDefault && (
                      <>
                        <DropdownMenuItem
                          onClick={() => {
                            setSelectedTemplate(template);
                            setIsShareOpen(true);
                          }}
                        >
                          <Share2 className="h-4 w-4 mr-2" />
                          {template.isShared ? 'Unshare' : 'Share'}
                        </DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem
                          className="text-red-600"
                          onClick={() => handleDelete(template)}
                        >
                          <Trash2 className="h-4 w-4 mr-2" />
                          Delete
                        </DropdownMenuItem>
                      </>
                    )}
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                <div className="flex items-center gap-2">
                  <Badge className={getTypeBadgeColor(template.type)}>
                    {template.type}
                  </Badge>
                  <Badge variant="outline">{template.format}</Badge>
                </div>
                <div className="flex items-center justify-between text-sm text-gray-600">
                  <div className="flex items-center gap-1">
                    <User className="h-3 w-3" />
                    <span>{template.createdBy}</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <Calendar className="h-3 w-3" />
                    <span>{new Date(template.updatedAt).toLocaleDateString()}</span>
                  </div>
                </div>
                <div className="text-sm text-gray-600">
                  <span className="font-medium">{template.sections.length}</span> sections •{' '}
                  <span className="font-medium">{template.usageCount}</span> uses • v{template.version}
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {filteredTemplates.length === 0 && (
        <div className="text-center py-12 text-gray-500">
          <FileText className="h-12 w-12 mx-auto mb-4 opacity-50" />
          <p>No templates found</p>
        </div>
      )}

      <Dialog open={isPreviewOpen} onOpenChange={setIsPreviewOpen}>
        <DialogContent className="max-w-3xl">
          <DialogHeader>
            <DialogTitle>Template Preview</DialogTitle>
            <DialogDescription>
              {selectedTemplate?.name} - {selectedTemplate?.description}
            </DialogDescription>
          </DialogHeader>
          {selectedTemplate && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label className="text-sm font-medium">Type</Label>
                  <p className="text-sm">{selectedTemplate.type}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Format</Label>
                  <p className="text-sm">{selectedTemplate.format}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Version</Label>
                  <p className="text-sm">{selectedTemplate.version}</p>
                </div>
                <div>
                  <Label className="text-sm font-medium">Usage Count</Label>
                  <p className="text-sm">{selectedTemplate.usageCount}</p>
                </div>
              </div>
              <div>
                <Label className="text-sm font-medium">Sections</Label>
                <div className="mt-2 flex flex-wrap gap-2">
                  {selectedTemplate.sections.map((section) => (
                    <Badge key={section} variant="secondary">
                      {section}
                    </Badge>
                  ))}
                </div>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>

      <Dialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Template</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete "{selectedTemplate?.name}"? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsDeleteOpen(false)}>
              Cancel
            </Button>
            <Button variant="destructive" onClick={confirmDelete}>
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>Create Custom Template</DialogTitle>
            <DialogDescription>
              Create a new custom report template
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="text-center py-8 text-gray-500">
              <FileText className="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>Template creation will be implemented here</p>
              <p className="text-sm mt-2">Story 06.04 - Report templates (advanced features)</p>
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsCreateOpen(false)}>
              Cancel
            </Button>
            <Button disabled>Create Template</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
