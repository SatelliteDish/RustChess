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
    pub struct Transform {
        x: i32,
        y: i32,
    }
    impl Transform {
        pub fn new(x: i32, y: i32) -> Transform {
            Transform {
                x,
                y,
            }
        }
    }
}

