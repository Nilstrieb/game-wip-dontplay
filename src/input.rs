use fnv::FnvHashSet;
use sfml::window::{mouse, Event, Key};
use crate::graphics::ScreenVec;
#[derive(Default, Debug)]
pub struct Input {
    down: FnvHashSet<Key>,
    pressed: FnvHashSet<Key>,
    pub lmb_down: bool,
    pub rmb_down: bool,
    pub mouse_down_loc: ScreenVec,
    pub mid_pressed: bool,
}
impl Input {
    pub fn update_from_event(&mut self, ev: &Event, egui_kbd: bool, egui_ptr: bool) {
        loop {}
    }
    /// Pressed event should be cleared every frame
    pub fn clear_pressed(&mut self) {
        loop {}
    }
    pub fn down(&self, key: Key) -> bool {
        loop {}
    }
    pub fn pressed(&self, key: Key) -> bool {
        loop {}
    }
}
