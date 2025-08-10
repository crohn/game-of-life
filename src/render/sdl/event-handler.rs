//! `EventHandler` is a wrapper around SDL2 `EventPump` that implements the
//! event polling and translates events into game's `Actions`.
//!
//! `EventHandler` works in two modes: `Normal` and `Command`.
//! In `Normal` mode, the player can interact directly with the game (mainly the
//! board) using keys and mouse buttons that are bound to some events.
//! In `Command` mode, the player can write commands to interact with the game.
//! Like in Vi program, Players can enter `Command` mode when pressing `:`
//! character.
//!
//!
//! ## `Normal` mode key bindings
//!
//! -         `Esc` - quit, exit game
//! -       `Space` - start/stop simulation
//! -           `+` - increase simulation speed
//! -           `-` - decrease simulation speed
//! -           `'` - toggle grid
//! -           `:` - enter Command mode
//!
//! When selection is active:
//! -  `left` | `h` - move selection left
//! -  `down` | `j` - move selection down
//! -    `up` | `k` - move selection up
//! - `right` | `l` - move selection right
//! -           `r` - rotate selection clockwise
//! -           `R` - rotate selection counter-clockwise
//! -           `T` - toggle cells (dead->alive/alive->dead)
//! -           `x` - clear selection
//!
//! ## `Normal` mode mouse bindings
//!
//! -     LMB Click - toggle cell (dead->alive/alive->dead)
//! - S + LMB Click - toggle select cell, effectively creating selection
//!
//! When selection is active:
//!
//! -     LMB Click - re-center selection
//! - S + LMB Click - toggle select cell, effectively adding to existing/clearing selection
//!
//!
//! ## `Command` mode
//! -- NEXT --

use sdl2::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Mod, Scancode},
    mouse::MouseButton,
};

use crate::core::Coords;

enum PollResult {
    Continue,
    Quit,
}

pub enum Mode {
    Command,
    Normal,
}

pub enum Action {
    Quit,
    Continue,

    //
    SwitchMode(Mode),
    // Command
    AppendCommandChar(String),
    CancelCommand,
    DelCommandChar,
    ExecCommand,

    // Normal
    SelClear,
    SelLRot,
    SelMoveDown,
    SelMoveLeft,
    SelMoveRight,
    SelMoveUp,
    SelRRot,
    SelReCenter(Coords),
    SelToggle(Coords), // toggle selection
    SelToggleCell,     // toggle selected cell value
    SimGridToggle,
    SimSpeedDecr,
    SimSpeedIncr,
    SimStartStop,
    SimToggleCell(Coords), // toggle clicked cell value
}

pub struct EventHandler {
    pub mode: Mode,
    event_pump: EventPump,
    pub(crate) game_has_sel: bool,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler {
            event_pump,
            mode: Mode::Normal,
            game_has_sel: false,
        }
    }

    /// Poll the event pump for events and translate them into game Actions.
    ///
    /// The actions are pushed onto `actions`, ready to be processed by who has
    /// that responsibility.
    pub fn poll(&mut self, actions: &mut Vec<Action>) -> Action {
        // NOTE ~ this was previously implementing using
        // EventPumpu::poll_iter(), but that locks `&mut ref` for the whole
        // loop, making impossible to further borrow `self.event_pump`, eg. to
        // check keyboard state.
        while let Some(event) = self.event_pump.poll_event() {
            if let PollResult::Quit = match self.mode {
                Mode::Command => Self::handle_event_command(event, actions),
                Mode::Normal => self.handle_event_normal(event, actions),
            } {
                return Action::Quit;
            }
        }
        Action::Continue
    }

    /// Command mode event handler
    fn handle_event_command(event: Event, actions: &mut Vec<Action>) -> PollResult {
        match event {
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::Backspace) => actions.push(Action::DelCommandChar),
                Some(Keycode::Return) => actions.push(Action::ExecCommand),
                Some(Keycode::Escape) => {
                    actions.push(Action::CancelCommand);
                    actions.push(Action::SwitchMode(Mode::Normal));
                }
                _ => {}
            },
            Event::TextInput { text, .. } => actions.push(Action::AppendCommandChar(text)),
            _ => {}
        }
        PollResult::Continue
    }

    /// Normal mode event handler
    ///
    /// You can find a list of key and mouse bindings in module's description.
    ///
    /// NOTE ~ I found a bit weird that I was no table to detect `+` keypress
    /// using Keycode alone and I had to use the associated plain key, eg. `=`,
    /// detecting `Shift` keypress. I'll investigate this further, checking
    /// whether using scancodes works better.
    fn handle_event_normal(&self, event: Event, actions: &mut Vec<Action>) -> PollResult {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                keymod,
                ..
            } => match (keycode, keymod) {
                (Keycode::Escape, _) => return PollResult::Quit,
                (Keycode::Space, _) => actions.push(Action::SimStartStop),
                (Keycode::Equals, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    // (Shift=) -> +
                    actions.push(Action::SimSpeedIncr)
                }
                (Keycode::Minus, _) => actions.push(Action::SimSpeedDecr),
                (Keycode::Quote, _) => actions.push(Action::SimGridToggle),
                (Keycode::Semicolon, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    // (Shift;) -> :
                    actions.push(Action::AppendCommandChar(":".to_string()));
                    actions.push(Action::SwitchMode(Mode::Command));
                }

                //
                // Active Selection
                //
                (Keycode::H | Keycode::Left, _) if self.game_has_sel => {
                    actions.push(Action::SelMoveLeft)
                }
                (Keycode::J | Keycode::Down, _) if self.game_has_sel => {
                    actions.push(Action::SelMoveDown)
                }
                (Keycode::K | Keycode::Up, _) if self.game_has_sel => {
                    actions.push(Action::SelMoveUp)
                }
                (Keycode::L | Keycode::Right, _) if self.game_has_sel => {
                    actions.push(Action::SelMoveRight)
                }
                (Keycode::R, Mod::LSHIFTMOD | Mod::RSHIFTMOD) if self.game_has_sel => {
                    actions.push(Action::SelLRot)
                }
                (Keycode::R, _) if self.game_has_sel => actions.push(Action::SelRRot),
                (Keycode::T, Mod::LSHIFTMOD | Mod::RSHIFTMOD) if self.game_has_sel => {
                    actions.push(Action::SelToggleCell)
                }
                (Keycode::X, _) if self.game_has_sel => actions.push(Action::SelClear),

                _ => {}
            },

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => match mouse_btn {
                MouseButton::Left if self.is_shift_pressed() => {
                    actions.push(Action::SelToggle(Coords { x, y }));
                }
                MouseButton::Left if self.game_has_sel => {
                    actions.push(Action::SelReCenter(Coords { x, y }));
                }
                MouseButton::Left => {
                    actions.push(Action::SimToggleCell(Coords { x, y }));
                }

                _ => {}
            },

            Event::Quit { .. } => return PollResult::Quit,

            _ => {}
        }
        PollResult::Continue
    }

    /// Returns true if either left or right shift keys are pressed.
    ///
    /// This method checks event pump's keyboard state, and it is necessary when
    /// the event does not provide they modifiers, eg. mouse events.
    fn is_shift_pressed(&self) -> bool {
        let kbd_state = self.event_pump.keyboard_state();
        kbd_state.is_scancode_pressed(Scancode::LShift)
            || kbd_state.is_scancode_pressed(Scancode::RShift)
    }
}
