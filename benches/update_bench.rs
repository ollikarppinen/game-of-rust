#![feature(test)]

extern crate test;

use game_of_rust::{state_mgmt, state::State, config::Config, rle};
use test::Bencher;

#[bench]

fn pattern_60P5H2V0(b: &mut Bencher) {
    let mut state = State::new();
    state.cell_update_interval = 0.0;
    state.paused = false;
    let mut config = Config::new();
    let _ = rle::load_pattern("./patterns/60P5H2V0.rle", &mut state, 0, 0);
    b.iter(|| test::black_box(state_mgmt::update(&mut state, &mut config)));
}

#[bench]
fn pattern_gosperguninlineinverter(b: &mut Bencher) {
    let mut state = State::new();
    state.cell_update_interval = 0.0;
    state.paused = false;
    let mut config = Config::new();
    let _ = rle::load_pattern("./patterns/gosperguninlineinverter.rle", &mut state, 0, 0);
    b.iter(|| test::black_box(state_mgmt::update(&mut state, &mut config)));
}

#[bench]

fn pattern_empty(b: &mut Bencher) {
    let mut state = State::new();
    state.cell_update_interval = 0.0;
    state.paused = false;
    let mut config = Config::new();
    b.iter(|| test::black_box(state_mgmt::update(&mut state, &mut config)));
}

#[bench]

fn pattern_p82pihassler(b: &mut Bencher) {
    let mut state = State::new();
    state.cell_update_interval = 0.0;
    state.paused = false;
    let mut config = Config::new();
    let _ = rle::load_pattern("./patterns/p82pihassler.rle", &mut state, 0, 0);
    b.iter(|| test::black_box(state_mgmt::update(&mut state, &mut config)));
}
