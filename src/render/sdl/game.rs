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
    //
    sim_perio_ms: u64,
    timer_acc_ms: u64,
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
            sim_perio_ms: 33,
            timer_acc_ms: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            self.timer.start();

            self.timer_acc_ms += self.timer.frame_duration;

            self.actions.clear();
            let PollResult::Continue = self.event_handler.poll(&mut self.actions) else {
                break 'running;
            };

            self.update();

            while self.timer_acc_ms >= self.sim_perio_ms {
                if self.game_state.running {
                    self.state.next();
                }
                self.timer_acc_ms -= self.sim_perio_ms;
            }

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
                    self.state.toggle_cell(&coords);
                }
                Action::ToggleGrid => self.game_state.toggle_grid(),
                Action::Deselect => self.game_state.deselect_cell(),
                Action::SelectUp => {
                    self.game_state
                        .select_cell(0, -1, self.state.cols, self.state.rows)
                }
                Action::SelectRight => {
                    self.game_state
                        .select_cell(1, 0, self.state.cols, self.state.rows)
                }
                Action::SelectDown => {
                    self.game_state
                        .select_cell(0, 1, self.state.cols, self.state.rows)
                }
                Action::SelectLeft => {
                    self.game_state
                        .select_cell(-1, 0, self.state.cols, self.state.rows)
                }
                Action::Toggle => {
                    if let Some(coords) = &self.game_state.selected_cell {
                        self.state.toggle_cell(coords);
                    }
                }
            }
        }
    }
}
