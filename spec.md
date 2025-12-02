# Voile Protocol Private Exit-Liquidity Simulator

## Overview
An interactive web interface that simulates Voile Protocol's private exit-liquidity architecture on Miden, demonstrating how users can generate private exit notes and receive liquidity through zero-knowledge proofs without revealing sensitive transaction details.

## Core Features

### Step-by-Step Simulation Flow
- **Step 1: Generate Private Exit Note** - User creates an encrypted exit note with hidden amount, timing, and wallet ID
- **Step 2: Create Zero-Knowledge Proof** - System generates a cryptographic proof that validates the exit request without revealing private data
- **Step 3: Submit to LP Network** - Proof is submitted to liquidity providers for validation
- **Step 4: LP Validation** - Mock LP interface validates proof authenticity without accessing sensitive user data
- **Step 5: Private Transfer** - Scripted liquidity transfer is triggered upon successful proof validation
- **Step 6: Settlement** - Original note is consumed and LP is repaid, completing the private exit

### Visualization Components
- **Encrypted Exit Notes Display** - Shows only cryptographic commitments and hashes, with clear indicators that actual amounts, timing, and wallet IDs remain hidden
- **Zero-Knowledge Proof Viewer** - Visual representation of proof generation and validation process
- **LP Interface Mock** - Simulated liquidity provider dashboard showing proof validation without data exposure
- **Settlement Tracker** - Real-time visualization of note consumption and LP repayment

### Interactive Elements
- **Progress Animations** - Smooth transitions between simulation steps with loading states and completion indicators
- **Educational Tooltips** - Contextual explanations of privacy concepts, zero-knowledge proofs, and liquidity mechanisms
- **Step Navigation** - Users can move forward/backward through the simulation steps
- **Reset Functionality** - Ability to restart the simulation with new parameters

### Privacy Education
- Clear explanations of what data remains private vs. what is publicly verifiable
- Visual emphasis on the separation between public commitments and private transaction details
- Interactive demonstrations of how zero-knowledge proofs enable validation without data exposure

## Data Storage
The backend stores:
- Simulation step templates and educational content
- Mock transaction data for demonstration purposes
- Progress tracking for user sessions
- Tooltip and help text content

## Technical Requirements
- All simulation data is generated dynamically in the frontend
- No real cryptographic operations or actual blockchain interactions
- Mock data and animations to demonstrate the conceptual flow
- Responsive design for desktop and mobile viewing
