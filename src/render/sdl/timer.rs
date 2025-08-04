use sdl2::TimerSubsystem;

const ONE_SECOND_MILLIS: u64 = 1000;

// Timer gets FPS, which is the number of frames per second we want to render.
// Each frame has a duration equal to 1s / FPS.
// Frame duration represents the available time for each game loop cycle and the
// purpose of loop cycles is to process events, update the state and then
// render. This means that we need to decouple the frequency of the loop cycles
// and the frequency of game-of-life updates, because they are effectively
// unrelated: we don't want to pause event processing because we want a slower
// simulation.
//
// In order to do so, Game needs to and keep track of the simulation period and
// a time accumulator. When the time accumulator goes beyond that duration, the
// simulation can advance.
pub struct Timer {
    start: Option<u64>,
    pub(crate) frame_duration: u64,
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
