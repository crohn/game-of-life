// use sdl2::rect::Rect;

// use crate::{
//     core::{Cell, coords_from_index},
//     render::sdl::renderer::RenderingContext,
// };

// use super::Widget;

// pub struct Board {}
// impl Widget for Board {
//     fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
//         let scale = ctx.layout.scale;
//         let grid = if ctx.game_state.show_grid { 1 } else { 0 };

//         ctx.canvas.set_draw_color(ctx.theme.palette.cell_alive);
//         for (i, cell) in ctx.state.curr.iter().enumerate() {
//             let coords = coords_from_index(i, ctx.state.cols);
//             let rect = Rect::new(
//                 coords.x * scale as i32,
//                 coords.y * scale as i32,
//                 scale - grid,
//                 scale - grid,
//             );
//             if matches!(cell, Cell::Alive) {
//                 ctx.canvas.fill_rect(rect)?;
//             }
//         }

//         Ok(())
//     }
// }
