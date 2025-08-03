use sdl2::{EventPump, event::Event, keyboard::Keycode, mouse::MouseButton};

pub enum PollResult {
    Continue,
    Quit,
}

pub enum Action {
    Pause,
    PlayPause,
    ShowHelp,
    ToggleCell(i32, i32),
    ToggleGrid,
    Deselect,
    SelectUp,
    SelectRight,
    SelectDown,
    SelectLeft,
    Toggle,
}

pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler { event_pump }
    }

    // : enters cmd mode
    #[rustfmt::skip]
    pub fn poll(&mut self, actions: &mut Vec<Action>) -> PollResult {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return PollResult::Quit,

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return PollResult::Quit,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => actions.push(Action::PlayPause),
                Event::KeyDown { keycode: Some(Keycode::QUOTE), .. } => actions.push(Action::ToggleGrid),
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    actions.push(Action::Pause);
                    actions.push(Action::ShowHelp);
                }
                Event::KeyDown { keycode: Some(Keycode::X), ..} => actions.push(Action::Deselect),
                Event::KeyDown { keycode: Some(Keycode::S), ..} => actions.push(Action::Toggle),
                Event::KeyDown { keycode: Some(Keycode::UP), ..} => actions.push(Action::SelectUp),
                Event::KeyDown { keycode: Some(Keycode::RIGHT), ..} => actions.push(Action::SelectRight),
                Event::KeyDown { keycode: Some(Keycode::DOWN), ..} => actions.push(Action::SelectDown),
                Event::KeyDown { keycode: Some(Keycode::LEFT), ..} => actions.push(Action::SelectLeft),

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    actions.push(Action::ToggleCell(x, y))
                }

                _ => {}
            }
        }
        PollResult::Continue
    }
}
