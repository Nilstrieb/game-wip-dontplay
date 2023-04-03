use fnv::FnvHashSet;
use sfml::window::{Event, Key};

#[derive(Default)]
pub struct KbInput {
    down: FnvHashSet<Key>,
    pressed: FnvHashSet<Key>,
}

impl KbInput {
    pub fn update_from_event(&mut self, ev: &Event) {
        match ev {
            Event::KeyPressed { code, .. } => {
                self.pressed.insert(*code);
                self.down.insert(*code);
            }
            Event::KeyReleased { code, .. } => {
                self.down.remove(code);
            }
            _ => {}
        }
    }
    /// Pressed event should be cleared every frame
    pub fn clear_pressed(&mut self) {
        self.pressed.clear();
    }
    pub fn down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }
    pub fn pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
}
