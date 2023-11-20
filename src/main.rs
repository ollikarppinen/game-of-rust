use sdl2::{pixels::Color, render::Canvas};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::time::Instant;

const MS_PER_UPDATE: f32 = 16.0;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const CELL_WIDTH: u32 = 10;
const CELL_HEIGHT: u32 = 10;
const GRID_WIDTH: u32 = WINDOW_WIDTH / CELL_WIDTH;
const GRID_HEIGHT: u32 = WINDOW_HEIGHT / CELL_HEIGHT;
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
        let tmp;
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
    rect_x: i32,
    rect_y: i32,
    rect_xd: i32,
    rect_yd: i32,
    running: bool
}

impl Game {
    pub fn new(canvas: Canvas<Window>, event_pump: sdl2::EventPump) -> Game {
        Game {
            canvas: canvas,
            event_pump: event_pump,
            frame_count: 0,
            rect_x: 350,
            rect_y: 250,
            rect_xd: 1,
            rect_yd: 1,
            running: true
        }
    }

    pub fn update(&mut self, mut state: State, t: f32, dt: f32) -> State {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { ..  } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.running = false;
                },
                Event::MouseButtonDown { x, y, .. } => {
                    let grid_x = x / CELL_WIDTH as i32;
                    let grid_y = y / CELL_HEIGHT as i32;
                    let grid_i = grid_x + grid_y * GRID_WIDTH as i32;
                    println!("Mouse clicked at: {}, {}", x, y);
                    println!("Grid at: {}, {}", grid_x, grid_y);
                    println!("Grid i: {}", grid_i);
                    let cell = Cell::new(t);
                    state.grid[grid_i as usize] = Some(Box::new(cell))
                },
                _ => {}
            }
        }

        self.frame_count = (self.frame_count + 1) % 255;

        self.rect_x = self.rect_x + self.rect_xd;
        self.rect_y = self.rect_y + self.rect_yd;

        if self.rect_x <= 0 { self.rect_xd = 1; }
        if self.rect_y <= 0 { self.rect_yd = 1; }

        if self.rect_x >= 790 { self.rect_xd = -1; }
        if self.rect_y >= 590 { self.rect_yd = -1; }

        state
    }

    pub fn render(&mut self, state: &State) {
        let color = Color::RGB(self.frame_count, 64, 255 - self.frame_count);
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::BLACK);

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
        self.canvas.fill_rect(Rect::new(self.rect_x, self.rect_y, 10, 10)).expect("could not fill rect");
        self.canvas.present();
    }
}

pub struct Cell {
    created_at: f32
}

impl Cell {
    pub fn new(t: f32) -> Cell {
        Cell {
            created_at: t
        }
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
