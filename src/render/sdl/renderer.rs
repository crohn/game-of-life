use crate::core::{Cell, Config, State, coords_from_index};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);
const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    config: &'a Config,
}

impl<'a> Renderer<'a> {
    pub fn new(config: &'a Config, canvas: Canvas<Window>) -> Self {
        Renderer { canvas, config }
    }

    pub fn render_board(&mut self, state: &State) -> Result<(), String> {
        let scale = self.config.scale();

        self.canvas.set_draw_color(COLOR_DEAD);
        self.canvas.clear();

        self.canvas.set_draw_color(COLOR_ALIVE);
        for (i, cell) in state.curr.iter().enumerate() {
            let coords = coords_from_index(i, state.cols);
            let rect = Rect::new(
                coords.x * scale as i32,
                coords.y * scale as i32,
                scale,
                scale,
            );
            if matches!(cell, Cell::Alive) {
                self.canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, state: &State) -> Result<(), String> {
        self.render_board(state)?;
        self.canvas.present();
        Ok(())
    }
}
