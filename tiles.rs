use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::{derive::Inspect, Inspect};

pub struct TileDef {
    pub layer: (),
    //ADD pub blend_graphic: String,
}

impl ::egui_inspect::Inspect for TileDef {
    fn inspect(&self, ui: &mut ::egui::Ui, id_source: u64) {}
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
        ::egui::CollapsingHeader::new("TileDef")
            .id_source(id_source)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(::egui::Label::new("pub layer : ()").sense(::egui::Sense::click()))
                        .clicked()
                    {
                        ui.output_mut(|o| {
                            o.copied_text = {
                                let res = ::std::fmt::format(format_args!("{0:?}", self.layer));
                                res
                            };
                        });
                    }
                    ::egui_inspect::Inspect::inspect_mut(&mut self.layer, ui, 0usize as u64)
                });
            });
    }
}

impl Debug for TileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

// this is actually used
#[derive(Inspect)]
pub struct TileDb {
    unknown_bg: TileDef,
}

impl Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
