use colored::Colorize;
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
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let piece = create_test_piece(Transform::new(5,5));
        println!("{:?}",piece);
    }

    fn create_test_piece(transform: Transform) -> Piece{
        let mut rng = rand::thread_rng();
        let piece_type = match rng.gen_range(0..6) {
            0 => PieceType::Pawn{has_moved: false},
            1 => PieceType::Bishop,
            2 => PieceType::Knight,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            _ => PieceType::King,
        };
        let color = match rng.gen_range(0..2) {
            0 => Color::White,
            _ => Color::Black,

        };
        Piece::new(color,transform,piece_type)
    }
}