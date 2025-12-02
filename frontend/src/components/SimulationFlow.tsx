import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { ChevronLeft, ChevronRight, RotateCcw } from 'lucide-react';
import { useStepTemplates } from '../hooks/useQueries';
import StepContent from './StepContent';
import StepIndicator from './StepIndicator';

const TOTAL_STEPS = 6;

export default function SimulationFlow() {
  const [currentStep, setCurrentStep] = useState(1);
  const [isAnimating, setIsAnimating] = useState(false);
  const { data: stepTemplates, isLoading } = useStepTemplates();

  const stepMap = new Map(stepTemplates || []);
  const progress = (currentStep / TOTAL_STEPS) * 100;

  const handleNext = () => {
    if (currentStep < TOTAL_STEPS) {
      setIsAnimating(true);
      setTimeout(() => {
        setCurrentStep(currentStep + 1);
        setIsAnimating(false);
      }, 300);
    }
  };

  const handlePrevious = () => {
    if (currentStep > 1) {
      setIsAnimating(true);
      setTimeout(() => {
        setCurrentStep(currentStep - 1);
        setIsAnimating(false);
      }, 300);
    }
  };

  const handleReset = () => {
    setIsAnimating(true);
    setTimeout(() => {
      setCurrentStep(1);
      setIsAnimating(false);
    }, 300);
  };

  const handleStepClick = (step: number) => {
    setIsAnimating(true);
    setTimeout(() => {
      setCurrentStep(step);
      setIsAnimating(false);
    }, 300);
  };

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="max-w-6xl mx-auto">
          <Card className="p-8">
            <div className="flex items-center justify-center">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
            </div>
          </Card>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 md:py-12">
      <div className="max-w-6xl mx-auto space-y-8">
        {/* Hero Section */}
        <div className="text-center space-y-4 mb-12">
          <h2 className="text-3xl md:text-4xl font-bold bg-gradient-to-r from-primary via-accent to-primary bg-clip-text text-transparent">
            Experience Private Exit Liquidity
          </h2>
          <p className="text-muted-foreground max-w-2xl mx-auto">
            Explore how Voile Protocol enables private exits through zero-knowledge proofs on Miden, 
            ensuring your transaction details remain confidential while maintaining verifiability.
          </p>
        </div>

        {/* Progress Bar */}
        <div className="space-y-2">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Progress</span>
            <span className="text-primary font-medium">Step {currentStep} of {TOTAL_STEPS}</span>
          </div>
          <Progress value={progress} className="h-2" />
        </div>

        {/* Step Indicators */}
        <StepIndicator 
          currentStep={currentStep} 
          totalSteps={TOTAL_STEPS}
          onStepClick={handleStepClick}
        />

        {/* Main Content */}
        <div className={`transition-opacity duration-300 ${isAnimating ? 'opacity-0' : 'opacity-100'}`}>
          <StepContent 
            step={currentStep}
            description={stepMap.get(`step${currentStep}`) || ''}
          />
        </div>

        {/* Navigation Controls */}
        <Card className="p-6 bg-card/50 backdrop-blur-sm">
          <div className="flex items-center justify-between gap-4">
            <Button
              variant="outline"
              onClick={handlePrevious}
              disabled={currentStep === 1 || isAnimating}
              className="flex-1 md:flex-none"
            >
              <ChevronLeft className="w-4 h-4 mr-2" />
              Previous
            </Button>

            <Button
              variant="outline"
              onClick={handleReset}
              disabled={isAnimating}
              className="hidden md:flex"
            >
              <RotateCcw className="w-4 h-4 mr-2" />
              Reset
            </Button>

            {currentStep < TOTAL_STEPS ? (
              <Button
                onClick={handleNext}
                disabled={isAnimating}
                className="flex-1 md:flex-none"
              >
                Next
                <ChevronRight className="w-4 h-4 ml-2" />
              </Button>
            ) : (
              <Button
                onClick={handleReset}
                disabled={isAnimating}
                className="flex-1 md:flex-none"
              >
                <RotateCcw className="w-4 h-4 mr-2" />
                Start Over
              </Button>
            )}
          </div>
        </Card>
      </div>
    </div>
  );
}
