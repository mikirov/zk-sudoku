use crate::commitment::{Commitment, Randomness};
use crate::error::ProofError;
use crate::structs::{Challenge, Statement, Witness};
use crate::utils::{get_column, get_row, get_subgrid};
use sudoku::Sudoku;

use rand::prelude::SliceRandom;

pub struct Prover {
    statement: Statement,
    witness: Witness,
}

pub struct PermutedCommittedSudoku {
    pub commitments: Vec<Commitment>,
    pub randomness: Vec<Randomness>,
    pub permuted_sudoku: Vec<u8>,
}

impl Prover {
    pub fn sudoku_instance(sudoku: Sudoku, solved_sudoku: Sudoku) -> Self {
        Prover {
            statement: Statement::new(sudoku),
            witness: Witness::new(solved_sudoku),
        }
    }

    pub fn permute_and_commit(&self) -> PermutedCommittedSudoku {
        let mut rng = rand::thread_rng();
        let mut values: Vec<u8> = (1..=9).collect();
        values.shuffle(&mut rng);

        let mut commitments: Vec<Commitment> = vec![];
        let mut randomness: Vec<Randomness> = vec![];
        let mut permuted_sudoku = vec![];

        for &val in self.witness.witness.to_bytes().iter() {
            if val == 0 {
                let (commit, rand) = Commitment::new(&[0]);
                commitments.push(commit);
                randomness.push(rand);
                permuted_sudoku.push(0);
                continue;
            }
            let permuted_val = values[(val - 1) as usize];
            permuted_sudoku.push(permuted_val);

            let (commit, rand) = Commitment::new(&[permuted_val]);
            commitments.push(commit);
            randomness.push(rand);
        }

        PermutedCommittedSudoku {
            commitments,
            randomness,
            permuted_sudoku,
        }
    }

    pub fn reveal(
        &self,
        committed: &PermutedCommittedSudoku,
        challenge: &Challenge,
    ) -> Result<(Vec<Commitment>, Vec<Randomness>), ProofError> {
        let i = challenge.number;

        let (uncommited_values_vec, randomness_vec) = match i {
            0..=8 => {
                let row_values = get_row(i + 1, &committed.permuted_sudoku);
                Some((
                    row_values.iter().map(|&val| Commitment::from_value(val)).collect(),
                    get_row(i + 1, &committed.randomness),
                ))
            },
            9..=17 => {
                let col_values = get_column(i - 9 + 1, &committed.permuted_sudoku);
                Some((
                    col_values.iter().map(|&val| Commitment::from_value(val)).collect(),
                    get_column(i - 9 + 1, &committed.randomness),
                ))
            },
            18..=26 => {
                let subgrid_values = get_subgrid(i - 18 + 1, &committed.permuted_sudoku);
                Some(( 
                    subgrid_values.iter().map(|&val| Commitment::from_value(val)).collect(),
                    get_subgrid(i - 18 + 1, &committed.randomness),
                ))
            },
            27 => {
                let known_values: Vec<Commitment> = self
                    .statement
                    .statement
                    .iter()
                    .enumerate()
                    .filter_map(|(index, v)| {
                        if v != Some(0) {
                            return Some(Commitment::from_value(committed.permuted_sudoku[index]));
                        }
                        None
                    })
                    .collect();

                let random_values: Vec<Randomness> = self
                    .statement
                    .statement
                    .iter()
                    .enumerate()
                    .filter_map(|(index, v)| {
                        if v != Some(0) {
                            return Some(committed.randomness[index].clone());
                        }
                        None
                    })
                    .collect();
                Some((known_values, random_values))
            }
            _ => None,
        }
        .unwrap();

        Ok((uncommited_values_vec, randomness_vec))
    }
}
