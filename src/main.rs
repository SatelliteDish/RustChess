use chess::set;
use std::io;

fn main() {
    let mut board = set::board::Board::new();
    board.print();
    take_turn();
    board.print();
    //board.get_cell(set::Transform::new(3,3)).unwrap().add_piece(Some(set::piece::Piece::new(set::Color::White, set::Transform::new(3,3),set::piece::PieceType::Rook)));
    //board.find_valid_moves(Some(&mut set::piece::Piece::new(set::Color::White,set::Transform::new(3,3),set::piece::PieceType::Rook)).unwrap());
    
}

fn take_turn() {
    let input = take_input();
}

fn take_input() -> set::board::Request {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let request = {
        match parse_input(input.trim().to_string()){
            Ok(r) => r,
            Err(e) => set::board::Request::new(
                set::piece::Piece::new(
                    set::Color::Black,
                    set::Transform::new(1,2),
                    set::piece::PieceType::Pawn{has_moved: false},
                ),
                set::Transform::new(1,2)
            )
        }
    };
    request
}

fn parse_input(input: String) -> Result<set::board::Request, io::Error> {
    let piece_type = set::piece::PieceType::Pawn{has_moved: false};
    for chars in input.chars() {
        match chars {
            'P' => {
                println!("Moving Pawn");
            },
            'N' => {
                println!("Moving Knight");
            },
            'B' => {
                println!("Moving Bishop");
            },
            'R' => {
                println!("Moving Rook");
            },
            'Q' => {
                println!("Moving Queen");
            },
            'K' => {
                println!("Moving King");
            },
            'O' => {
                println!("Castleing");
            },
            _ => (),
        }
    }

    let request = set::board::Request::new(
        set::piece::Piece::new(
            set::Color::Black,
            set::Transform::new(1,2),
            set::piece::PieceType::Pawn{has_moved: false},
        ),
        set::Transform::new(1,2)
    );
    Ok(request)
}