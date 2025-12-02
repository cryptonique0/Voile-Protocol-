import { Shield, Info } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { useHelpTexts } from '../hooks/useQueries';

export default function Header() {
  const { data: helpTexts } = useHelpTexts();
  const helpMap = new Map(helpTexts || []);

  return (
    <header className="border-b border-border bg-card/50 backdrop-blur-sm sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-primary to-accent flex items-center justify-center">
              <Shield className="w-6 h-6 text-primary-foreground" />
            </div>
            <div>
              <h1 className="text-xl font-bold text-foreground">Voile Protocol</h1>
              <p className="text-xs text-muted-foreground">Private Exit Liquidity Simulator</p>
            </div>
          </div>
          
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="outline" size="sm">
                <Info className="w-4 h-4 mr-2" />
                About
              </Button>
            </DialogTrigger>
            <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
              <DialogHeader>
                <DialogTitle>About Voile Protocol</DialogTitle>
                <DialogDescription className="space-y-4 pt-4 text-sm">
                  <div className="border-l-4 border-primary pl-4 py-2 bg-muted/50 rounded-r">
                    <p className="font-semibold text-foreground">
                      Voile is a privacy-first exit-liquidity protocol on Miden that lets users create unstake exits locally and submit only proofs, keeping intent, amount, and timing fully private.
                    </p>
                  </div>
                  
                  <div>
                    <h4 className="font-semibold text-foreground mb-2">How It Works</h4>
                    <p className="text-muted-foreground">
                      Users generate unstake exits locally and submit only zero-knowledge proofs to the chain. 
                      Liquidity providers advance stablecoins against encrypted exit notes without learning user identity or amounts. 
                      When an unstake unlocks, settlement occurs through scripted note transfers that repay LPs automatically.
                    </p>
                  </div>

                  <div>
                    <h4 className="font-semibold text-foreground mb-2">Key Benefits</h4>
                    <ul className="space-y-1 text-muted-foreground list-disc list-inside">
                      <li><strong>Silent Exits:</strong> No on-chain signals for attackers to exploit</li>
                      <li><strong>Protected Strategies:</strong> Competitive strategies remain hidden</li>
                      <li><strong>MEV Protection:</strong> Eliminates liquidation hunting and exit prediction</li>
                      <li><strong>Instant Liquidity:</strong> No waiting for unstake periods</li>
                    </ul>
                  </div>

                  <div>
                    <h4 className="font-semibold text-foreground mb-2">About This Simulator</h4>
                    <p className="text-muted-foreground">
                      {helpMap.get('privacy') || 'Your transaction details remain private. Only cryptographic commitments are visible.'}
                    </p>
                    <p className="text-muted-foreground mt-2">
                      {helpMap.get('progress') || 'Follow the step-by-step process to understand how private exit liquidity works.'}
                    </p>
                    <p className="text-muted-foreground mt-2">
                      {helpMap.get('reset') || 'You can restart the simulation at any time with new parameters.'}
                    </p>
                  </div>

                  <div className="pt-2 border-t border-border">
                    <p className="text-xs text-muted-foreground italic">
                      Powered by Miden's privacy-native execution: local computation, private accounts, and zero-knowledge proofs.
                    </p>
                  </div>
                </DialogDescription>
              </DialogHeader>
            </DialogContent>
          </Dialog>
        </div>
      </div>
    </header>
  );
}
