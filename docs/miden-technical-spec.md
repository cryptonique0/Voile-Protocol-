# Voile Protocol — Technical Specification (Miden Implementation)

## Document Overview

This document provides the technical specification for implementing Voile Protocol on Miden, including:
- Miden architecture integration details
- MASM smart contract specifications
- Transaction flow and state management
- Security model and privacy guarantees

---

## 1. Miden Architecture Overview

### 1.1 Core Components

Miden is a **rollup for high-throughput, private applications** with the following architecture:

| Component | Description | Voile Usage |
|-----------|-------------|-------------|
| **Miden Client** | Local transaction execution and proof generation | Users generate exit notes and proofs locally |
| **Miden Operator** | Verifies proofs and maintains state database | Verifies exit/settlement proofs, tracks commitments |
| **Private Accounts** | Only commitment tracked on-chain | User & LP account state remains hidden |
| **Private Notes** | Only commitment tracked on-chain | Exit notes with encrypted unstake details |
| **MASM** | Miden Assembly for smart contracts | Custom exit, transfer, and settlement logic |
| **Miden VM** | Turing-complete execution environment | Powers complex exit logic |

### 1.2 Privacy Model

**Private Accounts:**
- Miden operator tracks only a **commitment** to account data
- Users can execute smart contracts only when they know the interface and state
- Perfect for hiding exit positions and balances

**Private Notes:**
- Miden operator tracks only a **commitment** to note data
- Users communicate note details **off-chain** (via side-channels)
- Ideal for private exit tickets that LPs can't observe

---

## 2. Voile Smart Contracts (MASM)

### 2.1 User Account Component

Voile users interact via custom **private accounts** with the following interface:

```masm
# Voile User Account (MASM)
# Private account component for managing exit positions

export.create_exit_note
    # Input: unstake_amount, unlock_timestamp, terms
    # Output: exit_note_commitment
    
    # Validate user has sufficient unstake position
    push.unstake_amount
    exec.account::get_unstake_balance
    exec.assert_sufficient_balance
    
    # Generate exit note with custom script
    push.unlock_timestamp
    push.terms
    exec.notes::create_private_note
    
    # Store note commitment in account state
    exec.account::update_state
    
    # Return note commitment
end

export.receive_liquidity
    # Input: note_commitment (from LP)
    # Output: success
    
    # Validate LP note
    exec.notes::validate_lp_note
    
    # Consume LP note and receive funds
    exec.notes::consume_note
    
    # Update account balance
    exec.account::add_balance
end

export.settle_exit
    # Input: exit_note_id, settlement_proof
    # Output: success
    
    # Verify unstake has unlocked
    exec.time::get_current_block
    exec.assert_unlock_passed
    
    # Create settlement note for LP
    exec.notes::create_settlement_note
    
    # Consume original exit note
    exec.notes::consume_exit_note
    
    # Transfer repayment to LP
    exec.account::transfer_to_lp
end
```

### 2.2 Exit Note Script

The **exit note** is a private note with a custom MASM script executed when an LP consumes it:

```masm
# Exit Note Script (MASM)
# Executed when LP consumes the exit note

begin
    # --- LP Validation ---
    # Verify LP is authorized (reputation, whitelist, etc.)
    exec.lp::validate_eligibility
    
    # --- Exit Details Validation ---
    # Verify note details match commitment
    push.exit_note_data
    exec.crypto::verify_commitment
    
    # --- Calculate Liquidity Advance ---
    # Based on unstake amount and fee structure
    push.unstake_amount
    push.unlock_timestamp
    exec.pricing::calculate_advance_amount
    # Stack: [advance_amount]
    
    # --- Transfer Funds ---
    # Transfer from LP to user's private account
    dup.0  # Duplicate advance_amount
    push.user_account_id
    exec.account::transfer
    
    # --- Lock Repayment Claim ---
    # Create repayment claim note for LP
    push.lp_account_id
    push.repayment_amount
    push.unlock_timestamp
    exec.notes::create_repayment_claim
    
    # --- Update Exit Note State ---
    # Mark as consumed
    exec.notes::mark_consumed
    
    # Return success
    push.1
end
```

