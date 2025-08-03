use sdl2::{pixels::Color, rect::Rect};

use crate::render::sdl::{renderer::RenderingContext, widget::Widget};

pub struct Border {
    pub(crate) color: Color,
    pub(crate) thickness: u32,
}

pub struct Pane {
    pub(crate) rect: Rect,
    pub(crate) color: Color,
    pub(crate) border: Option<Border>,
    pub(crate) child: Option<Box<dyn Widget>>,
}

impl Pane {
    // Renders the border as an outer Rect, returning a new one to be used as
    // actual content.
    //
    // The SDL2 API does not provide a way to render a bordered Rect. The
    // simplest strategy is to draw two filled rectangles, where the larger one
    // acts as border and the smaller one is the actual expected content.
    fn render_border(&self, ctx: &mut RenderingContext) -> Result<Rect, String> {
        if let Some(border) = &self.border {
            ctx.canvas.set_draw_color(border.color);
            ctx.canvas.fill_rect(self.rect)?;
            Ok(Rect::new(
                self.rect.x + border.thickness as i32,
                self.rect.y + border.thickness as i32,
                self.rect.width().saturating_sub(2 * border.thickness),
                self.rect.height().saturating_sub(2 * border.thickness),
            ))
        } else {
            Ok(self.rect)
        }
    }
}

impl Widget for Pane {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let inner_rect = self.render_border(ctx)?;

        ctx.canvas.set_draw_color(self.color);
        ctx.canvas.fill_rect(inner_rect)?;

        if let Some(child) = &self.child {
            let viewport = ctx.canvas.viewport();
            ctx.canvas.set_viewport(inner_rect);
            child.render(ctx)?;
            ctx.canvas.set_viewport(viewport);
        }

        Ok(())
    }
}
