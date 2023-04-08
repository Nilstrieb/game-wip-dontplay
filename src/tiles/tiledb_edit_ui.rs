use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};

use super::TileDb;

pub fn tiledb_edit_ui(ctx: &egui::Context, tile_db: &mut TileDb) {
    egui::Window::new("Tiledb editor").show(ctx, |ui| {
        ui.label(format!("Number of tile defs: {}", tile_db.db.len()));
        ui.separator();
        for (i, def) in tile_db.db.iter_mut().enumerate() {
            ui.label(i.to_string());
            match &mut def.light {
                Some(light) => {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut light.x));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut light.y));
                }
                None => {
                    if ui.button("Insert light emit").clicked() {
                        def.light = Some(ScreenVec {
                            x: TILE_SIZE as ScreenSc / 2,
                            y: TILE_SIZE as ScreenSc / 2,
                        });
                    }
                }
            }
        }
        ui.separator();
        if ui.button("Add new default").clicked() {
            tile_db.db.push(super::TileDef::default());
        }
    });
}
