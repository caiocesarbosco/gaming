pub mod character {

    #[derive(Clone)]
    pub struct Player {
        pub name: char,
        pub colour: [f64; 4],
        pub pos_x: i32,
        pub pos_y: i32,
    }

    impl Player {
        pub fn new(name: char, colour: [f64; 4], pos_x: i32, pos_y: i32) -> Player {
            Player {
                name,
                colour,
                pos_x,
                pos_y,
            }
        }
    }

}