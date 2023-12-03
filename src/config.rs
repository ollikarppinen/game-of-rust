use sdl2::pixels::Color;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const MS_PER_UPDATE: f32 = 16.0;
const MIN_CELL_WIDTH: f32 = 1.0;
const MIN_CELL_HEIGHT: f32 = 1.0;
const MAX_CELL_WIDTH: f32 = 100.0;
const MAX_CELL_HEIGHT: f32 = 100.0;
const INTRO_DURATION_MS: f32 = 5000.0;
const FONT_PATH: &'static str = "./ARCADECLASSIC.TTF";
const CHAR_WIDTH: f32 = 30.0;
const CHAR_HEIGHT: f32 = 50.0;
const BACKGROUND_COLOR: Color = Color::WHITE;
const GRID_COLOR: Color = Color::BLACK;
const CELL_COLOR: Color = Color::BLACK;
const FONT_COLOR: Color = Color::RGBA(255, 0, 0, 200);
const HOVER_COLOR: Color = Color::GREY;
const CAMERA_XY_ACCELERATION: f32 = 0.1;
const CAMERA_XY_ACCELERATION_MAX: f32 = 1.0;
const CAMERA_XY_VELOCITY_MAX: f32 = 50.0;
const CAMERA_Z_ACCELERATION: f32 = 0.01;
const CAMERA_Z_ACCELERATION_MAX: f32 = 0.01;
const CAMERA_Z_VELOCITY_MAX: f32 = 0.05;

pub struct Config {
    pub window_width: f32,
    pub window_height: f32,
    pub dt: f32,
    pub min_cell_width: f32,
    pub min_cell_height: f32,
    pub max_cell_width: f32,
    pub max_cell_height: f32,
    pub intro_duration_ms: f32,
    pub font_path: &'static str,
    pub char_width: f32,
    pub char_height: f32,
    pub background_color: Color,
    pub grid_color: Color,
    pub cell_color: Color,
    pub font_color: Color,
    pub hover_color: Color,
    pub camera_xy_acceleration: f32,
    pub camera_xy_acceleration_max: f32,
    pub camera_xy_velocity_max: f32,
    pub camera_z_acceleration: f32,
    pub camera_z_acceleration_max: f32,
    pub camera_z_velocity_max: f32,
    pub font: Option<sdl2::ttf::Font<'static, 'static>>,
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
            camera_xy_acceleration: CAMERA_XY_ACCELERATION,
            camera_xy_acceleration_max: CAMERA_XY_ACCELERATION_MAX,
            camera_xy_velocity_max: CAMERA_XY_VELOCITY_MAX,
            camera_z_acceleration: CAMERA_Z_ACCELERATION,
            camera_z_acceleration_max: CAMERA_Z_ACCELERATION_MAX,
            camera_z_velocity_max: CAMERA_Z_VELOCITY_MAX,
            font: None,
        }
    }
}
