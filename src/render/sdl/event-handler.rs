use sdl2::{EventPump, event::Event, keyboard::Keycode};

pub enum Action {
    PlayPause,
}

pub struct EventHandler {
    event_pump: EventPump,
}

impl EventHandler {
    pub fn new(event_pump: EventPump) -> Self {
        EventHandler { event_pump }
    }

    pub fn poll(&mut self, actions: &mut Vec<Action>) -> Option<()> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return None,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return None,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => actions.push(Action::PlayPause),
                _ => {}
            }
        }
        Some(())
    }
}
