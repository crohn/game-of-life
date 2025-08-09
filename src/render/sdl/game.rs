use crate::{
    core::State,
    render::sdl::{
        command::{Command, parse},
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
                Action::CursorDown => self.game_state.move_cursor(0, 1, &self.state),
                Action::CursorLeft => self.game_state.move_cursor(-1, 0, &self.state),
                Action::CursorRight => self.game_state.move_cursor(1, 0, &self.state),
                Action::CursorUp => self.game_state.move_cursor(0, -1, &self.state),
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
                    self.event_handler.mode = Mode::Normal;
                    if let Some(input) = self.game_state.command.take() {
                        return match parse(&input) {
                            Ok(command) => self.execute_command(command),
                            // TODO print error on statusbar
                            Err(_) => PollResult::Continue,
                        };
                    }
                }
                Action::HideCursor => self.game_state.hide_cursor(),
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
                    self.game_state
                        .add_cursor(x / scale as i32, y / scale as i32, &self.state);
                }
                Action::ToggleCursorCell => {
                    for coords in &self.game_state.cursor {
                        self.state.toggle_cell(coords.x, coords.y);
                    }
                }
                Action::ToggleGrid => self.game_state.toggle_grid(),
            }
        }
        PollResult::Continue
    }

    fn execute_command(&mut self, command: Command) -> PollResult {
        match command {
            Command::BoardClear => self.state.clear(),
            Command::Cursor(x, y) => self.game_state.add_cursor(x, y, &self.state),
            Command::Quit => return PollResult::Quit,
        }
        PollResult::Continue
    }
}