### 2.3 Settlement Note Script

The **settlement note** is created when the unstake unlocks and automatically repays the LP:

```masm
# Settlement Note Script (MASM)
# Executed when unstake unlocks to repay LP

begin
    # --- Verify Unlock Timestamp ---
    exec.time::get_current_block
    # Stack: [current_block]
    
    push.unlock_timestamp
    # Stack: [unlock_timestamp, current_block]
    
    exec.assert_gte  # Assert current >= unlock
    
    # --- Calculate Repayment Amount ---
    push.principal_amount
    push.fee_rate
    push.time_elapsed
    exec.pricing::calculate_repayment
    # Stack: [repayment_amount]
    
    # --- Verify User Has Funds ---
    push.user_account_id
    exec.account::get_balance
    # Stack: [user_balance, repayment_amount]
    
    dup.1
    exec.assert_gte  # Assert balance >= repayment
    
    # --- Transfer Repayment to LP ---
    push.repayment_amount
    push.lp_account_id
    exec.account::transfer
    
    # --- Consume Original Exit Note ---
    push.exit_note_id
    exec.notes::consume_note
    
    # --- Consume Repayment Claim ---
    push.repayment_claim_id
    exec.notes::consume_note
    
    # Return success
    push.1
end
```

### 2.4 LP Account Component

LPs use custom account components to interact with exit notes:

```masm
# LP Account Component (MASM)

export.advance_liquidity
    # Input: exit_note_commitment
    # Output: success
    
    # Query exit note commitment from Miden operator
    push.exit_note_commitment
    exec.state::query_note_commitment
    
    # Validate exit note (off-chain coordination for details)
    exec.lp::validate_exit_note
    
    # Create liquidity advance transaction
    push.advance_amount
    exec.notes::create_advance_note
    
    # Consume exit note (triggers exit note script)
    push.exit_note_commitment
    exec.notes::consume_note
    
    # Receive repayment claim note
    exec.notes::store_repayment_claim
end

export.claim_repayment
    # Input: repayment_claim_id
    # Output: success
    
    # Verify settlement conditions met
    exec.lp::verify_settlement_ready
    
    # Consume settlement note
    exec.notes::consume_settlement_note
    
    # Receive repayment funds
    exec.account::add_balance
end
```

---

## 3. Transaction Flow

### 3.1 Exit Note Creation Flow

