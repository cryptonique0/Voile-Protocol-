import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { CheckCircle2, Loader2, Shield } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useTooltips } from '../../hooks/useQueries';

interface ValidationStep {
  label: string;
  status: 'pending' | 'validating' | 'complete';
}

export default function ValidationVisualization() {
  const [steps, setSteps] = useState<ValidationStep[]>([
    { label: 'Verify proof structure', status: 'pending' },
    { label: 'Check cryptographic signature', status: 'pending' },
    { label: 'Validate commitment', status: 'pending' },
    { label: 'Confirm eligibility', status: 'pending' },
  ]);
  const [currentStepIndex, setCurrentStepIndex] = useState(0);
  const { data: tooltips } = useTooltips();
  const tooltipMap = new Map(tooltips || []);

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentStepIndex((prev) => {
        if (prev >= steps.length) {
          clearInterval(interval);
          return prev;
        }
        
        setSteps((currentSteps) =>
          currentSteps.map((step, index) => {
            if (index < prev) return { ...step, status: 'complete' as const };
            if (index === prev) return { ...step, status: 'validating' as const };
            return step;
          })
        );
        
        return prev + 1;
      });
    }, 1200);

    return () => clearInterval(interval);
  }, []);

  const allComplete = currentStepIndex >= steps.length;

  return (
    <div className="space-y-6">
      <div className="grid md:grid-cols-2 gap-6">
        {/* Validation Steps */}
        <Card className="p-6">
          <h3 className="font-semibold mb-4 flex items-center gap-2">
            <Shield className="w-5 h-5 text-primary" />
            Validation Process
          </h3>
          <div className="space-y-3">
            {steps.map((step, index) => (
              <div
                key={index}
                className={`flex items-center gap-3 p-3 rounded-lg transition-all ${
                  step.status === 'complete'
                    ? 'bg-primary/10 border border-primary/20'
                    : step.status === 'validating'
                    ? 'bg-accent/10 border border-accent/20'
                    : 'bg-muted/30'
                }`}
              >
                {step.status === 'complete' ? (
                  <CheckCircle2 className="w-5 h-5 text-primary flex-shrink-0" />
                ) : step.status === 'validating' ? (
                  <Loader2 className="w-5 h-5 text-accent animate-spin flex-shrink-0" />
                ) : (
                  <div className="w-5 h-5 rounded-full border-2 border-muted-foreground/30 flex-shrink-0" />
                )}
                <span className={`text-sm ${step.status === 'pending' ? 'text-muted-foreground' : 'font-medium'}`}>
                  {step.label}
                </span>
              </div>
            ))}
          </div>
        </Card>

        {/* LP Dashboard */}
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Card className="p-6 cursor-help">
                <h3 className="font-semibold mb-4">LP Dashboard View</h3>
                <div className="space-y-4">
                  <div className="p-4 rounded-lg bg-background/50 border border-border">
                    <div className="text-xs text-muted-foreground mb-2">Proof Status:</div>
                    <Badge variant={allComplete ? 'default' : 'outline'} className="text-sm">
                      {allComplete ? 'Valid & Verified' : 'Validating...'}
                    </Badge>
                  </div>
                  
                  <div className="p-4 rounded-lg bg-background/50 border border-border">
                    <div className="text-xs text-muted-foreground mb-2">Private Data Visibility:</div>
                    <div className="flex items-center gap-2">
                      <Badge variant="outline" className="border-destructive/50 text-destructive">
                        Amount: Hidden
                      </Badge>
                      <Badge variant="outline" className="border-destructive/50 text-destructive">
                        ID: Hidden
                      </Badge>
                    </div>
                  </div>

                  <div className="p-4 rounded-lg bg-background/50 border border-border">
                    <div className="text-xs text-muted-foreground mb-2">Public Information:</div>
                    <div className="flex items-center gap-2">
                      <Badge variant="outline" className="border-primary/50 text-primary">
                        Proof Valid
                      </Badge>
                      <Badge variant="outline" className="border-primary/50 text-primary">
                        Eligible
                      </Badge>
                    </div>
                  </div>
                </div>
              </Card>
            </TooltipTrigger>
            <TooltipContent className="max-w-xs">
              <p>{tooltipMap.get('lpInterface') || 'Liquidity providers can verify proof authenticity without accessing user details.'}</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>

      {/* Result */}
      {allComplete && (
        <Card className="p-6 bg-primary/5 border-primary/20 animate-in fade-in duration-500">
          <div className="flex items-center gap-4">
            <CheckCircle2 className="w-10 h-10 text-primary flex-shrink-0" />
            <div>
              <h4 className="font-semibold text-primary mb-1">Validation Successful</h4>
              <p className="text-sm text-muted-foreground">
                The zero-knowledge proof has been validated. The LP can now proceed with the private 
                transfer without ever learning your transaction details. Your privacy is fully preserved.
              </p>
            </div>
          </div>
        </Card>
      )}
    </div>
  );
}
