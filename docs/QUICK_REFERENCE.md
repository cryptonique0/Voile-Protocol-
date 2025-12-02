# Voile Protocol — Quick Reference Guide

## One-Page Overview

### What is Voile?

**Voile is a privacy-first exit-liquidity protocol built on Miden** that lets users create unstake exits locally and submit only proofs, keeping intent, amount, and timing fully private.

---

## Key Miden Technologies Used

| Technology | How Voile Uses It |
|-----------|-------------------|
| **Private Accounts** | User & LP accounts store only commitments on-chain; full state hidden |
| **Private Notes** | Exit notes contain encrypted details; only commitments visible |
| **MASM Scripts** | Custom note scripts define exit, transfer, and settlement logic |
| **Local Execution** | Users generate transactions and proofs on their device |
| **Miden Operator** | Verifies proofs and maintains public commitment database |
| **Side-Channels** | Off-chain communication between users and LPs |

---

## 3-Step User Flow

### 1️⃣ **Create Exit Note**
```
User → Miden Client (Local)
  ├─ Execute: create_exit_note(amount, timestamp)
  ├─ Generate: ZK proof
  └─ Submit: proof + note_commitment → Miden Operator
```
**Result**: Exit note commitment stored on-chain; details remain private

### 2️⃣ **LP Advances Liquidity**
```
LP → Query: note_commitment from Miden Operator
  ├─ Coordinate: off-chain with user (side-channel)
  ├─ Execute: consume exit note (triggers MASM script)
  └─ Transfer: stablecoins to user's private account
```
**Result**: User receives instant liquidity; LP receives repayment claim

### 3️⃣ **Automatic Settlement**
```
User → Monitors: unstake unlock timestamp
  ├─ Execute: settle_exit() when unlocked
  ├─ Script: settlement note (MASM) calculates repayment
  └─ Transfer: assets to LP's private account
```
**Result**: LP receives repayment; exit note consumed

---

## Privacy Guarantees

| Data | On-Chain (Public) | Off-Chain (Private) |
|------|-------------------|---------------------|
| **Note Commitment** | ✅ Visible | Hash only |
| **Account Commitment** | ✅ Visible | Hash only |
| **Unstake Amount** | ❌ Hidden | ✅ User knows |
| **Unlock Timestamp** | ❌ Hidden | ✅ User knows |
| **User Identity** | ❌ Hidden | ✅ User knows |
| **Exit Intent** | ❌ Hidden | No signal |

---

## MASM Smart Contracts

### Exit Note Script
```masm
begin
    exec.lp::validate_eligibility
    exec.pricing::calculate_advance_amount
    exec.account::transfer  # LP → User
    exec.notes::create_repayment_claim
    push.1  # Success
end
```

### Settlement Note Script
```masm
begin
    exec.time::get_current_block
    exec.assert_unlock_passed
    exec.pricing::calculate_repayment
    exec.account::transfer  # User → LP
    exec.notes::consume_exit_note
    push.1  # Success
end
```

---

## Technical Stack

### On-Chain (Miden)
- **Miden Operator**: Proof verification, state database
- **Miden VM**: Executes MASM scripts
- **Public Database**: Stores commitments only

### Client-Side
- **Miden Client SDK**: Local execution, proof generation
- **Private Storage**: Account state, note details, keys
- **Side-Channel**: Encrypted messaging with LPs

### Smart Contracts
- **MASM**: Miden Assembly for account & note logic
- **Custom Scripts**: Exit, transfer, settlement logic
- **Standardized Scripts**: P2ID, P2IDR, SWAP (for reference)

---

## Performance

| Metric | Value |
|--------|-------|
| **Proof Generation** | 1-5 seconds (local) / 10-30 sec (delegated) |
| **Throughput** | 1000+ TPS (Miden target) |
| **Proof Size** | 100-500 KB |
| **Privacy Leakage** | Zero (ZK proofs) |
| **On-Chain Data** | Minimal (commitments only) |

---

## Development Setup

### Prerequisites
```bash
# Install Miden Client SDK
npm install @miden/client-sdk

# Install MASM compiler
cargo install miden-asm
```

### Create Exit Note (TypeScript)
```typescript
import { MidenClient } from '@miden/client-sdk';

const client = new MidenClient(rpcEndpoint);

const { noteCommitment } = await client.createPrivateNote({
  script: exitNoteScript,  // MASM
  data: { amount, timestamp, terms },
});

const proof = await client.generateProof(tx);
await client.submitProof(proof);
```

---

## Key Benefits

