use sdl2::rect::{Rect, Point};
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;
use crate::state::State;
use crate::{Coord, utils};
use crate::Config;

pub fn render(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> () {
    let color = Color::WHITE;
    canvas.set_draw_color(color);
    canvas.clear();

    render_hover(canvas, &state);
    render_state(canvas, &state, &config);
    render_grid(canvas, &state, &config);

    if state.paused {
        let message: String = "Paused".to_string();
        let _ = render_message(&message, canvas, &state, &config);
    }

    canvas.present();
}

fn render_message(message: &String, canvas: &mut Canvas<Window>, _state: &State, config: &Config) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("./ARCADECLASSIC.TTF", 512)?;
    let texture_creator = canvas.texture_creator();
    let surface = font
        .render(message)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let message_width = message.len() as u32 * 30;
    let message_height = 50;
    let target = Rect::new(
        (config.window_width - message_width) as i32 / 2,
        (config.window_height - message_height) as i32 / 2,
        message_width,
        message_height
    );
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn render_grid(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    if state.cell_height <= 3 {
        return
    } else {
        let i = (255 - (state.cell_height as i32 - 3) * 15) as u8;
        canvas.set_draw_color(Color::RGB(i, i, i));
    }

    let mut i: i32 = 0;
    let max_i: i32 = if config.window_height > config.window_width { config.window_height as i32 } else { config.window_width as i32 };
    let di: i32 = if config.window_height > config.window_width { state.cell_width as i32 } else { state.cell_height as i32 };
    while i < max_i {
        let coord = utils::screen_coord_to_game_coord(i, i, state);
        let x = coord.x * state.cell_width as i32 - state.camera_x_offset;
        let y = coord.y * state.cell_height as i32 - state.camera_y_offset;
        canvas.draw_line(
            Point::new(
                x,
                0
            ),
            Point::new(
                x,
                config.window_height as i32
            )
        ).expect("could not draw line");
        canvas.draw_line(
            Point::new(
                0,
                y
            ),
            Point::new(
                config.window_width as i32,
                y
            )
        ).expect("could not draw line");
        i += di;
    }
}

fn render_state(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    let mut y: i32 = 0;  
    while y < config.window_height as i32 {
        let mut x: i32 = 0;
        while x < config.window_width as i32 {
            let coord = utils::screen_coord_to_game_coord(x, y, state);
            if state.cell_coords.contains(&coord) {
                render_cell(canvas, state, &coord, Color::BLACK);
            }
            x += state.cell_width as i32;
        }
        y += state.cell_height as i32;
    }
}

fn render_hover(canvas: &mut Canvas<Window>, state: &State) {
    let coord = utils::screen_coord_to_game_coord(
        state.cursor_x,
        state.cursor_y,
        state
    );
    render_cell(canvas, state, &coord, Color::GRAY);
}

fn render_cell(canvas: &mut Canvas<Window>, state: &State, coord: &Coord, color: Color) {
    let x = coord.x * state.cell_width as i32 - state.camera_x_offset;
    let y = coord.y * state.cell_height as i32 - state.camera_y_offset;
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, state.cell_width, state.cell_height)).expect("could not fill rect");
}
