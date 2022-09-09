pub mod set {
    const BOARD_SIZE: i32 = 8;

    pub mod board;
    pub mod piece;

    #[derive(Clone, Debug, PartialEq,Copy)]
    pub enum Color{
        White,
        Black
    }

    #[derive(Debug, Clone,Copy,Eq, Hash, PartialEq)]
    pub struct Position {
        x: i32,
        y: i32,
    }
    impl Position {
        pub fn new(x: i32, y: i32) -> Position {
            Position {
                x,
                y,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
