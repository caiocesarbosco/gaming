pub mod strings {

    pub static GAME_NAME: &str = "My Game";

}

pub mod windows_resolutions {

    pub static STANDARD: [u32; 2] = [512, 512];
    pub static WINDOW_SIZE: i32 = 512;
    pub static PIXEL_SIZE: f64 = 32.0;
    pub static WORLD_SIZE: i32 = WINDOW_SIZE/PIXEL_SIZE as i32;
}



pub mod colors {

    pub static RED: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
    pub static GREEN: [f64; 4] = [0.0, 1.0, 0.0, 1.0];
    pub static BLUE: [f64; 4] = [0.0, 0.0, 1.0, 1.0];
    pub static WHITE: [f64; 4] = [1.0;4];
    pub static BLACK: [f64; 4] = [0.0, 0.0, 0.0, 1.0];
}


