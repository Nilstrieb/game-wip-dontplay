use fnv::FnvHashSet;
use sfml::window::{mouse, Event, Key};
use crate::graphics::ScreenVec;
#[derive(Default, Debug)]
pub(crate) struct Input {}
impl Input {
    pub(crate) fn update_from_event(
        &mut self,
        ev: &Event,
        egui_kbd: bool,
        egui_ptr: bool,
    ) {
        loop {}
    }
    /// Pressed event should be cleared every frame
    pub(crate) fn clear_pressed(&mut self) {
        loop {}
    }
    pub(crate) fn down(&self, key: Key) -> bool {
        loop {}
    }
    pub(crate) fn pressed(&self, key: Key) -> bool {
        loop {}
    }
}
