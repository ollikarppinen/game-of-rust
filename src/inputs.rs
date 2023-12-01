use sdl2::{keyboard::{self, Keycode}, event::Event};

use crate::{Config, utils, state::State};

// pub mod utils;

pub fn handle_inputs(state: &mut State, event_pump: &mut sdl2::EventPump, config: &Config) -> () {
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
        if state.camera_y_acceleration < 5.0 { state.camera_y_acceleration += 0.02 }
    } else if state.camera_y_acceleration > 0.0 {
        state.camera_y_acceleration = 0.0;
        state.camera_y_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
        if state.camera_y_acceleration > -5.0 { state.camera_y_acceleration -= 0.02 }
    } else if state.camera_y_acceleration < 0.0 {
        state.camera_y_acceleration = 0.0;
        state.camera_y_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
        if state.camera_x_acceleration < 5.0 { state.camera_x_acceleration += 0.02 }
    } else if state.camera_x_acceleration > 0.0 {
        state.camera_x_acceleration = 0.0;
        state.camera_x_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
        if state.camera_x_acceleration > -5.0 { state.camera_x_acceleration -= 0.02 }
    } else if state.camera_x_acceleration < 0.0 {
        state.camera_x_acceleration = 0.0;
        state.camera_x_velocity = 0.0;
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
                println!("dt: {}", config.dt);
            },
            Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                if state.cell_update_interval < 5000.0 { state.cell_update_interval *= 2.0 }
                println!("dt: {}", config.dt);
            },
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                // ZOOM OUT
                let old_center_coord = utils::screen_coord_to_game_coord(
                    (config.window_width as f32 / 2.0).round() as i32,
                    (config.window_height as f32 / 2.0).round() as i32,
                    state
                );
                if state.cell_width > config.min_cell_width { state.cell_width -= 1.0 }
                if state.cell_height > config.min_cell_height { state.cell_height -= 1.0 }
                let new_center_coord = utils::screen_coord_to_game_coord(
                    (config.window_width as f32 / 2.0).round() as i32,
                    (config.window_height as f32 / 2.0).round() as i32,
                    state
                );

                state.camera_x_offset += (old_center_coord.x - new_center_coord.x) * state.cell_width as i32;
                state.camera_y_offset += (old_center_coord.y - new_center_coord.y) * state.cell_height as i32;
            },
            Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                // ZOOM IN
                let old_center_coord = utils::screen_coord_to_game_coord(
                    (config.window_width as f32 / 2.0).round() as i32,
                    (config.window_height as f32 / 2.0).round() as i32,
                    state
                );
                if state.cell_width < config.max_cell_width { state.cell_width += 1.0 }
                if state.cell_height < config.max_cell_height { state.cell_height += 1.0 }
                let new_center_coord = utils::screen_coord_to_game_coord(
                    (config.window_width as f32 / 2.0).round() as i32,
                    (config.window_height as f32 / 2.0).round() as i32,
                    state
                );
                state.camera_x_offset += (old_center_coord.x - new_center_coord.x) * state.cell_width as i32;
                state.camera_y_offset += (old_center_coord.y - new_center_coord.y) * state.cell_height as i32;
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
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                println!("FPS: {}", state.timestep.frame_rate().unwrap_or(0));
            },
            Event::MouseButtonDown { x, y, .. } => {
                let coord = utils::screen_coord_to_game_coord(x, y, state);
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
