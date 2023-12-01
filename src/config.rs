use sdl2::pixels::Color;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MS_PER_UPDATE: f32 = 16.0;
const MIN_CELL_WIDTH: f32 = 1.0;
const MIN_CELL_HEIGHT: f32 = 1.0;
const MAX_CELL_WIDTH: f32 = 20.0;
const MAX_CELL_HEIGHT: f32 = 20.0;
const INTRO_DURATION_MS: f32 = 5000.0;
const FONT_PATH: &'static str = "./ARCADECLASSIC.TTF";
const CHAR_WIDTH: u32 = 30;
const CHAR_HEIGHT: u32 = 50;
const BACKGROUND_COLOR: Color = Color::WHITE;
const GRID_COLOR: Color = Color::BLACK;
const CELL_COLOR: Color = Color::BLACK;
const FONT_COLOR: Color = Color::RGBA(255, 0, 0, 200);
const HOVER_COLOR: Color = Color::GREY;

#[derive(Debug)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub dt: f32,
    pub min_cell_width: f32,
    pub min_cell_height: f32,
    pub max_cell_width: f32,
    pub max_cell_height: f32,
    pub intro_duration_ms: f32,
    pub font_path: &'static str,
    pub char_width: u32,
    pub char_height: u32,
    pub background_color: Color,
    pub grid_color: Color,
    pub cell_color: Color,
    pub font_color: Color,
    pub hover_color: Color,
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
            max_cell_height: MAX_CELL_HEIGHT,
            intro_duration_ms: INTRO_DURATION_MS,
            font_path: FONT_PATH,
            char_width: CHAR_WIDTH,
            char_height: CHAR_HEIGHT,
            background_color: BACKGROUND_COLOR,
            grid_color: GRID_COLOR,
            cell_color: CELL_COLOR,
            font_color: FONT_COLOR,
            hover_color: HOVER_COLOR,
        }
    }
}
