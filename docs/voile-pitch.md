# Voile Protocol
## Private Exit-Liquidity for Miden

---

### The Problem

Traditional on-chain exits expose critical information:

- 🎯 **Liquidation Hunting** — Attackers see unstake requests and time their liquidations
- 📊 **Exit Prediction** — Large exits enable frontrunning and price manipulation
- 💸 **Slippage Games** — MEV bots exploit predictable exit patterns
- 🔍 **Strategy Copying** — Competitive traders monitor and replicate successful strategies

**All because unstake requests, amounts, timing, and identities are visible on-chain.**

---

### The Solution: Voile

**A privacy-first exit-liquidity protocol built on Miden.**

Users generate unstake exits **locally** and submit **only proofs** → intent, size, and timing stay hidden.

LPs advance stablecoins against **encrypted exit notes** without learning user identity or amounts.

When unstake unlocks, settlement occurs through **scripted note transfers** that repay LPs automatically.

---

### How It Works (Technical Implementation)

```
┌─────────────────────────────────────────────────────────────────────┐
│  1. Generate Private Exit Note (Miden Client - Local)              │
│     • User's Miden client creates a private note with custom MASM  │
│       script containing exit logic                                  │
│     • Note contains: unstake amount, timing, wallet ID (encrypted)  │
│     • Only note commitment is stored in Miden operator's database   │
│     • Private account holds the pending unstake position            │
└─────────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────────┐
│  2. Local Transaction Execution & Proof Generation                  │
│     • Miden client executes transaction locally                     │
│     • Generates ZK proof proving:                                   │
│       - Valid unstake position exists                               │
│       - Exit parameters are legitimate                              │
│       - Note commitment is correct                                  │
│     • Can use delegated proving service for low-powered devices     │
└─────────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────────┐
│  3. Submit to Miden Network (On-Chain)                              │
│     • Proof + note commitment submitted to Miden operator           │
│     • Miden operator verifies proof and updates state database      │
│     • LPs query commitments from public database                    │
│     • LPs DON'T see: amount, timing, identity (private note model)  │
└─────────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────────┐
│  4. LP Validation & Liquidity Advance (Scripted Transfer)          │
│     • LP validates ZK proof via Miden operator                      │
│     • LP communicates with user off-chain (side-channel)            │
│     • LP creates a P2ID-like note script to advance stablecoins     │
│     • Scripted transfer executes via Miden's note consumption       │
│     • LP remains blind to user details (private account model)      │
└─────────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────────┐
│  5. Automatic Settlement (Custom Note Script Execution)             │
│     • When unstake unlocks, custom settlement note is created       │
│     • Original exit note is consumed (note consumption on Miden)    │
│     • Settlement script (MASM) executes automatically:              │
│       - Calculates repayment amount                                 │
│       - Transfers assets to LP's private account                    │
│     • Miden operator verifies settlement proof                      │
│     • All actions remain private via Miden's private note model     │
└─────────────────────────────────────────────────────────────────────┘
```

**Key Miden Components Used:**
- **Private Accounts** (user & LP account state remains hidden)
- **Private Notes** (exit tickets with only commitments visible)
- **Custom Note Scripts** (MASM-based exit & settlement logic)
- **Local Execution** (user generates proofs on their device)
- **Miden Operator** (verifies proofs, maintains state database)
- **Side-Channel Communication** (off-chain coordination between users & LPs)

---

### Why Miden?

Miden is a **rollup for high-throughput, private applications**, secured by Ethereum and Agglayer. Its architecture is purpose-built for financial privacy:

| Feature | How Voile Uses It |
|---------|-------------------|
| **Local Transaction Execution** | Users execute unstake-exit transactions locally on their device and generate ZK proofs; only proofs are submitted to the Miden operator |
| **Private Accounts** | The Miden operator only tracks a **commitment** to account data. Users can execute smart contracts only when they know the interface and state — perfect for private exit positions |
| **Private Notes** | Voile creates **private exit notes** where only a commitment is tracked on-chain. LPs consume these notes via off-chain communication (side-channels) while remaining blind to details |
| **Customized Note Scripts** | Voile implements custom note scripts in **MASM (Miden Assembly)** for exit logic, LP validation, and automated settlement — executed when notes are consumed |
| **Client-Side Proving** | Low fees due to client-side proving; users can also use **delegated proving** on low-powered devices |
| **Turing-Complete Miden VM** | Express arbitrary exit logic and settlement rules thanks to the underlying Turing-complete VM |

**Miden v0.12** — approaching mainnet readiness with 2026 launch.

---

### Key Benefits

#### For Users
- ✅ **Complete Privacy** — Intent, amount, timing, identity stay off-chain
- ✅ **Instant Liquidity** — No waiting for unstake periods
- ✅ **MEV Protection** — No on-chain signals for attackers
- ✅ **Strategy Protection** — Competitive strategies remain hidden

#### For Liquidity Providers
- ✅ **Verifiable Security** — ZK proofs guarantee valid positions
- ✅ **Automated Repayment** — Settlement notes execute trustlessly
- ✅ **Privacy for LPs** — LP identities and positions also remain private
- ✅ **Yield Opportunities** — Earn on privacy-preserving liquidity provision

---

### 5-Line Pitch

**Voile is a private exit-liquidity protocol built for Miden.**

Users generate unstake exits locally and submit only proofs, so their intent, size, and timing stay hidden.

Liquidity providers advance stablecoins against encrypted exit notes without learning user identity or amounts.

When an unstake unlocks, settlement occurs through scripted note transfers that repay LPs automatically.

**Voile enables silent exits, protected strategies, and safer collateral rotation** — all using Miden's privacy-native execution.

---

### One-Liner

**Voile** is a privacy-first exit-liquidity protocol on Miden that lets users create unstake exits locally and submit only proofs, keeping intent, amount, and timing fully private.

---

### Tweet-Length Pitch

Voile: a privacy-first exit-liquidity layer on Miden. Users build unstake exits locally and submit only ZK proofs, keeping intent, amount, and timing private. LPs advance stablecoins against encrypted exit notes; settlement is automatic on unstake. Silent exits, safer strategies.

---

### Demo

**Try the Interactive Simulator:**
```bash
cd frontend
npm install
npm run dev
```

Explore:
- Step-by-step visualization of private exits
- Zero-knowledge proof generation
- Mock LP interface
- Settlement tracking
- Educational tooltips

---

### Technical Resources & Links

#### Miden Documentation
- **Official Docs**: [docs.miden.xyz](https://docs.miden.xyz/intro)
- **Miden GitHub**: [github.com/0xMiden](https://github.com/0xMiden)
- **Miden Assembly (MASM)**: [MASM Documentation](https://0xmiden.github.io/miden-vm/user_docs/assembly/main.html)
- **Telegram**: [t.me/BuildOnMiden](https://t.me/BuildOnMiden)
- **Roadmap**: [miden.xyz/roadmap](https://miden.xyz/roadmap)

#### Voile Protocol
- **Repository**: [GitHub link]
- **Demo**: [Live demo link]
- **Contact**: [Your contact info]

#### Key Concepts
- Private Accounts (only commitment tracked on-chain)
- Private Notes (off-chain details, on-chain commitments)
- MASM (Miden Assembly for smart contract logic)
- Local Transaction Execution (client-side proving)
- Custom Note Scripts (Turing-complete exit logic)

---

### Voile Protocol
🛡️ **Silent exits, powered by Miden.**
