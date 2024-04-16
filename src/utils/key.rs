use sdl2::{keyboard::Scancode, EventPump};

pub struct KeyHandle {
    event: EventPump
}

impl KeyHandle {
    pub fn new(event: EventPump) ->  Self {
        Self { event }
    }
}

impl std::ops::Index<Scancode> for KeyHandle {
    type Output = bool;
    fn index(&self, key: Scancode) -> &Self::Output {
        match self.event.keyboard_state().is_scancode_pressed(key) {
            true => &true,
            _ => &false
        }
    }
}
