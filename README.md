# Voile Protocol

A private exit-liquidity layer built on Miden's edge-execution architecture.

## Overview

Voile Protocol enables users to unstake and exit their positions privately. Users generate their unstake-exit transaction locally, keep all sensitive details private, and submit only a proof to the chain. **No validator or external observer ever sees the unstake request, the amount, or the timing.**

### How It Works

1. **Local Computation**: The user's device creates a private **exit note** containing their pending unstake and desired terms
2. **Encryption**: The exit note is encrypted locally and only its cryptographic commitment appears on-chain
3. **Proof Generation**: A zero-knowledge proof is generated that attests to the validity of the exit without revealing its contents
4. **On-Chain Submission**: Only the proof and commitment are submitted to the chain

## Features

- **Privacy-Preserving**: Exit amounts, timing, and terms remain completely private
- **Edge Execution**: All sensitive computation happens locally on the user's device
- **Zero-Knowledge Proofs**: Prove validity without revealing information
- **Double-Spend Prevention**: Nullifiers prevent replay attacks
- **Flexible Exit Terms**: Support for immediate, standard, delayed, and custom exit terms

## Installation

Add Voile Protocol to your `Cargo.toml`:

```toml
[dependencies]
voile-protocol = "0.1.0"
```

## Usage

### Creating a Private Exit Note

```rust
use voile_protocol::{ExitNote, ExitTerms, EncryptionKey};

// Create an exit note with your unstake details
let owner = [/* your 32-byte owner identifier */];
let amount = 1000u64; // Amount to unstake
let terms = ExitTerms::Standard;

let exit_note = ExitNote::new(amount, owner, terms);

// Encrypt the note for private storage
let encryption_key = EncryptionKey::generate();
let encrypted_note = exit_note.encrypt(&encryption_key);

// Get the commitment for on-chain publication
let commitment = exit_note.commitment();
println!("On-chain commitment: {}", commitment);
```

### Generating a Proof for Submission

```rust
use voile_protocol::{ExitNote, ExitTerms, ProofGenerator};

// Create your exit note
let owner = [/* your owner identifier */];
let exit_note = ExitNote::new(1000, owner, ExitTerms::Immediate);

// Generate the proof
let generator = ProofGenerator::default();
let owner_secret = [/* your secret key */];
let proof = generator.generate(&exit_note, &owner_secret).unwrap();

// Submit to chain (only proof is revealed, not the note contents)
let proof_bytes = proof.to_hex();
println!("Proof for submission: {}", proof_bytes);
```

### Verifying a Proof On-Chain

```rust
use voile_protocol::{ExitProof, ProofVerifier};

// Verifier (on-chain component)
let mut verifier = ProofVerifier::default();

// Verify the proof
let proof = ExitProof::from_bytes(&proof_bytes).unwrap();
match verifier.verify(&proof) {
    Ok(()) => {
        // Proof is valid - execute the exit
        verifier.mark_nullifier_used(*proof.nullifier());
        println!("Exit executed successfully");
    }
    Err(e) => println!("Verification failed: {}", e),
}
```

## Exit Terms

Voile supports multiple exit term types:

| Term | Description |
|------|-------------|
| `Immediate` | Exit immediately with potential penalty |
| `Standard` | Follow the normal unstaking period |
| `Delayed { blocks }` | Wait additional blocks for better rates |
| `Custom { min_rate_bps, max_slippage_bps }` | Specify custom parameters |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     User's Device                            │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────────┐   │
│  │ Exit Note   │──▶│ Encryption  │──▶│ Proof Generator │   │
│  │ (Private)   │   │             │   │                 │   │
│  └─────────────┘   └─────────────┘   └────────┬────────┘   │
│         │                                      │            │
│         ▼                                      ▼            │
│  ┌─────────────┐                    ┌─────────────────┐    │
│  │ Commitment  │                    │  ZK Proof       │    │
│  └──────┬──────┘                    └────────┬────────┘    │
└─────────┼────────────────────────────────────┼─────────────┘
          │              On-Chain               │
          ▼                                     ▼
┌─────────────────────────────────────────────────────────────┐
│                      Blockchain                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Commitment (32 bytes) + Proof (160 bytes)           │   │
│  │  No exit amount, timing, or terms visible            │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                 │
│                            ▼                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Proof Verifier                          │   │
│  │  - Validates proof cryptographically                 │   │
│  │  - Verifies verification tag                         │   │
│  │  - Checks nullifier not used                         │   │
│  │  - Executes exit if valid                            │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Security

- **Commitment Scheme**: Uses Keccak256 for binding commitments with random blinding factors
- **Encryption**: XOR-based stream cipher with Keccak256 key derivation and random nonces
- **Proof System**: Hash-based proof of knowledge with Fiat-Shamir transform and verification tags
- **Domain Separation**: Each chain/deployment uses a unique domain separator to prevent cross-chain replay
- **Nullifiers**: Derived from note ID and owner secret to prevent double-spending
- **Verification Tags**: Cryptographic binding ensures proofs cannot be forged without knowing the secret

## Testing

Run the test suite:

```bash
cargo test
```

## License

MIT License