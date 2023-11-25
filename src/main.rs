use sdl2::{pixels::Color, render::Canvas};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, self};
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

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
    paused: bool,
    offset_x: i32,
    offset_y: i32
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
            paused: false,
            offset_x: INITIAL_X_OFFSET,
            offset_y: INITIAL_Y_OFFSET
        }
    }
    
    pub fn initial_state(&mut self) -> State {
        let mut state = State::new();
        // return state;

        let file = File::open("./initial_cells.txt").expect("file wasn't found.");
        let reader = BufReader::new(file);
        let mut coords: Vec<i32> = reader
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect();

        while !coords.is_empty() {
            match (coords.pop(), coords.pop()) {
                (Some(x), Some(y)) => {
                    state.cell_coords.insert(Coord::new(x, y));
                },
                _ => {}
            }
        }

        state
    }

    pub fn integrate(&mut self, mut state: State, t: f32) -> State {
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
            self.offset_y -= 1;
            println!("offset_y: {}", self.offset_y);
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
            self.offset_y += 1;
            println!("offset_y: {}", self.offset_y);
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
            self.offset_x -= 1;
            println!("offset_x: {}", self.offset_x);
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
            self.offset_x += 1;
            println!("offset_x: {}", self.offset_x);
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { ..  } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.paused = !self.paused;
                },
                Event::KeyDown { keycode: Some(Keycode::Plus), .. } => {
                    if self.config.dt > 1.0 { self.config.dt /= 2.0 }
                    println!("dt: {}", self.config.dt);
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if self.config.dt < 1000.0 { self.config.dt *= 2.0 }
                    println!("dt: {}", self.config.dt);
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    // state.reset();
                    self.offset_x = 0;
                    self.offset_y = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                },
                Event::MouseButtonDown { x, y, .. } => {
                    let coord = Coord::new(
                        ((x + self.offset_x) as f32 / self.config.cell_width as f32) as i32,
                        ((y + self.offset_y) as f32 / self.config.cell_height as f32) as i32
                    );
                    println!("click coord: {}", coord);
                    state.cell_coords.insert(coord);
                },
                _ => {}
            }
        }

        state
    }

    pub fn update(&mut self, state: State, t: f32) -> State {
        self.frame_count = (self.frame_count + 1) % 255;

        if self.paused { return state }

        self.state_update = t;

        let mut new_state = State::new();

        for cell_coord in &state.cell_coords {
            if state.should_live(&cell_coord) { new_state.cell_coords.insert(cell_coord.clone()); }
            // update_coords.push(cell_coord.clone());
            for neighbor_coord in cell_coord.neighbors() {
                // update_coords.push(neighbor_coord);
                if state.should_live(&neighbor_coord) { new_state.cell_coords.insert(neighbor_coord.clone()); }
            }
        }

        new_state
    }

    pub fn dt(&self) -> f32 {
        self.config.dt
    }

    pub fn render(&mut self, state: &State) {
        let color = Color::WHITE;
        self.canvas.set_draw_color(color);
        self.canvas.clear();

        self.render_hover();
        self.render_state(state);
        self.render_grid();

        self.canvas.present();
    }

    fn render_grid(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);

        let mut x = self.offset_x % self.config.cell_width as i32;
        while x < self.config.window_width as i32 {
            self.canvas.draw_line(
                Point::new(
                    x,
                    0
                ),
                Point::new(
                    x,
                    self.config.window_height as i32
                )
            ).expect("could not draw line");
            x += self.config.cell_width as i32;
        }
        let mut y = self.offset_y % self.config.cell_height as i32;
        while y < self.config.window_height as i32 {
            self.canvas.draw_line(
                Point::new(
                    0,
                    y
                ),
                Point::new(
                    self.config.window_width as i32,
                    y
                )
            ).expect("could not draw line");
            y += self.config.cell_height as i32;
        }
    }

    fn render_state(&mut self, state: &State) {
        for coord in &state.cell_coords {
            self.render_live_cell(&coord);
        }
    }

    fn render_hover(&mut self) {
        let coord = Coord::new(
            ((self.event_pump.mouse_state().x() + self.offset_x) as f32 / self.config.cell_width as f32) as i32,
            ((self.event_pump.mouse_state().y() + self.offset_y) as f32 / self.config.cell_height as f32) as i32
        );
        self.render_hover_cell(coord);
    }

    fn render_live_cell(&mut self, coord: &Coord) {
        let x = coord.x * self.config.cell_width as i32;
        let y = coord.y * self.config.cell_height as i32;
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.fill_rect(Rect::new(x + self.offset_x, y + self.offset_y, self.config.cell_width, self.config.cell_height)).expect("could not fill rect");
    }

    fn render_hover_cell(&mut self, coord: Coord) {
        let x = coord.x * self.config.cell_width as i32;
        let y = coord.y * self.config.cell_height as i32;
        self.canvas.set_draw_color(Color::GRAY);
        self.canvas.fill_rect(Rect::new(x as i32 + self.offset_x, y as i32 + self.offset_y, self.config.cell_width, self.config.cell_height)).expect("could not fill rect");
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
    cell_coords: HashSet<Coord>
}

impl State {
    pub fn new() -> State {
        State {
            cell_coords: HashSet::new()
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
    let canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    let mut game = Game::new(canvas, event_pump, config);
    let mut state = game.initial_state();

    // https://gafferongames.com/post/fix_your_timestep/
    let mut t: f32 = 0.0;
    let mut timestep = TimeStep::new();
    let mut accumulator = -1000.0;

    while game.running {
        let frame_time = timestep.delta();
        accumulator += frame_time;

        state = game.integrate(state, t);

        while accumulator >= game.dt() {
            state = game.update(state, t);
            t += game.dt();
            accumulator -= game.dt();
        }

        // const double alpha = accumulator / dt;

        // State state = currentState * alpha +
        //     previousState * ( 1.0 - alpha );

        // render( state );

        game.render(&state);
    }

    Ok(())
}
