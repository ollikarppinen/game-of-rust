use config::Config;
use timestep::TimeStep;
use coord::Coord;

mod rendering;
mod inputs;
mod state_mgmt;
mod utils;
mod state;
mod config;
mod timestep;
mod coord;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let config = Config::new();
    let window = video_subsystem.window("game-of-rust", config.window_width, config.window_height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    let mut state = state_mgmt::initial_state();

    // https://gafferongames.com/post/fix_your_timestep/
    let mut t: f32 = 0.0;
    let mut timestep = TimeStep::new();
    let mut accumulator = -1000.0;

    while state.running {
        let frame_time = timestep.delta();
        accumulator += frame_time;

        while accumulator >= config.dt {
            inputs::handle_inputs(&mut state, &mut event_pump, &config);
            state_mgmt::update(&mut state, t, config.dt);
            t += config.dt;
            accumulator -= config.dt;
        }

        // const double alpha = accumulator / dt;

        // State state = currentState * alpha +
        //     previousState * ( 1.0 - alpha );

        // render( state );

        rendering::render(&mut canvas, &state, &config);
    }

    Ok(())
}
