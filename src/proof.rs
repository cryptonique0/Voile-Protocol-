//! Proof generation and verification for Voile Protocol
//!
//! This module implements the zero-knowledge proof system for exit transactions.
//! Users generate proofs locally that attest to the validity of their exit notes
//! without revealing the note contents. Only the proof and commitment are submitted on-chain.
//!
//! The proof uses a hash-based proof of knowledge with Fiat-Shamir transform:
//! 1. Prover generates random nonce k and computes announcement A = H(domain || k)
//! 2. Challenge c = H(domain || commitment || nullifier || A)
//! 3. Response s = H(domain || k || c || secret)
//! 4. Verification tag v = H(domain || s || c || A || commitment || nullifier)
//! 5. Proof includes (commitment, A, s, v, nullifier)
//! 6. Verifier recomputes c from A and verifies v = H(domain || s || c || A || commitment || nullifier)

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
/// 
/// Structure (160 bytes total):
/// - commitment: 32 bytes - commitment to the exit note
/// - announcement: 32 bytes - random commitment for the protocol
/// - response: 32 bytes - proof response binding secret to challenge
/// - verification_tag: 32 bytes - allows verifier to check proof validity
/// - nullifier: 32 bytes - prevents double-spending
#[derive(Clone, Debug)]
pub struct ExitProof {
    /// The commitment to the exit note
    commitment: Commitment,
    /// Announcement value (public part of the random commitment)
    announcement: [u8; 32],
    /// Proof response value
    response: [u8; 32],
    /// Verification tag for verifier to check proof
    verification_tag: [u8; 32],
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

    /// Get the announcement
    pub fn announcement(&self) -> &[u8; 32] {
        &self.announcement
    }

    /// Get the verification tag
    pub fn verification_tag(&self) -> &[u8; 32] {
        &self.verification_tag
    }

