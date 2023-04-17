use crate::game::GameState;
use egui_inspect::{derive::Inspect, inspect};

#[derive(Default, Debug, Inspect)]
pub(crate) struct DebugState {}
fn debug_panel_ui(
    mut debug: &mut DebugState,
    mut game: &mut GameState,
    ctx: &egui::Context,
    mut scale: &mut u8,
) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            inspect! {
                ui, scale, game, debug
            }
        });
    });
}
pub(crate) fn do_debug_ui(
    ctx: &egui::Context,
    debug: &mut DebugState,
    game: &mut GameState,
    scale: &mut u8,
) {
    debug_panel_ui(debug, game, ctx, scale);
}
