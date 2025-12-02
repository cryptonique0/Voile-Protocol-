import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { CheckCircle2, Loader2 } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useTooltips } from '../../hooks/useQueries';

export default function ZKProofVisualization() {
  const [progress, setProgress] = useState(0);
  const [isComplete, setIsComplete] = useState(false);
  const [proofHash, setProofHash] = useState('');
  const { data: tooltips } = useTooltips();
  const tooltipMap = new Map(tooltips || []);

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          setIsComplete(true);
          setProofHash('0x' + Array.from({ length: 64 }, () => 
            Math.floor(Math.random() * 16).toString(16)
          ).join(''));
          return 100;
        }
        return prev + 2;
      });
    }, 50);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="space-y-6">
      <div className="grid md:grid-cols-2 gap-6">
        {/* Proof Generation */}
        <Card className="p-6">
          <h3 className="font-semibold mb-4 flex items-center gap-2">
            {isComplete ? (
              <CheckCircle2 className="w-5 h-5 text-primary" />
            ) : (
              <Loader2 className="w-5 h-5 text-primary animate-spin" />
            )}
            Proof Generation
          </h3>
          <div className="space-y-4">
            <div>
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-muted-foreground">Computing proof...</span>
                <span className="text-sm font-medium text-primary">{progress}%</span>
              </div>
              <Progress value={progress} className="h-2" />
            </div>
            
            {isComplete && (
              <div className="space-y-3 animate-in fade-in duration-500">
                <div className="p-3 rounded-lg bg-primary/10 border border-primary/20">
                  <div className="text-xs text-muted-foreground mb-1">Proof Hash:</div>
                  <div className="font-mono text-xs break-all text-primary">{proofHash}</div>
                </div>
                <div className="flex flex-wrap gap-2">
                  <Badge variant="outline" className="border-primary/50 text-primary">
                    Valid
                  </Badge>
                  <Badge variant="outline" className="border-primary/50 text-primary">
                    Zero-Knowledge
                  </Badge>
                  <Badge variant="outline" className="border-primary/50 text-primary">
                    Succinct
                  </Badge>
                </div>
              </div>
            )}
          </div>
        </Card>

        {/* Proof Properties */}
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Card className="p-6 cursor-help">
                <h3 className="font-semibold mb-4">Proof Properties</h3>
                <div className="space-y-3">
                  <div className="flex items-start gap-3">
                    <div className="w-2 h-2 rounded-full bg-primary mt-2" />
                    <div>
                      <div className="font-medium text-sm">Completeness</div>
                      <div className="text-xs text-muted-foreground">Valid statements always produce valid proofs</div>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <div className="w-2 h-2 rounded-full bg-primary mt-2" />
                    <div>
                      <div className="font-medium text-sm">Soundness</div>
                      <div className="text-xs text-muted-foreground">Invalid statements cannot produce valid proofs</div>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <div className="w-2 h-2 rounded-full bg-primary mt-2" />
                    <div>
                      <div className="font-medium text-sm">Zero-Knowledge</div>
                      <div className="text-xs text-muted-foreground">Reveals nothing beyond validity</div>
                    </div>
                  </div>
                </div>
              </Card>
            </TooltipTrigger>
            <TooltipContent className="max-w-xs">
              <p>{tooltipMap.get('zkProofs') || 'Zero-knowledge proofs validate transactions without revealing sensitive data.'}</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>

      {/* Visual Diagram */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <div className="flex flex-col md:flex-row gap-6 items-center">
          <img 
            src="/assets/generated/zk-proof-diagram.dim_600x400.png" 
            alt="Zero-Knowledge Proof Diagram"
            className="w-full md:w-1/2 rounded-lg border border-border"
          />
          <div className="flex-1">
            <h4 className="font-semibold mb-2 text-accent">Zero-Knowledge Proof</h4>
            <p className="text-sm text-muted-foreground leading-relaxed mb-3">
              The zero-knowledge proof cryptographically proves that your exit note is valid and you have 
              the right to request liquidity, without revealing the amount, your identity, or any other 
              sensitive details.
            </p>
            <p className="text-sm text-muted-foreground leading-relaxed">
              This proof can be verified by anyone, including liquidity providers, while maintaining 
              complete privacy of your transaction data.
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
}
