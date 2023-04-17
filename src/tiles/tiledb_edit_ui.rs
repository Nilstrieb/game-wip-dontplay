use std::fmt::Debug;

use egui_inspect::{derive::Inspect, Inspect};

use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};

#[derive(Debug, Default, Inspect)]
pub struct TileDbEdit {
    open: bool,
    #[opaque]
    layer: Layer,
}
impl TileDbEdit {
    pub(crate) fn ui(&mut self, ctx: &egui::Context, tile_db: &mut TileDb) {
        if !self.open {
            return;
        }
        egui::Window::new("Tiledb editor").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.layer, Layer::Bg, "Bg");
                ui.selectable_value(&mut self.layer, Layer::Mid, "Mid");
                ui.selectable_value(&mut self.layer, Layer::Fg, "Fg");
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| match self.layer {
                    Layer::Bg => db_ui(&mut tile_db.bg, ui),
                    Layer::Fg => db_ui(&mut tile_db.fg, ui),
                    Layer::Mid => db_ui(&mut tile_db.mid, ui),
                });
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Layer {
    Bg,
    Fg,
    Mid,
}

impl Default for Layer {
    fn default() -> Self {
        Self::Bg
    }
}

use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};

trait SpecialUi {
    fn special_ui(&mut self, ui: &mut egui::Ui);
}

impl SpecialUi for TileDef<Mid> {
    fn special_ui(&mut self, ui: &mut egui::Ui) {
        match &mut self.layer.bb {
            Some(bb) => {
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut bb.x));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut bb.y));
                    ui.label("w");
                    ui.add(egui::DragValue::new(&mut bb.w));
                    ui.label("h");
                    ui.add(egui::DragValue::new(&mut bb.h));
                });
            }
            None => {
                if ui.button("Insert bb").clicked() {
                    self.layer.bb = Some(DEFAULT_TILE_BB);
                }
            }
        }
        ui.checkbox(&mut self.layer.platform, "platform");
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
        ui.text_edit_singleline(&mut def.graphic_name);
        match &mut def.light {
            Some(light) => {
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut light.x));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut light.y));
                });
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
        ui.separator();
    }
    if ui.button("Add new default").clicked() {
        db.push(Default::default());
    }
}
