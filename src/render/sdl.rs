use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::core::{Cell, State, coords_from_index};

const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);

const FPS: u32 = 30;
const FRAME_DURATION_MS: u32 = 1000 / FPS;

pub fn run(state: &mut State, scale: u32) -> Result<(), String> {
    let cols = state.cols;
    let rows = state.rows;

    let sdl_context = sdl2::init()?;

    let video = sdl_context.video()?;
    let timer = sdl_context.timer()?;

    let window = video
        .window("game-of-sdl2", cols * scale, rows * scale)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut rect = Rect::new(0, 0, scale, scale);
    'running: loop {
        let frame_start = timer.ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(COLOR_DEAD);
        canvas.clear();

        canvas.set_draw_color(COLOR_ALIVE);
        for (i, cell) in state.curr.iter().enumerate() {
            if matches!(cell, Cell::Alive) {
                let coords = coords_from_index(i, cols);
                rect.x = coords.x * scale as i32;
                rect.y = coords.y * scale as i32;

                canvas.fill_rect(rect)?;
            }
        }

        canvas.present();

        state.next();

        let frame_time = timer.ticks() - frame_start;
        if frame_time < FRAME_DURATION_MS {
            timer.delay(FRAME_DURATION_MS - frame_time);
        }
    }

    Ok(())
}
