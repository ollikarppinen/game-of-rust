use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Coord;
use crate::state::State;

pub fn initial_state() -> State {
    let mut state = State::new();

    let file = File::open("./initial_cells.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let mut coords: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    while !coords.is_empty() {
        match (coords.pop(), coords.pop()) {
            (Some(x), Some(y)) => {
                state.cell_coords.insert(Coord::new(x, y));
            },
            _ => {}
        }
    }

    state
}

pub fn update(state: &mut State, dt: f32) -> () {
    update_camera(state, state.t, dt);
    update_cells(state, state.t, dt);
}

fn update_cells(state: &mut State, t: f32, dt: f32) -> () {
    if state.paused { return }
    if state.t_since_last_cell_update < state.cell_update_interval {
        state.t_since_last_cell_update += dt;
        return
    }

    let mut new_cell_coords = HashSet::new();

    for cell_coord in &state.cell_coords {
        if state.should_live(&cell_coord) { new_cell_coords.insert(cell_coord.clone()); }
        for neighbor_coord in cell_coord.neighbors() {
            if state.should_live(&neighbor_coord) { new_cell_coords.insert(neighbor_coord.clone()); }
        }
    }

    state.t_since_last_cell_update -= state.cell_update_interval;
    state.cell_coords = new_cell_coords;
}

fn update_camera(state: &mut State, t: f32, dt: f32) -> () {
    state.camera_x_velocity += state.camera_x_acceleration;
    state.camera_y_velocity += state.camera_y_acceleration;
    state.camera_x_offset += state.camera_x_velocity as i32;
    state.camera_y_offset += state.camera_y_velocity as i32;
}
