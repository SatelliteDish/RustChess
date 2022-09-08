use chess::set;

fn main() {
    let mut board = set::board::Board{board: Vec::new()}.init();
    board.print();
}