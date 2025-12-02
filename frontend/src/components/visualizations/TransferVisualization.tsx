import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { ArrowRight, Loader2, CheckCircle2 } from 'lucide-react';

export default function TransferVisualization() {
  const [progress, setProgress] = useState(0);
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          setIsComplete(true);
          return 100;
        }
        return prev + 1;
      });
    }, 30);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="space-y-6">
      {/* Transfer Flow */}
      <div className="grid md:grid-cols-3 gap-4 items-center">
        <Card className="p-6 bg-gradient-to-br from-accent/10 to-accent/5">
          <div className="text-center">
            <div className="w-16 h-16 rounded-full bg-accent/20 flex items-center justify-center mx-auto mb-3">
              <img 
                src="/assets/generated/liquidity-pool-icon-transparent.dim_200x200.png" 
                alt="LP Pool"
                className="w-10 h-10 object-contain"
              />
            </div>
            <h4 className="font-semibold mb-1">Liquidity Provider</h4>
            <p className="text-xs text-muted-foreground">LP Pool Alpha</p>
            <Badge variant="outline" className="mt-2 border-accent/50 text-accent">
              Source
            </Badge>
          </div>
        </Card>

        <div className="flex flex-col items-center gap-2">
          <ArrowRight className={`w-8 h-8 text-primary ${!isComplete && 'animate-pulse'}`} />
          <div className="text-center">
            <div className="text-xs text-muted-foreground mb-1">Private Transfer</div>
            <Badge variant={isComplete ? 'default' : 'outline'}>
              {isComplete ? 'Complete' : 'In Progress'}
            </Badge>
          </div>
        </div>

        <Card className="p-6 bg-gradient-to-br from-primary/10 to-primary/5">
          <div className="text-center">
            <div className="w-16 h-16 rounded-full bg-primary/20 flex items-center justify-center mx-auto mb-3">
              <div className="w-10 h-10 rounded-full bg-primary/40 flex items-center justify-center text-primary-foreground font-bold">
                U
              </div>
            </div>
            <h4 className="font-semibold mb-1">Your Wallet</h4>
            <p className="text-xs text-muted-foreground">Private Address</p>
            <Badge variant="outline" className="mt-2 border-primary/50 text-primary">
              Destination
            </Badge>
          </div>
        </Card>
      </div>

      {/* Transfer Progress */}
      <Card className="p-6">
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold flex items-center gap-2">
              {isComplete ? (
                <CheckCircle2 className="w-5 h-5 text-primary" />
              ) : (
                <Loader2 className="w-5 h-5 text-primary animate-spin" />
              )}
              Transfer Status
            </h3>
            <span className="text-sm font-medium text-primary">{progress}%</span>
          </div>
          <Progress value={progress} className="h-3" />
          <p className="text-sm text-muted-foreground">
            {isComplete
              ? 'Liquidity has been privately transferred to your wallet. The transaction details remain confidential.'
              : 'Executing private transfer using zero-knowledge proof validation...'}
          </p>
        </div>
      </Card>

      {/* Privacy Guarantee */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <div className="flex gap-4">
          <img 
            src="/assets/generated/privacy-shield.dim_300x300.png" 
            alt="Privacy Shield"
            className="w-20 h-20 object-contain opacity-80"
          />
          <div className="flex-1">
            <h4 className="font-semibold mb-2 text-accent">Privacy Preserved</h4>
            <div className="space-y-2 text-sm text-muted-foreground">
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-1.5 rounded-full bg-primary" />
                <span>Transfer amount remains private</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-1.5 rounded-full bg-primary" />
                <span>Your wallet address is not revealed to the LP</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-1.5 rounded-full bg-primary" />
                <span>Transaction timing stays confidential</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-1.5 rounded-full bg-primary" />
                <span>Only cryptographic proofs are exchanged</span>
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
}
