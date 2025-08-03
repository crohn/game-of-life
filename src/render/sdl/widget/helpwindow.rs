// use sdl2::rect::Rect;

// use crate::render::sdl::{renderer::RenderingContext, widget::Widget};

// pub struct HelpWindow {}
// impl Widget for HelpWindow {
//     fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
//         if !ctx.game_state.show_help {
//             return Ok(());
//         }

//         let window_geom = ctx.layout.window_geometry();

//         let rect = Rect::new(
//             8 + window_geom.x as i32,
//             8 + window_geom.y as i32,
//             window_geom.w - 16,
//             window_geom.h - 16,
//         );

//         let blend_mode = ctx.canvas.blend_mode();
//         ctx.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

//         ctx.canvas.set_draw_color(ctx.theme.palette.help_bg);
//         ctx.canvas.fill_rect(rect)?;

//         ctx.canvas.set_blend_mode(blend_mode);

//         Ok(())
//     }
// }
