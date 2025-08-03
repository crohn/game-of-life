use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;

pub struct Text<'a> {
    pub(crate) text: &'a str,
    pub(crate) color: Color,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl<'a> Widget for Text<'a> {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let surface = ctx
            .font
            .render(self.text)
            .blended(self.color)
            .map_err(|e| e.to_string())?;

        let texture = ctx
            .texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        let rect = Rect::new(self.x, self.y, width, height);
        ctx.canvas.copy(&texture, None, rect)
    }
}
