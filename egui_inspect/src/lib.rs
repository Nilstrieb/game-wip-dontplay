use egui::Ui;
#[cfg(feature = "derive")]
pub use egui_inspect_derive as derive;

pub trait Inspect {
    fn inspect(&self, ui: &mut Ui, id_source: u64);
    fn inspect_mut(&mut self, ui: &mut Ui, id_source: u64) {
        loop {}
    }
}
impl Inspect for () {
    fn inspect(&self, ui: &mut Ui, _id_source: u64) {
        loop {}
    }
}
