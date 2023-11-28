use sdl2::{keyboard::{self, Keycode}, event::Event};

use crate::{State, Config, Game};

pub fn handle_inputs(state: &mut State, event_pump: &mut sdl2::EventPump, config: &Config) -> () {
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
        state.camera_y_offset += 1;
        println!("offset_y: {}", state.camera_y_offset);
    }
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
        state.camera_y_offset -= 1;
        println!("offset_y: {}", state.camera_y_offset);
    }
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
        state.camera_x_offset += 1;
        println!("offset_x: {}", state.camera_x_offset);
    }
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
        state.camera_x_offset -= 1;
        println!("offset_x: {}", state.camera_x_offset);
    }

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { ..  } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                state.running = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                state.paused = !state.paused;
            },
            Event::KeyDown { keycode: Some(Keycode::Plus), .. } => {
                // if config.dt > 1.0 { config.dt /= 2.0 }
                println!("dt: {}", config.dt);
            },
            Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                // if config.dt < 1000.0 { config.dt *= 2.0 }
                println!("dt: {}", config.dt);
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                state.reset();
                state.camera_x_offset = 0;
                state.camera_y_offset = 0;
            },
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                for coord in &state.cell_coords {
                    println!("{}", coord.x);
                    println!("{}", coord.y);
                }
            },
            Event::MouseButtonDown { x, y, .. } => {
                let coord = Game::screen_coord_to_game_coord(x, y, state.camera_x_offset, state.camera_y_offset, config);
                println!("x: {}, y: {}, offset x: {}, offset y: {}, coord: {}", x, y, state.camera_x_offset, state.camera_y_offset, coord);
                if state.cell_coords.contains(&coord) {
                    state.cell_coords.remove(&coord);
                } else {
                    state.cell_coords.insert(coord);
                }
            },
            _ => {}
        }
    }
}
