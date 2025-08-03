use super::renderer::RenderingContext;

pub trait Widget {
    fn render(&self, ctx: &mut RenderingContext) -> Result<(), String>;
}

pub mod board;
pub mod cmdline;
pub mod pane;
pub mod statusbar;
pub mod text;
