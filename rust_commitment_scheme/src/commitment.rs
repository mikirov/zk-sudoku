// src/commitments.rs

use rand::Rng;
use sha2::{Sha256, Digest};

pub struct Commitment {
    c: Vec<u8>,
}

pub struct Randomness {
    r: Vec<u8>,
}

impl Commitment {
    pub fn new(m: &[u8]) -> (Self, Randomness) {
        let mut rng = rand::thread_rng();
        let r: [u8; 32] = rng.gen();
        let mut hasher = Sha256::new();
        hasher.update(m);
        hasher.update(&r);
        let c = hasher.finalize().to_vec();

        (Self { c }, Randomness { r: r.to_vec() })
    }

    pub fn verify(&self, m: &[u8], r: &Randomness) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(m);
        hasher.update(&r.r);
        hasher.finalize().as_slice() == self.c.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment() {
        let message = b"Hello, World!";
        let (commitment, randomness) = Commitment::new(message);

        // Verification should succeed with correct data
        assert!(commitment.verify(message, &randomness));

        // Corrupting the message
        let corrupted_message = b"Hello, World?";
        assert!(!commitment.verify(corrupted_message, &randomness));

        // Corrupting the randomness
        let mut corrupted_randomness = randomness.r.clone();
        corrupted_randomness[0] ^= 0x01; // flipping one bit
        assert!(!commitment.verify(message, &Randomness { r: corrupted_randomness }));
    }
}
