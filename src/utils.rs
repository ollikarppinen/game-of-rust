use crate::{Coord, state::State};

pub fn screen_coord_to_game_coord(x: i32, y: i32, state: &State) -> Coord {
    let mut x = x + state.camera_position_x.round() as i32;
    let mut y = y + state.camera_position_y.round() as i32;
    if x < 0 { x = x - state.cell_width.round() as i32 + 1 }
    if y < 0 { y = y - state.cell_height.round() as i32 + 1 }
    x = (x as f32 / state.cell_width.round() as f32) as i32;
    y = (y as f32 / state.cell_height.round() as f32) as i32;
    Coord::new(x, y)
}
