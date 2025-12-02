//! Error types for Voile Protocol

use thiserror::Error;

/// Errors that can occur during Voile Protocol operations
#[derive(Error, Debug)]
pub enum VoileError {
    /// Invalid commitment format or data
    #[error("Invalid commitment: {0}")]
    InvalidCommitment(String),

    /// Encryption operation failed
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// Decryption operation failed
    #[error("Decryption error: {0}")]
    DecryptionError(String),

    /// Invalid exit note data
    #[error("Invalid exit note: {0}")]
    InvalidExitNote(String),

    /// Proof generation failed
    #[error("Proof generation error: {0}")]
    ProofGenerationError(String),

    /// Proof verification failed
    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),

    /// Invalid key format
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}
