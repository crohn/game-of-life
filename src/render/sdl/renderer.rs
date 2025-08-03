use super::layout::Layout;
use crate::{
    core::State,
    render::sdl::{
        game_state::GameState,
        theme::Theme,
        widget::{
            Widget,
            cmdline::Cmdline,
            pane::{Border, Pane},
            statusbar::Statusbar,
            text::Text,
        },
    },
};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

pub struct RenderingContext<'a, 'b> {
    pub(crate) canvas: &'a mut Canvas<Window>,
    pub(crate) font: &'a Font<'a, 'a>,
    pub(crate) layout: &'a Layout,
    pub(crate) texture_creator: &'a TextureCreator<WindowContext>,
    pub(crate) theme: &'a Theme,
    pub(crate) state: &'b State,
    pub(crate) game_state: &'b GameState,
}

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
    pub(crate) layout: Layout,
    texture_creator: TextureCreator<WindowContext>,
    theme: Theme,
    widgets: Vec<Box<dyn Widget>>,
}

impl<'a> Renderer<'a> {
    pub fn new(layout: Layout, canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let texture_creator = canvas.texture_creator();
        let theme = Theme::default();
        let widgets: Vec<Box<dyn Widget>> = vec![
            Box::new(Pane {
                color: Color::RGB(0xff, 0x00, 0x00),
                rect: layout.board,
                border: Some(Border {
                    color: Color::RGB(0x00, 0x00, 0x00),
                    thickness: 10,
                }),
                child: Some(Box::new(Text {
                    color: Color::RGB(0xff, 0xff, 0xff),
                    text: "Hello world",
                    x: 4,
                    y: 4,
                })),
            }),
            Box::new(Statusbar {}),
            Box::new(Cmdline {}),
        ];

        Renderer {
            canvas,
            font,
            layout,
            texture_creator,
            theme,
            widgets,
        }
    }

    pub fn draw(&mut self, state: &State, game_state: &GameState) -> Result<(), String> {
        let mut rendering_ctx = RenderingContext {
            canvas: &mut self.canvas,
            font: &self.font,
            layout: &self.layout,
            texture_creator: &self.texture_creator,
            theme: &self.theme,
            state,
            game_state,
        };

        rendering_ctx.canvas.set_draw_color(self.theme.palette.bg);
        rendering_ctx.canvas.clear();

        for widget in &self.widgets {
            widget.render(&mut rendering_ctx)?;
        }

        rendering_ctx.canvas.present();

        Ok(())
    }
}
