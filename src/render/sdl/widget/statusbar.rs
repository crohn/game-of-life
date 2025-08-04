use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::core::{Cell, State};
use crate::render::sdl::game_state::GameState;
use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;
use crate::render::sdl::widget::pane::Pane;
use crate::render::sdl::widget::text::Text;

const TEXT_PAUSED: &str = "<PAUSED>";
const TEXT_RUNNING: &str = "<RUNNING>";

const CELL_ALIVE: &str = "ALIVE";
const CELL_DEAD: &str = "DEAD";

pub struct Statusbar;

impl Statusbar {
    fn create_textbox<'a>(text: &'a str, color: Color) -> Option<Box<dyn Widget + 'a>> {
        Some(Box::new(Text {
            text,
            color,
            x: 4,
            y: 2,
        }))
    }

    fn create_pane<'a>(
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

    fn text_coords(game_state: &GameState, state: &State) -> Option<String> {
        if let Some(selected) = &game_state.selected_cell {
            let mut str = format!("({},{}) -> ", selected.x, selected.y);
            let cell_state = match state.get_cell(&selected) {
                Cell::Alive => CELL_ALIVE,
                Cell::Dead => CELL_DEAD,
            };
            str.push_str(cell_state);
            Some(str)
        } else {
            None
        }
    }

    fn text_generation(state: &State) -> String {
        state.generation.to_string()
    }

    fn text_period(game_state: &GameState) -> String {
        let mut str = game_state.sim_period_ms.to_string();
        str.push_str(" ms");
        str
    }

    fn text_running(game_state: &GameState) -> &str {
        if game_state.running {
            TEXT_RUNNING
        } else {
            TEXT_PAUSED
        }
    }
}

impl Widget for Statusbar {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String> {
        let text_running = Self::text_running(ctx.game_state);
        let text_generation = Self::text_generation(ctx.state);
        let text_period = Self::text_period(ctx.game_state);
        let text_coords = Self::text_coords(ctx.game_state, ctx.state);

        let mut text = format!("{} {} {}", text_running, text_generation, text_period);
        if let Some(coords) = text_coords {
            text.push(' ');
            text.push_str(coords.as_str());
        }

        let child = Self::create_textbox(text.as_str(), ctx.theme.palette.status_text);
        let pane = Self::create_pane(ctx.layout.statusbar, ctx.theme.palette.status_bg, child);
        pane.render(ctx)
    }
}
