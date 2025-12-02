import { useState } from 'react';
import { ThemeProvider } from 'next-themes';
import { Toaster } from '@/components/ui/sonner';
import Header from './components/Header';
import Footer from './components/Footer';
import SimulationFlow from './components/SimulationFlow';

export default function App() {
  return (
    <ThemeProvider attribute="class" defaultTheme="dark" enableSystem>
      <div className="min-h-screen flex flex-col bg-background">
        <Header />
        <main className="flex-1">
          <SimulationFlow />
        </main>
        <Footer />
        <Toaster />
      </div>
    </ThemeProvider>
  );
}