```
┌─────────────────────────────────────────────────────────────┐
│ USER (Miden Client - Local Execution)                       │
├─────────────────────────────────────────────────────────────┤
│ 1. User calls create_exit_note() on their private account   │
│    - Input: unstake_amount, unlock_timestamp, terms         │
│                                                              │
│ 2. Account validates sufficient unstake balance             │
│                                                              │
│ 3. Miden client creates private note:                       │
│    - Note contains: amount, timestamp, wallet ID (encrypted)│
│    - Custom exit note script (MASM) attached                │
│    - Note commitment computed locally                       │
│                                                              │
│ 4. Miden client executes transaction locally                │
│    - Updates account state (decrements available unstake)   │
│    - Stores note in local database                          │
│                                                              │
│ 5. Miden client generates ZK proof:                         │
│    - Proves valid unstake position                          │
│    - Proves correct note commitment                         │
│    - Proves valid account state transition                  │
│                                                              │
│ 6. Submit proof + note commitment to Miden operator         │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ MIDEN OPERATOR (On-Chain Verification)                      │
├─────────────────────────────────────────────────────────────┤
│ 1. Receives: proof + note_commitment + account_commitment   │
│                                                              │
│ 2. Verifies ZK proof                                        │
│    - Validates proof structure                              │
│    - Checks cryptographic signatures                        │
│    - Ensures state transition is valid                      │
│                                                              │
│ 3. Updates state database:                                  │
│    - Stores note_commitment in public database              │
│    - Updates account_commitment                             │
│    - Stores in Merkle tree for future proofs                │
│                                                              │
│ 4. Returns: transaction_id, block_number                    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ PUBLIC DATABASE                                             │
├─────────────────────────────────────────────────────────────┤
│ • note_commitment: 0x7a3f... (visible)                      │
│ • account_commitment: 0x9c2e... (visible)                   │
│ • block_number: 12345 (visible)                             │
│                                                              │
│ • Note details: HIDDEN (off-chain)                          │
│ • Account state: HIDDEN (off-chain)                         │
│ • Unstake amount: HIDDEN (off-chain)                        │
│ • Unlock timestamp: HIDDEN (off-chain)                      │
│ • User identity: HIDDEN (off-chain)                         │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 LP Liquidity Advance Flow

```
┌─────────────────────────────────────────────────────────────┐
│ LP (Queries Miden Operator)                                 │
├─────────────────────────────────────────────────────────────┤
│ 1. LP queries public database for note commitments          │
│    - Sees: note_commitment: 0x7a3f...                       │
│    - Does NOT see: amount, timestamp, user ID               │
│                                                              │
│ 2. LP requests note details via side-channel:               │
│    - Encrypted messaging with user                          │
│    - User shares: amount, timestamp, terms (off-chain)      │
│    - User provides proof of commitment validity             │
│                                                              │
│ 3. LP validates exit note:                                  │
│    - Verifies commitment matches shared details             │
│    - Checks user reputation/history (optional)              │
│    - Calculates risk and advance amount                     │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ LP (Miden Client - Local Execution)                         │
├─────────────────────────────────────────────────────────────┤
│ 1. LP calls advance_liquidity() on their private account    │
│    - Input: exit_note_commitment                            │
│                                                              │
│ 2. LP's client creates transaction to consume exit note:    │
│    - Transaction includes note consumption                  │
│    - Exit note script (MASM) executes locally               │
│    - Script validates LP, calculates advance, transfers     │
│                                                              │
│ 3. Transaction updates state:                               │
│    - LP account: balance decreases (advance paid)           │
│    - User account: balance increases (liquidity received)   │
│    - Repayment claim note created for LP                    │
│    - Exit note marked as consumed                           │
│                                                              │
│ 4. LP's Miden client generates ZK proof:                    │
│    - Proves valid note consumption                          │
│    - Proves correct script execution                        │
│    - Proves valid state transitions                         │
│                                                              │
│ 5. Submit proof to Miden operator                           │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ MIDEN OPERATOR (Verifies & Updates)                         │
├─────────────────────────────────────────────────────────────┤
│ 1. Verifies LP's proof                                      │
│                                                              │
│ 2. Updates state database:                                  │
│    - Marks exit note as consumed                            │
│    - Updates account commitments (LP & user)                │
│    - Stores repayment claim note commitment                 │
└─────────────────────────────────────────────────────────────┘
```

### 3.3 Automated Settlement Flow

```
┌─────────────────────────────────────────────────────────────┐
│ USER (Monitors Unstake Unlock)                              │
├─────────────────────────────────────────────────────────────┤
│ 1. User's Miden client monitors blockchain time             │
│                                                              │
│ 2. When unlock_timestamp reached:                           │
│    - Client automatically creates settlement transaction    │
│    - Calls settle_exit() on user account                    │
│                                                              │
│ 3. Settlement transaction:                                  │
│    - Creates settlement note with custom MASM script        │
│    - Settlement script calculates repayment amount          │
│    - Transfers funds from user to LP                        │
│    - Consumes original exit note                            │
│    - Consumes LP's repayment claim note                     │
│                                                              │
│ 4. Miden client generates settlement proof                  │
│                                                              │
│ 5. Submits proof to Miden operator                          │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ MIDEN OPERATOR (Final Settlement)                           │
├─────────────────────────────────────────────────────────────┤
│ 1. Verifies settlement proof                                │
│                                                              │
│ 2. Updates state database:                                  │
│    - Marks exit note as fully settled                       │
│    - Updates account commitments (user & LP)                │
│    - Removes consumed notes                                 │
│                                                              │
│ 3. LP receives repayment in their private account           │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. State Management

