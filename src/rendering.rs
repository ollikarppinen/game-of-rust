use sdl2::rect::{Rect, Point};
use sdl2::{render::Canvas, pixels::Color};
use sdl2::video::Window;
use crate::{State, Coord, utils};
use crate::Config;

pub fn render(canvas: &mut Canvas<Window>, state: &State, config: &Config) -> () {
    let color = Color::WHITE;
    canvas.set_draw_color(color);
    canvas.clear();

    render_hover(canvas, &state, &config);
    render_state(canvas, &state, &config);
    render_grid(canvas, &state, &config);

    canvas.present();
}

fn render_grid(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    canvas.set_draw_color(Color::BLACK);

    let mut i: i32 = 0;
    let max_i: i32 = if config.window_height > config.window_width { config.window_height as i32 } else { config.window_width as i32 };
    let di: i32 = if config.window_height > config.window_width { config.cell_width as i32 } else { config.cell_height as i32 };
    while i < max_i {
        let coord = utils::screen_coord_to_game_coord(i, i, state.camera_x_offset, state.camera_y_offset, &config);
        let x = coord.x * config.cell_width as i32 - state.camera_x_offset;
        let y = coord.y * config.cell_height as i32 - state.camera_y_offset;
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
            let coord = utils::screen_coord_to_game_coord(x, y, state.camera_x_offset, state.camera_y_offset, &config);
            if state.cell_coords.contains(&coord) {
                render_cell(canvas, state, config, &coord, Color::BLACK);
            }
            x += config.cell_width as i32;
        }
        y += config.cell_height as i32;
    }
}

fn render_hover(canvas: &mut Canvas<Window>, state: &State, config: &Config) {
    let coord = utils::screen_coord_to_game_coord(
        state.cursor_x,
        state.cursor_y,
        state.camera_x_offset,
        state.camera_y_offset,
        &config
    );
    render_cell(canvas, state, config, &coord, Color::GRAY);
}

fn render_cell(canvas: &mut Canvas<Window>, state: &State, config: &Config, coord: &Coord, color: Color) {
    let x = coord.x * config.cell_width as i32 - state.camera_x_offset;
    let y = coord.y * config.cell_height as i32 - state.camera_y_offset;
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, config.cell_width, config.cell_height)).expect("could not fill rect");
}
