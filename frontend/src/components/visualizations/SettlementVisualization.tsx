import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { CheckCircle2, Sparkles } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useTooltips } from '../../hooks/useQueries';

export default function SettlementVisualization() {
  const [showSuccess, setShowSuccess] = useState(false);
  const { data: tooltips } = useTooltips();
  const tooltipMap = new Map(tooltips || []);

  useEffect(() => {
    const timer = setTimeout(() => setShowSuccess(true), 500);
    return () => clearTimeout(timer);
  }, []);

  return (
    <div className="space-y-6">
      {/* Success Banner */}
      {showSuccess && (
        <Card className="p-8 bg-gradient-to-r from-primary/10 via-accent/10 to-primary/10 border-primary/20 animate-in fade-in duration-500">
          <div className="flex flex-col items-center text-center gap-4">
            <div className="relative">
              <img 
                src="/assets/generated/settlement-complete.dim_400x400.png" 
                alt="Settlement Complete"
                className="w-32 h-32 object-contain"
              />
              <Sparkles className="w-8 h-8 text-primary absolute -top-2 -right-2 animate-pulse" />
            </div>
            <div>
              <h3 className="text-2xl font-bold text-primary mb-2">Settlement Complete!</h3>
              <p className="text-muted-foreground max-w-2xl">
                Your private exit has been successfully completed. The original note has been consumed, 
                and the liquidity provider has been repaid—all while maintaining complete privacy.
              </p>
            </div>
          </div>
        </Card>
      )}

      {/* Settlement Details */}
      <div className="grid md:grid-cols-2 gap-6">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Card className="p-6 cursor-help">
                <h3 className="font-semibold mb-4 flex items-center gap-2">
                  <CheckCircle2 className="w-5 h-5 text-primary" />
                  Note Consumption
                </h3>
                <div className="space-y-3">
                  <div className="p-4 rounded-lg bg-primary/5 border border-primary/20">
                    <div className="text-xs text-muted-foreground mb-2">Original Note Status:</div>
                    <Badge variant="default" className="text-sm">Consumed</Badge>
                  </div>
                  <div className="p-4 rounded-lg bg-background/50 border border-border">
                    <div className="text-xs text-muted-foreground mb-2">Nullifier Generated:</div>
                    <div className="font-mono text-xs break-all text-primary">
                      0x{Array.from({ length: 64 }, () => Math.floor(Math.random() * 16).toString(16)).join('')}
                    </div>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    The original exit note has been cryptographically consumed and cannot be reused.
                  </p>
                </div>
              </Card>
            </TooltipTrigger>
            <TooltipContent className="max-w-xs">
              <p>{tooltipMap.get('settlement') || 'Final settlement process ensures privacy and security for both users and LPs.'}</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>

        <Card className="p-6">
          <h3 className="font-semibold mb-4 flex items-center gap-2">
            <CheckCircle2 className="w-5 h-5 text-primary" />
            LP Repayment
          </h3>
          <div className="space-y-3">
            <div className="p-4 rounded-lg bg-primary/5 border border-primary/20">
              <div className="text-xs text-muted-foreground mb-2">Repayment Status:</div>
              <Badge variant="default" className="text-sm">Completed</Badge>
            </div>
            <div className="p-4 rounded-lg bg-background/50 border border-border">
              <div className="text-xs text-muted-foreground mb-2">LP Pool:</div>
              <div className="font-medium text-sm">LP Pool Alpha</div>
            </div>
            <p className="text-xs text-muted-foreground">
              The liquidity provider has been repaid with interest, completing the exit cycle.
            </p>
          </div>
        </Card>
      </div>

      {/* Privacy Summary */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <h3 className="font-semibold mb-4 text-accent">Privacy Summary</h3>
        <div className="grid md:grid-cols-2 gap-4">
          <div className="space-y-3">
            <h4 className="text-sm font-medium">What Remained Private:</h4>
            <div className="space-y-2 text-sm text-muted-foreground">
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Transaction amount</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Your wallet address</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Transaction timing</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Exit note details</span>
              </div>
            </div>
          </div>
          <div className="space-y-3">
            <h4 className="text-sm font-medium">What Was Verified:</h4>
            <div className="space-y-2 text-sm text-muted-foreground">
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Proof validity</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Exit eligibility</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Cryptographic commitments</span>
              </div>
              <div className="flex items-center gap-2">
                <CheckCircle2 className="w-4 h-4 text-primary flex-shrink-0" />
                <span>Settlement completion</span>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Call to Action */}
      <Card className="p-6 bg-gradient-to-r from-primary/5 to-accent/5 border-primary/20">
        <div className="text-center">
          <h4 className="font-semibold mb-2">Understanding Voile Protocol</h4>
          <p className="text-sm text-muted-foreground mb-4">
            You've completed the simulation! Voile Protocol demonstrates how zero-knowledge proofs 
            enable private, trustless liquidity exits on Miden while preserving user privacy at every step.
          </p>
          <div className="flex flex-wrap gap-2 justify-center">
            <Badge variant="outline" className="border-primary/50 text-primary">Zero-Knowledge</Badge>
            <Badge variant="outline" className="border-primary/50 text-primary">Privacy-First</Badge>
            <Badge variant="outline" className="border-primary/50 text-primary">Trustless</Badge>
            <Badge variant="outline" className="border-primary/50 text-primary">Verifiable</Badge>
          </div>
        </div>
      </Card>
    </div>
  );
}
