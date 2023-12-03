use crate::{Coord, state::State};

pub fn game_coord(x: f32, y: f32, state: &State) -> Coord {
    let mut x = x + state.camera_x;
    let mut y = y + state.camera_y;
    if x < 0.0 { x -= state.cell_width }
    if y < 0.0 { y -= state.cell_height }
    x /= state.cell_width;
    y /= state.cell_height;
    Coord::new(x.floor() as i32, y.floor() as i32)
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 - t) + end * t
}
