use sdl2::{
    Sdl, TimerSubsystem, event::Event, keyboard::Keycode, pixels::Color, rect::Rect,
    render::Canvas, video::Window,
};

use crate::core::{Cell, State, coords_from_index};

const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);

const FPS: u32 = 30;
const FRAME_DURATION_MS: u32 = 1000 / FPS;

pub struct SdlContext {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    timer: TimerSubsystem,

    scale: u32,
}

impl SdlContext {
    pub fn new(width: u32, height: u32, scale: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let timer = sdl_context.timer()?;
        let video = sdl_context.video()?;

        let window = video
            .window("game-of-sdl2", width * scale, height * scale)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(SdlContext {
            sdl_context,
            canvas,
            timer,
            scale,
        })
    }
}

pub struct Game {
    running: bool,
    sdl_context: SdlContext,
    state: State,
}

impl Game {
    pub fn new(sdl_context: SdlContext, state: State) -> Self {
        Game {
            running: false,
            sdl_context,
            state,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let scale = self.sdl_context.scale;

        let mut event_pump = self.sdl_context.sdl_context.event_pump()?;
        let mut rect = Rect::new(0, 0, scale, scale);
        'running: loop {
            let frame_start = self.sdl_context.timer.ticks();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => self.running = !self.running,
                    _ => {}
                }
            }

            self.update();
            self.render(&mut rect)?;

            let frame_time = self.sdl_context.timer.ticks() - frame_start;
            if frame_time < FRAME_DURATION_MS {
                self.sdl_context.timer.delay(FRAME_DURATION_MS - frame_time);
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        if self.running {
            self.state.next()
        }
    }

    fn render(&mut self, rect: &mut Rect) -> Result<(), String> {
        let cols = self.state.cols;
        let scale = self.sdl_context.scale;

        self.sdl_context.canvas.set_draw_color(COLOR_DEAD);
        self.sdl_context.canvas.clear();

        self.sdl_context.canvas.set_draw_color(COLOR_ALIVE);
        for (i, cell) in self.state.curr.iter().enumerate() {
            if matches!(cell, Cell::Alive) {
                let coords = coords_from_index(i, cols);
                rect.x = coords.x * scale as i32;
                rect.y = coords.y * scale as i32;

                self.sdl_context.canvas.fill_rect(*rect)?;
            }
        }

        Ok(self.sdl_context.canvas.present())
    }
}