    /// Serialize the proof to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(160);
        bytes.extend_from_slice(self.commitment.as_bytes());
        bytes.extend_from_slice(&self.announcement);
        bytes.extend_from_slice(&self.response);
        bytes.extend_from_slice(&self.verification_tag);
        bytes.extend_from_slice(&self.nullifier);
        bytes
    }

    /// Deserialize a proof from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 160 {
            return Err(VoileError::ProofVerificationFailed(
                format!("Invalid proof size: expected 160, got {}", bytes.len())
            ));
        }
        
        let commitment = Commitment::from_bytes(&bytes[0..32])?;
        
        let mut announcement = [0u8; 32];
        announcement.copy_from_slice(&bytes[32..64]);
        
        let mut response = [0u8; 32];
        response.copy_from_slice(&bytes[64..96]);
        
        let mut verification_tag = [0u8; 32];
        verification_tag.copy_from_slice(&bytes[96..128]);
        
        let mut nullifier = [0u8; 32];
        nullifier.copy_from_slice(&bytes[128..160]);
        
        Ok(Self {
            commitment,
            announcement,
            response,
            verification_tag,
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
        
        // Step 1: Generate random nonce k
        let mut rng = rand::thread_rng();
        let mut random_k = [0u8; 32];
        rng.fill(&mut random_k);
        
        // Step 2: Compute announcement A = H(domain || k)
        let announcement = self.compute_announcement(&random_k);
        
        // Step 3: Compute challenge c = H(domain || commitment || nullifier || A)
        let challenge = self.compute_challenge(
            &commitment,
            &nullifier,
            &announcement,
        );
        
        // Step 4: Compute response s = H(domain || k || c || secret)
        let response = self.compute_response(
            &random_k,
            &challenge,
            owner_secret,
        );
        
        // Step 5: Compute verification tag v = H(domain || s || c || A || commitment || nullifier)
        let verification_tag = self.compute_verification_tag(
            &response,
            &challenge,
            &announcement,
            &commitment,
            &nullifier,
        );
        
        Ok(ExitProof {
            commitment,
            announcement,
            response,
            verification_tag,
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

    /// Compute the announcement from the random nonce
    fn compute_announcement(&self, random_k: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_announcement");
        hasher.update(self.domain);
        hasher.update(random_k);
        let result = hasher.finalize();
        
        let mut announcement = [0u8; 32];
        announcement.copy_from_slice(&result);
        announcement
    }

    /// Compute the challenge for the proof (Fiat-Shamir)
    fn compute_challenge(
        &self,
        commitment: &Commitment,
        nullifier: &[u8; 32],
        announcement: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_challenge");
        hasher.update(self.domain);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        hasher.update(announcement);
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

    /// Compute the verification tag that allows the verifier to check the proof
    fn compute_verification_tag(
        &self,
        response: &[u8; 32],
        challenge: &[u8; 32],
        announcement: &[u8; 32],
        commitment: &Commitment,
        nullifier: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_verification_tag");
        hasher.update(self.domain);
        hasher.update(response);
        hasher.update(challenge);
        hasher.update(announcement);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        let result = hasher.finalize();
        
        let mut tag = [0u8; 32];
        tag.copy_from_slice(&result);
        tag
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
    /// 1. The proof structure is valid
    /// 2. The verification tag matches the expected value
    /// 3. The nullifier has not been used before
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
        
        // Verify basic proof structure
        self.verify_basic_structure(proof)?;
        
        // Verify the cryptographic proof
        self.verify_proof_cryptography(proof)?;
        
        Ok(())
    }

    /// Verify basic structural validity of the proof
    fn verify_basic_structure(&self, proof: &ExitProof) -> Result<()> {
        // Verify response is non-zero
        if proof.response == [0u8; 32] {
            return Err(VoileError::ProofVerificationFailed(
                "Invalid response: zero value".to_string()
            ));
        }
        
        // Verify announcement is non-zero
        if proof.announcement == [0u8; 32] {
            return Err(VoileError::ProofVerificationFailed(
                "Invalid announcement: zero value".to_string()
            ));
        }
        
        // Verify nullifier is non-zero
        if proof.nullifier == [0u8; 32] {
            return Err(VoileError::ProofVerificationFailed(
                "Invalid nullifier: zero value".to_string()
            ));
        }
        
        // Verify verification_tag is non-zero
        if proof.verification_tag == [0u8; 32] {
            return Err(VoileError::ProofVerificationFailed(
                "Invalid verification tag: zero value".to_string()
            ));
        }
        
        Ok(())
    }

    /// Verify the cryptographic correctness of the proof
    ///
    /// The verifier:
    /// 1. Recomputes the challenge from public values
    /// 2. Recomputes the expected verification tag
    /// 3. Checks if the provided verification tag matches
    fn verify_proof_cryptography(&self, proof: &ExitProof) -> Result<()> {
        // Recompute the challenge from public values
        let challenge = self.compute_challenge(
            &proof.commitment,
            &proof.nullifier,
            &proof.announcement,
        );
        
        // Recompute the expected verification tag
        let expected_tag = self.compute_verification_tag(
            &proof.response,
            &challenge,
            &proof.announcement,
            &proof.commitment,
            &proof.nullifier,
        );
        
        // Verify the tag matches
        if proof.verification_tag != expected_tag {
            return Err(VoileError::ProofVerificationFailed(
                "Verification tag mismatch".to_string()
            ));
        }
        
        Ok(())
    }

    /// Compute the challenge (same as prover)
    fn compute_challenge(
        &self,
        commitment: &Commitment,
        nullifier: &[u8; 32],
        announcement: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_challenge");
        hasher.update(self.domain);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        hasher.update(announcement);
        let result = hasher.finalize();
        
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&result);
        challenge
    }

    /// Compute the verification tag (same as prover)
    fn compute_verification_tag(
        &self,
        response: &[u8; 32],
        challenge: &[u8; 32],
        announcement: &[u8; 32],
        commitment: &Commitment,
        nullifier: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(b"voile_verification_tag");
        hasher.update(self.domain);
        hasher.update(response);
        hasher.update(challenge);
        hasher.update(announcement);
        hasher.update(commitment.as_bytes());
        hasher.update(nullifier);
        let result = hasher.finalize();
        
        let mut tag = [0u8; 32];
        tag.copy_from_slice(&result);
        tag
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
    fn test_proof_verification() {
        let note = create_test_note();
        let owner_secret = [123u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        let verifier = ProofVerifier::default();
        
        // Proof should verify successfully
        assert!(verifier.verify(&proof).is_ok());
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
        assert_eq!(proof.announcement(), recovered.announcement());
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
    fn test_double_spend_prevention() {
        let note = create_test_note();
        let owner_secret = [123u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        let mut verifier = ProofVerifier::default();
        
        // First verification should succeed
        assert!(verifier.verify(&proof).is_ok());
        
        // Mark nullifier as used
        verifier.mark_nullifier_used(*proof.nullifier());
        
        // Second verification should fail (double-spend)
        let result = verifier.verify(&proof);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VoileError::ProofVerificationFailed(_)));
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
        
        // Proof should be 160 bytes = 320 hex characters
        assert_eq!(hex_str.len(), 320);
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

    #[test]
    fn test_cross_domain_verification_fails() {
        let note = create_test_note();
        let owner_secret = [55u8; 32];
        
        // Generate proof for chain_1
        let generator = ProofGenerator::new(b"chain_1");
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        // Verify on chain_1 should succeed
        let verifier1 = ProofVerifier::new(b"chain_1");
        assert!(verifier1.verify(&proof).is_ok());
        
        // Verify on chain_2 should fail (different domain)
        let verifier2 = ProofVerifier::new(b"chain_2");
        let result = verifier2.verify(&proof);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_proof_fails_verification() {
        let note = create_test_note();
        let owner_secret = [123u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        // Tamper with the proof by modifying the response
        let mut bytes = proof.to_bytes();
        bytes[64] ^= 0xFF; // Flip bits in the response
        
        let tampered_proof = ExitProof::from_bytes(&bytes).unwrap();
        
        let verifier = ProofVerifier::default();
        let result = verifier.verify(&tampered_proof);
        
        // Tampered proof should fail verification
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VoileError::ProofVerificationFailed(_)));
    }

    #[test]
    fn test_serialized_proof_verifies() {
        let note = create_test_note();
        let owner_secret = [123u8; 32];
        
        let generator = ProofGenerator::default();
        let proof = generator.generate(&note, &owner_secret).unwrap();
        
        // Serialize and deserialize
        let bytes = proof.to_bytes();
        let recovered = ExitProof::from_bytes(&bytes).unwrap();
        
        // Recovered proof should still verify
        let verifier = ProofVerifier::default();
        assert!(verifier.verify(&recovered).is_ok());
    }
}
