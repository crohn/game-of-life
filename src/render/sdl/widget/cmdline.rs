use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;
use crate::render::sdl::widget::pane::Pane;
use crate::render::sdl::widget::text::Text;

pub struct Cmdline {}

impl Cmdline {
    fn create_textbox<'a>(&self, text: &'a str, color: Color) -> Box<dyn Widget + 'a> {
        Box::new(Text {
            text,
            color,
            x: 4,
            y: 2,
        })
    }

    fn create_pane<'a>(
        &self,
        layout: Rect,
        color: Color,
        child: Option<Box<dyn Widget + 'a>>,
    ) -> Pane<'a> {
        Pane {
            rect: layout,
            color,
            border: None,
            child,
        }
    }
}

impl Widget for Cmdline {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let text: Option<&str> = ctx.game_state.command.as_deref();
        let child = text.map(|t| self.create_textbox(t, ctx.theme.palette.cmdline_text));
        let pane = self.create_pane(ctx.layout.cmdline, ctx.theme.palette.cmdline_bg, child);
        pane.render(ctx)
    }
}
