use std::fmt::Debug;
use egui_inspect::{derive::Inspect, Inspect};
use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};
#[derive(Debug, Default, Inspect)]
pub(crate) struct TileDbEdit {}
impl TileDbEdit {
    pub(crate) fn ui(&mut self, ctx: &egui::Context, tile_db: &mut TileDb) {
        loop {}
    }
}
use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};
