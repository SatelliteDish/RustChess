pub mod set {
    use std::ops::Not;
    pub const BOARD_SIZE: usize = 8;
    #[derive(Debug)]
    pub struct Board {
        pub board: Vec<Vec<Cell>>,
    }
    impl Board {
        pub fn new() -> Board {
            let mut board: Vec<Vec<Cell>> = Vec::new();
            for _row in 0..BOARD_SIZE {
                let mut color = Color::Black;
                let mut cell_row = Vec::new();
                for _col in 0..BOARD_SIZE {
                    cell_row.push(Cell{color});
                    color = !color;
                }
                board.push(cell_row);
            }
            Board { board }
        }
    }
    #[derive(Debug)]
    pub struct Cell {
        pub color: Color,
    }

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Color {
        White,
        Black,
    }
    impl Not for Color {
        type Output = Self;
        fn not(self) -> Self::Output {
            match self {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::set::*;

    #[test]
    fn board_is_valid_size() {
        let expected_size = BOARD_SIZE;
        let test_board = Board::new();
        let length = test_board.board.len();
        let height = test_board.board[0].len();
        assert!({
            let proper_ratio = { length == height };
            let proper_size = { length == expected_size };
            proper_ratio && proper_size
        });
    }

    #[test]
    fn board_is_proper_pattern() {
        let mut is_valid = true;
        let mut is_white = false;
        let board = Board::new();
        for _row in board.board.iter() {
            for _col in _row.iter() {
                let is_black = _col.color == Color::Black;
                if (is_white && is_black) || (!is_white && !is_black) {
                    is_valid = false;
                }
                is_white = !is_white;
            }
        }
        assert!(is_valid);
    }
}
