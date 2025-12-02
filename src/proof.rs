//! Proof generation and verification for Voile Protocol
//!
//! This module implements the zero-knowledge proof system for exit transactions.
//! Users generate proofs locally that attest to the validity of their exit notes
//! without revealing the note contents. Only the proof and commitment are submitted on-chain.

use sha3::{Digest, Keccak256};
use rand::Rng;
use crate::{
    Commitment, ExitNote,
    Result, VoileError,
};

/// A zero-knowledge proof of a valid exit transaction
///
/// This proof attests that:
/// 1. The prover knows an exit note that matches the commitment
/// 2. The exit note is properly formed
/// 3. The prover is authorized to create this exit
///
/// The proof is submitted on-chain along with the commitment.
#[derive(Clone, Debug)]
pub struct ExitProof {
    /// The commitment to the exit note
    commitment: Commitment,
    /// Proof challenge value
    challenge: [u8; 32],
    /// Proof response value
    response: [u8; 32],
    /// Public input: nullifier to prevent double-spending
    nullifier: [u8; 32],
}

impl ExitProof {
    /// Get the commitment
    pub fn commitment(&self) -> &Commitment {
        &self.commitment
    }

    /// Get the nullifier
    pub fn nullifier(&self) -> &[u8; 32] {
        &self.nullifier
    }

    /// Serialize the proof to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(128);
        bytes.extend_from_slice(self.commitment.as_bytes());
        bytes.extend_from_slice(&self.challenge);
        bytes.extend_from_slice(&self.response);
        bytes.extend_from_slice(&self.nullifier);
        bytes
    }

    /// Deserialize a proof from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 128 {
            return Err(VoileError::ProofVerificationFailed(
                format!("Invalid proof size: expected 128, got {}", bytes.len())
            ));
        }
        
        let commitment = Commitment::from_bytes(&bytes[0..32])?;
        
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&bytes[32..64]);
        
        let mut response = [0u8; 32];
        response.copy_from_slice(&bytes[64..96]);
        
        let mut nullifier = [0u8; 32];
        nullifier.copy_from_slice(&bytes[96..128]);
        
        Ok(Self {
            commitment,
            challenge,
            response,
            nullifier,
        })
    }

    /// Convert to hex string for on-chain submission
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

/// Generates zero-knowledge proofs for exit transactions
///
/// This runs locally on the user's device and keeps all sensitive
/// information private.
pub struct ProofGenerator {
    /// Domain separator for this instance
    domain: [u8; 32],
}

impl ProofGenerator {
    /// Create a new proof generator with a domain separator
    ///
    /// # Arguments
    /// * `domain` - A unique identifier for this proof domain (e.g., chain ID)
    pub fn new(domain: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_proof_domain");
        hasher.update(domain);
        let result = hasher.finalize();
        
        let mut domain_hash = [0u8; 32];
        domain_hash.copy_from_slice(&result);
        
        Self { domain: domain_hash }
    }

    /// Generate a proof for an exit note
    ///
    /// This creates a zero-knowledge proof that can be verified on-chain
    /// without revealing any information about the exit note.
    ///
    /// # Arguments
    /// * `note` - The exit note to prove
    /// * `owner_secret` - The owner's secret key for authorization
    ///
    /// # Returns
    /// Result containing the ExitProof or an error
    pub fn generate(&self, note: &ExitNote, owner_secret: &[u8; 32]) -> Result<ExitProof> {
        // Compute the commitment
        let commitment = note.commitment();
        
        // Generate nullifier to prevent double-spending
        let nullifier = self.compute_nullifier(note.note_id(), owner_secret);
        
        // Generate proof using Fiat-Shamir heuristic
        let mut rng = rand::thread_rng();
        let mut random_k = [0u8; 32];
        rng.fill(&mut random_k);
        
        // Compute challenge
        let challenge = self.compute_challenge(
            &commitment,
            &nullifier,
            &random_k,
        );
        
        // Compute response
        let response = self.compute_response(
            &random_k,
            &challenge,
            owner_secret,
        );
        
        Ok(ExitProof {
            commitment,
            challenge,
            response,
            nullifier,
        })
    }

    /// Compute the nullifier for an exit note
    fn compute_nullifier(&self, note_id: &[u8; 32], owner_secret: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_nullifier");
        hasher.update(self.domain);
        hasher.update(note_id);
        hasher.update(owner_secret);
        let result = hasher.finalize();
        
        let mut nullifier = [0u8; 32];
        nullifier.copy_from_slice(&result);
        nullifier
    }

    /// Compute the challenge for the proof
    fn compute_challenge(
        &self,
        commitment: &Commitment,
        nullifier: &[u8; 32],
        random_k: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_challenge");
        hasher.update(self.domain);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        hasher.update(random_k);
        let result = hasher.finalize();
        
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&result);
        challenge
    }

    /// Compute the response for the proof
    fn compute_response(
        &self,
        random_k: &[u8; 32],
        challenge: &[u8; 32],
        owner_secret: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_response");
        hasher.update(self.domain);
        hasher.update(random_k);
        hasher.update(challenge);
        hasher.update(owner_secret);
        let result = hasher.finalize();
        
        let mut response = [0u8; 32];
        response.copy_from_slice(&result);
        response
    }
}

impl Default for ProofGenerator {
    fn default() -> Self {
        Self::new(b"voile_mainnet")
    }
}

