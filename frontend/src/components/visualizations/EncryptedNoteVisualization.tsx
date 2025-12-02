import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Lock, Eye, EyeOff } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';
import { useTooltips } from '../../hooks/useQueries';

export default function EncryptedNoteVisualization() {
  const [isGenerating, setIsGenerating] = useState(true);
  const [commitment, setCommitment] = useState('');
  const { data: tooltips } = useTooltips();
  const tooltipMap = new Map(tooltips || []);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsGenerating(false);
      setCommitment('0x' + Array.from({ length: 64 }, () => 
        Math.floor(Math.random() * 16).toString(16)
      ).join(''));
    }, 1500);
    return () => clearTimeout(timer);
  }, []);

  return (
    <div className="space-y-6">
      <div className="grid md:grid-cols-2 gap-6">
        {/* Private Data (Hidden) */}
        <Card className="p-6 bg-destructive/5 border-destructive/20">
          <div className="flex items-center gap-2 mb-4">
            <EyeOff className="w-5 h-5 text-destructive" />
            <h3 className="font-semibold text-destructive">Private Data (Hidden)</h3>
          </div>
          <div className="space-y-3">
            <div className="flex items-center justify-between p-3 rounded-lg bg-background/50 backdrop-blur-sm">
              <span className="text-sm text-muted-foreground">Amount:</span>
              <div className="flex items-center gap-2">
                <Lock className="w-4 h-4 text-destructive" />
                <span className="font-mono text-sm blur-sm select-none">1,250.00 MIDEN</span>
              </div>
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-background/50 backdrop-blur-sm">
              <span className="text-sm text-muted-foreground">Wallet ID:</span>
              <div className="flex items-center gap-2">
                <Lock className="w-4 h-4 text-destructive" />
                <span className="font-mono text-sm blur-sm select-none">0xA7B3...9F2E</span>
              </div>
            </div>
            <div className="flex items-center justify-between p-3 rounded-lg bg-background/50 backdrop-blur-sm">
              <span className="text-sm text-muted-foreground">Timestamp:</span>
              <div className="flex items-center gap-2">
                <Lock className="w-4 h-4 text-destructive" />
                <span className="font-mono text-sm blur-sm select-none">2025-12-02 14:32</span>
              </div>
            </div>
          </div>
        </Card>

        {/* Public Commitment (Visible) */}
        <Card className="p-6 bg-primary/5 border-primary/20">
          <div className="flex items-center gap-2 mb-4">
            <Eye className="w-5 h-5 text-primary" />
            <h3 className="font-semibold text-primary">Public Commitment (Visible)</h3>
          </div>
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <div className="space-y-3 cursor-help">
                  <div className="p-4 rounded-lg bg-background/50 backdrop-blur-sm border border-primary/20">
                    <div className="text-xs text-muted-foreground mb-2">Cryptographic Commitment:</div>
                    {isGenerating ? (
                      <div className="space-y-2">
                        <div className="h-4 bg-primary/20 rounded animate-pulse" />
                        <div className="h-4 bg-primary/20 rounded animate-pulse w-3/4" />
                      </div>
                    ) : (
                      <div className="font-mono text-xs break-all text-primary">{commitment}</div>
                    )}
                  </div>
                  <div className="flex items-center gap-2">
                    <Badge variant="outline" className="border-primary/50 text-primary">
                      Verifiable
                    </Badge>
                    <Badge variant="outline" className="border-primary/50 text-primary">
                      Non-revealing
                    </Badge>
                  </div>
                </div>
              </TooltipTrigger>
              <TooltipContent className="max-w-xs">
                <p>{tooltipMap.get('encryptedNotes') || 'Shows only cryptographic commitments and hashes. Actual amounts, timing, and wallet IDs remain hidden.'}</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </Card>
      </div>

      {/* Explanation */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <div className="flex gap-4">
          <img 
            src="/assets/generated/privacy-shield.dim_300x300.png" 
            alt="Privacy Shield"
            className="w-20 h-20 object-contain opacity-80"
          />
          <div className="flex-1">
            <h4 className="font-semibold mb-2 text-accent">How It Works</h4>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Your private exit note contains sensitive information that remains encrypted. Only a cryptographic 
              commitment is generated and made public. This commitment proves the note exists without revealing 
              any details about the amount, your wallet, or timing.
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
}
