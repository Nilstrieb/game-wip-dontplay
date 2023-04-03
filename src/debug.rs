use sfml::window::Key;

use crate::input::KbInput;

#[derive(Default)]
pub struct DebugState {
    pub panel: bool,
    pub freecam: bool,
}

impl DebugState {
    pub fn update(&mut self, input: &KbInput) {
        if input.pressed(Key::F12) {
            self.panel ^= true;
        }
        if input.pressed(Key::F10) {
            self.freecam ^= true;
        }
    }
}
