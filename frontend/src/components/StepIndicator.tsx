import { Check } from 'lucide-react';

interface StepIndicatorProps {
  currentStep: number;
  totalSteps: number;
  onStepClick: (step: number) => void;
}

const stepLabels = [
  'Generate Note',
  'Create Proof',
  'Submit to LP',
  'LP Validation',
  'Transfer',
  'Settlement'
];

export default function StepIndicator({ currentStep, totalSteps, onStepClick }: StepIndicatorProps) {
  return (
    <div className="relative">
      {/* Desktop View */}
      <div className="hidden md:flex items-center justify-between">
        {Array.from({ length: totalSteps }, (_, i) => i + 1).map((step, index) => (
          <div key={step} className="flex items-center flex-1">
            <button
              onClick={() => onStepClick(step)}
              className="flex flex-col items-center gap-2 group cursor-pointer"
            >
              <div
                className={`w-12 h-12 rounded-full flex items-center justify-center font-semibold transition-all duration-300 ${
                  step < currentStep
                    ? 'bg-primary text-primary-foreground'
                    : step === currentStep
                    ? 'bg-primary text-primary-foreground ring-4 ring-primary/20 scale-110'
                    : 'bg-muted text-muted-foreground group-hover:bg-muted/80'
                }`}
              >
                {step < currentStep ? <Check className="w-6 h-6" /> : step}
              </div>
              <span
                className={`text-xs font-medium transition-colors ${
                  step <= currentStep ? 'text-foreground' : 'text-muted-foreground'
                }`}
              >
                {stepLabels[index]}
              </span>
            </button>
            {index < totalSteps - 1 && (
              <div className="flex-1 h-0.5 mx-2 bg-border relative">
                <div
                  className="absolute inset-0 bg-primary transition-all duration-500"
                  style={{
                    width: step < currentStep ? '100%' : '0%',
                  }}
                />
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Mobile View */}
      <div className="md:hidden flex items-center justify-center gap-2">
        {Array.from({ length: totalSteps }, (_, i) => i + 1).map((step) => (
          <button
            key={step}
            onClick={() => onStepClick(step)}
            className={`w-10 h-10 rounded-full flex items-center justify-center font-semibold transition-all duration-300 ${
              step < currentStep
                ? 'bg-primary text-primary-foreground'
                : step === currentStep
                ? 'bg-primary text-primary-foreground ring-4 ring-primary/20 scale-110'
                : 'bg-muted text-muted-foreground'
            }`}
          >
            {step < currentStep ? <Check className="w-5 h-5" /> : step}
          </button>
        ))}
      </div>
    </div>
  );
}