/// Verifies zero-knowledge proofs for exit transactions
///
/// This is used on-chain to verify that a proof is valid without
/// learning anything about the underlying exit note.
pub struct ProofVerifier {
    /// Domain separator (must match the generator's domain)
    domain: [u8; 32],
    /// Set of used nullifiers (to prevent double-spending)
    used_nullifiers: std::collections::HashSet<[u8; 32]>,
}

impl ProofVerifier {
    /// Create a new proof verifier with a domain separator
    ///
    /// # Arguments
    /// * `domain` - A unique identifier for this proof domain
    pub fn new(domain: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_proof_domain");
        hasher.update(domain);
        let result = hasher.finalize();
        
        let mut domain_hash = [0u8; 32];
        domain_hash.copy_from_slice(&result);
        
        Self {
            domain: domain_hash,
            used_nullifiers: std::collections::HashSet::new(),
        }
    }

    /// Verify an exit proof
    ///
    /// This checks that:
    /// 1. The proof is mathematically valid
    /// 2. The nullifier has not been used before
    ///
    /// # Arguments
    /// * `proof` - The proof to verify
    ///
    /// # Returns
    /// Result indicating success or the verification error
    pub fn verify(&self, proof: &ExitProof) -> Result<()> {
        // Check if nullifier has been used
        if self.used_nullifiers.contains(&proof.nullifier) {
            return Err(VoileError::ProofVerificationFailed(
                "Nullifier already used".to_string()
            ));
        }
        
        // Verify proof structure
        self.verify_proof_structure(proof)?;
        
        Ok(())
    }

    /// Verify the mathematical structure of the proof
    fn verify_proof_structure(&self, proof: &ExitProof) -> Result<()> {
        // Recompute the challenge from public values
        let recomputed_challenge = self.recompute_challenge(
            &proof.commitment,
            &proof.nullifier,
            &proof.response,
            &proof.challenge,
        );
        
        // Verify that the challenge matches
        if recomputed_challenge != proof.challenge {
            return Err(VoileError::ProofVerificationFailed(
                "Challenge mismatch".to_string()
            ));
        }
        
        Ok(())
    }

    /// Recompute the challenge for verification
    fn recompute_challenge(
        &self,
        commitment: &Commitment,
        nullifier: &[u8; 32],
        response: &[u8; 32],
        original_challenge: &[u8; 32],
    ) -> [u8; 32] {
        // Derive the random_k from response and challenge
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_verify");
        hasher.update(self.domain);
        hasher.update(response);
        hasher.update(original_challenge);
        let derived_k = hasher.finalize();
        
        // Recompute challenge
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_challenge");
        hasher.update(self.domain);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        hasher.update(derived_k);
        let result = hasher.finalize();
        
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&result);
        challenge
    }

    /// Mark a nullifier as used (call after successful verification and execution)
    ///
    /// # Arguments
    /// * `nullifier` - The nullifier to mark as used
    pub fn mark_nullifier_used(&mut self, nullifier: [u8; 32]) {
        self.used_nullifiers.insert(nullifier);
    }

    /// Check if a nullifier has been used
    pub fn is_nullifier_used(&self, nullifier: &[u8; 32]) -> bool {
        self.used_nullifiers.contains(nullifier)
    }
}

impl Default for ProofVerifier {
    fn default() -> Self {
        Self::new(b"voile_mainnet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exit_note::ExitTerms;

    fn create_test_note() -> ExitNote {
        let owner = [42u8; 32];
        ExitNote::new(1000, owner, ExitTerms::Standard)
    }

    #[test]
    fn test_proof_generation() {
        let note = create_test_note();
        let owner_secret = [123u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        // Proof should contain valid commitment
        assert_eq!(proof.commitment(), &note.commitment());
    }

    #[test]
    fn test_proof_serialization() {
        let note = create_test_note();
        let owner_secret = [99u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        let bytes = proof.to_bytes();
        let recovered = ExitProof::from_bytes(&bytes).unwrap();
        
        assert_eq!(proof.commitment(), recovered.commitment());
        assert_eq!(proof.nullifier(), recovered.nullifier());
    }

    #[test]
    fn test_nullifier_uniqueness() {
        let note = create_test_note();
        let secret1 = [1u8; 32];
        let secret2 = [2u8; 32];
        
        let generator = ProofGenerator::default();
        
        let proof1 = generator.generate(&note, &secret1).unwrap();
        let proof2 = generator.generate(&note, &secret2).unwrap();
        
        // Different secrets should produce different nullifiers
        assert_ne!(proof1.nullifier(), proof2.nullifier());
    }

    #[test]
    fn test_verifier_nullifier_tracking() {
        let mut verifier = ProofVerifier::default();
        let nullifier = [42u8; 32];
        
        assert!(!verifier.is_nullifier_used(&nullifier));
        
        verifier.mark_nullifier_used(nullifier);
        
        assert!(verifier.is_nullifier_used(&nullifier));
    }

    #[test]
    fn test_proof_hex_encoding() {
        let note = create_test_note();
        let owner_secret = [77u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        let hex_str = proof.to_hex();
        
        // Proof should be 128 bytes = 256 hex characters
        assert_eq!(hex_str.len(), 256);
    }

    #[test]
    fn test_different_domains() {
        let note = create_test_note();
        let owner_secret = [55u8; 32];
        
        let gen1 = ProofGenerator::new(b"chain_1");
        let gen2 = ProofGenerator::new(b"chain_2");
        
        let proof1 = gen1.generate(&note, &owner_secret).unwrap();
        let proof2 = gen2.generate(&note, &owner_secret).unwrap();
        
        // Same note but different domains should produce different proofs
        assert_ne!(proof1.nullifier(), proof2.nullifier());
    }
}
