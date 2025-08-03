use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;
use crate::render::sdl::widget::pane::Pane;
use crate::render::sdl::widget::text::Text;

const TEXT_PAUSED: &str = "<PAUSED>";
const TEXT_RUNNING: &str = "<RUNNING>";

pub struct Statusbar {}

impl Statusbar {
    fn status_text(&self, running: bool) -> &'static str {
        if running { TEXT_RUNNING } else { TEXT_PAUSED }
    }

    fn create_textbox(&self, text: &'static str, color: Color) -> Option<Box<dyn Widget>> {
        Some(Box::new(Text {
            text,
            color,
            x: 4,
            y: 2,
        }))
    }

    fn create_pane(&self, layout: Rect, color: Color, child: Option<Box<dyn Widget>>) -> Pane {
        Pane {
            rect: layout,
            color,
            border: None,
            child,
        }
    }
}

impl Widget for Statusbar {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let text = self.status_text(ctx.game_state.running);
        let child = self.create_textbox(text, ctx.theme.palette.status_text);
        let pane = self.create_pane(ctx.layout.statusbar, ctx.theme.palette.status_bg, child);
        pane.render(ctx)
    }
}
