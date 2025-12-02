//! # Voile Protocol
//!
//! A private exit-liquidity layer built on Miden's edge-execution architecture.
//!
//! Users generate their unstake-exit transaction locally, keep all sensitive details private,
//! and submit only a proof to the chain. No validator or external observer ever sees the
//! unstake request, the amount, or the timing.
//!
//! ## Architecture
//!
//! Inside Voile, the user's device creates a private exit note, which contains their pending
//! unstake and the terms they want. This note is encrypted and only its commitment appears on-chain.

pub mod commitment;
pub mod encryption;
pub mod exit_note;
pub mod proof;
pub mod error;

pub use commitment::Commitment;
pub use encryption::{EncryptedNote, EncryptionKey};
pub use exit_note::ExitNote;
pub use proof::{ExitProof, ProofGenerator, ProofVerifier};
pub use error::VoileError;

/// Result type for Voile Protocol operations
pub type Result<T> = std::result::Result<T, VoileError>;
