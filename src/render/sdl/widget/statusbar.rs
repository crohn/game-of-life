use sdl2::rect::Rect;

use crate::render::sdl::{renderer::RenderingContext, widget::Widget};

pub struct StatusBar {}
impl StatusBar {
    fn render_bar(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let rect = Rect::new(
            ctx.layout.statbar.x as i32,
            ctx.layout.statbar.y as i32,
            ctx.layout.statbar.w,
            ctx.layout.statbar.h,
        );

        ctx.canvas.set_draw_color(ctx.theme.palette.status_bg);
        ctx.canvas.fill_rect(rect)
    }

    fn render_text(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let running = ctx.game_state.running;

        let surface = ctx
            .font
            .render(if running { "<Running>" } else { "<Paused>" })
            .solid(ctx.theme.palette.status_text)
            .map_err(|e| e.to_string())?;

        let text_width = surface.width();

        let texture = ctx
            .texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let rect = Rect::new(
            ctx.layout.statbar.x as i32 + 4,
            ctx.layout.statbar.y as i32,
            text_width,
            ctx.layout.statbar.h,
        );

        ctx.canvas.copy(&texture, None, rect)
    }
}

impl Widget for StatusBar {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        self.render_bar(ctx)?;
        self.render_text(ctx)
    }
}
