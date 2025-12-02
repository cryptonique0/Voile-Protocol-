# Voile Protocol — Private Exit-Liquidity for Miden

## Overview

Voile is a private exit-liquidity protocol built on Miden's edge-execution architecture. Users generate unstake exits locally and submit only zero-knowledge proofs to the chain, keeping their intent, size, and timing completely hidden. Liquidity providers advance stablecoins against encrypted exit notes without learning user identity or amounts. When an unstake unlocks, settlement occurs through scripted note transfers that repay LPs automatically.

Voile enables **silent exits**, **protected strategies**, and **safer collateral rotation** — all powered by Miden's privacy-native execution.

---

## Why Voile + Miden?

**Miden is a rollup for high-throughput, private applications**, secured by Ethereum and Agglayer. Its architecture is purpose-built for financial privacy:

- **Local Transaction Execution**: The Miden client supports local transaction execution and proof generation. Users execute unstake-exit transactions on their device and submit only proofs to the Miden operator for verification.

- **Private Accounts**: The Miden operator only tracks a **commitment** to account data in the public database. Users can execute smart contracts only when they know the interface and the state — perfect for hiding exit positions.

- **Private Notes**: Like private accounts, the Miden operator only tracks a **commitment** to notes in the public database. Users communicate note details off-chain (via side-channels) to consume private notes in transactions — ideal for private exit tickets.

- **Customized Note Scripts**: Voile writes custom note scripts in **MASM (Miden Assembly)** to express arbitrary exit logic, LP validation, and automated settlement. Note scripts execute when notes are consumed, powered by the Turing-complete Miden VM.

- **Delegated Proving**: The Miden client supports delegated proving, allowing users to offload proof generation to external services when using low-powered devices.

- **Lower Fees**: Client-side proving reduces costs compared to traditional on-chain execution.

- **Ethereum Security**: All transactions are secured by Ethereum and Agglayer.

**Status**: Miden v0.12 — approaching mainnet readiness with 2026 launch.

---

## What Problems Does Voile Solve?

Traditional on-chain exits expose critical information that enables execution-based attacks:

- **Liquidation Hunting**: Attackers see unstake requests and time their liquidations
- **Exit Prediction**: Large exits are visible, allowing frontrunning and price manipulation
- **Slippage Games**: MEV bots exploit predictable exit patterns
- **Strategy Copying**: Competitive traders monitor and replicate successful exit strategies

**Voile removes all on-chain exit signals** by keeping unstake requests, amounts, timing, and wallet identities off-chain, inside the user's proof.

---

## How It Works

### 1. **Generate Private Exit Note (Miden Client)**
The user's Miden client creates a private "exit note" with a custom MASM script containing:
- Pending unstake amount (encrypted)
- Exit terms and timing (encrypted)
- Wallet identity (never revealed)
- Custom note script logic for consumption

Only a cryptographic **commitment** is stored in the Miden operator's state database. The user's **private account** holds the pending unstake position.

### 2. **Local Transaction Execution & Proof Generation**
The user's Miden client:
- Executes the transaction **locally** on their device
- Generates a ZK proof that demonstrates:
  - They own a valid unstake position
  - The exit parameters are legitimate
  - The note commitment is correct
- Can use **delegated proving** for low-powered devices

The proof reveals **nothing** about the actual values.

### 3. **Submit to Miden Network**
The proof and note commitment are submitted to the **Miden operator**:
- The operator **verifies the proof** and updates the state database
- Liquidity providers query note commitments from the public database
- LPs see: A valid cryptographic commitment + proof
- LPs **DON'T see**: amounts, identity, timing (private note model)

### 4. **LP Validation & Liquidity Advance**
LPs interact with the note commitment:
- Validate the ZK proof via the Miden operator
- Communicate with the user **off-chain** (side-channel) to coordinate
- Create a **P2ID-like note script** (MASM) to advance stablecoins
- Execute a **scripted transfer** via Miden's note consumption mechanism
- Remain blind to user identity and details (private account model)

### 5. **Private Transfer**
Upon validation, liquidity is transferred to the user through:
- **Miden's private account system** (only commitment tracked)
- Note consumption executing the transfer script
- User receives funds without revealing identity

### 6. **Automatic Settlement (Custom Note Script)**
When the original unstake unlocks:
- A custom **settlement note** (MASM script) is created
- The original **exit note is consumed** (note consumption on Miden)
- The settlement script executes automatically:
  - Calculates repayment amount
  - Transfers assets to LP's **private account**
