use crate::set::*;
    pub struct Board {
        pub board: Vec<Vec<Cell>>,
    }
    
    #[derive(Debug)]
            pub struct Cell {
                color: Color,
                piece: Option<piece::Piece>,
            }
        
            impl Cell {
                pub fn new(color: Color, piece: Option<piece::Piece>) -> Cell {
                    Cell {
                        color,
                        piece,
                    }
                }
            }

    impl Board {
        pub fn print(&self) {
            for row in self.board.iter() {

                let mut s = String::new();
                for col in row.iter() {

                    match &col.piece {
                        Some(piece_type) => {
                            s = s + &piece_type.representation.to_string();
                        }
                        None => match col.color {
                            Color::White => s = s + "⬜",
                            Color::Black => s = s + "⬛",
                        }
                    };
                }
                println!("{}",s);
            }
        }

        pub fn init(mut self) -> Board{

            self.board = Vec::new();

            for row in 0..BOARD_SIZE {
                let mut next = {
                    if row%2 == 0 {
                        Color::White
                    }
                    else{
                        Color::Black
                    }
                };
            
                self.board.push(Vec::new());

                for col in 0..BOARD_SIZE {
                    let pos = Postition::new(row, col);
                    let piece: Option<piece::Piece> = {
                        let col = {
                            if row < BOARD_SIZE/2 {
                                Color::White
                            }
                            else {
                                Color::Black
                            }
                        };
                        if row == 0 || row == BOARD_SIZE - 1 {
                            None
                        }
                        else if row == 1 || row == BOARD_SIZE - 2 {
                            Some(piece::Piece::new(col,pos,piece::PieceType::Pawn))
                        }
                        else {
                            None
                        }
                    };
                    self.board[row as usize].push(Cell::new(next.clone(), piece));
                    match next {
                        Color::White => {
                            next = Color::Black;
                        },
                        Color::Black => {
                            next = Color::White;
                        },
                    };
                }
            }
            self
        }
    }