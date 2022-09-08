pub mod set {
    const BOARD_SIZE: i32 = 8;

    pub mod board;
    pub mod piece;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Color{
        White,
        Black
    }

    #[derive(Debug)]
    pub struct Postition {
        x: i32,
        y: i32,
    }
    impl Postition {
        pub fn new(x: i32, y: i32) -> Postition {
            Postition {
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