### 4.1 Miden Operator State Database

The Miden operator maintains the following state:

| Data Structure | Contents | Privacy Level |
|----------------|----------|---------------|
| **Account Commitments** | Hash of account state | Public (commitment only) |
| **Note Commitments** | Hash of note details | Public (commitment only) |
| **Nullifiers** | Consumed note IDs | Public |
| **Merkle Trees** | State commitment tree | Public (roots only) |
| **Block Headers** | Block metadata | Public |

**Key Point**: Only commitments are public. Actual account and note data remain private.

### 4.2 User's Local State

Users maintain the following in their Miden client:

- **Private Account Data**: Full account state, balance, unstake positions
- **Note Details**: Plaintext details of all notes (exit, repayment, etc.)
- **Transaction History**: All past transactions and proofs
- **Keys**: Cryptographic keys for account and note encryption

### 4.3 LP's Local State

LPs maintain:

- **Private Account Data**: Full account state, balance, liquidity pools
- **Active Advances**: List of exit notes they've advanced liquidity for
- **Repayment Claims**: Notes representing expected repayments
- **Off-Chain Coordination Data**: Communication logs with users

---

## 5. Security Model

### 5.1 Privacy Guarantees

| Information | Visibility | Leakage Risk |
|-------------|-----------|--------------|
| **Unstake Amount** | Off-chain only | None (zero-knowledge proofs) |
| **Unlock Timestamp** | Off-chain only | None (encrypted in note) |
| **User Identity** | Off-chain only | None (private accounts) |
| **Exit Intent** | Off-chain only | None (no on-chain signal) |
| **LP Identity** | Off-chain only | None (private accounts) |
| **Advance Amount** | Off-chain only | None (side-channel only) |

### 5.2 Proof System

**ZK Proof Properties:**
- **Soundness**: Invalid transactions cannot produce valid proofs
- **Completeness**: Valid transactions always produce valid proofs
- **Zero-Knowledge**: Proofs reveal nothing beyond validity

**Proof Components:**
- Account state transition validity
- Note commitment correctness
- Script execution correctness
- Balance sufficiency
- Time constraint satisfaction

### 5.3 Attack Vectors & Mitigations

| Attack | Mitigation |
|--------|-----------|
| **Front-running** | No on-chain signals; exit notes are private |
| **MEV Extraction** | Proofs submitted directly; no mempool exposure |
| **Liquidation Hunting** | Exit intent hidden; unstake positions private |
| **Strategy Copying** | All transaction details encrypted |
| **Sybil Attacks** | LP reputation system (optional); stake requirements |
| **Timing Analysis** | Batch transactions; randomize submission times |

---

## 6. Performance Characteristics

### 6.1 Proof Generation

- **Local Execution**: ~1-5 seconds on modern hardware
- **Delegated Proving**: ~10-30 seconds (depending on service)
- **Proof Size**: ~100-500 KB (depending on complexity)

### 6.2 Transaction Throughput

- **Parallel Processing**: Multiple exit notes can be created simultaneously
- **No Global State Contention**: Private accounts eliminate congestion
- **Miden Target**: 1000+ TPS (approaching mainnet)

### 6.3 Cost Analysis

- **Proof Generation**: Free (client-side)
- **Proof Verification**: Low cost (operator verifies off-chain)
- **State Storage**: Minimal (only commitments)
- **Settlement**: Gas costs amortized across many transactions

