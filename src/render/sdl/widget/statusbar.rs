use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::core::State;
use crate::render::sdl::game_state::GameState;
use crate::render::sdl::renderer::RenderingContext;
use crate::render::sdl::widget::Widget;
use crate::render::sdl::widget::pane::Pane;
use crate::render::sdl::widget::text::Text;

const TEXT_PAUSED: &str = "<PAUSED>";
const TEXT_RUNNING: &str = "<RUNNING>";

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

    /// Display center of cursor selection.
    ///
    /// Center is computed taking `x_min, x_max, y_min, y_max` and then
    /// calculating the middle points, eg. `(x_max + x_min) / 2`.
    fn text_coords(_game_state: &GameState, _state: &State) -> Option<String> {
        // if game_state.cursor.is_empty() {
        None
        // } else {
        //     let (x_min, x_max, y_min, y_max) = game_state.cursor.iter().enumerate().fold(
        //         (0i32, 0i32, 0i32, 0i32),
        //         |mut bounds, (i, coords)| {
        //             if i == 0 {
        //                 bounds.0 = coords.x;
        //                 bounds.1 = coords.x;
        //                 bounds.2 = coords.y;
        //                 bounds.3 = coords.y;
        //             } else {
        //                 bounds.0 = coords.x.min(bounds.0);
        //                 bounds.1 = coords.x.max(bounds.1);
        //                 bounds.2 = coords.y.min(bounds.2);
        //                 bounds.3 = coords.y.max(bounds.3);
        //             }
        //             bounds
        //         },
        //     );
        //     let x_center = (f64::from(x_min) + f64::from(x_max)) / 2.0;
        //     let y_center = (f64::from(y_min) + f64::from(y_max)) / 2.0;
        //     Some(format!(
        //         " ({},{}) -- {} {} {} {}",
        //         x_center.round(),
        //         y_center.round(),
        //         x_min,
        //         x_max,
        //         y_min,
        //         y_max
        //     ))
        // }
    }

    fn text_generation(state: &State) -> String {
        state.generation.to_string()
    }

    fn text_period(game_state: &GameState) -> String {
        format!("{} ms", game_state.sim_period_ms)
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
            text.push_str(coords.as_str());
        }

        let child = Self::create_textbox(text.as_str(), ctx.theme.palette.status_text);
        let pane = Self::create_pane(ctx.layout.statusbar, ctx.theme.palette.status_bg, child);
        pane.render(ctx)
    }
}
