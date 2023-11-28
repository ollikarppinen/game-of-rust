const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MS_PER_UPDATE: f32 = 16.0;
const CELL_WIDTH: u32 = 10;
const CELL_HEIGHT: u32 = 10;

#[derive(Debug)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub dt: f32,
    pub cell_width: u32,
    pub cell_height: u32
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            dt: MS_PER_UPDATE,
            cell_width: CELL_WIDTH,
            cell_height: CELL_HEIGHT
        }
    }
}
