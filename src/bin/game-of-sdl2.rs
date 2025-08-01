use game_of_life::{
    core::{Cell, Config, Coords, State},
    render::sdl::{
        event_handler::EventHandler, game::Game, layout::Layout, renderer::Renderer, timer::Timer,
    },
};

const COLS: u32 = 80;
const ROWS: u32 = 25;
const SCALE: u32 = 10;
const FPS: u64 = 30;

fn main() -> Result<(), String> {
    let config = Config {
        cols: COLS,
        rows: ROWS,
    };

    let layout = Layout::new(&config, SCALE);
    let window = layout.window_geometry();

    let sdl_ctx = sdl2::init()?;
    let video_sys = sdl_ctx.video()?;
    let timer_sys = sdl_ctx.timer()?;
    let event_pump = sdl_ctx.event_pump()?;

    let window = video_sys
        .window("game-of-sdl2", window.w, window.h)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let event_handler = EventHandler::new(event_pump);
    let renderer = Renderer::new(layout, canvas);
    let timer = Timer::new(timer_sys, FPS);

    let mut state = State::new(&config);

    state.set_cell(Coords { x: 40, y: 39 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 41 }, Cell::Alive);
    state.set_cell(Coords { x: 39, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 41, y: 41 }, Cell::Alive);

    let mut game = Game::new(event_handler, renderer, timer, state);
    game.run()?;

    Ok(())
}
