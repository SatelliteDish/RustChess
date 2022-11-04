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
                    cell_row.push(Cell {color,piece: None});
                    color = !color;
                }
                cells.push(cell_row);
            }
            Board { cells }
        }
        pub fn start() -> Board {
            let board = Board::new();
            board
        }
        pub fn get_cell(&mut self, transform: Transform) -> &Cell {
            &self.cells[transform.x][transform.y]
        }
    }
    #[derive(Debug)]
    pub struct Cell {
        pub color: Color,
        pub piece: Option<Piece>,
    }
    impl Cell {
        pub fn get_piece(&self) -> &Option<Piece> {
            &self.piece
        }
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
    #[derive(PartialEq, Debug)]
    pub struct Piece {}
    pub struct Transform {
        pub x: usize,
        pub y: usize,
    }

    impl Transform {
        pub fn from(x: usize, y: usize) -> Transform {
            Transform { x, y }
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
        let mut board = Board::start();
        let mut is_valid = true;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let transform: Transform = Transform::from(row, col);
                let cell: &Cell = board.get_cell(transform);
                let piece: &Option<Piece> = cell.get_piece();
                let is_back_rank: bool = {
                    if row == 0 || row == BOARD_SIZE - 1 {
                        true
                    } else {
                        false
                    }
                };
                let is_pawn_rank = {
                    if row == 1 || row == BOARD_SIZE - 2 {
                        true
                    } else {
                        false
                    }
                };
                if is_back_rank || is_pawn_rank {
                    match piece {
                        Some(_t) => (),
                        None => {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }
        }
        assert!(is_valid);
    }
}