- The Miden operator verifies the settlement proof
- All actions remain private through Miden's **private note model**

---

## Key Benefits

### For Users
✅ **Complete Privacy**: Intent, amount, timing, and identity stay off-chain  
✅ **Instant Liquidity**: No waiting for unstake periods  
✅ **MEV Protection**: No on-chain signals for attackers to exploit  
✅ **Strategy Protection**: Competitive strategies remain hidden  

### For Liquidity Providers
✅ **Verifiable Security**: ZK proofs guarantee valid positions without data exposure  
✅ **Automated Repayment**: Settlement notes execute trustlessly  
✅ **Privacy for LPs**: LP identities and positions also remain private  
✅ **Yield Opportunities**: Earn on liquidity provision in a privacy-preserving way  

---

## Interactive Simulator

This repository contains an **interactive web interface** that simulates Voile Protocol's private exit-liquidity architecture. The simulator demonstrates:

- **Step-by-step visualization** of the private exit process
- **Zero-knowledge proof generation** and validation
- **Mock LP interface** showing privacy-preserving interactions
- **Settlement tracking** and note consumption
- **Educational tooltips** explaining privacy concepts

### Running the Simulator

```bash
# Install dependencies
cd frontend
npm install

# Start development server
npm run dev
```

The simulator is purely educational — no real cryptographic operations or blockchain interactions occur.

---

## Building on Miden: Technical Implementation

### Core Components

Voile leverages the following Miden primitives:

#### 1. **Private Accounts**
```masm
# Voile user accounts are private accounts
# Only commitment tracked in Miden operator's state database
# Account code and state visible only to the user
```

- Users hold unstake positions in **private accounts**
- Account state remains hidden from Miden operator and LPs
- Only account commitment stored on-chain

#### 2. **Private Notes with Custom Scripts**
```masm
# Exit Note Script (MASM)
# Executed when LP consumes the note

begin
    # Validate LP eligibility
    # Calculate liquidity advance amount
    # Transfer funds from LP to user
    # Lock repayment claim for LP
end
```

- **Exit notes** contain encrypted unstake details
- Custom MASM scripts define consumption logic
- Only note commitment visible in public database
- Note details communicated off-chain (side-channel)

#### 3. **Settlement Note Script**
```masm
# Settlement Note Script (MASM)
# Executed when unstake unlocks

begin
    # Verify unstake unlock timestamp
    # Calculate repayment amount (principal + fee)
    # Transfer assets from user to LP
    # Consume original exit note
end
```

- Automated execution when conditions met
- Turing-complete logic via Miden VM
- Trustless repayment to LPs

### Development Workflow

#### Step 1: Write MASM Smart Contracts
All account and note logic is written in **MASM (Miden Assembly)**:

- **Account components**: Define user wallet behavior
- **Note scripts**: Define exit, transfer, and settlement logic
- **Transaction scripts**: Orchestrate multi-step operations

