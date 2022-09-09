use chess::set;
use std::io;

fn main() {
    let mut board = set::board::Board::new();
    board.print();
    take_turn(&board);
    board.print();
}

fn take_turn(board: &set::board::Board) {
    let input = take_input();
    println!("{:?}",input);
}

fn take_input(board: &set::board::Board) -> set::board::Request {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let request = {
        match parse_input(input.trim().to_string()){
            Ok(r) => r,
            Err(e) => set::board::Request::new(
                set::piece::Piece::new(
                    set::Color::Black,
                    set::Position::new(1,2),
                    set::piece::PieceType::Pawn{has_moved: false},
                    board,
                ),
                set::Position::new(1,2)
            )
        }
    };
    request
}

fn parse_input(input: String, board: set::board::Board) -> Result<set::board::Request, io::Error> {
    let request = set::board::Request::new(
        set::piece::Piece::new(
            set::Color::Black,
            set::Position::new(1,2),
            set::piece::PieceType::Pawn{has_moved: false},
            board,
        ),
        set::Position::new(1,2)
    );
    Ok(request)
}