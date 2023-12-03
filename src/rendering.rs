use sdl2::rect::{Rect, Point};
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;
use crate::state::State;
use crate::{Coord, utils};
use crate::Config;

pub fn render(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> () {
    let color = config.background_color;
    canvas.set_draw_color(color);
    canvas.clear();

    render_hover(canvas, &state, &config);
    render_state(canvas, &state, &config);
    render_grid(canvas, &state, &config);

    if state.t < config.intro_duration_ms { let _ = render_intro(canvas, &state, &config); }
    if state.paused { render_paused(canvas, &state, &config) }

    canvas.present();
}

fn render_intro(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> Result<(), String> {
    let message: String = "Game of Rust".to_string();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(config.font_path, 64)?;
    let texture_creator = canvas.texture_creator();
    let mut color = config.font_color.clone();
    color.a = 255;
    if state.t > config.intro_duration_ms / 2.0 {
        let x = (255.0 * ((state.t - config.intro_duration_ms / 2.0) / (config.intro_duration_ms / 2.0))) as u8;
        color.a -= x;
    }
    let surface = font
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(config.font_path, 64)?;
    let texture_creator = canvas.texture_creator();
    let surface = font
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

    let dx: i32 = state.cell_width as i32;
    let dy: i32 = state.cell_height as i32;
    let mut x: i32 = state.camera_position_x.round() as i32 % dx;
    let mut y: i32 = state.camera_position_y.round() as i32 % dy;
    if x < 0 { x *= -1 } else { x = dx - x }
    if y < 0 { y *= -1 } else { y = dy - y  }
    let max_x = config.window_width.round() as i32;
    let max_y = config.window_height.round() as i32;

    while x < max_x {
        canvas.draw_line(Point::new(x, 0), Point::new(x, max_y)).expect("could not draw line");
        x += dx;
    }
    while y < max_y {
        canvas.draw_line(Point::new(0, y), Point::new(max_x, y)).expect("could not draw line");
        y += dy;
    }
}

fn render_state(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    let mut y: i32 = 0;  
    while y < config.window_height as i32 {
        let mut x: i32 = 0;
        while x < config.window_width as i32 {
            let coord = utils::screen_coord_to_game_coord(x, y, state);
            if state.cell_coords.contains(&coord) {
                render_cell(canvas, state, &coord, config.cell_color);
            }
            x += state.cell_width.round() as i32;
        }
        y += state.cell_height.round() as i32;
    }
}

fn render_hover(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    if state.cell_height < 5.0 { return }

    let coord = utils::screen_coord_to_game_coord(
        state.cursor_x,
        state.cursor_y,
        state
    );
    render_cell(canvas, state, &coord, config.hover_color);
}

fn render_cell(canvas: &mut Canvas<Window>, state: &State, coord: &Coord, color: Color) {
    let x = coord.x * state.cell_width.round() as i32 - state.camera_position_x.round() as i32;
    let y = coord.y * state.cell_height.round() as i32 - state.camera_position_y.round() as i32;
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, state.cell_width.round() as u32, state.cell_height.round() as u32)).expect("could not fill rect");
}
