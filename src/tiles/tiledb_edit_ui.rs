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
        loop {}
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
        loop {}
    }
}
use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};
trait SpecialUi {
    fn special_ui(&mut self, ui: &mut egui::Ui);
}
impl SpecialUi for TileDef<Mid> {
    fn special_ui(&mut self, ui: &mut egui::Ui) {
        loop {}
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
    loop {}
}
