
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn it_works() {
        let piece = create_test_piece(Transform::new(5,5));
        println!("{:?}",piece);
    }

    fn create_test_piece(transform: Transform) -> Piece{
        let mut rng = rand::thread_rng();
        let piece_type = match rng.gen_range(0..6) {
            0 => PieceType::Pawn{has_moved: false},
            1 => PieceType::Bishop,
            2 => PieceType::Knight,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            _ => PieceType::King,
        };
        let color = match rng.gen_range(0..2) {
            0 => Color::White,
            _ => Color::Black,

        };
        Piece::new(color,transform,piece_type)
    }
}