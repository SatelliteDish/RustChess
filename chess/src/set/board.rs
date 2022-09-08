use crate::set::*;
use std::io;

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

pub struct Board {
    pub board: Vec<Vec<Cell>>,
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
    pub fn new() -> Board {
        let mut board = Vec::new();
        for row in 0..BOARD_SIZE {
            let mut next = {
                if row%2 == 0 {
                    Color::White
                }
                else{
                    Color::Black
                }
            };
        
            board.push(Vec::new());
            for col in 0..BOARD_SIZE {
                let pos = Postition::new(row, col);
                let piece: Option<piece::Piece> = {
                    let color = {
                        if row < BOARD_SIZE/2 {
                            Color::White
                        }
                        else {
                            Color::Black
                        }
                    };
                    if row == 0 || row == BOARD_SIZE - 1 { //First Ranks
                        Some(piece::Piece::new(color.clone(),pos,{
                            match col {
                                0|7 => piece::PieceType::Rook,//Rooks
                                1|6 => piece::PieceType::Knight,//Knights
                                2|5 => piece::PieceType::Bishop,//Bishops
                                _ => {
                                    if next == color {
                                        piece::PieceType::Queen
                                    }
                                    else {
                                        piece::PieceType::King
                                    }
                                }
                            }
                        }))
                    }
                    
                    else if row == 1 || row == BOARD_SIZE - 2 { //Pawn Ranks
                        Some(piece::Piece::new(color,pos,piece::PieceType::Pawn))
                    }
                    else { //Middle Ranks
                        None
                    }
                };
                board[row as usize].push(Cell::new(next.clone(), piece));
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
        Board{
            board,
        }
    }

    fn make_move(piece: piece::Piece,position: Postition) -> Result<String,io::Error> {
        Ok("Temp".to_string())
    }
}

#[derive(Debug)]
pub struct Request {
    piece: piece::Piece,
    position: Postition,
}
impl Request {
    pub fn new(piece: piece::Piece, position: Postition) -> Request {
        Request{
            piece,
            position,
        }
    }
}