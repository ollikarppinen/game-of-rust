const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MS_PER_UPDATE: f32 = 16.0;
const MIN_CELL_WIDTH: u32 = 1;
const MIN_CELL_HEIGHT: u32 = 1;
const MAX_CELL_WIDTH: u32 = 20;
const MAX_CELL_HEIGHT: u32 = 20;

#[derive(Debug)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub dt: f32,
    pub min_cell_width: u32,
    pub min_cell_height: u32,
    pub max_cell_width: u32,
    pub max_cell_height: u32,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            dt: MS_PER_UPDATE,
            min_cell_width: MIN_CELL_WIDTH,
            max_cell_width: MAX_CELL_WIDTH,
            min_cell_height: MIN_CELL_HEIGHT,
            max_cell_height: MAX_CELL_HEIGHT
        }
    }
}