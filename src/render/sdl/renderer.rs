use super::layout::Layout;
use crate::{
    core::{Cell, State, coords_from_index},
    render::sdl::game_state::GameState,
};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

const COLOR_ALIVE: Color = Color::RGB(0xff, 0xff, 0xff);
const COLOR_DEAD: Color = Color::RGB(0x00, 0x00, 0x00);
const COLOR_STATUSBAR_BG: Color = Color::RGB(0x00, 0x00, 0xff);
const COLOR_STATUSBAR_FG: Color = Color::RGB(0x00, 0x00, 0x00);

pub struct Renderer<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
    layout: Layout,
    texture_creator: TextureCreator<WindowContext>,
}

impl<'a> Renderer<'a> {
    pub fn new(layout: Layout, canvas: Canvas<Window>, font: Font<'a, 'a>) -> Self {
        let texture_creator = canvas.texture_creator();

        Renderer {
            canvas,
            font,
            layout,
            texture_creator,
        }
    }

    pub fn draw(&mut self, state: &State, game_state: &GameState) -> Result<(), String> {
        self.render_board(state)?;
        self.render_statusbar()?;
        self.render_statusbar_text(game_state.running)?;
        self.render_help(game_state.show_help)?;
        self.canvas.present();
        Ok(())
    }

    fn render_board(&mut self, state: &State) -> Result<(), String> {
        let scale = self.layout.scale;

        self.canvas.set_draw_color(COLOR_DEAD);
        self.canvas.clear();

        self.canvas.set_draw_color(COLOR_ALIVE);
        for (i, cell) in state.curr.iter().enumerate() {
            let coords = coords_from_index(i, state.cols);
            let rect = Rect::new(
                coords.x * scale as i32,
                coords.y * scale as i32,
                scale,
                scale,
            );
            if matches!(cell, Cell::Alive) {
                self.canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }

    fn render_statusbar(&mut self) -> Result<(), String> {
        let rect = Rect::new(
            self.layout.statbar.x as i32,
            self.layout.statbar.y as i32,
            self.layout.statbar.w,
            self.layout.statbar.h,
        );

        self.canvas.set_draw_color(COLOR_STATUSBAR_BG);
        self.canvas.fill_rect(rect)
    }

    fn render_statusbar_text(&mut self, running: bool) -> Result<(), String> {
        let surface = self
            .font
            .render(if running { "<Running>" } else { "<Paused>" })
            .solid(COLOR_STATUSBAR_FG)
            .map_err(|e| e.to_string())?;

        let text_width = surface.width();

        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;

        let rect = Rect::new(
            self.layout.statbar.x as i32 + 4,
            self.layout.statbar.y as i32,
            text_width,
            self.layout.statbar.h,
        );

        self.canvas.copy(&texture, None, rect)
    }

    fn render_help(&mut self, show_help: bool) -> Result<(), String> {
        if !show_help {
            return Ok(());
        }

        let window_geom = self.layout.window_geometry();

        let rect = Rect::new(
            8 + window_geom.x as i32,
            8 + window_geom.y as i32,
            window_geom.w - 16,
            window_geom.h - 16,
        );

        let blend_mode = self.canvas.blend_mode();
        self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        self.canvas
            .set_draw_color(Color::RGBA(0xff, 0xff, 0xff, 0x7f));
        self.canvas.fill_rect(rect)?;

        self.canvas.set_blend_mode(blend_mode);

        Ok(())
    }
}
