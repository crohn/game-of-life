//!  +---------------
//!  |
//!  | board
//!  |
//!  +---------------
//!  | statusbar
//!  +---------------
//!  | cli
//!  +---------------

const STATUSBAR_HEIGHT: u32 = 20;
const CMDLINE_HEIGHT: u32 = 20;

use crate::core::Config;

pub struct Geometry {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Geometry {
    pub fn baseline(&self) -> u32 {
        self.y + self.h
    }
}

pub struct Layout {
    pub(crate) scale: u32,

    board: Geometry,
    cmdline: Geometry,
    pub(crate) statbar: Geometry,
}

impl Layout {
    pub fn new(config: &Config, scale: u32) -> Self {
        let width = config.cols * scale;

        let board = Geometry {
            x: 0,
            y: 0,
            w: width,
            h: config.rows * scale,
        };
        let statbar = Geometry {
            x: 0,
            y: board.baseline(),
            w: width,
            h: STATUSBAR_HEIGHT,
        };
        let cmdline = Geometry {
            x: 0,
            y: statbar.baseline(),
            w: width,
            h: CMDLINE_HEIGHT,
        };

        Layout {
            scale,
            board,
            statbar,
            cmdline,
        }
    }

    pub fn window_geometry(&self) -> Geometry {
        // NOTE x, y can be used to position the window
        Geometry {
            x: 0,
            y: 0,
            w: self.board.w,
            h: self.board.h + self.statbar.h + self.cmdline.h,
        }
    }
}
