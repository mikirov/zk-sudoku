pub fn get_row<T: Clone>(index: usize, board: &[T]) -> Vec<T> {
    board[(index - 1) * 9..index * 9].to_vec()
}

pub fn get_column<T: Clone>(index: usize, board: &[T]) -> Vec<T> {
    (index - 1..board.len())
        .step_by(9)
        .map(|i| board[i].clone())
        .collect()
}

pub fn get_subgrid<T: Clone>(index: usize, board: &[T]) -> Vec<T> {
    let row = (index - 1) / 3 * 3;
    let col = (index - 1) % 3 * 3;
    let mut result = vec![];

    for i in row..row + 3 {
        for j in col..col + 3 {
            result.push(board[i * 9 + j].clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use sudoku::Sudoku;

    #[test]
    fn test_grid_functions() {
        let numbers: [u8; 81] = [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];
        let board = Sudoku::from_bytes(numbers).unwrap();

        let row = get_row(1, &board.to_bytes().to_vec());
        assert_eq!(row, vec![5, 3, 0, 0, 7, 0, 0, 0, 0]);

        let column = get_column(1, &board.to_bytes().to_vec());
        assert_eq!(column, vec![5, 6, 0, 8, 4, 7, 0, 0, 0]);

        let subgrid = get_subgrid(5, &board.to_bytes().to_vec());
        assert_eq!(subgrid, vec![0, 6, 0, 8, 0, 3, 0, 2, 0]);
    }
}
