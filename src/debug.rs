use egui_inspect::inspect;
use sfml::{audio::SoundSource, window::Key};

use crate::{
    game::GameState,
    input::Input,
    math::{px_per_frame_to_km_h, WorldPos},
    res::Res,
    stringfmt::LengthDisp,
    texture_atlas::AtlasBundle,
    tiles::tiledb_edit_ui::tiledb_edit_ui,
};

#[derive(Default, Debug)]
pub struct DebugState {
    panel: bool,
    pub freecam: bool,
    tiledb_edit: bool,
}

impl DebugState {
    pub fn update(&mut self, input: &Input) {
        if input.pressed(Key::F12) {
            self.panel ^= true;
        }
        if input.pressed(Key::F10) {
            self.freecam ^= true;
        }
    }
}

fn debug_panel_ui(
    debug: &mut DebugState,
    mut game: &mut GameState,
    ctx: &egui::Context,
    res: &mut Res,
    mut scale: &mut u8,
) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        if debug.freecam {
            ui.label("Cam x");
            ui.add(egui::DragValue::new(&mut game.camera_offset.x));
            ui.label("Cam y");
            ui.add(egui::DragValue::new(&mut game.camera_offset.y));
            let co = game.camera_offset;
            ui.label(format!(
                "Cam Depth: {}",
                LengthDisp(co.y as f32 - WorldPos::SURFACE as f32)
            ));
            ui.label(format!(
                "Cam offset from center: {}",
                LengthDisp(co.x as f32 - WorldPos::CENTER as f32)
            ));
        } else {
            ui.label(format!(
                "Player Depth: {}",
                LengthDisp(game.world.player.feet_y() as f32 - WorldPos::SURFACE as f32)
            ));
            ui.label(format!(
                "Player offset from center: {}",
                LengthDisp(game.world.player.col_en.en.pos.x as f32 - WorldPos::CENTER as f32)
            ));
            ui.label(format!(
                "Hspeed: {} ({} km/h)",
                game.world.player.hspeed,
                px_per_frame_to_km_h(game.world.player.hspeed)
            ));
            ui.label(format!(
                "Vspeed: {} ({} km/h)",
                game.world.player.vspeed,
                px_per_frame_to_km_h(game.world.player.vspeed)
            ));
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
            .show(ui, |ui| {
                inspect! {
                    ui,
                    scale,
                    game,
                    debug.tiledb_edit
                }
            });
        if ui.button("Reload graphics").clicked() {
            res.atlas = AtlasBundle::new().unwrap();
            game.tile_db.update_rects(&res.atlas.rects);
        }
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
) {
    if debug.panel {
        debug_panel_ui(debug, game, ctx, res, scale);
    }
    if debug.tiledb_edit {
        tiledb_edit_ui(ctx, &mut game.tile_db);
    }
}
