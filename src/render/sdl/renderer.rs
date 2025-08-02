use super::layout::Layout;
use crate::{
    core::State,
    render::sdl::{
        game_state::GameState,
        widget::{Widget, board::Board, helpwindow::HelpWindow, statusbar::StatusBar},
    },
};
use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);

pub struct RenderingContext<'a, 'b> {
    pub(crate) canvas: &'a mut Canvas<Window>,
    pub(crate) font: &'a Font<'a, 'a>,
    pub(crate) layout: &'a Layout,
    pub(crate) texture_creator: &'a TextureCreator<WindowContext>,
    pub(crate) state: &'b State,
    pub(crate) game_state: &'b GameState,
}

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
    layout: Layout,
    texture_creator: TextureCreator<WindowContext>,
    widgets: Vec<Box<dyn Widget>>,
}

impl<'a> Renderer<'a> {
    pub fn new(layout: Layout, canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let texture_creator = canvas.texture_creator();
        let widgets: Vec<Box<dyn Widget>> = vec![
            Box::new(Board {}),
            Box::new(StatusBar {}),
            Box::new(HelpWindow {}),
        ];

        Renderer {
            canvas,
            font,
            layout,
            texture_creator,
            widgets,
        }
    }

    pub fn draw(&mut self, state: &State, game_state: &GameState) -> Result<(), String> {
        let mut rendering_ctx = RenderingContext {
            canvas: &mut self.canvas,
            font: &self.font,
            layout: &self.layout,
            texture_creator: &self.texture_creator,
            state,
            game_state,
        };

        rendering_ctx.canvas.set_draw_color(COLOR_DEAD);
        rendering_ctx.canvas.clear();

        for widget in &self.widgets {
            widget.render(&mut rendering_ctx)?;
        }

        rendering_ctx.canvas.present();

        Ok(())
    }
}
