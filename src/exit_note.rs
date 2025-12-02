//! Exit Note for Voile Protocol
//!
//! The ExitNote is the core data structure that contains a user's pending unstake
//! and the terms they want. This note is created locally on the user's device,
//! encrypted, and only its commitment appears on-chain.

use crate::{
    Commitment, EncryptedNote, EncryptionKey,
    Result, VoileError,
};
use rand::Rng;

/// Represents the terms of an exit request
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExitTerms {
    /// Immediate exit with potential penalty
    Immediate,
    /// Standard exit following the normal unstaking period
    Standard,
    /// Delayed exit for better rates
    Delayed { blocks: u64 },
    /// Custom terms with specified parameters
    Custom { min_rate_bps: u16, max_slippage_bps: u16 },
}

impl ExitTerms {
    /// Serialize terms to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ExitTerms::Immediate => vec![0],
            ExitTerms::Standard => vec![1],
            ExitTerms::Delayed { blocks } => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(&blocks.to_le_bytes());
                bytes
            }
            ExitTerms::Custom { min_rate_bps, max_slippage_bps } => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(&min_rate_bps.to_le_bytes());
                bytes.extend_from_slice(&max_slippage_bps.to_le_bytes());
                bytes
            }
        }
    }

    /// Deserialize terms from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(VoileError::InvalidExitNote("Empty terms data".to_string()));
        }
        
        match bytes[0] {
            0 => Ok(ExitTerms::Immediate),
            1 => Ok(ExitTerms::Standard),
            2 => {
                if bytes.len() < 9 {
                    return Err(VoileError::InvalidExitNote(
                        "Delayed terms missing blocks".to_string()
                    ));
                }
                let blocks = u64::from_le_bytes(
                    bytes[1..9].try_into().map_err(|_| {
                        VoileError::InvalidExitNote("Invalid blocks data".to_string())
                    })?
                );
                Ok(ExitTerms::Delayed { blocks })
            }
            3 => {
                if bytes.len() < 5 {
                    return Err(VoileError::InvalidExitNote(
                        "Custom terms missing parameters".to_string()
                    ));
                }
                let min_rate_bps = u16::from_le_bytes(
                    bytes[1..3].try_into().map_err(|_| {
                        VoileError::InvalidExitNote("Invalid min_rate_bps data".to_string())
                    })?
                );
                let max_slippage_bps = u16::from_le_bytes(
                    bytes[3..5].try_into().map_err(|_| {
                        VoileError::InvalidExitNote("Invalid max_slippage_bps data".to_string())
                    })?
                );
                Ok(ExitTerms::Custom { min_rate_bps, max_slippage_bps })
            }
            _ => Err(VoileError::InvalidExitNote(
                format!("Unknown terms type: {}", bytes[0])
            )),
        }
    }
}

/// A private exit note containing unstake details
///
/// This note is created locally on the user's device and contains all the
/// sensitive information about the exit request. Only the commitment to
/// the encrypted note appears on-chain.
#[derive(Clone, Debug)]
pub struct ExitNote {
    /// Unique identifier for this exit note
    note_id: [u8; 32],
    /// The amount to unstake (in base units)
    amount: u64,
    /// The owner's public key or address (32 bytes)
    owner: [u8; 32],
    /// The terms of the exit
    terms: ExitTerms,
    /// Timestamp when the note was created
    created_at: u64,
    /// Blinding factor for commitment
    blinding_factor: [u8; 32],
}

impl ExitNote {
    /// Create a new exit note
    ///
    /// # Arguments
    /// * `amount` - The amount to unstake
    /// * `owner` - The owner's identifier (32 bytes)
    /// * `terms` - The exit terms
    ///
    /// # Returns
    /// A new ExitNote with generated ID and blinding factor
    pub fn new(amount: u64, owner: [u8; 32], terms: ExitTerms) -> Self {
        let mut rng = rand::thread_rng();
        
        let mut note_id = [0u8; 32];
        rng.fill(&mut note_id);
        
        let mut blinding_factor = [0u8; 32];
        rng.fill(&mut blinding_factor);
        
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        Self {
            note_id,
            amount,
            owner,
            terms,
            created_at,
            blinding_factor,
        }
    }

    /// Get the note ID
    pub fn note_id(&self) -> &[u8; 32] {
        &self.note_id
    }

    /// Get the unstake amount
    pub fn amount(&self) -> u64 {
        self.amount
    }

    /// Get the owner
    pub fn owner(&self) -> &[u8; 32] {
        &self.owner
    }

    /// Get the exit terms
    pub fn terms(&self) -> &ExitTerms {
        &self.terms
    }

    /// Get the creation timestamp
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Serialize the exit note to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let terms_bytes = self.terms.to_bytes();
        let mut bytes = Vec::with_capacity(32 + 8 + 32 + 8 + 32 + terms_bytes.len() + 2);
        
        bytes.extend_from_slice(&self.note_id);
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(&self.owner);
        bytes.extend_from_slice(&self.created_at.to_le_bytes());
        bytes.extend_from_slice(&self.blinding_factor);
        bytes.extend_from_slice(&(terms_bytes.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&terms_bytes);
        
        bytes
    }