---

## 7. Integration Guide

### 7.1 Prerequisites

- **Miden Client SDK**: Install latest version
- **MASM Compiler**: For writing smart contracts
- **Miden Node Access**: RPC endpoint for state queries

### 7.2 Development Steps

1. **Design Account Components**
   - Define user and LP account interfaces
   - Write MASM code for account logic

2. **Design Note Scripts**
   - Exit note script (consumption logic)
   - Settlement note script (repayment logic)
   - Test scripts in Miden VM simulator

3. **Implement Client Integration**
   - Integrate Miden client SDK
   - Handle local transaction execution
   - Manage proof generation and submission

4. **Build Off-Chain Coordination**
   - Implement side-channel communication
   - Encrypted messaging between users and LPs
   - Note detail sharing protocol

5. **Deploy & Test**
   - Deploy account components to Miden testnet
   - Test end-to-end flows
   - Monitor performance and optimize

### 7.3 Example: Creating an Exit Note

```typescript
import { MidenClient } from '@miden/client-sdk';

const client = new MidenClient(rpcEndpoint);

// User creates exit note
async function createExitNote(
  unstakeAmount: bigint,
  unlockTimestamp: number,
  terms: ExitTerms
) {
  // 1. Prepare transaction inputs
  const accountId = await client.getAccountId();
  const noteScript = loadExitNoteScript(); // MASM script
  
  // 2. Create private note
  const note = await client.createPrivateNote({
    script: noteScript,
    data: {
      amount: unstakeAmount,
      unlock: unlockTimestamp,
      terms: terms,
    },
  });
  
  // 3. Execute transaction locally
  const tx = await client.executeTransaction({
    account: accountId,
    note: note,
    operation: 'create_exit_note',
  });
  
  // 4. Generate proof
  const proof = await client.generateProof(tx);
  
  // 5. Submit to Miden operator
  const result = await client.submitProof(proof);
  
  return {
    noteCommitment: note.commitment,
    transactionId: result.txId,
  };
}
```

---

## 8. Future Enhancements

### 8.1 Rust Compiler (WIP)

Miden is developing a **Rust to MASM compiler**, which will allow:
- Writing contracts in Rust instead of assembly
- Type-safe contract development
- Better IDE support and tooling

### 8.2 Network Transactions (WIP)

Future support for **network transactions** will enable:
- Outsourced proving for all users
- Public shared state for orderbook-style LPs
- More complex multi-party interactions

### 8.3 Block and Epoch Proofs (WIP)

Recursive proof aggregation will provide:
- Batch verification of many transactions
- Lower per-transaction verification costs
- Improved scalability

---

## 9. References

- **Miden Documentation**: [docs.miden.xyz](https://docs.miden.xyz/intro)
- **Miden GitHub**: [github.com/0xMiden](https://github.com/0xMiden)
- **MASM Reference**: [0xmiden.github.io/miden-vm/user_docs/assembly/main.html](https://0xmiden.github.io/miden-vm/user_docs/assembly/main.html)
- **Telegram**: [t.me/BuildOnMiden](https://t.me/BuildOnMiden)

---

## 10. Conclusion

Voile Protocol leverages Miden's unique architecture to provide:

✅ **Complete Privacy**: Exit intent, amounts, and timing remain off-chain  
✅ **Trustless Execution**: ZK proofs guarantee correctness without revealing data  
✅ **Automated Settlement**: Custom MASM scripts execute trustlessly  
✅ **High Performance**: Parallel processing and local execution  
✅ **Low Costs**: Client-side proving eliminates gas overhead  

**Miden v0.12** is approaching mainnet readiness with a 2026 launch. Voile is positioned to be one of the first privacy-preserving DeFi protocols on the platform.

---

**Document Version**: 1.0  
**Last Updated**: December 2, 2025  
**Status**: Draft Technical Specification
