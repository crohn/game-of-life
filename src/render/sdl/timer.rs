use sdl2::TimerSubsystem;

const ONE_SECOND_MILLIS: u64 = 1000;

pub struct Timer {
    start: Option<u64>,
    frame_duration: u64,
    timer: TimerSubsystem,
}

impl Timer {
    pub fn new(timer: TimerSubsystem, target_fps: u64) -> Self {
        Timer {
            start: None,
            frame_duration: ONE_SECOND_MILLIS / target_fps,
            timer,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(self.timer.ticks64());
    }

    pub fn delay_if_early(&mut self) {
        if let Some(start) = self.start {
            let frame_time = self.timer.ticks64() - start;
            if frame_time < self.frame_duration {
                self.timer.delay((self.frame_duration - frame_time) as u32);
            }
        }
    }
}
