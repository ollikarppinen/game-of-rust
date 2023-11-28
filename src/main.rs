use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

mod rendering;
mod inputs;
mod state_mgmt;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MS_PER_UPDATE: f32 = 100.0;
const CELL_WIDTH: u32 = 10;
const CELL_HEIGHT: u32 = 10;
const INITIAL_X_OFFSET: i32 = 0;
const INITIAL_Y_OFFSET: i32 = 0;

#[derive(Debug)]
pub struct Config {
    window_width: u32,
    window_height: u32,
    dt: f32,
    cell_width: u32,
    cell_height: u32
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

#[derive(Debug)]
pub struct TimeStep {
    last_time:   Instant,
    delta_time:  f32,
    frame_count: u32,
    frame_time:  f32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time:   Instant::now(),
            delta_time:  0.0,
            frame_count: 0,
            frame_time:  0.0,
        }
    }

    pub fn delta(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros()
            as f32
            * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    // provides the framerate in FPS
    pub fn frame_rate(&mut self) -> Option<u32> {
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        let tmp: u32;
        // per second
        if self.frame_time >= 1000.0 {
            tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            return Some(tmp);
        }
        None
    }
}

pub struct Game {
    config: Config,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    frame_count: u8,
    state_update: f32,
    running: bool,
    paused: bool
}

impl Game {
    pub fn new(canvas: Canvas<Window>, event_pump: sdl2::EventPump, config: Config) -> Game {
        Game {
            config: config,
            canvas: canvas,
            event_pump: event_pump,
            frame_count: 0,
            state_update: 0.0,
            running: true,
            paused: false
        }
    }

    fn screen_coord_to_game_coord(x: i32, y: i32, offset_x: i32, offset_y: i32, config: &Config) -> Coord {
        let mut x = x + offset_x;
        let mut y = y + offset_y;
        if x < 0 { x = x - config.cell_width as i32 + 1 }
        if y < 0 { y = y - config.cell_height as i32 + 1 }
        x = (x as f32 / config.cell_width as f32) as i32;
        y = (y as f32 / config.cell_height as f32) as i32;
        Coord::new(x, y)
    }
}

#[derive(Hash)]
pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        coords.push(Coord::new(self.x, self.y + 1));
        coords.push(Coord::new(self.x, self.y - 1));
        coords.push(Coord::new(self.x + 1, self.y));
        coords.push(Coord::new(self.x + 1, self.y + 1));
        coords.push(Coord::new(self.x + 1, self.y - 1));
        coords.push(Coord::new(self.x - 1, self.y));
        coords.push(Coord::new(self.x - 1, self.y + 1));
        coords.push(Coord::new(self.x - 1, self.y - 1));
        coords
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Copy for Coord {}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Coord::new(self.x, self.y)
    }
}

pub struct State {
    cell_coords: HashSet<Coord>,
    camera_x_offset: i32,
    camera_y_offset: i32,
    cursor_x: i32,
    cursor_y: i32,
    running: bool,
    paused: bool
}

impl State {
    pub fn new() -> State {
        State {
            cell_coords: HashSet::new(),
            camera_x_offset: INITIAL_X_OFFSET,
            camera_y_offset: INITIAL_Y_OFFSET,
            cursor_x: 0,
            cursor_y: 0,
            running: true,
            paused: false
        }
    }

    pub fn reset(&mut self) {
        self.cell_coords = HashSet::new();
    }

    pub fn is_live(&self, coord: &Coord) -> bool {
        return self.cell_coords.contains(&coord);
    }

    pub fn should_live(&self, coord: &Coord) -> bool {
        match self.neighbor_count(&coord) {
            2 => { self.is_live(&coord) },
            3 => { true },
            _ => { false }
        }
    }

    pub fn neighbor_count(&self, coord: &Coord) -> u8 {
        let mut count = 0;

        for coord in coord.neighbors() {
            if self.is_live(&coord) { count += 1 }
        }

        count
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let config = Config::new();
    let window = video_subsystem.window("game-of-rust", config.window_width, config.window_height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    let mut state = state_mgmt::initial_state();

    // https://gafferongames.com/post/fix_your_timestep/
    let mut t: f32 = 0.0;
    let mut timestep = TimeStep::new();
    let mut accumulator = -1000.0;

    while state.running {
        let frame_time = timestep.delta();
        accumulator += frame_time;

        inputs::handle_inputs(&mut state, &mut event_pump, &config);

        while accumulator >= config.dt {
            state_mgmt::update(&mut state, t);
            t += config.dt;
            accumulator -= config.dt;
        }

        // const double alpha = accumulator / dt;

        // State state = currentState * alpha +
        //     previousState * ( 1.0 - alpha );

        // render( state );

        rendering::render(&mut canvas, &state, &config);
    }

    Ok(())
}
