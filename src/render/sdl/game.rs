use crate::{
    core::{Coords, State},
    render::sdl::{event_handler::PollResult, game_state::GameState},
};

use super::{
    event_handler::{Action, EventHandler},
    renderer::Renderer,
    timer::Timer,
};

pub struct Game<'a> {
    actions: Vec<Action>,
    event_handler: EventHandler,
    game_state: GameState,
    renderer: Renderer<'a>,
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
        let game_state = GameState::default();

        Game {
            actions: Vec::new(),
            event_handler,
            game_state,
            renderer,
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

            self.renderer.draw(&self.state, &self.game_state)?;

            self.timer.delay_if_early();
        }

        Ok(())
    }

    fn update(&mut self) {
        for action in &self.actions {
            match action {
                Action::Pause => self.game_state.pause(),
                Action::PlayPause => self.game_state.toggle(),
                Action::ShowHelp => self.game_state.help(),
                Action::ToggleCell(x, y) => {
                    let scale = self.renderer.layout.scale;
                    let coords = Coords {
                        x: x / scale as i32,
                        y: y / scale as i32,
                    };
                    self.state.toggle_cell(coords);
                }
                Action::ToggleGrid => self.game_state.toggle_grid(),
            }
        }

        if self.game_state.running {
            self.state.next();
        }
    }
}
