use std::fmt::Write;
use egui::TextBuffer;
use egui_inspect::{derive::Inspect, inspect};
use sfml::audio::SoundSource;
use crate::{
    cmdline::CmdLine, command::CmdVec, game::GameState,
    math::{px_per_frame_to_km_h, WorldPos},
    res::Res, stringfmt::LengthDisp, texture_atlas::AtlasBundle,
    tiles::tiledb_edit_ui::TileDbEdit,
};
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
    egui::Window::new("Debug (F12)")
        .show(
            ctx,
            |ui| {
                if debug.freecam {
                    ui.label("Cam x");
                    ui.add(egui::DragValue::new(&mut game.camera_offset.x));
                    ui.label("Cam y");
                    ui.add(egui::DragValue::new(&mut game.camera_offset.y));
                    let co = game.camera_offset;
                    ui.label(
                        format!(
                            "Cam Depth: {}", LengthDisp(co.y as f32 - WorldPos::SURFACE
                            as f32)
                        ),
                    );
                    ui.label(
                        format!(
                            "Cam offset from center: {}", LengthDisp(co.x as f32 -
                            WorldPos::CENTER as f32)
                        ),
                    );
                } else {
                    ui.label(
                        format!(
                            "Player Depth: {}", LengthDisp(game.world.player.feet_y() as
                            f32 - WorldPos::SURFACE as f32)
                        ),
                    );
                    ui.label(
                        format!(
                            "Player offset from center: {}", LengthDisp(game.world.player
                            .col_en.en.pos.x as f32 - WorldPos::CENTER as f32)
                        ),
                    );
                    ui.label(
                        format!(
                            "Hspeed: {} ({} km/h)", game.world.player.hspeed,
                            px_per_frame_to_km_h(game.world.player.hspeed)
                        ),
                    );
                    ui.label(
                        format!(
                            "Vspeed: {} ({} km/h)", game.world.player.vspeed,
                            px_per_frame_to_km_h(game.world.player.vspeed)
                        ),
                    );
                }
                ui.label("Music volume");
                let mut vol = res.surf_music.volume();
                ui.add(egui::DragValue::new(&mut vol));
                res.surf_music.set_volume(vol);
                ui.separator();
                egui::ScrollArea::both()
                    .id_source("insp_scroll")
                    .max_height(240.)
                    .max_width(340.0)
                    .show(
                        ui,
                        |ui| {
                            inspect! {
                                ui, scale, game, debug
                            }
                        },
                    );
                if ui.button("Reload graphics").clicked() {
                    res.atlas = AtlasBundle::new().unwrap();
                    game.tile_db.update_rects(&res.atlas.rects);
                }
                ui.separator();
                egui::ScrollArea::vertical()
                    .show(
                        ui,
                        |ui| {
                            gamedebug_core::for_each_imm(|info| match info {
                                gamedebug_core::Info::Msg(msg) => {
                                    ui.label(msg);
                                }
                                gamedebug_core::Info::Rect(_, _, _, _, _) => todo!(),
                            });
                        },
                    );
                gamedebug_core::clear_immediates();
            },
        );
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
