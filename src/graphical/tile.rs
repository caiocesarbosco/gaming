pub mod tile {

    use crate::constants::constants::colors::{BLACK, WHITE};

    type Colour = [f64; 4];

    #[derive(Clone)]
    pub struct Tile {
        pub asset: Colour,
    }

    impl Tile {

        pub fn empty() -> Self {
            Tile {
                asset: WHITE, 
            }
        }

        pub fn wall() -> Self {
            Tile {
                asset: BLACK,
            }
        }

    }
}