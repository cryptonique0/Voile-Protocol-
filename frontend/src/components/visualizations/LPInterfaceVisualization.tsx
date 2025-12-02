import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Send, Loader2, CheckCircle2 } from 'lucide-react';

export default function LPInterfaceVisualization() {
  const [isSubmitting, setIsSubmitting] = useState(true);
  const [isSubmitted, setIsSubmitted] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsSubmitting(false);
      setIsSubmitted(true);
    }, 2000);
    return () => clearTimeout(timer);
  }, []);

  return (
    <div className="space-y-6">
      <div className="grid md:grid-cols-3 gap-4">
        {/* LP Pool 1 */}
        <Card className="p-5 bg-gradient-to-br from-primary/5 to-accent/5 border-primary/20">
          <div className="flex items-center gap-3 mb-3">
            <img 
              src="/assets/generated/liquidity-pool-icon-transparent.dim_200x200.png" 
              alt="LP Pool"
              className="w-12 h-12 object-contain"
            />
            <div>
              <div className="font-semibold">LP Pool Alpha</div>
              <div className="text-xs text-muted-foreground">Capacity: 50K MIDEN</div>
            </div>
          </div>
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">APY:</span>
              <span className="font-medium text-primary">12.5%</span>
            </div>
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Utilization:</span>
              <span className="font-medium">68%</span>
            </div>
            <Badge variant="outline" className="w-full justify-center border-primary/50 text-primary">
              {isSubmitted ? 'Proof Received' : 'Available'}
            </Badge>
          </div>
        </Card>

        {/* LP Pool 2 */}
        <Card className="p-5 bg-gradient-to-br from-primary/5 to-accent/5 border-primary/20">
          <div className="flex items-center gap-3 mb-3">
            <img 
              src="/assets/generated/liquidity-pool-icon-transparent.dim_200x200.png" 
              alt="LP Pool"
              className="w-12 h-12 object-contain"
            />
            <div>
              <div className="font-semibold">LP Pool Beta</div>
              <div className="text-xs text-muted-foreground">Capacity: 75K MIDEN</div>
            </div>
          </div>
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">APY:</span>
              <span className="font-medium text-primary">11.8%</span>
            </div>
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Utilization:</span>
              <span className="font-medium">54%</span>
            </div>
            <Badge variant="outline" className="w-full justify-center border-primary/50 text-primary">
              {isSubmitted ? 'Proof Received' : 'Available'}
            </Badge>
          </div>
        </Card>

        {/* LP Pool 3 */}
        <Card className="p-5 bg-gradient-to-br from-primary/5 to-accent/5 border-primary/20">
          <div className="flex items-center gap-3 mb-3">
            <img 
              src="/assets/generated/liquidity-pool-icon-transparent.dim_200x200.png" 
              alt="LP Pool"
              className="w-12 h-12 object-contain"
            />
            <div>
              <div className="font-semibold">LP Pool Gamma</div>
              <div className="text-xs text-muted-foreground">Capacity: 100K MIDEN</div>
            </div>
          </div>
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">APY:</span>
              <span className="font-medium text-primary">13.2%</span>
            </div>
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Utilization:</span>
              <span className="font-medium">72%</span>
            </div>
            <Badge variant="outline" className="w-full justify-center border-primary/50 text-primary">
              {isSubmitted ? 'Proof Received' : 'Available'}
            </Badge>
          </div>
        </Card>
      </div>

      {/* Submission Status */}
      <Card className="p-6 bg-accent/5 border-accent/20">
        <div className="flex items-center gap-4">
          {isSubmitting ? (
            <>
              <Loader2 className="w-8 h-8 text-primary animate-spin" />
              <div>
                <h4 className="font-semibold text-accent">Broadcasting to LP Network</h4>
                <p className="text-sm text-muted-foreground">
                  Your zero-knowledge proof is being submitted to available liquidity providers...
                </p>
              </div>
            </>
          ) : (
            <>
              <CheckCircle2 className="w-8 h-8 text-primary" />
              <div>
                <h4 className="font-semibold text-accent">Successfully Submitted</h4>
                <p className="text-sm text-muted-foreground">
                  Your proof has been received by 3 liquidity providers. They can now validate it 
                  without accessing your private transaction details.
                </p>
              </div>
            </>
          )}
        </div>
      </Card>

      {/* Encrypted Data Pattern */}
      <Card className="p-6 overflow-hidden relative">
        <img 
          src="/assets/generated/encrypted-data-pattern.dim_800x400.png" 
          alt="Encrypted Data Pattern"
          className="w-full h-32 object-cover rounded-lg opacity-30"
        />
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="text-center">
            <h4 className="font-semibold mb-1">Data Remains Encrypted</h4>
            <p className="text-sm text-muted-foreground">
              LPs receive only the proof, not your private data
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
}
