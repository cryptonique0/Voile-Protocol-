import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Lock, FileKey, Send, CheckCircle2, ArrowRightLeft, Coins } from 'lucide-react';
import EncryptedNoteVisualization from './visualizations/EncryptedNoteVisualization';
import ZKProofVisualization from './visualizations/ZKProofVisualization';
import LPInterfaceVisualization from './visualizations/LPInterfaceVisualization';
import ValidationVisualization from './visualizations/ValidationVisualization';
import TransferVisualization from './visualizations/TransferVisualization';
import SettlementVisualization from './visualizations/SettlementVisualization';

interface StepContentProps {
  step: number;
  description: string;
}

const stepIcons = [Lock, FileKey, Send, CheckCircle2, ArrowRightLeft, Coins];
const stepTitles = [
  'Generate Private Exit Note',
  'Create Zero-Knowledge Proof',
  'Submit to LP Network',
  'LP Validation Process',
  'Private Transfer Execution',
  'Settlement & Completion'
];

export default function StepContent({ step, description }: StepContentProps) {
  const Icon = stepIcons[step - 1];

  const renderVisualization = () => {
    switch (step) {
      case 1:
        return <EncryptedNoteVisualization />;
      case 2:
        return <ZKProofVisualization />;
      case 3:
        return <LPInterfaceVisualization />;
      case 4:
        return <ValidationVisualization />;
      case 5:
        return <TransferVisualization />;
      case 6:
        return <SettlementVisualization />;
      default:
        return null;
    }
  };

  return (
    <Card className="overflow-hidden border-2">
      <CardHeader className="bg-gradient-to-r from-primary/10 via-accent/10 to-primary/10">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
            <Icon className="w-6 h-6 text-primary" />
          </div>
          <div>
            <CardTitle className="text-2xl">{stepTitles[step - 1]}</CardTitle>
            <CardDescription className="text-base mt-1">{description}</CardDescription>
          </div>
        </div>
      </CardHeader>
      <CardContent className="p-6">
        {renderVisualization()}
      </CardContent>
    </Card>
  );
}
