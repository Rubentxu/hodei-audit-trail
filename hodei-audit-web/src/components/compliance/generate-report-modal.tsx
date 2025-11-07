'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';
import { Progress } from '@/components/ui/progress';
import { Calendar, FileText, Loader2, Save, Eye } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { useSession } from 'next-auth/react';

type ReportType = 'SOC 2' | 'PCI-DSS' | 'GDPR' | 'HIPAA' | 'ISO 27001';
type ReportFormat = 'PDF' | 'JSON' | 'CSV';

interface GenerateReportModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onGenerate: (config: ReportGenerationConfig) => void;
}

interface ReportGenerationConfig {
  name: string;
  type: ReportType;
  format: ReportFormat;
  startDate: string;
  endDate: string;
  sections: string[];
  template: string;
  includeMetadata: boolean;
  includeSignatures: boolean;
  emailNotification: boolean;
  emailRecipients: string;
}

const REPORT_TYPES: ReportType[] = ['SOC 2', 'PCI-DSS', 'GDPR', 'HIPAA', 'ISO 27001'];
const REPORT_FORMATS: ReportFormat[] = ['PDF', 'JSON', 'CSV'];

const SECTIONS_BY_TYPE: Record<ReportType, string[]> = {
  'SOC 2': [
    'Security',
    'Availability',
    'Processing Integrity',
    'Confidentiality',
    'Privacy',
    'Controls Assessment',
    'Exceptions',
    'Management Assertion',
  ],
  'PCI-DSS': [
    'Requirement 1: Install and maintain firewall configuration',
    'Requirement 2: Do not use vendor-supplied defaults',
    'Requirement 3: Protect stored cardholder data',
    'Requirement 4: Encrypt transmission of cardholder data',
    'Requirement 5: Protect all systems against malware',
    'Requirement 6: Develop and maintain secure systems',
    'Requirement 7: Restrict access to cardholder data',
    'Requirement 8: Identify and authenticate access',
    'Requirement 9: Restrict physical access',
    'Requirement 10: Track and monitor all access',
    'Requirement 11: Regularly test security systems',
    'Requirement 12: Maintain information security policy',
  ],
  'GDPR': [
    'Data Processing Activities',
    'Legal Basis for Processing',
    'Data Subject Rights',
    'Data Retention',
    'International Transfers',
    'Data Protection Impact Assessment',
    'Breach Notification',
    'Privacy by Design',
  ],
  'HIPAA': [
    'Administrative Safeguards',
    'Physical Safeguards',
    'Technical Safeguards',
    'Organizational Requirements',
    'Policies and Procedures',
    'Workforce Training',
    'Incident Response',
    'Risk Assessment',
  ],
  'ISO 27001': [
    'Information Security Policies',
    'Organization of Information Security',
    'Human Resource Security',
    'Asset Management',
    'Access Control',
    'Cryptography',
    'Physical and Environmental Security',
    'Operations Security',
    'Communications Security',
    'System Acquisition Development',
    'Supplier Relationships',
    'Incident Management',
    'Business Continuity',
    'Compliance',
  ],
};

const PREDEFINED_TEMPLATES = [
  { id: 'standard', name: 'Standard Template', description: 'Default template with all standard sections' },
  { id: 'executive', name: 'Executive Summary', description: 'High-level summary for executives' },
  { id: 'detailed', name: 'Detailed Technical', description: 'Comprehensive technical details' },
  { id: 'custom', name: 'Custom Template', description: 'Create your own template' },
];

