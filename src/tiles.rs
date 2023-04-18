use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::{derive::Inspect, Inspect};

pub struct TileDef {
    pub layer: (),
    //ADD pub blend_graphic: String,
}

impl ::egui_inspect::Inspect for TileDef {
    fn inspect(&self, ui: &mut ::egui::Ui, id_source: u64) {}
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {}
}

impl Debug for TileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

// this is actually used
pub struct TileDb {
    unknown_bg: TileDef,
}
impl ::egui_inspect::Inspect for TileDb {
    fn inspect(&self, ui: &mut ::egui::Ui, id_source: u64) {}
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
        ::egui_inspect::Inspect::inspect_mut(&mut self.unknown_bg, ui, 0usize as u64)
    }
}

impl Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
