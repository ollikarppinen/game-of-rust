use sdl2::{pixels::Color, render::Canvas};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, self};
use sdl2::rect::{Rect, Point};
use sdl2::video::Window;
use std::time::Instant;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const MS_PER_UPDATE: f32 = 250.0;
const CELL_WIDTH: u32 = 10;
const CELL_HEIGHT: u32 = 10;
const GRID_WIDTH: u32 = 80;
const GRID_HEIGHT: u32 = 60;
const GRID_SIZE: u32 = GRID_WIDTH * GRID_HEIGHT;

#[derive(Debug)]
pub struct Config {
    window_width: u32,
    window_height: u32,
    dt: f32,
    cell_width: u32,
    cell_height: u32,
    grid_width: u32,
    grid_height: u32,
    grid_size: u32
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_width: WINDOW_WIDTH,
            window_height: WINDOW_HEIGHT,
            dt: MS_PER_UPDATE,
            cell_width: CELL_WIDTH,
            cell_height: CELL_HEIGHT,
            grid_width: GRID_WIDTH,
            grid_height: GRID_HEIGHT,
            grid_size: GRID_SIZE
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
            offset_x: 0,
            offset_y: 0
        }
    }

    pub fn integrate(&mut self, mut state: State, t: f32) -> State {
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) && self.offset_y > -500 {
            self.offset_y -= 10;
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) && self.offset_y < 500 {
            self.offset_y += 10;
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) && self.offset_x > -500 {
            self.offset_x -= 10;
        }
        if self.event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) && self.offset_x < 500 {
            self.offset_x += 10;
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
                    if self.config.dt > 50.0 { self.config.dt -= 200.0 }
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if self.config.dt < 500.0 { self.config.dt += 200.0 }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    state.reset();
                },
                Event::MouseButtonDown { x, y, .. } => {
                    let grid_i = Game::get_grid_i(x, y, self.offset_x, self.offset_y);
                    match &grid_i {
                        Some(i) => {
                            state.game_grid[*i] = Some(Cell::new(Some(t), None));
                        },
                        _ => {}
                    }
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

        for i in 0..self.config.grid_size {
            new_state.game_grid[i as usize] = state.next(i as i32, t);
        }

        new_state
    }

    pub fn get_grid_i(x: i32, y: i32, offset_x: i32, offset_y: i32) -> Option<usize> {
        let x_with_offset = x - offset_x;
        let y_with_offset = y - offset_y;
        if x_with_offset >= 0 && y_with_offset >= 0 && x_with_offset < WINDOW_WIDTH as i32 && y_with_offset < WINDOW_HEIGHT as i32 {
            let grid_x = x_with_offset / CELL_WIDTH as i32;
            let grid_y = y_with_offset / CELL_HEIGHT as i32;
            let grid_i = grid_x + grid_y * GRID_WIDTH as i32;
            return Some(grid_i as usize);
        } else {
            return None;
        }
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

        self.canvas.draw_line(
            Point::new(self.offset_x, self.offset_y),
            Point::new(self.config.window_width as i32 + self.offset_x, self.offset_y)
        ).expect("could not draw line");
        self.canvas.draw_line(
            Point::new(self.config.window_width as i32 + self.offset_x, self.offset_y),
            Point::new(self.config.window_width as i32 + self.offset_x, self.config.window_height as i32 + self.offset_y)
        ).expect("could not draw line");

        for i in 0..self.config.grid_height {
            self.canvas.draw_line(
                Point::new(self.offset_x, (i * self.config.cell_height + 10) as i32 + self.offset_y),
                Point::new(self.config.window_width as i32 + self.offset_x, (i * self.config.cell_height + 10) as i32 + self.offset_y)
            ).expect("could not draw line");
        }
        for i in 0..self.config.grid_width {
            self.canvas.draw_line(
                Point::new((i * self.config.cell_width)  as i32 + self.offset_x, self.offset_y),
                Point::new((i * self.config.cell_width) as i32 + self.offset_x, self.config.window_height as i32 + self.offset_y)
            ).expect("could not draw line");
        }
    }

    fn render_state(&mut self, state: &State) {
        for i in 0..GRID_SIZE {
            if state.is_live(i as i32) {
                let x = i % self.config.grid_width * self.config.cell_width;
                let y = i / self.config.grid_width * self.config.cell_height;
                self.render_live_cell(i);
            }
        }
    }

    fn render_hover(&mut self) {
        match Game::get_grid_i(self.event_pump.mouse_state().x(), self.event_pump.mouse_state().y(), self.offset_x, self.offset_y) {
            Some(i) => {
                self.render_hover_cell(i as u32);
            },
            None => {}
        };
    }

    fn render_live_cell(&mut self, i: u32) {
        let x = i % self.config.grid_width * self.config.cell_width;
        let y = i / self.config.grid_width * self.config.cell_height;
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.fill_rect(Rect::new(x as i32 + self.offset_x, y as i32 + self.offset_y, self.config.cell_width, self.config.cell_height)).expect("could not fill rect");
    }

    fn render_hover_cell(&mut self, i: u32) {
        let x = i as u32 % self.config.grid_width * self.config.cell_width;
        let y = i as u32 / self.config.grid_width * self.config.cell_height;
        self.canvas.set_draw_color(Color::GRAY);
        self.canvas.fill_rect(Rect::new(x as i32 + self.offset_x, y as i32 + self.offset_y, self.config.cell_width, self.config.cell_height)).expect("could not fill rect");
    }
}


