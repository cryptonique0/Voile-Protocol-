//! Commitment scheme for Voile Protocol
//!
//! Commitments are cryptographic primitives that allow a party to commit to a value
//! while keeping it hidden, and later reveal the committed value. In Voile, commitments
//! are used to represent exit notes on-chain without revealing their contents.

use sha3::{Digest, Keccak256};
use crate::{Result, VoileError};

/// A cryptographic commitment to a value
///
/// The commitment is computed as: H(value || blinding_factor)
/// where H is Keccak256
#[derive(Clone, PartialEq, Eq)]
pub struct Commitment {
    /// The commitment hash (32 bytes)
    hash: [u8; 32],
}

impl Commitment {
    /// Create a new commitment from a value and blinding factor
    ///
    /// # Arguments
    /// * `value` - The value to commit to
    /// * `blinding_factor` - Random bytes to hide the value
    ///
    /// # Returns
    /// A new Commitment instance
    pub fn new(value: &[u8], blinding_factor: &[u8; 32]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(value);
        hasher.update(blinding_factor);
        let result = hasher.finalize();
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        
        Self { hash }
    }

    /// Verify that a value and blinding factor match this commitment
    ///
    /// # Arguments
    /// * `value` - The claimed original value
    /// * `blinding_factor` - The claimed blinding factor
    ///
    /// # Returns
    /// `true` if the value and blinding factor produce this commitment
    pub fn verify(&self, value: &[u8], blinding_factor: &[u8; 32]) -> bool {
        let computed = Self::new(value, blinding_factor);
        self.hash == computed.hash
    }

    /// Get the commitment hash as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.hash
    }

    /// Create a commitment from raw bytes
    ///
    /// # Arguments
    /// * `bytes` - 32-byte commitment hash
    ///
    /// # Returns
    /// Result containing the Commitment or an error
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 32 {
            return Err(VoileError::InvalidCommitment(
                format!("Expected 32 bytes, got {}", bytes.len())
            ));
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(bytes);
        Ok(Self { hash })
    }

    /// Convert the commitment to a hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.hash)
    }

    /// Create a commitment from a hex string
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s).map_err(|e| {
            VoileError::InvalidCommitment(format!("Invalid hex: {}", e))
        })?;
        Self::from_bytes(&bytes)
    }
}

impl std::fmt::Debug for Commitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Commitment({})", self.to_hex())
    }
}

impl std::fmt::Display for Commitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_creation_and_verification() {
        let value = b"test_unstake_amount_1000";
        let blinding = [42u8; 32];
        
        let commitment = Commitment::new(value, &blinding);
        
        assert!(commitment.verify(value, &blinding));
        assert!(!commitment.verify(b"wrong_value", &blinding));
        assert!(!commitment.verify(value, &[0u8; 32]));
    }

    #[test]
    fn test_commitment_serialization() {
        let value = b"exit_note_data";
        let blinding = [123u8; 32];
        
        let commitment = Commitment::new(value, &blinding);
        let hex_str = commitment.to_hex();
        
        let recovered = Commitment::from_hex(&hex_str).unwrap();
        assert_eq!(commitment, recovered);
    }

    #[test]
    fn test_commitment_bytes_roundtrip() {
        let value = b"private_exit_data";
        let blinding = [99u8; 32];
        
        let commitment = Commitment::new(value, &blinding);
        let bytes = commitment.as_bytes();
        
        let recovered = Commitment::from_bytes(bytes).unwrap();
        assert_eq!(commitment, recovered);
    }

    #[test]
    fn test_invalid_commitment_length() {
        let result = Commitment::from_bytes(&[0u8; 16]);
        assert!(result.is_err());
    }
}
