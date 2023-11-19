use sdl2::{pixels::Color, render::Canvas};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::time::Instant;

const MS_PER_UPDATE: f32 = 16.0;
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
    frame_count: u8,
    rect_x: i32,
    rect_y: i32,
    rect_xd: i32,
    rect_yd: i32,
}

impl Game {
    pub fn new(canvas: Canvas<Window>) -> Game {
        Game {
            canvas: canvas,
            frame_count: 0,
            rect_x: 350,
            rect_y: 250,
            rect_xd: 1,
            rect_yd: 1,
        }
    }

    pub fn update(&mut self, t: f32) {
        self.frame_count = (self.frame_count + 1) % 255;

        self.rect_x = self.rect_x + self.rect_xd;
        self.rect_y = self.rect_y + self.rect_yd;

        if self.rect_x <= 0 { self.rect_xd = 1; }
        if self.rect_y <= 0 { self.rect_yd = 1; }

        if self.rect_x >= 700 { self.rect_xd = -1; }
        if self.rect_y >= 500 { self.rect_yd = -1; }
    }

    pub fn render(&mut self) {
        let color = Color::RGB(self.frame_count, 64, 255 - self.frame_count);
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.fill_rect(Rect::new(self.rect_x, self.rect_y, 100, 100)).expect("could not fill rect");
        self.canvas.present();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game-of-rust", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    let mut timestep = TimeStep::new();
    let mut game = Game::new(canvas);
    let mut lag = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { ..  } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        lag += timestep.delta();
        while lag >= MS_PER_UPDATE {
            game.update(MS_PER_UPDATE * 0.01);
            lag -= MS_PER_UPDATE;
        }

        game.render();
    }

    Ok(())
}
