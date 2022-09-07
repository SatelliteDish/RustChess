use colored::Colorize;

const BOARD_SIZE: i32 = 8;

#[derive(Clone)]
enum Color{
    White,
    Black
}

enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
struct Piece{
    color: Color,
    position: Postition,
    pieceType: PieceType,
}

impl Piece {
    fn move_piece(position: Postition){

    }
}

struct Postition {
    x: i32,
    y: i32,
}

struct Cell {
    color: Color,
    piece: Option<Piece>,
}

fn main() {
    let board = init_board();
    print_board(&board);
}

fn init_board() -> Vec<Vec<Cell>> {
    let mut board: Vec<Vec<Cell>> = Vec::new();

    let mut next = Color::White;

    for row in 0..BOARD_SIZE {
        let mut s = String::new();
        if row%2 == 0 {
            next = Color::White;
        }
        else{
            next = Color::Black;
        }
        
        board.push(Vec::new());

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
            board[row as usize].push(build_cell(next.clone(), None));
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

    board
}

fn print_board(board: &Vec<Vec<Cell>>) {
    for row in board.iter() {
        let mut s = String::new();
        for col in row.iter() {
            match col.color {
                Color::White => {
                    s = s + "⬜";
                },
                Color::Black => {
                    s = s + "⬛";
                },
            }
        }
        println!("{}",s);
    }
}

fn build_cell(color: Color, piece: Option<Piece>) -> Cell {
    Cell{
        color,
        piece,
    }
}

fn build_piece(color: Color, position: Postition, pieceType: PieceType) -> Piece {
    Piece {
        color,
        position,
        pieceType,
    }
}

fn build_position(x: i32, y: i32) -> Postition {
    Postition {
        x,
        y,
    }
}