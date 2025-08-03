use sdl2::rect::Rect;

pub struct Layout {
    pub(crate) board: Rect,
    pub(crate) statusbar: Rect,
    pub(crate) cmdline: Rect,
}

impl Layout {
    #[rustfmt::skip]
    pub fn new(width: u32) -> Self {
        let board = Rect::new(0, 0, width, 250);
        let statusbar = Rect::new(0, 250, width, 20);
        let cmdline = Rect::new(0, 270, width, 20);

        Layout { board, statusbar, cmdline }
    }
}
