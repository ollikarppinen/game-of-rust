use sdl2::{keyboard::{self, Keycode}, event::Event};

use crate::{Config, utils, state::State};

pub fn handle_inputs(state: &mut State, event_pump: &mut sdl2::EventPump, config: &Config) -> () {
    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
        if state.camera_y_acceleration < 5.0 { state.camera_y_acceleration += config.camera_xy_acceleration }
    } else if state.camera_y_acceleration > 0.0 {
        state.camera_y_acceleration = 0.0;
        state.camera_y_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
        if state.camera_y_acceleration > -5.0 { state.camera_y_acceleration -= config.camera_xy_acceleration }
    } else if state.camera_y_acceleration < 0.0 {
        state.camera_y_acceleration = 0.0;
        state.camera_y_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
        if state.camera_x_acceleration < 5.0 { state.camera_x_acceleration += config.camera_xy_acceleration }
    } else if state.camera_x_acceleration > 0.0 {
        state.camera_x_acceleration = 0.0;
        state.camera_x_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
        if state.camera_x_acceleration > -5.0 { state.camera_x_acceleration -= config.camera_xy_acceleration }
    } else if state.camera_x_acceleration < 0.0 {
        state.camera_x_acceleration = 0.0;
        state.camera_x_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::Z) {
        if state.camera_z_acceleration > -0.1 { state.camera_z_acceleration -= 0.001 }
    } else if state.camera_z_acceleration < 0.0 {
        state.camera_z_acceleration = 0.0;
        state.camera_z_velocity = 0.0;
    }

    if event_pump.keyboard_state().is_scancode_pressed(keyboard::Scancode::X) {
        if state.camera_z_acceleration < 0.1 { state.camera_z_acceleration += 0.001 }
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
                state.reset_cell_coords();
                state.reset_camera_offset();
            },
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                for coord in &state.cell_coords {
                    println!("{}", coord.x);
                    println!("{}", coord.y);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                println!("x offset: {}", state.camera_position_x);
                println!("y offset: {}", state.camera_position_y);
                println!("cell size: {}", state.cell_width);
                let center_x = config.window_width as f32 / state.cell_width;
                println!("center x: {}", center_x);
                let center_y = config.window_height as f32 / state.cell_height;
                println!("center y: {}", center_y);
            },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                println!("FPS: {}", state.timestep.frame_rate().unwrap_or(0));
            },
            Event::MouseButtonDown { x, y, .. } => {
                let coord = utils::screen_coord_to_game_coord(x, y, state);
                println!("x: {}, y: {}, offset x: {}, offset y: {}, coord: {}", x, y, state.camera_position_x, state.camera_position_y, coord);
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
