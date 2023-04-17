use crate::{
    command::CmdVec, game::GameState, res::Res, texture_atlas::AtlasBundle,
    tiles::tiledb_edit_ui::TileDbEdit,
};
use egui_inspect::{derive::Inspect, inspect};
use sfml::audio::SoundSource;
#[derive(Default, Debug, Inspect)]
pub(crate) struct DebugState {
    pub(crate) panel: bool,
    pub(crate) freecam: bool,
    pub(crate) tiledb_edit: TileDbEdit,
    pub(crate) show_atlas: bool,
    pub(crate) console: Console,
}
#[derive(Default, Debug, Inspect)]
pub(crate) struct Console {
    pub(crate) show: bool,
}
fn debug_panel_ui(
    mut debug: &mut DebugState,
    mut game: &mut GameState,
    ctx: &egui::Context,
    res: &mut Res,
    mut scale: &mut u8,
) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        ui.label("Music volume");
        let mut vol = res.surf_music.volume();
        ui.add(egui::DragValue::new(&mut vol));
        res.surf_music.set_volume(vol);
        ui.separator();
        egui::ScrollArea::both()
            .id_source("insp_scroll")
            .max_height(240.)
            .max_width(340.0)
            .show(ui, |ui| {
                inspect! {
                    ui, scale, game, debug
                }
            });

        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            gamedebug_core::for_each_imm(|info| match info {
                gamedebug_core::Info::Msg(msg) => {
                    ui.label(msg);
                }
                gamedebug_core::Info::Rect(_, _, _, _, _) => todo!(),
            });
        });
        gamedebug_core::clear_immediates();
    });
}
pub(crate) fn do_debug_ui(
    ctx: &egui::Context,
    debug: &mut DebugState,
    game: &mut GameState,
    res: &mut Res,
    scale: &mut u8,
    cmd: &mut CmdVec,
) {
    if debug.panel {
        debug_panel_ui(debug, game, ctx, res, scale);
    }
    debug.tiledb_edit.ui(ctx, &mut game.tile_db);
    if debug.console.show {
        console_ui(ctx, debug, cmd);
    }
}
fn console_ui(ctx: &egui::Context, debug: &mut DebugState, cmd: &mut CmdVec) {
    loop {}
}
