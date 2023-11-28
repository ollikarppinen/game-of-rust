use std::collections::HashSet;

use crate::Coord;

const INITIAL_X_OFFSET: i32 = 0;
const INITIAL_Y_OFFSET: i32 = 0;

pub struct State {
    pub cell_coords: HashSet<Coord>,
    pub camera_x_offset: i32,
    pub camera_y_offset: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub running: bool,
    pub paused: bool
}

impl State {
    pub fn new() -> State {
        State {
            cell_coords: HashSet::new(),
            camera_x_offset: INITIAL_X_OFFSET,
            camera_y_offset: INITIAL_Y_OFFSET,
            cursor_x: 0,
            cursor_y: 0,
            running: true,
            paused: false
        }
    }

    pub fn reset(&mut self) {
        self.cell_coords = HashSet::new();
    }

    pub fn is_live(&self, coord: &Coord) -> bool {
        return self.cell_coords.contains(&coord);
    }

    pub fn should_live(&self, coord: &Coord) -> bool {
        match self.neighbor_count(&coord) {
            2 => { self.is_live(&coord) },
            3 => { true },
            _ => { false }
        }
    }

    pub fn neighbor_count(&self, coord: &Coord) -> u8 {
        let mut count = 0;

        for coord in coord.neighbors() {
            if self.is_live(&coord) { count += 1 }
        }

        count
    }
}
