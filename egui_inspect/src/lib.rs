#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;

pub trait Inspect {
    fn inspect_mut(&mut self) {
        loop {}
    }
}
impl Inspect for () {}
