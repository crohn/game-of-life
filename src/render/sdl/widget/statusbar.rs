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
    fn status_text(&self, running: bool, generation: u32, selected: Option<String>) -> String {
        if let Some(selected) = selected {
            format!(
                "{} - {} - {}",
                if running { TEXT_RUNNING } else { TEXT_PAUSED },
                generation,
                selected
            )
        } else {
            format!(
                "{} - {}",
                if running { TEXT_RUNNING } else { TEXT_PAUSED },
                generation,
            )
        }
    }

    fn create_textbox<'a>(&self, text: &'a str, color: Color) -> Option<Box<dyn Widget + 'a>> {
        Some(Box::new(Text {
            text,
            color,
            x: 4,
            y: 2,
        }))
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

impl Widget for Statusbar {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let sel = &ctx.game_state.selected_cell;

        let selected = if let Some(s) = sel {
            Some(format!(
                "({},{}) -> {}",
                s.x,
                s.y,
                ctx.state.get_cell(s).as_value()
            ))
        } else {
            None
        };

        let text = self.status_text(ctx.game_state.running, ctx.state.generation, selected);
        let child = self.create_textbox(text.as_str(), ctx.theme.palette.status_text);
        let pane = self.create_pane(ctx.layout.statusbar, ctx.theme.palette.status_bg, child);
        pane.render(ctx)
    }
}
