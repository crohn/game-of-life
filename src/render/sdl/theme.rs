use sdl2::pixels::Color;

pub struct Theme {
    pub(crate) palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            palette: Palette::default(),
        }
    }
}

pub struct Palette {
    pub(crate) bg: Color,
    // text: Color,
    pub(crate) cell_alive: Color,
    // pub(crate) cell_dead: Color,
    pub(crate) help_bg: Color,
    pub(crate) cmdline_bg: Color,
    pub(crate) cmdline_text: Color,
    pub(crate) status_bg: Color,
    pub(crate) status_text: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            bg: Color::RGBA(0x00, 0x00, 0x00, 0xff),
            // text: todo!(),
            cell_alive: Color::RGBA(0xff, 0xff, 0xff, 0xff),
            // cell_dead: Color::RGBA(0x88, 0x88, 0x88, 0xff),
            help_bg: Color::RGBA(0x00, 0x00, 0x00, 0x9a),
            cmdline_bg: Color::RGBA(0x00, 0x00, 0x00, 0xff),
            cmdline_text: Color::RGBA(0xff, 0xff, 0xff, 0xff),
            status_bg: Color::RGBA(0x78, 0xb9, 0xbf, 0xff),
            status_text: Color::RGBA(0x00, 0x00, 0x00, 0xff),
        }
    }
}