export function GenerateReportModal({ open, onOpenChange, onGenerate }: GenerateReportModalProps) {
  const { data: session } = useSession();
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentStep, setCurrentStep] = useState('');
  const [config, setConfig] = useState<ReportGenerationConfig>({
    name: '',
    type: 'SOC 2',
    format: 'PDF',
    startDate: '',
    endDate: '',
    sections: [],
    template: 'standard',
    includeMetadata: true,
    includeSignatures: true,
    emailNotification: false,
    emailRecipients: '',
  });

  const handleGenerate = async () => {
    setIsGenerating(true);
    setProgress(0);
    setCurrentStep('Initializing...');

    const steps = [
      'Validating configuration...',
      'Collecting audit data...',
      'Generating report sections...',
      'Applying templates...',
      'Formatting report...',
      'Finalizing report...',
      'Sending notifications...',
    ];

    for (let i = 0; i < steps.length; i++) {
      setCurrentStep(steps[i]);
      setProgress((i + 1) * (100 / steps.length));
      await new Promise(resolve => setTimeout(resolve, 800));
    }

    onGenerate(config);
    setIsGenerating(false);
    setProgress(0);
    setCurrentStep('');
    onOpenChange(false);

    setConfig({
      name: '',
      type: 'SOC 2',
      format: 'PDF',
      startDate: '',
      endDate: '',
      sections: [],
      template: 'standard',
      includeMetadata: true,
      includeSignatures: true,
      emailNotification: false,
      emailRecipients: '',
    });
  };

  const handleSectionToggle = (section: string) => {
    setConfig(prev => ({
      ...prev,
      sections: prev.sections.includes(section)
        ? prev.sections.filter(s => s !== section)
        : [...prev.sections, section],
    }));
  };

  const selectAllSections = () => {
    setConfig(prev => ({
      ...prev,
      sections: SECTIONS_BY_TYPE[prev.type],
    }));
  };

  const clearAllSections = () => {
    setConfig(prev => ({
      ...prev,
      sections: [],
    }));
  };

  const isValid = config.name && config.startDate && config.endDate && config.sections.length > 0;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Generate Compliance Report</DialogTitle>
          <DialogDescription>
            Create a new compliance report with custom configuration
          </DialogDescription>
        </DialogHeader>

        {!isGenerating ? (
          <div className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="report-name">Report Name *</Label>
              <Input
                id="report-name"
                placeholder="e.g., Q4 2024 SOC 2 Compliance"
                value={config.name}
                onChange={(e) => setConfig({ ...config, name: e.target.value })}
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label>Report Type *</Label>
                <Select
                  value={config.type}
                  onValueChange={(value) => {
                    const newType = value as ReportType;
                    setConfig({
                      ...config,
                      type: newType,
                      sections: [],
                    });
                  }}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {REPORT_TYPES.map((type) => (
                      <SelectItem key={type} value={type}>
                        {type}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-2">
                <Label>Format *</Label>
                <Select
                  value={config.format}
                  onValueChange={(value) => setConfig({ ...config, format: value as ReportFormat })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {REPORT_FORMATS.map((format) => (
                      <SelectItem key={format} value={format}>
                        {format}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="start-date">Start Date *</Label>
                <Input
                  id="start-date"
                  type="date"
                  value={config.startDate}
                  onChange={(e) => setConfig({ ...config, startDate: e.target.value })}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="end-date">End Date *</Label>
                <Input
                  id="end-date"
                  type="date"
                  value={config.endDate}
                  onChange={(e) => setConfig({ ...config, endDate: e.target.value })}
                />
              </div>
            </div>

            <div className="space-y-2">
              <Label>Template</Label>
              <Select
                value={config.template}
                onValueChange={(value) => setConfig({ ...config, template: value })}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {PREDEFINED_TEMPLATES.map((template) => (
                    <SelectItem key={template.id} value={template.id}>
                      <div>
                        <div className="font-medium">{template.name}</div>
                        <div className="text-xs text-gray-500">{template.description}</div>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label>Sections to Include *</Label>
                <div className="flex gap-2">
                  <Button variant="ghost" size="sm" onClick={selectAllSections}>
                    Select All
                  </Button>
                  <Button variant="ghost" size="sm" onClick={clearAllSections}>
                    Clear All
                  </Button>
                </div>
              </div>
              <div className="border rounded-lg p-4 max-h-40 overflow-y-auto space-y-2">
                {SECTIONS_BY_TYPE[config.type].map((section) => (
                  <div key={section} className="flex items-center space-x-2">
                    <Checkbox
                      id={section}
                      checked={config.sections.includes(section)}
                      onCheckedChange={() => handleSectionToggle(section)}
                    />
                    <Label htmlFor={section} className="text-sm">
                      {section}
                    </Label>
                  </div>
                ))}
              </div>
              <div className="flex items-center gap-2 flex-wrap">
                {config.sections.map((section) => (
                  <Badge key={section} variant="secondary">
                    {section}
                  </Badge>
                ))}
                {config.sections.length === 0 && (
                  <span className="text-sm text-gray-500">No sections selected</span>
                )}
              </div>
            </div>

            <div className="space-y-3">
              <Label>Options</Label>
              <div className="space-y-2">
                <div className="flex items-center space-x-2">
                  <Checkbox
                    id="include-metadata"
                    checked={config.includeMetadata}
                    onCheckedChange={(checked) =>
                      setConfig({ ...config, includeMetadata: checked as boolean })
                    }
                  />
                  <Label htmlFor="include-metadata" className="text-sm">
                    Include metadata and timestamps
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox
                    id="include-signatures"
                    checked={config.includeSignatures}
                    onCheckedChange={(checked) =>
                      setConfig({ ...config, includeSignatures: checked as boolean })
                    }
                  />
                  <Label htmlFor="include-signatures" className="text-sm">
                    Include digital signatures
                  </Label>
                </div>
                <div className="flex items-center space-x-2">
                  <Checkbox
                    id="email-notification"
                    checked={config.emailNotification}
                    onCheckedChange={(checked) =>
                      setConfig({ ...config, emailNotification: checked as boolean })
                    }
                  />
                  <Label htmlFor="email-notification" className="text-sm">
                    Send email notification when complete
                  </Label>
                </div>
              </div>
              {config.emailNotification && (
                <Input
                  placeholder="Email recipients (comma-separated)"
                  value={config.emailRecipients}
                  onChange={(e) => setConfig({ ...config, emailRecipients: e.target.value })}
                />
              )}
            </div>
          </div>
        ) : (
          <div className="space-y-6 py-8">
            <div className="text-center">
              <Loader2 className="h-12 w-12 animate-spin mx-auto text-blue-600" />
              <h3 className="text-lg font-semibold mt-4">Generating Report</h3>
              <p className="text-sm text-gray-600 mt-2">{currentStep}</p>
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span>Progress</span>
                <span>{Math.round(progress)}%</span>
              </div>
              <Progress value={progress} className="h-2" />
            </div>
            <div className="text-xs text-gray-500 text-center">
              Please do not close this dialog while generating
            </div>
          </div>
        )}

        <DialogFooter>
          {!isGenerating && (
            <>
              <Button variant="outline" onClick={() => onOpenChange(false)}>
                Cancel
              </Button>
              <Button onClick={handleGenerate} disabled={!isValid}>
                <FileText className="h-4 w-4 mr-2" />
                Generate Report
              </Button>
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
