use crate::{core::State, render::sdl::event_handler::PollResult};

use super::{
    event_handler::{Action, EventHandler},
    renderer::Renderer,
    timer::Timer,
};

pub struct Game<'a> {
    actions: Vec<Action>,
    event_handler: EventHandler,
    renderer: Renderer<'a>,
    running: bool,
    state: State,
    timer: Timer,
}

impl<'a> Game<'a> {
    pub fn new(
        event_handler: EventHandler,
        renderer: Renderer<'a>,
        timer: Timer,
        state: State,
    ) -> Self {
        Game {
            actions: Vec::new(),
            event_handler,
            renderer,
            running: false,
            state,
            timer,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            self.timer.start();

            self.actions.clear();
            let PollResult::Continue = self.event_handler.poll(&mut self.actions) else {
                break 'running;
            };

            self.update();

            self.renderer.draw(&self.state)?;

            self.timer.delay_if_early();
        }

        Ok(())
    }

    fn update(&mut self) {
        for action in &self.actions {
            match action {
                Action::PlayPause => self.running = !self.running,
            }
        }

        if self.running {
            self.state.next();
        }
    }
}
