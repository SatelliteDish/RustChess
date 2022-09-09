use colored::Colorize;
use std::collections::HashMap;
use crate::set::*;

#[derive(Debug, Clone,Copy)]
pub enum PieceType{
    Pawn{has_moved: bool},
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug,Clone)]
pub struct Piece {
    pub color: Color,
    pub position: Position,
    pub piece_type: PieceType,
    pub representation: String,
    pub possible_moves: HashMap<&board::Cell,Position>,
    board: &board::Board,
}
impl Piece {
    pub fn new(color: Color, position: Position, piece_type: PieceType, board: &board::Board) -> Piece {
        let representation = match piece_type {
            PieceType::Pawn{has_moved}   =>" P",
            PieceType::Bishop =>" B",
            PieceType::Knight =>" N",
            PieceType::Rook   =>" R",
            PieceType::Queen  =>" Q",
            PieceType::King   =>" K",
        };
        let representation = match color {
            Color::White => representation.black().on_white().bold(),
            Color::Black => representation.white().on_black().bold(),
        };
        Piece {
            color,
            position,
            piece_type,
            representation: representation.to_string(),
            possible_moves: HashMap::new(),
            board,
        }
    }
    pub fn find_valid_moves(&mut self, position: &Position) -> bool {
        self.possible_moves = HashMap::new();
        match self.piece_type {
            PieceType::Pawn{has_moved} => {
                let num = {
                    if self.color == Color::White {
                        1
                    }
                    else {
                        -1
                    }
                };
                self.possible_moves.insert(Position::new(self.position.x, self.position.y + num));
                if !self.has_moved {
                    num *= 2;
                    self.possible_moves.insert(Position::new(self.position.x, self.position.y + num));
                }
            },
            PieceType::Knight => (),
            PieceType::Bishop => (),
            PieceType::Rook => (),
            PieceType::Queen => (),
            PieceType::King => (),
        };
        false
    }
}