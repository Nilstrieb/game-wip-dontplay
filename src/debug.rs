use crate::game::GameState;
use egui_inspect::inspect;

fn debug_panel_ui(mut game: &mut GameState, ctx: &egui::Context, mut scale: &mut u8) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            inspect! {
                ui, scale, game
            }
        });
    });
}
pub(crate) fn do_debug_ui(ctx: &egui::Context, game: &mut GameState, scale: &mut u8) {
    debug_panel_ui(game, ctx, scale);
}
