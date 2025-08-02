use sdl2::{pixels::Color, rect::Rect};

use crate::{
    core::{Cell, coords_from_index},
    render::sdl::renderer::RenderingContext,
};

use super::Widget;

const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);

pub struct Board {}
impl Widget for Board {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let scale = ctx.layout.scale;

        ctx.canvas.set_draw_color(COLOR_ALIVE); // TODO move to theme
        for (i, cell) in ctx.state.curr.iter().enumerate() {
            let coords = coords_from_index(i, ctx.state.cols);
            let rect = Rect::new(
                coords.x * scale as i32,
                coords.y * scale as i32,
                scale,
                scale,
            );
            if matches!(cell, Cell::Alive) {
                ctx.canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }
}
