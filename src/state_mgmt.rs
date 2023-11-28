use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{State, Coord};

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

pub fn update(state: &mut State, t: f32) -> () {
    if state.paused { return }

    let mut new_cell_coords = HashSet::new();

    for cell_coord in &state.cell_coords {
        if state.should_live(&cell_coord) { new_cell_coords.insert(cell_coord.clone()); }
        for neighbor_coord in cell_coord.neighbors() {
            if state.should_live(&neighbor_coord) { new_cell_coords.insert(neighbor_coord.clone()); }
        }
    }

    state.cell_coords = new_cell_coords;
}
