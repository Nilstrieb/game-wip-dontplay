use crate::game::GameState;
use egui_inspect::inspect;

pub(crate) fn do_debug_ui(ctx: &egui::Context, mut game: &mut GameState, mut scale: &mut u8) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            inspect! {
                ui, scale, game
            }
        });
    });
}
