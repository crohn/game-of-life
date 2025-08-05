use crate::{
    core::{Coords, State},
    render::sdl::{
        event_handler::{Mode, PollResult},
        game_state::GameState,
    },
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

            let PollResult::Continue = self.update() else {
                break 'running;
            };

            while self.timer_acc_ms >= self.game_state.sim_period_ms {
                if self.game_state.running {
                    self.state.next();
                }
                self.timer_acc_ms -= self.game_state.sim_period_ms;
            }

            self.renderer.draw(&self.state, &self.game_state)?;

            self.timer.delay_if_early();
        }

        Ok(())
    }

    fn update(&mut self) -> PollResult {
        for action in &self.actions {
            match action {
                Action::AppendCommandChar(c) => {
                    self.game_state.command.get_or_insert_default().push_str(c)
                }
                Action::CancelCommand => self.game_state.command = None,
                Action::CursorDown => {
                    self.game_state
                        .select_cell(0, 1, self.state.cols, self.state.rows)
                }
                Action::CursorLeft => {
                    self.game_state
                        .select_cell(-1, 0, self.state.cols, self.state.rows)
                }
                Action::CursorRight => {
                    self.game_state
                        .select_cell(1, 0, self.state.cols, self.state.rows)
                }
                Action::CursorUp => {
                    self.game_state
                        .select_cell(0, -1, self.state.cols, self.state.rows)
                }
                Action::DelCommandChar => {
                    if let Some(command) = &mut self.game_state.command {
                        if command.len() > 1 {
                            command.pop();
                        } else {
                            self.game_state.command = None;
                            self.event_handler.mode = Mode::Normal;
                        }
                    }
                }
                Action::ExecCommand => {
                    match self.game_state.command.as_deref() {
                        Some(":q") => return PollResult::Quit,
                        _ => {}
                    }
                    self.game_state.command = None;
                }
                Action::HideCursor => self.game_state.deselect_cell(),
                Action::PlayPause => self.game_state.toggle_running(),
                Action::SpeedDecrease => {
                    self.game_state.sim_period_ms = (self.game_state.sim_period_ms + 33).min(330)
                }
                Action::SpeedIncrease => {
                    self.game_state.sim_period_ms = (self.game_state.sim_period_ms - 33).max(33)
                }
                Action::SwitchMode(Mode::Normal) => self.event_handler.mode = Mode::Normal,
                Action::SwitchMode(Mode::Command) => self.event_handler.mode = Mode::Command,
                Action::ToggleClickedCell(x, y) => {
                    let scale = self.renderer.layout.scale;
                    let coords = Coords {
                        x: x / scale as i32,
                        y: y / scale as i32,
                    };
                    self.state.toggle_cell(&coords);
                }
                Action::ToggleCursorCell => {
                    if let Some(coords) = &self.game_state.selected_cell {
                        self.state.toggle_cell(coords);
                    }
                }
                Action::ToggleGrid => self.game_state.toggle_grid(),
            }
        }
        PollResult::Continue
    }
}
