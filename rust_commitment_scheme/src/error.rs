#[derive(Debug, PartialEq)]
pub enum ProofError {
    InvalidSudoku,
    InvalidChallenge,
    InvalidDecommitmentLength,
    NonUniqueValues,
    CommitmentMismatch,
    CommitmentNotFound,
    NotSolvable,
}
