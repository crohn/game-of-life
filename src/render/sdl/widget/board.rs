use sdl2::rect::Rect;

use crate::core::{Cell, coords_from_index};
use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;

pub struct Board;

impl Widget for Board {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let scale = 10;
        let grid = if ctx.game_state.show_grid { 1 } else { 0 };

        for (i, cell) in ctx.state.curr.iter().enumerate() {
            let coords = coords_from_index(i, ctx.state.cols);
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

        if let Some(coords) = &ctx.game_state.selected_cell {
            ctx.canvas.set_draw_color(ctx.theme.palette.cell_selected);
            let rect = Rect::new(
                coords.x * scale as i32,
                coords.y * scale as i32,
                scale,
                scale,
            );
            ctx.canvas.draw_rect(rect)?;
        }

        Ok(())
    }
}
