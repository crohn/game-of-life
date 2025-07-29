use sdl2::{Sdl, TimerSubsystem, render::Canvas, video::Window};

pub struct SdlContext {
    pub(crate) sdl_context: Sdl,
    pub(crate) canvas: Canvas<Window>,
    pub(crate) timer: TimerSubsystem,

    pub(crate) scale: u32,
}

impl SdlContext {
    pub fn new(width: u32, height: u32, scale: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let timer = sdl_context.timer()?;
        let video = sdl_context.video()?;

        let window = video
            .window("game-of-sdl2", width * scale, height * scale)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(SdlContext {
            sdl_context,
            canvas,
            timer,
            scale,
        })
    }
}
