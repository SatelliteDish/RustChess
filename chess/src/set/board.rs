use crate::set::*;

#[derive(Debug,Clone)]
//Each square on the board is a cell
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
    white_pieces: Vec<piece::Piece>,
    black_pieces: Vec<piece::Piece>,
}

impl Board {

    //Prints the board to the console
    pub fn print(&self) {
        for row in self.board.iter().rev() {
            let mut s = String::new();
            for col in row.iter().rev() {
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
    pub fn new() -> Board {
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
                let transform = Transform::new(row, col);
                let piece: Option<piece::Piece> = {
                    let color = {
                        if row < BOARD_SIZE/2 {
                            Color::White
                        }
                        else {
                            Color::Black
                        }
                    };
                    if row == 0 || row == BOARD_SIZE - 1 { //If this is the first or last rank
                        Some(piece::Piece::new(color.clone(),transform,{
                            match col {
                                0|7 => piece::PieceType::Rook,//Places Rooks
                                1|6 => piece::PieceType::Knight,//Places Knights
                                2|5 => piece::PieceType::Bishop,//Places Bishops
                                3   => piece::PieceType::Queen,//Places Queen
                                _   => piece::PieceType::King,//Places King
                            }
                        }))
                    }
                    else if row == 1 || row == BOARD_SIZE - 2 { //If this is one of the pawn ranks
                        Some(piece::Piece::new(color,transform,piece::PieceType::Pawn{has_moved: false}))
                    }
                    else { //If this is one of the middle ranks
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
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
        }
    }


    //WIP
    //When done it will take a Request and return an Error if the move is invalid
    pub fn make_move(&mut self, request: Request) -> Result<(),String> {
        let piece = self.board[0][0].remove_piece();
        self.board[1][1].piece = piece;

        Ok(())
    }

    //Takes in Position and returns a reference to the cell at that posiiton.
    pub fn get_cell(&mut self, transform: Transform) -> Option<&mut Cell> {
        if transform.x < BOARD_SIZE && transform.y < BOARD_SIZE {
            Some(&mut self.board[transform.x as usize][transform.y as usize])
        }
        else {
            None
        }
    }

    //Takes in a Position and returns a reference to the Piece at that position
    //Can be used to tell if a cell is vacant`
    pub fn get_piece(&mut self, transform: Transform) -> Option<&piece::Piece> {
        match self.get_cell(transform) {
            Some(t) => t.piece.as_ref(),
            None => None
        }
    }


    //Finds where a piece can move on their rank and file
    //Unable to jump pieces
    //Used for Rook and Queen
    fn find_rook_moves (&self, transform: &Transform) -> Vec<Transform> {
        let x = transform.x;
        let y = transform.y;
        let mut moves: Vec<Transform> = Vec::new();
        let mut ans: Vec <Transform> = Vec::new();

        let mut pos = 0;
        for cell in self.board[x as usize].iter() {
            match &cell.piece {
                Some(_t) => {
                    if pos != y {
                        if pos > y {
                            ans.append(&mut moves);
                            break
                        }
                        moves.clear();
                    }
                    else {
                        ans.append(&mut moves);
                        moves.clear();
                    }
                },
                None    => {
                    moves.push(Transform::new(x,pos));
                },

            }
            pos += 1;
        }
        pos = 0;
        ans.append(&mut moves);
        moves.clear();

        for cell in self.board.iter() {
            match &cell[y as usize].piece {
                Some(_t) => {
                    if pos != x {
                        if pos > x {
                            ans.append(&mut moves);
                            break
                        }
                        moves.clear();
                    }
                    else {
                        ans.append(&mut moves);
                        moves.clear();
                    }
                },
                None    => {
                    moves.push(Transform::new(pos,y));
                },
            }
            pos += 1;
        }
        ans.append(&mut moves);
        ans
    }

    //Takes in a piece and updates that pieces internal list of possible moves, sorted x position then y
    pub fn find_valid_moves(&mut self, piece: &mut piece::Piece) {
        piece.possible_moves = self.find_rook_moves(&piece.transform); 
        piece.possible_moves.append(&mut self.find_bishop_moves(&piece.transform));
        piece.sort_possible_moves();
        println!("{:?}", piece.possible_moves);
        self.display_possible_moves(piece);
    }

    //Finds all valid moves assuming Diagonal movement with no jumping pieces
    pub fn find_bishop_moves (&mut self, transform: &Transform) -> Vec<Transform> {
        let mut x = transform.x;
        let mut y = transform.y;

        if x > y {
            x = x - y;
            y = 0;
        }
        else {
            y = y - x;
            x = 0;
        }

        let mut moves: Vec<Transform> = Vec::new();
        let mut ans: Vec<Transform> = Vec::new();

        while x != BOARD_SIZE-1 && y != BOARD_SIZE-1 {
            let current_cell = self.get_cell(Transform::new(x,y)).expect("Cell not found!");
            match &current_cell.piece {
                Some(t) => {
                    if x == transform.x {
                        ans.append(&mut moves);
                        moves.clear();
                    }
                    else {
                        if x > transform.x {
                            ans.append(&mut moves);
                            break;
                        }
                        moves.clear();
                    }
                },
                None => moves.push(Transform::new(x,y)),
            };
            x += 1;
            y += 1;
        }
        ans.append(&mut moves);
        moves.clear();

        if transform.x + transform.y < 7 {
            y = transform.x + transform.y;
            x = 0;
        }
        else {
            x = transform.x + transform.y - 7;
            y = 7;
        }

        while x < BOARD_SIZE && y >= 0 {
            let current_cell = self.get_cell(Transform::new(x,y)).expect("Cell not found!");
            match &current_cell.piece {
                Some(t) => {
                    if x == transform.x {
                        ans.append(&mut moves);
                        moves.clear();
                    }
                    else {
                        if x > transform.x {
                            ans.append(&mut moves);
                            break;
                        }
                        moves.clear();
                    }
                },
                None => moves.push(Transform::new(x,y)),
            };
            x += 1;
            y -= 1;
        }
        ans
    }

    //Takes in a piece and prints the board with all that piece's possible moves highlighted in red
    //Does not show possible captures, I'm very aware of this but want to work on input before I deal with this
    pub fn display_possible_moves(&mut self, piece: &piece::Piece) {
        for row in (0..BOARD_SIZE).rev() {
            let mut s = String::new();
            for col in (0..BOARD_SIZE).rev() {
                if piece.is_possible_move(&Transform::new(row,col)) {
                    s = s + "🟥";
                }
                else {
                    match &self.board[row as usize][col as usize].piece {
                        Some(piece_type) => {
                            s = s + &piece_type.representation.to_string();
                        }
                        None => match self.board[row as usize][col as usize].color {
                            Color::White => s = s + "⬜",
                            Color::Black => s = s + "⬛",
                        }
                    };
                }
            }
            println!("{}",s);
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Request<'a> {
    piece: &'a piece::Piece,
    transform: Transform,
}
impl <'a> Request<'a> {
    pub fn new(piece: &'a piece::Piece, transform: Transform) -> Request<'a> {
        Request{
            piece,
            transform,
        }
    }
}