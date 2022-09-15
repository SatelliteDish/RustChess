use colored::Colorize;
use std::collections::HashMap;
use crate::set::*;

#[derive(Debug, Clone,Copy,PartialEq)]
pub enum PieceType{
    Pawn{has_moved: bool},
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Piece {
    pub color: Color,
    pub transform: Transform,
    pub piece_type: PieceType,
    pub representation: String,
    pub possible_moves: Vec<Transform>,
}
impl Piece {
    pub fn new(color: Color, transform: Transform, piece_type: PieceType) -> Piece {
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
            transform,
            piece_type,
            representation: representation.to_string(),
            possible_moves: Vec::new(),
        }
    }

    pub fn sort_possible_moves(&mut self) {
        for i in 0..self.possible_moves.len() - 1 {
            for j in 0..self.possible_moves.len() - i - 1 {
                if (self.possible_moves[j].x > self.possible_moves[j+1].x) 
                || (self.possible_moves[j].x == self.possible_moves[j+1].x && self.possible_moves[j].y > self.possible_moves[j+1].y) {
                    self.possible_moves.swap(j,j+1);
                }
            }
        }
    }

    pub fn is_possible_move(&self, transform: &Transform) -> bool {
        let mut ans = false;
        for tran in self.possible_moves.iter() {
            if tran == transform {
                ans = true;
                break;
            }
        }
        ans
    }
}