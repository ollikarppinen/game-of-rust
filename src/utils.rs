use crate::{Coord, Config};

pub fn screen_coord_to_game_coord(x: i32, y: i32, offset_x: i32, offset_y: i32, config: &Config) -> Coord {
    let mut x = x + offset_x;
    let mut y = y + offset_y;
    if x < 0 { x = x - config.cell_width as i32 + 1 }
    if y < 0 { y = y - config.cell_height as i32 + 1 }
    x = (x as f32 / config.cell_width as f32) as i32;
    y = (y as f32 / config.cell_height as f32) as i32;
    Coord::new(x, y)
}
