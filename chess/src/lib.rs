pub mod set {
    use std::ops::Not;
    pub const BOARD_SIZE: usize = 8;
    #[derive(Debug)]
    pub struct Board {
        pub cells: Vec<Vec<Cell>>,
    }
    impl Board {
        pub fn new() -> Board {
            let mut cells: Vec<Vec<Cell>> = Vec::new();
            for _row in 0..BOARD_SIZE {
                let mut color = Color::Black;
                let mut cell_row = Vec::new();
                for _col in 0..BOARD_SIZE {
                    cell_row.push(Cell{color});
                    color = !color;
                }
                cells.push(cell_row);
            }
            Board { cells }
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
        let length = test_board.cells.len();
        let height = test_board.cells[0].len();
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
        for _row in board.cells.iter() {
            for _col in _row.iter() {
                let is_black = _col.color == Color::Black;
                if (is_white && is_black) || (!is_white && !is_black) {
                    is_valid = false;
                    break;
                }
                is_white = !is_white;
            }
        }
        assert!(is_valid);
    }

    #[test]
    fn start_position_is_correct() {
        let board = Board::Start();
        let mut is_valid = true;
        for row in board.cells.iter() {
            for col in row.iter() {
                let piece: Option<Piece> = col.get_piece();
                if (row == 0 || BOARD_SIZE - 1) || (row == 1 || BOARD_SIZE - 2) {
                    let mut _type: PieceType;
                    match piece {
                        Some(t) => _type = t,
                        None => {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }
        }
    } 
}
