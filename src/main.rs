use chess::set;
use std::io;
use unicode_segmentation::UnicodeSegmentation; // 1.6.0


struct Board {
    board: set::board::Board,
}

impl Board{
    pub fn get_board(&mut self) -> &mut set::board::Board {
        &mut self.board
    }
}


fn main() {
    let mut board = set::board::Board::new();
    board.print();
    let request = take_input(&mut board).clone();
    //board.make_move(request);
    board.print();
}

fn take_input(board: &mut set::board::Board) -> set::board::Request {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error");

    let start_pos = get_transform_from_notation(input).expect("Invalid Input");

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Error");

    let end_pos = get_transform_from_notation(input).expect("Invalid Input");
    
    let piece = board.get_cell(start_pos).unwrap().piece.as_ref().unwrap();

    set::board::Request::new(piece,end_pos)
}

fn get_transform_from_notation(notation: String) -> Option<set::Transform> {
    if notation.graphemes(true).count() != 3 {
        println!{"{}",notation.graphemes(true).count()};
        return None;
    }

    let mut x = 9;
    let mut y = 9; 

    for part in notation.graphemes(true) {
        let temp = {match part {
            "a"|"A"|"1" => 0,
            "b"|"B"|"2" => 1,
            "c"|"C"|"3" => 2,
            "d"|"D"|"4" => 3,
            "e"|"E"|"5" => 4,
            "f"|"F"|"6" => 5,
            "g"|"G"|"7" => 6,
            "h"|"H"|"8" => 7,
            _ => {println!("{}",part);10},
        }};
        if x == 9 {
            x = temp;
        }
        else {
            y = temp;
        }
    }
    println!("{} {}",x,y);
    Some(set::Transform::new(x,y))
}