    /// Deserialize an exit note from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 114 {
            return Err(VoileError::InvalidExitNote(
                format!("Exit note too short: {} bytes", bytes.len())
            ));
        }
        
        let mut note_id = [0u8; 32];
        note_id.copy_from_slice(&bytes[0..32]);
        
        let amount = u64::from_le_bytes(
            bytes[32..40].try_into().map_err(|_| {
                VoileError::InvalidExitNote("Invalid amount data".to_string())
            })?
        );
        
        let mut owner = [0u8; 32];
        owner.copy_from_slice(&bytes[40..72]);
        
        let created_at = u64::from_le_bytes(
            bytes[72..80].try_into().map_err(|_| {
                VoileError::InvalidExitNote("Invalid created_at data".to_string())
            })?
        );
        
        let mut blinding_factor = [0u8; 32];
        blinding_factor.copy_from_slice(&bytes[80..112]);
        
        let terms_len = u16::from_le_bytes(
            bytes[112..114].try_into().map_err(|_| {
                VoileError::InvalidExitNote("Invalid terms length data".to_string())
            })?
        ) as usize;
        
        if bytes.len() < 114 + terms_len {
            return Err(VoileError::InvalidExitNote(
                "Exit note truncated".to_string()
            ));
        }
        
        let terms = ExitTerms::from_bytes(&bytes[114..114 + terms_len])?;
        
        Ok(Self {
            note_id,
            amount,
            owner,
            terms,
            created_at,
            blinding_factor,
        })
    }

    /// Compute the commitment for this exit note
    ///
    /// This commitment can be safely published on-chain without revealing
    /// any of the note's contents.
    pub fn commitment(&self) -> Commitment {
        let note_bytes = self.to_bytes();
        Commitment::new(&note_bytes, &self.blinding_factor)
    }

    /// Encrypt the exit note for private storage
    ///
    /// # Arguments
    /// * `key` - The encryption key
    ///
    /// # Returns
    /// An encrypted version of this note
    pub fn encrypt(&self, key: &EncryptionKey) -> EncryptedNote {
        let plaintext = self.to_bytes();
        EncryptedNote::encrypt(key, &plaintext)
    }

    /// Decrypt an exit note
    ///
    /// # Arguments
    /// * `encrypted` - The encrypted note
    /// * `key` - The encryption key
    ///
    /// # Returns
    /// Result containing the decrypted ExitNote or an error
    pub fn decrypt(encrypted: &EncryptedNote, key: &EncryptionKey) -> Result<Self> {
        let plaintext = encrypted.decrypt(key)?;
        Self::from_bytes(&plaintext)
    }

    /// Verify that this note matches a given commitment
    pub fn verify_commitment(&self, commitment: &Commitment) -> bool {
        let expected = self.commitment();
        expected == *commitment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_note_creation() {
        let owner = [42u8; 32];
        let note = ExitNote::new(1000, owner, ExitTerms::Standard);
        
        assert_eq!(note.amount(), 1000);
        assert_eq!(note.owner(), &owner);
        assert_eq!(note.terms(), &ExitTerms::Standard);
    }

    #[test]
    fn test_exit_note_serialization() {
        let owner = [1u8; 32];
        let note = ExitNote::new(5000, owner, ExitTerms::Immediate);
        
        let bytes = note.to_bytes();
        let recovered = ExitNote::from_bytes(&bytes).unwrap();
        
        assert_eq!(note.amount(), recovered.amount());
        assert_eq!(note.owner(), recovered.owner());
        assert_eq!(note.terms(), recovered.terms());
    }

    #[test]
    fn test_exit_note_commitment() {
        let owner = [99u8; 32];
        let note = ExitNote::new(10000, owner, ExitTerms::Standard);
        
        let commitment = note.commitment();
        
        // Commitment should be deterministic for the same note
        assert!(note.verify_commitment(&commitment));
    }

    #[test]
    fn test_exit_note_encryption() {
        let owner = [7u8; 32];
        let note = ExitNote::new(2500, owner, ExitTerms::Delayed { blocks: 100 });
        
        let key = EncryptionKey::generate();
        let encrypted = note.encrypt(&key);
        
        let decrypted = ExitNote::decrypt(&encrypted, &key).unwrap();
        
        assert_eq!(note.amount(), decrypted.amount());
        assert_eq!(note.owner(), decrypted.owner());
        assert_eq!(note.terms(), decrypted.terms());
    }

    #[test]
    fn test_exit_terms_serialization() {
        let terms_cases = vec![
            ExitTerms::Immediate,
            ExitTerms::Standard,
            ExitTerms::Delayed { blocks: 1000 },
            ExitTerms::Custom { min_rate_bps: 9500, max_slippage_bps: 50 },
        ];
        
        for terms in terms_cases {
            let bytes = terms.to_bytes();
            let recovered = ExitTerms::from_bytes(&bytes).unwrap();
            assert_eq!(terms, recovered);
        }
    }

    #[test]
    fn test_different_notes_different_commitments() {
        let owner = [1u8; 32];
        let note1 = ExitNote::new(1000, owner, ExitTerms::Standard);
        let note2 = ExitNote::new(1000, owner, ExitTerms::Standard);
        
        // Different notes (different IDs/blinding) should have different commitments
        assert_ne!(note1.commitment(), note2.commitment());
    }
}
