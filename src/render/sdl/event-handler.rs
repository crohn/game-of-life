use sdl2::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};

pub enum PollResult {
    Continue,
    Quit,
}

pub enum Mode {
    Command,
    Normal,
}

pub enum Action {
    SwitchMode(Mode),
    // Command
    AppendCommandChar(String),
    CancelCommand,
    DelCommandChar,
    ExecCommand,
    // Normal
    CursorDown,
    CursorLeft,
    CursorRight,
    CursorUp,
    HideCursor,
    PlayPause,
    SpeedDecrease,
    SpeedIncrease,
    ToggleClickedCell(i32, i32),
    ToggleCursorCell,
    ToggleGrid,
}

pub struct EventHandler {
    pub mode: Mode,
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler {
            event_pump,
            mode: Mode::Normal,
        }
    }

    pub fn poll(&mut self, actions: &mut Vec<Action>) -> PollResult {
        for event in self.event_pump.poll_iter() {
            if let PollResult::Quit = match self.mode {
                Mode::Command => Self::handle_event_command(event, actions),
                Mode::Normal => Self::handle_event_normal(event, actions),
            } {
                return PollResult::Quit;
            }
        }
        PollResult::Continue
    }

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

    // it would be probably more appropriate to switch controls to scancode,
    // because for example keycode PLUS is not caught, while the combination of
    // LShift+EQUALS is.
    fn handle_event_normal(event: Event, actions: &mut Vec<Action>) -> PollResult {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                keymod,
                ..
            } => match (keycode, keymod) {
                // SHIFT+Equals -> Plus
                (Keycode::Equals, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    actions.push(Action::SpeedIncrease)
                }
                (Keycode::Escape, _) => return PollResult::Quit,
                (Keycode::Minus, _) => actions.push(Action::SpeedDecrease),
                (Keycode::Quote, _) => actions.push(Action::ToggleGrid),
                (Keycode::Return, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    actions.push(Action::PlayPause)
                }
                // SHIFT+Semicolon -> Colon
                (Keycode::Semicolon, Mod::LSHIFTMOD | Mod::RSHIFTMOD) => {
                    actions.push(Action::AppendCommandChar(":".to_string()));
                    actions.push(Action::SwitchMode(Mode::Command));
                }
                (Keycode::Space, _) => actions.push(Action::ToggleCursorCell),
                (Keycode::H | Keycode::Left, _) => actions.push(Action::CursorLeft),
                (Keycode::J | Keycode::Down, _) => actions.push(Action::CursorDown),
                (Keycode::K | Keycode::Up, _) => actions.push(Action::CursorUp),
                (Keycode::L | Keycode::Right, _) => actions.push(Action::CursorRight),
                (Keycode::X, _) => actions.push(Action::HideCursor),
                _ => {}
            },

            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if matches!(mouse_btn, MouseButton::Left) {
                    actions.push(Action::ToggleClickedCell(x, y));
                }
            }

            Event::Quit { .. } => return PollResult::Quit,

            _ => {}
        }
        PollResult::Continue
    }
}
