use std::fmt::Debug;

use egui_inspect::Inspect;

use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};

use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};

pub fn tiledb_edit_ui(ctx: &egui::Context, tile_db: &mut TileDb) {
    egui::Window::new("Tiledb editor").show(ctx, |ui| {
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                ui.heading("Bg");
                db_ui(&mut tile_db.bg, ui);
                ui.heading("Mid");
                db_ui(&mut tile_db.mid, ui);
                ui.heading("Fg");
                db_ui(&mut tile_db.fg, ui);
            });
    });
}

trait SpecialUi {
    fn special_ui(&mut self, ui: &mut egui::Ui);
}

impl SpecialUi for TileDef<Mid> {
    fn special_ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.layer.bb {
            Some(bb) => {
                ui.label("x");
                ui.add(egui::DragValue::new(&mut bb.x));
                ui.label("y");
                ui.add(egui::DragValue::new(&mut bb.y));
                ui.label("w");
                ui.add(egui::DragValue::new(&mut bb.w));
                ui.label("h");
                ui.add(egui::DragValue::new(&mut bb.h));
            }
            None => {
                if ui.button("Insert bb").clicked() {
                    self.layer.bb = Some(DEFAULT_TILE_BB);
                }
            }
        }
    }
}

impl SpecialUi for TileDef<Bg> {
    fn special_ui(&mut self, _ui: &mut egui::Ui) {}
}

impl SpecialUi for TileDef<Fg> {
    fn special_ui(&mut self, _ui: &mut egui::Ui) {}
}

fn db_ui<Layer: TileLayer + Debug>(db: &mut Vec<TileDef<Layer>>, ui: &mut egui::Ui)
where
    <Layer as TileLayer>::SpecificDef: Debug + Default + Inspect,
    TileDef<Layer>: SpecialUi,
{
    for (i, def) in db.iter_mut().enumerate() {
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
        def.special_ui(ui);
    }
    ui.separator();
    if ui.button("Add new default").clicked() {
        db.push(Default::default());
    }
}
