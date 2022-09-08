use colored::Colorize;
use crate::set::*;

#[derive(Debug)]
    pub enum PieceType{
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }

    #[derive(Debug)]
    pub struct Piece {
        pub color: Color,
        pub position: Postition,
        pub piece_type: PieceType,
        pub representation: String,
    }
    impl Piece {
        pub fn new(color: Color, position: Postition, piece_type: PieceType) -> Piece {
            let representation = match piece_type {
                PieceType::Pawn   =>" P",
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
            }
        }
    }