use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::TimerSubsystem;
use std::time::{Duration, Instant};

fn render(canvas: &mut WindowCanvas, color: Color, rect_x: i32, rect_y: i32) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.set_draw_color(Color::BLACK);
    canvas.fill_rect(Rect::new(rect_x, rect_y, 100, 100)).expect("could not fill rect");
    canvas.present();
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

    let timer = sdl_context.timer()?;
    let frequency = TimerSubsystem::performance_frequency(&timer) as u32;
    println!("frequency: {}", frequency);

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;
    let mut i = 0;

    let mut rect_x = 350;
    let mut rect_y = 250;

    let mut rect_xd = 1;
    let mut rect_yd = 1;

    'running: loop {
        let frame_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { ..  } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        i = (i + 1) % 255;

        rect_x = rect_x + rect_xd;
        rect_y = rect_y + rect_yd;

        if rect_x <= 0 { rect_xd = 1; }
        if rect_y <= 0 { rect_yd = 1; }

        if rect_x >= 700 { rect_xd = -1; }
        if rect_y >= 500 { rect_yd = -1; }

        let frame_duration = frame_start.elapsed().as_nanos() as u32;

        render(&mut canvas, Color::RGB(i, 64, 255 - i), rect_x, rect_y);
        let sleep_duration = (1_000_000_000u32 - frame_duration) / frequency;
        if (i as u32 % 1_000 == 0) {
            println!("frame_duration: {}", frame_duration);
            println!("frequency: {}", frame_duration);
            println!("sleep duration: {}", sleep_duration);
        }
        ::std::thread::sleep(Duration::new(0, sleep_duration))
    }

    Ok(())
}
