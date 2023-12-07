use game_of_rust::{config::Config, state::State, inputs, state_mgmt, rendering, rle};
use once_cell::sync::Lazy;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut config = Config::new();
    let window = video_subsystem.window("game-of-rust", config.window_width as u32, config.window_height as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    static ttf_context: Lazy<sdl2::ttf::Sdl2TtfContext> = Lazy::new(|| {
        sdl2::ttf::init().expect("could not create ttf context")
    });
    config.font = Some(ttf_context.load_font(config.font_path, 64)?);
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    // let mut state = state_mgmt::initial_state();
    let mut state = State::new();
    /*
    patterns:
    p960_2c5gun
    60P5H2V0
    otcametapixel
    gosperguninlineinverter
    p82pihassler
    simkinglidergunboatbitvariant
    twogun
     */
    rle::load_pattern("./patterns/p960_2c5gun.rle", &mut state, 0, 0);

    // https://gafferongames.com/post/fix_your_timestep/
    let mut accumulator = 0.0;

    while state.running {
        let frame_time = state.timestep.delta();
        accumulator += frame_time;
        inputs::handle_inputs(&mut state, &mut event_pump, &config);
        state_mgmt::update_fps(&mut state, &config);

        while accumulator >= config.dt {
            state_mgmt::update(&mut state, &config);
            state_mgmt::update_camera(&mut state, &config);
            state.t += config.dt;
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