### For Users
- ✅ Complete privacy (no on-chain signals)
- ✅ Instant liquidity (no waiting)
- ✅ MEV protection (no frontrunning)
- ✅ Strategy protection (hidden activity)

### For Liquidity Providers
- ✅ Verifiable security (ZK proofs)
- ✅ Automated repayment (MASM scripts)
- ✅ Privacy for LPs (private accounts)
- ✅ Yield opportunities (fee earnings)

---

## Attack Mitigations

| Attack | Mitigation |
|--------|-----------|
| **Frontrunning** | No on-chain signals |
| **MEV Extraction** | Private transaction details |
| **Liquidation Hunting** | Exit intent hidden |
| **Strategy Copying** | All data encrypted |
| **Timing Analysis** | Batch submissions, randomize timing |

---

## Resources

### Documentation
- **Miden Docs**: [docs.miden.xyz/intro](https://docs.miden.xyz/intro)
- **Miden GitHub**: [github.com/0xMiden](https://github.com/0xMiden)
- **MASM Docs**: [0xmiden.github.io/miden-vm](https://0xmiden.github.io/miden-vm/user_docs/assembly/main.html)

### Community
- **Telegram**: [t.me/BuildOnMiden](https://t.me/BuildOnMiden)
- **Roadmap**: [miden.xyz/roadmap](https://miden.xyz/roadmap)

### Voile Docs
- **README**: Complete overview and architecture
- **Pitch Deck**: `docs/voile-pitch.md`
- **Technical Spec**: `docs/miden-technical-spec.md`
- **Simulator**: `frontend/` (educational demo)

---

## Miden Status

- **Current Version**: v0.12
- **Status**: Approaching mainnet readiness
- **Mainnet Launch**: 2026 (planned)
- **Security**: Ethereum + Agglayer
- **Note**: Breaking changes may still occur

---

## Architecture Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                     USER (Miden Client)                       │
│  • Local execution                                            │
│  • Proof generation                                           │
│  • Private account (commitment only on-chain)                 │
└──────────────────────┬───────────────────────────────────────┘
                       │ ZK Proof + Commitment
                       ↓
┌──────────────────────────────────────────────────────────────┐
│                   MIDEN OPERATOR (On-Chain)                   │
│  • Verifies proofs                                            │
│  • Stores commitments in public database                      │
│  • Maintains Merkle trees                                     │
└──────────────────────┬───────────────────────────────────────┘
                       │ Query Commitments
                       ↓
┌──────────────────────────────────────────────────────────────┐
│                    LP (Miden Client)                          │
│  • Queries note commitments                                   │
│  • Off-chain coordination with user                           │
│  • Consumes exit note (executes MASM script)                  │
│  • Private account (commitment only on-chain)                 │
└──────────────────────────────────────────────────────────────┘
```

---

## Sample Transaction Flow

```
1. User creates exit note locally
   └─> Proof + commitment → Miden Operator

2. LP sees commitment (not details)
   └─> Off-chain: LP contacts user via side-channel

3. LP consumes exit note
   └─> MASM script executes: validate, calculate, transfer
   └─> Proof → Miden Operator

4. Settlement triggers automatically when unstake unlocks
   └─> MASM script executes: calculate repayment, transfer
   └─> Proof → Miden Operator
   └─> LP receives repayment
```

---

## FAQs

**Q: What's visible on-chain?**  
A: Only cryptographic commitments (hashes). No amounts, timestamps, or identities.

**Q: How do LPs find exit notes?**  
A: LPs query note commitments from Miden operator, then coordinate off-chain with users.

**Q: Is this trustless?**  
A: Yes. ZK proofs guarantee correctness without revealing data. Miden operator cannot lie.

**Q: What if my device is slow?**  
A: Use Miden's delegated proving service to offload proof generation.

**Q: When does settlement happen?**  
A: Automatically when unstake unlocks. User's client monitors and triggers settlement.

**Q: Can LPs front-run?**  
A: No. Exit intent is hidden; only the user knows details until shared off-chain.

---

## Comparison: Traditional vs. Voile

| Feature | Traditional On-Chain | Voile on Miden |
|---------|---------------------|----------------|
| **Exit Visibility** | Public | Private |
| **Amount Visible** | Yes | No |
| **Timing Visible** | Yes | No |
| **Identity Visible** | Yes | No |
| **MEV Risk** | High | Zero |
| **Instant Liquidity** | No | Yes |
| **Strategy Protection** | None | Complete |

---

**Voile Protocol**  
🛡️ Silent exits, powered by Miden.

**Version**: 1.0  
**Last Updated**: December 2, 2025
