pub struct Config {
    pub(crate) cols: u32,
    pub(crate) rows: u32,
    scale: u32,
}

impl Config {
    pub fn new(cols: u32, rows: u32, scale: u32) -> Self {
        Config { cols, rows, scale }
    }

    pub fn width(&self) -> u32 {
        self.cols * self.scale
    }

    pub fn height(&self) -> u32 {
        self.rows * self.scale
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }
}
