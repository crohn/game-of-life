use sdl2::rect::Rect;

use crate::core::Config;

pub struct Layout {
    pub(crate) statusbar: Rect,
    pub(crate) cmdline: Rect,
    board: Rect,

    window_width: u32,

    pub(crate) scale: u32,
}

impl Layout {
    #[rustfmt::skip]
    pub fn new(config: &Config, scale: u32) -> Self {
        let window_width = config.cols * scale;
        let board_height = config.rows * scale;
        let bar_height = 20;

        let board = Rect::new(0, 0, window_width, board_height);
        let statusbar = Rect::new(0, board.bottom(), window_width, bar_height);
        let cmdline = Rect::new(0, statusbar.bottom(), window_width, bar_height);

        Layout { board, statusbar, cmdline, window_width, scale }
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }

    pub fn window_height(&self) -> u32 {
        self.board.height() + self.statusbar.height() + self.cmdline.height()
    }
}