Reference: [MASM Documentation](https://0xmiden.github.io/miden-vm/user_docs/assembly/main.html)

#### Step 2: Local Transaction Execution
Users interact with Voile via the **Miden client**:

1. User creates transaction locally
2. Miden client executes transaction
3. Client generates ZK proof
4. Proof + commitment submitted to Miden operator
5. Operator verifies proof and updates state database

#### Step 3: Off-Chain Coordination
Private notes require **side-channel communication**:

- Users and LPs exchange note details off-chain
- Encrypted messaging or direct communication
- LPs learn commitment but not plaintext details

#### Step 4: Note Consumption
When LPs interact with exit notes:

1. LP queries note commitment from Miden operator
2. LP communicates with user off-chain to get note details
3. LP creates transaction to consume note
4. Note script (MASM) executes locally
5. LP submits proof to Miden operator
6. Liquidity transferred via private account system

### Standardized vs. Custom Scripts

Miden provides **standardized note scripts**:
- **P2ID** (Pay-to-ID): Simple transfers to specific accounts
- **P2IDR** (Reclaimable): Sender can reclaim if not consumed
- **SWAP**: Atomic token swaps

**Voile uses custom note scripts** for:
- Complex exit logic (time-locks, conditions)
- LP validation and reputation checks
- Dynamic fee calculation
- Automated settlement triggers

### Security Model

- **Client-side proving**: No trust in Miden operator for privacy
- **ZK proofs**: Computational integrity guaranteed
- **Private accounts**: State hidden from network
- **Private notes**: Details known only to participants
- **Ethereum security**: Final settlement secured by Ethereum

### Performance Benefits

- **Parallel execution**: Multiple exit notes can be processed simultaneously
- **Low fees**: Client-side proving reduces costs
- **Instant finality**: Proofs verified immediately by operator
- **Scalability**: High-throughput via local execution

---

## Technical Architecture

### Frontend
- **React + TypeScript**: Modern UI framework
- **Tailwind CSS**: Responsive design system
- **Lucide Icons**: Clean iconography
- **Step-based Navigation**: Interactive flow control
- **Visualization Components**: Custom privacy-focused UI elements

### Backend (Mock)
- **Motoko** (Internet Computer): Stores simulation templates and educational content
- **Query Functions**: Retrieve step data, help text, and progress tracking

**Note**: This simulator is educational. A production Voile implementation would use:
- **Miden client** for local execution
- **MASM smart contracts** for account and note logic
- **Miden operator** for proof verification and state management

---

## Project Structure

```
voile-protocol-private-exit-liquidity-simulator/
├── README.md                 # This file
├── spec.md                   # Detailed specification
├── backend/
│   └── main.mo              # Motoko backend (mock data)
└── frontend/
    ├── index.html
    ├── tailwind.config.js
    └── src/
        ├── App.tsx
        ├── main.tsx
        ├── components/
        │   ├── Header.tsx
        │   ├── Footer.tsx
        │   ├── SimulationFlow.tsx
        │   ├── StepContent.tsx
        │   ├── StepIndicator.tsx
        │   └── visualizations/
        │       ├── EncryptedNoteVisualization.tsx
        │       ├── ZKProofVisualization.tsx
        │       ├── LPInterfaceVisualization.tsx
        │       ├── TransferVisualization.tsx
        │       ├── ValidationVisualization.tsx
        │       └── SettlementVisualization.tsx
        └── hooks/
            ├── useEditor.ts
            └── useQueries.ts
```

---

## 5-Line Pitch

**Voile is a private exit-liquidity protocol built for Miden.**  
Users generate unstake exits locally and submit only proofs, so their intent, size, and timing stay hidden.  
Liquidity providers advance stablecoins against encrypted exit notes without learning user identity or amounts.  
When an unstake unlocks, settlement occurs through scripted note transfers that repay LPs automatically.  
**Voile enables silent exits, protected strategies, and safer collateral rotation** — all using Miden's privacy-native execution.

---

## One-Liner

Voile is a privacy-first exit-liquidity protocol on Miden that lets users create unstake exits locally and submit only proofs, keeping intent, amount, and timing fully private.

---

## Tweet (≤280 chars)

Voile: a privacy-first exit-liquidity layer on Miden. Users build unstake exits locally and submit only ZK proofs, keeping intent, amount, and timing private. LPs advance stablecoins against encrypted exit notes; settlement is automatic on unstake. Silent exits, safer strategies.

---

## Contributing

This is an educational simulator. Contributions are welcome for:
- Enhanced visualizations
- Additional educational content
- Improved UX/UI
- Documentation improvements

---

## License

MIT License — See `LICENSE` file for details.

---

## Links & Resources

### Miden Documentation
- **Official Docs**: [docs.miden.xyz](https://docs.miden.xyz/intro)
- **Miden GitHub**: [github.com/0xMiden](https://github.com/0xMiden)
- **Miden Assembly (MASM)**: [MASM Documentation](https://0xmiden.github.io/miden-vm/user_docs/assembly/main.html)
- **Telegram**: [t.me/BuildOnMiden](https://t.me/BuildOnMiden)
- **Roadmap**: [miden.xyz/roadmap](https://miden.xyz/roadmap)

### Key Technical Concepts
- **Private Accounts**: Operator tracks only commitment to account data
- **Private Notes**: Off-chain details, on-chain commitments only
- **MASM**: Miden Assembly for writing smart contract logic
- **Local Transaction Execution**: Client-side proving
- **Custom Note Scripts**: Turing-complete exit and settlement logic
- **Delegated Proving**: Offload proof generation for low-powered devices

---

## Contact

For questions or collaboration: [Your contact info]

---

**Voile Protocol**: Silent exits, powered by Miden. 🛡️
