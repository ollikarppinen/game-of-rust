use sdl2::{keyboard::{self, Keycode}, event::Event};

use crate::{Config, utils::{self}, state::State};

pub fn handle_inputs(state: &mut State, event_pump: &mut sdl2::EventPump, config: &Config) -> () {
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
        state.camera_y_i = 1.0
    } else if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
        state.camera_y_i = -1.0
    } else {
        state.camera_y_i = 0.0
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
        state.camera_x_i = 1.0
    } else if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
        state.camera_x_i = -1.0
    } else {
        state.camera_x_i = 0.0
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Z) {
        if state.camera_z_acceleration > -config.camera_z_acceleration_max {
            state.camera_z_acceleration = (state.camera_z_acceleration - config.camera_z_acceleration).max(-config.camera_z_acceleration_max)
        }
    } else if state.camera_z_acceleration < 0.0 {
        state.camera_z_acceleration = 0.0;
        state.camera_z_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::X) {
        if state.camera_z_acceleration < config.camera_z_acceleration_max {
            state.camera_z_acceleration += (state.camera_z_acceleration + config.camera_z_acceleration).min(config.camera_z_acceleration_max)
        }
    } else if state.camera_z_acceleration > 0.0 {
        state.camera_z_acceleration = 0.0;
        state.camera_z_velocity = 0.0;
    }

    state.cursor_x = event_pump.mouse_state().x();
    state.cursor_y = event_pump.mouse_state().y();

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
                if state.cell_update_interval > config.dt { state.cell_update_interval /= 2.0 }
            },
            Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                if state.cell_update_interval < 5000.0 { state.cell_update_interval *= 2.0 }
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                state.reset_camera();
            },
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                for coord in &state.cell_coords {
                    println!("{}", coord.x);
                    println!("{}", coord.y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                let center_coord = utils::game_coord(config.window_width / 2.0, config.window_height / 2.0, state);
                println!("center coord: {}", center_coord);
            },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                println!("FPS: {}", state.timestep.frame_rate().unwrap_or(0));
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                println!("State: {}", state);
            },
            Event::MouseButtonDown { x, y, .. } => {
                let coord = utils::game_coord(x as f32, y as f32, state);
                println!("x: {}, y: {}, coord: {}", x, y, coord);
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
