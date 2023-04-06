use sfml::window::Key;

use crate::input::Input;

#[derive(Default, Debug)]
pub struct DebugState {
    pub panel: bool,
    pub freecam: bool,
}

impl DebugState {
    pub fn update(&mut self, input: &Input) {
        if input.pressed(Key::F12) {
            self.panel ^= true;
        }
        if input.pressed(Key::F10) {
            self.freecam ^= true;
        }
    }
}
