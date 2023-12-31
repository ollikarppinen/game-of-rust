use std::{collections::HashSet, fmt};

use crate::{timestep::TimeStep, coord::Coord};

const INITIAL_CAMERA_X: f32 = 390.0;
const INITIAL_CAMERA_Y: f32 = 390.0;
const INITIAL_CAMERA_Z: f32 = 1.0;
const INITIAL_CELL_UPDATE_INTERVAL: f32 = 20.0;
const INITIAL_CELL_WIDTH: f32 = 1.0;
const INITIAL_CELL_HEIGHT: f32 = 1.0;

pub struct State {
    pub cell_coords: HashSet<Coord>,
    pub cell_update_interval: f32,
    pub t_since_last_cell_update: f32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub running: bool,
    pub paused: bool,
    pub camera_x: f32,
    pub camera_y: f32,
    pub camera_z: f32,
    pub camera_x_i: f32,
    pub camera_y_i: f32,
    pub camera_z_i: f32,
    pub camera_x_velocity: f32,
    pub camera_y_velocity: f32,
    pub camera_z_velocity: f32,
    pub camera_x_acceleration: f32,
    pub camera_y_acceleration: f32,
    pub camera_z_acceleration: f32,
    pub cell_width: f32,
    pub cell_height: f32,
    pub t: f32,
    pub timestep: TimeStep,
    pub fps: u32,
    pub font: Option<sdl2::ttf::Font<'static, 'static>>,
}

impl State {
    pub fn new() -> State {
        State {
            cell_coords: HashSet::new(),
            cell_update_interval: INITIAL_CELL_UPDATE_INTERVAL,
            t_since_last_cell_update: 0.0,
            cursor_x: 0,
            cursor_y: 0,
            running: true,
            paused: false,
            camera_x: INITIAL_CAMERA_X,
            camera_y: INITIAL_CAMERA_Y,
            camera_z: INITIAL_CAMERA_Z,
            camera_x_i: 0.0,
            camera_y_i: 0.0,
            camera_z_i: 0.0,
            camera_x_velocity: 0.0,
            camera_y_velocity: 0.0,
            camera_z_velocity: 0.0,
            camera_x_acceleration: 0.0,
            camera_y_acceleration: 0.0,
            camera_z_acceleration: 0.0,
            cell_width: INITIAL_CELL_WIDTH,
            cell_height: INITIAL_CELL_HEIGHT,
            t: 0.0,
            timestep: TimeStep::new(),
            fps: 0,
            font: None
        }
    }

    pub fn reset_cell_coords(&mut self) {
        self.cell_coords = HashSet::new();
    }

    pub fn reset_camera(&mut self) {
        self.camera_x = INITIAL_CAMERA_X;
        self.camera_y = INITIAL_CAMERA_Y;
        self.camera_z = INITIAL_CAMERA_Z;
        self.cell_width = INITIAL_CELL_WIDTH;
        self.cell_height = INITIAL_CELL_HEIGHT;
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "camera_x: {}, camera_y: {}, cell_width: {}, cell_height: {}", self.camera_x, self.camera_y, self.cell_width, self.cell_height)
    }
}
