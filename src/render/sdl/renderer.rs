use super::layout::Layout;
use crate::core::{Cell, State, coords_from_index};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);
const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_STATUSBAR: Color = Color::RGB(0x00, 0x00, 0xff);

pub struct Renderer {
    canvas: Canvas<Window>,
    layout: Layout,
}

impl Renderer {
    pub fn new(layout: Layout, canvas: Canvas<Window>) -> Self {
        Renderer { canvas, layout }
    }

    pub fn draw(&mut self, state: &State) -> Result<(), String> {
        self.render_board(state)?;
        self.render_statusbar()?;
        self.canvas.present();
        Ok(())
    }

    pub fn render_board(&mut self, state: &State) -> Result<(), String> {
        let scale = self.layout.scale;

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

    pub fn render_statusbar(&mut self) -> Result<(), String> {
        let rect = Rect::new(
            self.layout.statbar.x as i32,
            self.layout.statbar.y as i32,
            self.layout.statbar.w,
            self.layout.statbar.h,
        );

        self.canvas.set_draw_color(COLOR_STATUSBAR);
        self.canvas.fill_rect(rect)
    }
}
