use colored::Colorize;

const BOARD_SIZE: i32 = 8;

#[derive(Clone, Debug)]
enum Color{
    White,
    Black
}

#[derive(Debug)]
enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
struct Piece {
    color: Color,
    position: Postition,
    piece_type: PieceType,
    representation: String,
}

struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    fn print(&self) {
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

    fn init(mut self) -> Board{

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
                let pos = build_position(row, col);
                let piece: Option<Piece> = {
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
                        Some(build_piece(col,pos,PieceType::Pawn))
                    }
                    else {
                        None
                    }
                };
                self.board[row as usize].push(build_cell(next.clone(), piece));
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

#[derive(Debug)]
struct Postition {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Cell {
    color: Color,
    piece: Option<Piece>,
}

fn main() {
    let mut board = Board{board: Vec::new()}.init();
    board.print();
}

fn build_cell(color: Color, piece: Option<Piece>) -> Cell {
    Cell{
        color,
        piece,
    }
}

fn build_piece(color: Color, position: Postition, piece_type: PieceType) -> Piece {

    let representation = match piece_type {
        PieceType::Pawn   =>" P",
        PieceType::Bishop =>" K",
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

fn build_position(x: i32, y: i32) -> Postition {
    Postition {
        x,
        y,
    }
}