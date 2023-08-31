use sudoku::Sudoku;
pub struct Challenge {
    pub number: usize,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub statement: Sudoku,
}

#[derive(Debug, PartialEq)]
pub struct Witness {
    pub witness: Sudoku,
}

impl Statement {
    pub fn new(sudoku: Sudoku) -> Statement {
        Statement { statement: sudoku }
    }
}

impl Witness {
    pub fn new(sudoku: Sudoku) -> Witness {
        Witness { witness: sudoku }
    }
}
