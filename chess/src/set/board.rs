use crate::set::*;
use std::io;
use std::mem;

#[derive(Debug,Clone)]
pub struct Cell {
    pub color: Color,
    pub piece: Option<piece::Piece>,
}
impl Cell {
    pub fn new(color: Color, piece: Option<piece::Piece>) -> Cell {
        Cell {
            color,
            piece,
        }
    }

    //Removes and return the Piece in the cell, if any.
    pub fn remove_piece(&mut self) -> Option<piece::Piece> {
        self.piece.take()
    }

    //Takes in a Piece and returns it
    pub fn add_piece(&mut self, piece: Option<piece::Piece>) {
        self.piece = piece;
    }
}

pub struct Board {
    pub board: Vec<Vec<Cell>>,
}

impl Board {

    //Prints the board to the console
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

    //Creates a new Board and initializes it to the starting positions
    pub fn new(&self) -> Board {
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
                let pos = Position::new(row, col);
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
                        },&self))
                    }
                    
                    else if row == 1 || row == BOARD_SIZE - 2 { //Pawn Ranks
                        Some(piece::Piece::new(color,pos,piece::PieceType::Pawn{has_moved: false},&self))
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


    //WIP
    //When done it will take a Request and return an Error if the move is invalid
    pub fn make_move(&mut self, mut request: Request) -> Result<(),String> {
        let mut piece = self.board[0][0].remove_piece();
        self.board[1][1].piece = piece;

        Ok(())
    }

    //Takes in Position and returns a reference to the cell at that posiiton.
    pub fn get_cell(&self, position: Position) -> Option<&Cell> {
        if(position.x < BOARD_SIZE && position.y < BOARD_SIZE) {
            Some(&self.board[position.x as usize][position.y as usize])
        }
        else {
            None
        }
    }

    //Takes in a Position and returns a reference to the Piece at that position
    //Can be used to tell if a cell is vacant`
    pub fn get_piece(&self, position: Position) -> Option<&piece::Piece> {
        match self.get_cell(position) {
            Some(T) => T.piece.as_ref(),
            None => None
        }
    }
    pub fn show_possible_moves(&self, piece: &piece::Piece) {
        for row in self.board.iter() {
            let mut s = String::new();
            for col in row.iter() {
                match &col.piece {
                    Some(piece_type) => {
                        s = s + &piece_type.representation.to_string();
                    }
                    None => {
                        if piece.possible_moves.contains_key(Position::new(row.posiiton.x,col.pos)) {
                            s = s + "🟥"
                        }
                        else {                        
                            match col.color {
                                Color::White => 
                                    s = s + "⬜",
                                Color::Black => s = s + "⬛",
                            }
                        }
                    }
                };
            }
            println!("{}",s);
        }
    }


}

#[derive(Debug)]
pub struct Request {
    piece: piece::Piece,
    position: Position,
}
impl Request {
    pub fn new(piece: piece::Piece, position: Position) -> Request {
        Request{
            piece,
            position,
        }
    }
}