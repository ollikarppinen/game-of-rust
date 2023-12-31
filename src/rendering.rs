use sdl2::rect::{Rect, Point};
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;

use crate::config::Config;
use crate::coord::Coord;
use crate::state::State;
use crate::utils;

pub fn render(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> () {
    let color = config.background_color;
    canvas.set_draw_color(color);
    canvas.clear();

    render_hover(canvas, &state, &config);
    render_state(canvas, &state, &config);
    render_grid(canvas, &state, &config);
    render_fps(canvas, &state, &config);

    if state.t < config.intro_duration_ms { let _ = render_intro(canvas, &state, &config); }
    if state.paused { render_paused(canvas, &state, &config) }

    canvas.present();
}

fn render_intro(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> Result<(), String> {
    let message: String = "Game of Rust".to_string();
    let texture_creator = canvas.texture_creator();
    let mut color = config.font_color.clone();
    color.a = 255;
    if state.t > config.intro_duration_ms / 2.0 {
        let x = (255.0 * ((state.t - config.intro_duration_ms / 2.0) / (config.intro_duration_ms / 2.0))) as u8;
        color.a -= x;
    }
    let surface = config.font.as_ref().unwrap()
        .render(&message)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let message_width = message.len() as f32 * config.char_width;
    let message_height = config.char_height;
    let x = (config.window_width - message_width) as i32 / 2;
    let y = (config.window_height - message_height) as i32 / 5;
    let target = Rect::new(
        x,
        y,
        message_width as u32,
        message_height as u32
    );
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn render_paused(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> () {
    render_blur(canvas, &state, &config);
    let message: String = "Paused".to_string();
    let _ = render_message_center(&message, canvas, &state, &config);
}

fn render_message_center(message: &String, canvas: &mut Canvas<Window>, _state: &State, config: &Config) -> Result<(), String> {
    let message_width = message.len() as f32 * config.char_width;
    let message_height = config.char_height;
    let x = (config.window_width - message_width) as i32 / 2;
    let y = (config.window_height - message_height) as i32 / 2;

    render_message(message, x, y, canvas, _state, config)
}

fn render_message(message: &String, x: i32, y: i32, canvas: &mut Canvas<Window>, _state: &State, config: &Config) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let surface = config.font.as_ref().unwrap()
        .render(message)
        .blended(config.font_color)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let message_width = message.len() as f32 * config.char_width;
    let message_height = config.char_height;
    let target = Rect::new(
        x,
        y,
        message_width as u32,
        message_height as u32
    );
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn render_blur(canvas: &mut Canvas<Window>, _state: &State, config: &Config) {
    let mut color = config.background_color.clone();
    color.a = 50;

    canvas.set_draw_color(color);
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.fill_rect(Rect::new(0, 0, config.window_width as u32, config.window_height as u32)).expect("could not fill rect");
}

fn render_grid(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    let mut color = config.grid_color.clone();
    color.a = (state.cell_height / config.max_cell_height * 255.0).round() as u8;
    canvas.set_draw_color(color);
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let dx: f32 = state.cell_width;
    let dy: f32 = state.cell_height;
    let mut x: f32 = state.camera_x % dx;
    let mut y: f32 = state.camera_y % dy;
    if x < 0.0 { x = -x } else { x = dx - x }
    if y < 0.0 { y = -y } else { y = dy - y  }
    let max_x = config.window_width;
    let max_y = config.window_height;

    while x < max_x {
        canvas.draw_line(Point::new(x.round() as i32, 0), Point::new(x.round() as i32, max_y.round() as i32)).expect("could not draw line");
        x += dx;
    }
    while y < max_y {
        canvas.draw_line(Point::new(0, y.round() as i32), Point::new(max_x.round() as i32, y.round() as i32)).expect("could not draw line");
        y += dy;
    }
}

fn render_state(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    let mut rects: Vec<Rect> = Vec::new();
    for cell_coord in state.cell_coords.iter() {
        let cell_wx = cell_coord.x as f32 * state.cell_width - state.camera_x;
        let cell_wy = cell_coord.y as f32 * state.cell_height - state.camera_y;
        if cell_wx > -state.cell_width && cell_wx < config.window_width && cell_wy > -state.cell_height && cell_wy < config.window_height {
            rects.push(
                Rect::new(
                    cell_wx.ceil() as i32,
                    cell_wy.ceil() as i32,
                    state.cell_width.ceil() as u32,
                    state.cell_height.ceil() as u32
                )
            );
        }
    }
    canvas.set_draw_color(config.cell_color);
    let _ =  canvas.fill_rects(&rects);
}

fn render_hover(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    if state.cell_height < 5.0 { return }

    let coord = utils::game_coord(
        state.cursor_x as f32,
        state.cursor_y as f32,
        state
    );
    render_cell(canvas, state, &coord, config.hover_color);
}

fn render_cell(canvas: &mut Canvas<Window>, state: &State, coord: &Coord, color: Color) {
    let x = coord.x as f32 * state.cell_width - state.camera_x;
    let y = coord.y as f32 * state.cell_height - state.camera_y;
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x.ceil() as i32, y.ceil() as i32, state.cell_width.ceil() as u32, state.cell_height.ceil() as u32)).expect("could not fill rect");
}

fn render_fps(canvas: &mut Canvas<Window>, state: &State, _config: &Config) {
    let _ = canvas.window_mut().set_title(&format!("game-of-rust, FPS: {}", state.fps));
}
