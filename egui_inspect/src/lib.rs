use egui::Ui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;

pub trait Inspect {
    fn inspect_mut(&mut self, ui: &mut Ui) {
        loop {}
    }
}
impl Inspect for () {}
