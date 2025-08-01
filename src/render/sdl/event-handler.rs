use sdl2::{EventPump, event::Event, keyboard::Keycode};

pub enum PollResult {
    Continue,
    Quit,
}

pub enum Action {
    Pause,
    PlayPause,
    ShowHelp,
}

pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler { event_pump }
    }

    pub fn poll(&mut self, actions: &mut Vec<Action>) -> PollResult {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return PollResult::Quit,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return PollResult::Quit,
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    actions.push(Action::Pause);
                    actions.push(Action::ShowHelp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => actions.push(Action::PlayPause),
                _ => {}
            }
        }
        PollResult::Continue
    }
}
