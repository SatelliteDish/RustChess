enum Color{
    White,
    Black
}

struct Location{
    x : i32,
    y : i32,
}

enum Pieces{
    Pawn{color: Color, loc: Location},
    Bishop{color: Color, loc: Location},
    Knight{color: Color, loc: Location},
    Rook{color: Color, loc: Location},
    Queen{color: Color, loc: Location},
    King{color: Color, loc: Location}
}

const BOARD_SIZE: i8 = 8;

fn main() {
    for row in 0..BOARD_SIZE {
        let mut s = String::new();
        let mut next: Color = {
            if(row%2 == 0){
                Color::White
            }
            else{
                Color::Black
            }
        };
        
        for col in 0..BOARD_SIZE {
            match next {
                Color::White =>{
                    s = s + "⬜";
                    next = Color::Black;
                },
                Color::Black =>{
                    s = s + "⬛";
                    next = Color::White;
                },
            }
        }
        println!("{}",s);
    }
}
