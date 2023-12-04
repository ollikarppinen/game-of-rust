use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Coord;
use crate::config::Config;
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

pub fn update(state: &mut State, config: &Config) -> () {
    update_cells(state, config);
}

pub fn update_fps(state: &mut State, config: &Config) -> () {
    match state.timestep.frame_rate() {
        Some(fps) => { state.fps = fps },
        None => {}
    }
}

fn update_cells(state: &mut State, config: &Config) -> () {
    if state.paused { return }
    if state.t_since_last_cell_update < state.cell_update_interval {
        state.t_since_last_cell_update += config.dt;
        return
    }

    let mut new_cell_coords = HashSet::new();

    // TODO: parallelize loop?
    // state.cell_coords.iter().for_each(|c| {
    //     if state.should_live(&c) { new_cell_coords.insert(c.clone()); }
    //     c.neighbors().iter().for_each(|c| {
    //         if state.should_live(&c) { new_cell_coords.insert(c.clone()); }
    //     });
    // });
    for cell_coord in &state.cell_coords {
        if state.should_live(&cell_coord) { new_cell_coords.insert(cell_coord.clone()); }
        for neighbor_coord in cell_coord.neighbors() {
            if state.should_live(&neighbor_coord) { new_cell_coords.insert(neighbor_coord.clone()); }
        }
    }

    state.t_since_last_cell_update -= state.cell_update_interval;
    state.cell_coords = new_cell_coords;
}

pub fn update_camera(state: &mut State, config: &Config) -> () {
    if state.camera_x_i != 0.0 {
        if state.camera_x_acceleration < config.camera_xy_acceleration_max {
            state.camera_x_acceleration = (state.camera_x_acceleration + config.camera_xy_acceleration).min(config.camera_xy_acceleration_max);
        }
        if state.camera_x_velocity < config.camera_xy_velocity_max {
            state.camera_x_velocity = (state.camera_x_velocity + state.camera_x_acceleration).min(config.camera_xy_velocity_max);
        }
    } else {
        state.camera_x_acceleration = 0.0;
        state.camera_x_velocity = 0.0;
    }
    state.camera_x += state.camera_x_velocity * state.camera_x_i;

    if state.camera_y_i != 0.0 {
        if state.camera_y_acceleration < config.camera_xy_acceleration_max {
            state.camera_y_acceleration = (state.camera_y_acceleration + config.camera_xy_acceleration).min(config.camera_xy_acceleration_max);
        }
        if state.camera_y_velocity < config.camera_xy_velocity_max {
            state.camera_y_velocity = (state.camera_y_velocity + state.camera_y_acceleration).min(config.camera_xy_velocity_max);
        }
    } else {
        state.camera_y_acceleration = 0.0;
        state.camera_y_velocity = 0.0;
    }
    state.camera_y += state.camera_y_velocity * state.camera_y_i;

    if state.camera_z_i != 0.0 {
        if state.camera_z_acceleration < config.camera_xy_acceleration_max {
            state.camera_z_acceleration = (state.camera_z_acceleration + config.camera_xy_acceleration).min(config.camera_xy_acceleration_max);
        }
        state.camera_z_velocity += state.camera_z_acceleration;
    } else {
        state.camera_z_acceleration = 0.0;
        state.camera_z_velocity = 0.0;
    }

    let old_cell_width = state.cell_width.clone();
    let old_cell_height = state.cell_height.clone();

    state.cell_width += state.camera_z_velocity * state.camera_z_i;
    state.cell_height += state.camera_z_velocity * state.camera_z_i;
    if state.cell_width < config.min_cell_width { state.cell_width = config.min_cell_width }
    if state.cell_width > config.max_cell_width { state.cell_width = config.max_cell_width }
    if state.cell_height < config.min_cell_height { state.cell_height = config.min_cell_height }
    if state.cell_height > config.max_cell_height { state.cell_height = config.max_cell_height }

    state.camera_x += (state.camera_x / old_cell_width - state.camera_x / state.cell_width + (config.window_width / old_cell_width - config.window_width / state.cell_width) / 2.0) * state.cell_width;
    state.camera_y += (state.camera_y / old_cell_height - state.camera_y / state.cell_height + (config.window_height / old_cell_height - config.window_height / state.cell_height) / 2.0) * state.cell_height;
}
