use std::collections::HashSet;

use crate::{Coord, timestep::TimeStep};

const INITIAL_X_OFFSET: f32 = 0.0;
const INITIAL_Y_OFFSET: f32 = 0.0;
const INITIAL_CELL_UPDATE_INTERVAL: f32 = 100.0;
const INITIAL_CELL_WIDTH: f32 = 10.0;
const INITIAL_CELL_HEIGHT: f32 = 10.0;

pub struct State {
    pub cell_coords: HashSet<Coord>,
    pub cell_update_interval: f32,
    pub t_since_last_cell_update: f32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub running: bool,
    pub paused: bool,
    pub camera_position_x: f32,
    pub camera_position_y: f32,
    pub camera_x_velocity: f32,
    pub camera_y_velocity: f32,
    pub camera_z_velocity: f32,
    pub camera_x_acceleration: f32,
    pub camera_y_acceleration: f32,
    pub camera_z_acceleration: f32,
    pub cell_width: f32,
    pub cell_height: f32,
    pub t: f32,
    pub timestep: TimeStep
}

impl State {
    pub fn new() -> State {
        State {
            cell_coords: HashSet::new(),
            cell_update_interval: INITIAL_CELL_UPDATE_INTERVAL,
            t_since_last_cell_update: 0.0,
            camera_position_x: INITIAL_X_OFFSET,
            camera_position_y: INITIAL_Y_OFFSET,
            cursor_x: 0,
            cursor_y: 0,
            running: true,
            paused: false,
            camera_x_velocity: 0.0,
            camera_y_velocity: 0.0,
            camera_z_velocity: 0.0,
            camera_x_acceleration: 0.0,
            camera_y_acceleration: 0.0,
            camera_z_acceleration: 0.0,
            cell_width: INITIAL_CELL_WIDTH,
            cell_height: INITIAL_CELL_HEIGHT,
            t: 0.0,
            timestep: TimeStep::new()
        }
    }

    pub fn reset_cell_coords(&mut self) {
        self.cell_coords = HashSet::new();
    }

    pub fn reset_camera_offset(&mut self) {
        self.camera_position_x = INITIAL_X_OFFSET;
        self.camera_position_y = INITIAL_Y_OFFSET;
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
