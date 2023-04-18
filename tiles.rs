use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::Inspect;

pub struct TileDef {
    pub layer: (),
    //ADD pub blend_graphic: String,
}

fn inspect_mut(_: &mut TileDef) {}

impl Debug for TileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

// this is actually used
pub struct TileDb {
    unknown_bg: TileDef,
}
impl Inspect for TileDb {
    fn inspect(&self, ui: &mut ::egui::Ui, id_source: u64) {}
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
        inspect_mut(&mut self.unknown_bg)
    }
}

impl Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
