use std::collections::HashSet;

use crate::commitment::{Commitment, Randomness};
use crate::error::ProofError;
use crate::structs::{Challenge, Statement};
use crate::utils::{get_column, get_row, get_subgrid};
use rand::Rng;
use sudoku::Sudoku;

#[derive(Debug, PartialEq)]
pub struct Verifier {
    statement: Statement,
    commitments: Vec<Commitment>,
}

impl Verifier {
    pub fn new(sudoku: Sudoku) -> Result<Self, ProofError> {
        if sudoku.is_uniquely_solvable() {
            Ok(Verifier {
                statement: Statement::new(sudoku),
                commitments: Vec::new(),
            })
        } else {
            Err(ProofError::NotSolvable)
        }
    }

    pub fn generate_challenge(&self) -> Challenge {
        let val: usize = rand::thread_rng().gen_range(0..=27);
        Challenge { number: val }
    }

    pub fn check(
        &self,
        challenge: &Challenge,
        decommitments: &(Vec<Commitment>, Vec<Randomness>),
    ) -> Result<(), ProofError> {
        // Extract the commitments and randomness from the tuple for clearer usage
        let (commitments_vec, randomness_vec) = decommitments;

        // Simplified version of checking unique values using HashSet
        if commitments_vec.len() != HashSet::<&Commitment>::from_iter(commitments_vec.iter()).len()
        {
            return Err(ProofError::NonUniqueValues);
        }

        // Iterate over the commitments and check their validity
        for (idx, commitment) in commitments_vec.iter().enumerate() {
            let corresponding_randomness = &randomness_vec[idx];

            let board: Vec<u8> = self.statement.statement.to_bytes().to_vec();
            let message: Vec<u8> = match challenge.number {
                0..=8 => get_row(challenge.number + 1, &board.as_slice()),
                9..=17 => get_column(challenge.number - 9 + 1, &board.as_slice()),
                18..=26 => get_subgrid(challenge.number - 18 + 1, &board.as_slice()),
                27 => board,
                _ => {
                    panic!("Challenge not valid");
                }
            };
            // Verify that the commitment matches with the statement and the randomness
            if !commitment.verify(message.as_slice(), corresponding_randomness) {
                return Err(ProofError::CommitmentMismatch);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prover::{PermutedCommittedSudoku, Prover};
    use std::convert::TryInto;
    use sudoku::Sudoku;

    #[test]
    fn test_verifier_check() {
        let sudoku: Sudoku = Sudoku::generate_unique();
        let solved: Sudoku = sudoku.solve_unique().unwrap();

        let prover: Prover = Prover::sudoku_instance(sudoku.clone(), solved.clone());
        let verifier: Verifier = Verifier::new(sudoku).unwrap();

        let permuted_committed_sudoku: PermutedCommittedSudoku = prover.permute_and_commit();
        let challenge: Challenge = verifier.generate_challenge();
        let proof = prover
            .reveal(&permuted_committed_sudoku, &challenge)
            .unwrap();
        verifier.check(&challenge, &proof).unwrap();

        // Corrupt the board and test again
        let mut corrupted = sudoku.to_bytes().to_vec();
        corrupted[0] = 9 - corrupted[0];
        let array: [u8; 81] = corrupted.try_into().expect("Wrong size");
        let corrupted_sudoku = Sudoku::from_bytes(array).unwrap();

        let corrupted_result = Verifier::new(corrupted_sudoku);
        assert_eq!(corrupted_result, Err(ProofError::NotSolvable));
    }
}
