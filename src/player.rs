use std::path::Path;
use egui_inspect::{derive::Inspect, inspect};
use s2dc::{vec2, MobileEntity};
use serde::{Deserialize, Serialize};
use crate::{
    math::{WorldPos, TILE_SIZE},
    world::{TPosSc, TilePos},
};
#[derive(Debug, Inspect, Serialize, Deserialize)]
pub(crate) struct Player {
    #[inspect_with(inspect_mobile_entity)]
    pub(crate) col_en: MobileEntity,
    pub(crate) vspeed: f32,
    pub(crate) hspeed: f32,
}
fn inspect_mobile_entity(en: &mut MobileEntity, ui: &mut egui::Ui, _id_src: u64) {
    loop {}
}
impl Player {
    pub(crate) fn new_at(pos: WorldPos) -> Self {
        loop {}
    }
    #[allow(dead_code)]
    pub(crate) fn center_tp(&self) -> TilePos {
        loop {}
    }
    pub(crate) fn can_jump(&self) -> bool {
        loop {}
    }
    pub(crate) fn feet_y(&self) -> i32 {
        loop {}
    }
    pub(crate) fn save(&self, path: &Path) {
        loop {}
    }
}
