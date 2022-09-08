use chess::set;

fn main() {
    let mut board = set::Board{board: Vec::new()}.init();
    board.print();
}