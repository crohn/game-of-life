use sdl2::rect::Rect;

use crate::core::Cell;
use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;

pub struct Board;

impl Widget for Board {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let scale = 10;
        let grid = if ctx.game_state.show_grid { 1 } else { 0 };

        for (coords, cell) in ctx.state.iter() {
            let rect = Rect::new(
                coords.x * scale as i32,
                coords.y * scale as i32,
                scale - grid,
                scale - grid,
            );

            match cell {
                Cell::Alive => {
                    ctx.canvas.set_draw_color(ctx.theme.palette.cell_alive);
                }
                Cell::Dead => {
                    ctx.canvas.set_draw_color(ctx.theme.palette.cell_dead);
                }
            }
            ctx.canvas.fill_rect(rect)?;
        }

        // cursor
        ctx.canvas.set_draw_color(ctx.theme.palette.cell_selected);
        for coords in ctx.game_state.selection.iter() {
            let coords = ctx.state.wrap_coords(coords);
            let rect = Rect::new(
                (coords.x * scale as i32) + 1,
                (coords.y * scale as i32) + 1,
                scale - 3,
                scale - 3,
            );
            ctx.canvas.draw_rect(rect)?;
        }

        Ok(())
    }
}