pub struct Cell {
    live: Option<f32>
}

impl Cell {
    pub fn new(live: Option<f32>, hover: Option<f32>) -> Cell {
        Cell {
            live: live
        }
    }
}

pub struct State {
    game_grid: [Option<Cell>; GRID_SIZE as usize]
}

impl State {
    pub fn new() -> State {
        State {
            game_grid: std::array::from_fn(|_| None)
        }
    }

    pub fn reset(&mut self) {
        self.game_grid = std::array::from_fn(|_| None);
    }

    pub fn is_live(&self, i: i32) -> bool {
        match &self.game_grid[i as usize] {
            Some(Cell { live: Some(_), .. } ) => { true },
            _ => { false }
        }
    }

    pub fn should_live(&self, i: i32) -> bool {
        match self.neighbor_count(i) {
            2 => { self.is_live(i) },
            3 => { true },
            _ => { false }
        }
    }

    pub fn get_live(&self, i: i32) -> Option<f32> {
        match &self.game_grid[i as usize] {
            Some(Cell { live: Some(t), .. } ) => {
                return Some(*t);
            },
            _ => { None }
        }
    }

    pub fn next(&self, i: i32, t: f32) -> Option<Cell> {
        if self.should_live(i) {
            return Some(
                Cell::new(
                    self.get_live(i).or(Some(t)),
                    None
                )
            );
        } else {
            None
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
        if !left && self.is_live(ii){ count += 1 }

        ii = i + 1;
        if !right && self.is_live(ii) { count += 1 }

        ii = i - grid_width - 1;
        if !top && !left && self.is_live(ii) { count += 1 }

        ii = i - grid_width;
        if !top && self.is_live(ii) { count += 1 }

        ii = i - grid_width + 1;
        if !top && !right && self.is_live(ii) { count += 1 }

        ii = i + grid_width - 1;
        if !bottom && !left && self.is_live(ii) { count += 1 }

        ii = i + grid_width;
        if !bottom && self.is_live(ii) { count += 1 }

        ii = i + grid_width + 1;
        if !bottom && !right && self.is_live(ii) { count += 1 }

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

    // https://gafferongames.com/post/fix_your_timestep/
    let mut t: f32 = 0.0;

    let mut timestep = TimeStep::new();
    let mut accumulator = 0.0;
    let mut state = State::new();
    state.game_grid[1255] = Some(Cell::new(Some(t), None));
    state.game_grid[1256] = Some(Cell::new(Some(t), None));
    state.game_grid[1257] = Some(Cell::new(Some(t), None));

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
