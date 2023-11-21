use sdl2::{pixels::Color, render::Canvas};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use std::time::Instant;

const MS_PER_UPDATE: f32 = 4.0;
const DEFAULT_MS_PER_STATE_UPDATE: f32 = 200.0;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const CELL_WIDTH: u32 = 10;
const CELL_HEIGHT: u32 = 10;
const GRID_WIDTH: u32 = WINDOW_WIDTH / CELL_WIDTH; // 80
const GRID_HEIGHT: u32 = WINDOW_HEIGHT / CELL_HEIGHT; // 60
const GRID_SIZE: u32 = GRID_WIDTH * GRID_HEIGHT;
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
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    frame_count: u8,
    state_update: f32,
    running: bool,
    paused: bool,
    ms_per_state_update: f32
}

impl Game {
    pub fn new(canvas: Canvas<Window>, event_pump: sdl2::EventPump) -> Game {
        Game {
            canvas: canvas,
            event_pump: event_pump,
            frame_count: 0,
            state_update: 0.0,
            running: true,
            paused: false,
            ms_per_state_update: DEFAULT_MS_PER_STATE_UPDATE
        }
    }

    pub fn update(&mut self, mut state: State, t: f32, _dt: f32) -> State {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { ..  } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.paused = !self.paused;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.ms_per_state_update = self.ms_per_state_update - 100.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.ms_per_state_update = self.ms_per_state_update + 100.0;
                },
                // Event::MouseMotion { x, y, ..} |
                Event::MouseButtonDown { x, y, .. } => {
                    let grid_x = x / CELL_WIDTH as i32;
                    let grid_y = y / CELL_HEIGHT as i32;
                    let grid_i = grid_x + grid_y * GRID_WIDTH as i32;
                    println!("Mouse clicked at: {}, {}", x, y);
                    println!("Grid at: {}, {}", grid_x, grid_y);
                    println!("Grid i: {}", grid_i);

                    state.grid[grid_i as usize] = if state.grid[grid_i as usize].is_none() {
                        Some(Box::new(Cell::new()))
                    } else {
                        None
                    }
                },
                _ => {}
            }
        }

        self.frame_count = (self.frame_count + 1) % 255;

        if self.paused || t - self.state_update < self.ms_per_state_update { return state }

        self.state_update = t;

        let mut new_state = State::new();

        for i in 0..GRID_SIZE {
            match &state.grid[i as usize] {
                Some(_) => {
                    match &state.neighbor_count(i as i32) {
                        2 |
                        3 => {
                            new_state.grid[i as usize] = Some(Box::new(Cell::new()))
                        },
                        _ => {}
                    }
                },
                None => {
                    match &state.neighbor_count(i as i32) {
                        3 => {
                            new_state.grid[i as usize] = Some(Box::new(Cell::new()))
                        },
                        _ => {}
                    }
                }
            }
        }

        new_state
    }

    pub fn render(&mut self, state: &State) {
        let color = Color::WHITE;
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::BLACK);

        for i in 0..GRID_HEIGHT {
            self.canvas.draw_line(
                Point::new(0, (i * CELL_HEIGHT + 10) as i32),
                Point::new(WINDOW_WIDTH as i32, (i * CELL_HEIGHT + 10) as i32)
            ).expect("could not draw line");
        }
        for i in 0..GRID_WIDTH {
            self.canvas.draw_line(
                Point::new((i * CELL_WIDTH) as i32, 0),
                Point::new((i * CELL_WIDTH) as i32, WINDOW_HEIGHT as i32)
            ).expect("could not draw line");
        }

        for i in 0..GRID_SIZE {
            match &state.grid[i as usize] {
                Some(_) => {
                    let x = i % GRID_WIDTH * CELL_WIDTH;
                    let y = i / GRID_WIDTH * CELL_HEIGHT;
                    self.canvas.fill_rect(Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_HEIGHT)).expect("could not fill rect");
                },
                None => {}
            }
        }
        self.canvas.present();
    }
}

pub struct Cell {}

impl Cell {
    pub fn new() -> Cell {
        Cell {}
    }
}

pub struct State {
    grid: [Option<Box<Cell>>; GRID_SIZE as usize]
}

impl State {
    pub fn new() -> State {
        State {
            grid: std::array::from_fn(|_| None)
        }
    }

    pub fn neighbor_count(&self, i: i32) -> u8 {
        let mut count = 0;

        let _grid_size = GRID_SIZE as i32;
        let grid_width = GRID_WIDTH as i32;
        let grid_height = GRID_HEIGHT as i32;

        let top = i / grid_width == 0;
        let bottom = i / grid_width >= grid_height - 1;
        let left = i % grid_width == 0;
        let right = i % grid_width >= grid_width - 1;

        let mut ii = i - 1;
        if !left && self.grid[ii as usize].is_some() { count += 1 }

        ii = i + 1;
        if !right && self.grid[ii as usize].is_some() { count += 1 }

        ii = i - grid_width - 1;
        if !top && !left && self.grid[ii as usize].is_some() { count += 1 }

        ii = i - grid_width;
        if !top && self.grid[ii as usize].is_some() { count += 1 }

        ii = i - grid_width + 1;
        if !top && !right && self.grid[ii as usize].is_some() { count += 1 }

        ii = i + grid_width - 1;
        if !bottom && !left && self.grid[ii as usize].is_some() { count += 1 }

        ii = i + grid_width;
        if !bottom && self.grid[ii as usize].is_some() { count += 1 }

        ii = i + grid_width + 1;
        if !bottom && !right && self.grid[ii as usize].is_some() { count += 1 }

        count
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game-of-rust", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    let mut game = Game::new(canvas, event_pump);

    // https://gafferongames.com/post/fix_your_timestep/
    let mut t: f32 = 0.0;

    let mut timestep = TimeStep::new();
    let mut accumulator = 0.0;
    let mut state = State::new();
    state.grid[1255] = Some(Box::new(Cell::new()));
    state.grid[1256] = Some(Box::new(Cell::new()));
    state.grid[1257] = Some(Box::new(Cell::new()));

    while game.running {
        let frame_time = timestep.delta();
        accumulator += frame_time;

        while accumulator >= MS_PER_UPDATE {
            state = game.update(state, t, MS_PER_UPDATE);
            t += MS_PER_UPDATE;
            accumulator -= MS_PER_UPDATE;
        }

        // const double alpha = accumulator / dt;

        // State state = currentState * alpha +
        //     previousState * ( 1.0 - alpha );

        // render( state );

        game.render(&state);
    }

    Ok(())
}
