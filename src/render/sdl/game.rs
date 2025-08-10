use crate::{
    core::State,
    render::sdl::{
        command::{Command, parse},
        event_handler::Mode,
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
            if let Action::Quit = self.event_handler.poll(&mut self.actions) {
                break 'running;
            };

            if let Action::Quit = self.update() {
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

    fn update(&mut self) -> Action {
        for action in &self.actions {
            match action {
                Action::Quit => unreachable!("Action quit should be handled on poll."),
                Action::Continue => {}
                // Normal mode actions
                Action::SelClear => self.game_state.clear_sel(),
                Action::SelLRot => self.game_state.rot_sel_counter(),
                Action::SelMoveDown => self.game_state.mv_sel_down(1),
                Action::SelMoveLeft => self.game_state.mv_sel_left(1),
                Action::SelMoveRight => self.game_state.mv_sel_right(1),
                Action::SelMoveUp => self.game_state.mv_sel_up(1),
                Action::SelRRot => self.game_state.rot_sel_clockwise(),
                Action::SelReCenter(coords) => self
                    .game_state
                    .recenter_sel(coords.rescale(self.renderer.layout.scale)),
                Action::SelToggle(coords) => self
                    .game_state
                    .add_to_sel(coords.rescale(self.renderer.layout.scale)),
                Action::SelToggleCell => {
                    for coords in self.game_state.iter_sel() {
                        self.state.toggle_cell(coords);
                    }
                }
                Action::SimGridToggle => self.game_state.toggle_grid(),
                Action::SimSpeedDecr => self.game_state.sim_speed_decr(),
                Action::SimSpeedIncr => self.game_state.sim_speed_incr(),
                Action::SimStartStop => self.game_state.toggle_running(),
                Action::SimToggleCell(coords) => self
                    .state
                    .toggle_cell(&coords.rescale(self.renderer.layout.scale)),
                // == TODO ==
                Action::AppendCommandChar(c) => {
                    self.game_state.command.get_or_insert_default().push_str(c)
                }
                Action::CancelCommand => self.game_state.command = None,
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
                            Err(_) => Action::Continue,
                        };
                    }
                }
                Action::SwitchMode(Mode::Normal) => self.event_handler.mode = Mode::Normal,
                Action::SwitchMode(Mode::Command) => self.event_handler.mode = Mode::Command,
            }
        }

        self.event_handler.game_has_sel = self.game_state.has_sel();

        Action::Continue
    }

    fn execute_command(&mut self, command: Command) -> Action {
        match command {
            Command::BoardClear => self.state.clear(),
            Command::Cursor(x, y) => self.game_state.add_to_sel((x, y)),
            Command::Quit => return Action::Quit,
        }
        Action::Continue
    }
}
