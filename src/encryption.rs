//! Encryption utilities for Voile Protocol
//!
//! This module provides encryption and decryption functionality for exit notes.
//! The encrypted notes contain the user's pending unstake and terms, but only
//! the commitment to the encrypted data appears on-chain.

use sha3::{Digest, Keccak256};
use rand::Rng;
use crate::{Result, VoileError};

/// Size of the encryption key in bytes
pub const KEY_SIZE: usize = 32;

/// Encryption key for exit notes
#[derive(Clone)]
pub struct EncryptionKey {
    /// The raw key bytes
    key: [u8; KEY_SIZE],
}

impl EncryptionKey {
    /// Generate a new random encryption key
    pub fn generate() -> Self {
        let mut key = [0u8; KEY_SIZE];
        rand::thread_rng().fill(&mut key);
        Self { key }
    }

    /// Create an encryption key from raw bytes
    ///
    /// # Arguments
    /// * `bytes` - 32-byte key material
    ///
    /// # Returns
    /// Result containing the EncryptionKey or an error
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != KEY_SIZE {
            return Err(VoileError::InvalidKey(
                format!("Expected {} bytes, got {}", KEY_SIZE, bytes.len())
            ));
        }
        let mut key = [0u8; KEY_SIZE];
        key.copy_from_slice(bytes);
        Ok(Self { key })
    }

    /// Get the key as bytes
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.key
    }

    /// Derive a nonce from a counter value
    fn derive_nonce(&self, counter: u64) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_nonce");
        hasher.update(self.key);
        hasher.update(counter.to_le_bytes());
        let result = hasher.finalize();
        let mut nonce = [0u8; 32];
        nonce.copy_from_slice(&result);
        nonce
    }

    /// Derive a keystream for XOR encryption
    fn derive_keystream(&self, nonce: &[u8; 32], length: usize) -> Vec<u8> {
        let mut keystream = Vec::with_capacity(length);
        let mut block_counter = 0u64;
        
        while keystream.len() < length {
            let mut hasher = Keccak256::new();
            hasher.update(self.key);
            hasher.update(nonce);
            hasher.update(block_counter.to_le_bytes());
            let block = hasher.finalize();
            
            for byte in block.iter() {
                if keystream.len() >= length {
                    break;
                }
                keystream.push(*byte);
            }
            block_counter += 1;
        }
        
        keystream
    }
}

/// An encrypted exit note
///
/// Contains the ciphertext and counter needed for decryption.
/// Only the commitment to this encrypted data appears on-chain.
#[derive(Clone)]
pub struct EncryptedNote {
    /// The encrypted data
    ciphertext: Vec<u8>,
    /// Counter used for nonce derivation
    counter: u64,
}

impl EncryptedNote {
    /// Encrypt plaintext data using the provided key
    ///
    /// # Arguments
    /// * `key` - The encryption key
    /// * `plaintext` - The data to encrypt
    ///
    /// # Returns
    /// A new EncryptedNote containing the ciphertext
    pub fn encrypt(key: &EncryptionKey, plaintext: &[u8]) -> Self {
        let counter = rand::thread_rng().gen();
        let nonce = key.derive_nonce(counter);
        let keystream = key.derive_keystream(&nonce, plaintext.len());
        
        let ciphertext: Vec<u8> = plaintext
            .iter()
            .zip(keystream.iter())
            .map(|(p, k)| p ^ k)
            .collect();
        
        Self { ciphertext, counter }
    }

    /// Decrypt the note using the provided key
    ///
    /// # Arguments
    /// * `key` - The encryption key
    ///
    /// # Returns
    /// Result containing the decrypted plaintext or an error
    pub fn decrypt(&self, key: &EncryptionKey) -> Result<Vec<u8>> {
        let nonce = key.derive_nonce(self.counter);
        let keystream = key.derive_keystream(&nonce, self.ciphertext.len());
        
        let plaintext: Vec<u8> = self.ciphertext
            .iter()
            .zip(keystream.iter())
            .map(|(c, k)| c ^ k)
            .collect();
        
        Ok(plaintext)
    }

    /// Get the ciphertext bytes
    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    /// Serialize the encrypted note to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.ciphertext.len());
        bytes.extend_from_slice(&self.counter.to_le_bytes());
        bytes.extend_from_slice(&self.ciphertext);
        bytes
    }

    /// Deserialize an encrypted note from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(VoileError::DecryptionError(
                "Encrypted note too short".to_string()
            ));
        }
        
        let counter = u64::from_le_bytes(
            bytes[0..8].try_into().map_err(|_| {
                VoileError::DecryptionError("Invalid counter data".to_string())
            })?
        );
        let ciphertext = bytes[8..].to_vec();
        
        Ok(Self { ciphertext, counter })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let key = EncryptionKey::generate();
        let plaintext = b"unstake_amount:1000,timing:immediate,terms:standard";
        
        let encrypted = EncryptedNote::encrypt(&key, plaintext);
        let decrypted = encrypted.decrypt(&key).unwrap();
        
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_different_keys_produce_different_ciphertext() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();
        let plaintext = b"private_exit_data";
        
        let enc1 = EncryptedNote::encrypt(&key1, plaintext);
        let enc2 = EncryptedNote::encrypt(&key2, plaintext);
        
        // Ciphertexts should be different (with overwhelming probability)
        assert_ne!(enc1.ciphertext(), enc2.ciphertext());
    }

    #[test]
    fn test_wrong_key_decryption() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();
        let plaintext = b"secret_unstake_request";
        
        let encrypted = EncryptedNote::encrypt(&key1, plaintext);
        let decrypted = encrypted.decrypt(&key2).unwrap();
        
        // Decryption with wrong key produces garbage
        assert_ne!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_encrypted_note_serialization() {
        let key = EncryptionKey::generate();
        let plaintext = b"exit_note_with_terms";
        
        let encrypted = EncryptedNote::encrypt(&key, plaintext);
        let bytes = encrypted.to_bytes();
        
        let recovered = EncryptedNote::from_bytes(&bytes).unwrap();
        let decrypted = recovered.decrypt(&key).unwrap();
        
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_key_from_bytes() {
        let original = EncryptionKey::generate();
        let bytes = original.as_bytes();
        
        let recovered = EncryptionKey::from_bytes(bytes).unwrap();
        
        assert_eq!(original.as_bytes(), recovered.as_bytes());
    }

    #[test]
    fn test_invalid_key_length() {
        let result = EncryptionKey::from_bytes(&[0u8; 16]);
        assert!(result.is_err());
    }
}
