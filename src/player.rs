use std::path::Path;
use egui_inspect::{derive::Inspect, inspect};
use s2dc::{vec2, MobileEntity};
use serde::{Deserialize, Serialize};
use crate::{
    math::{WorldPos, TILE_SIZE},
    world::{TPosSc, TilePos},
};
#[derive(Debug, Inspect, Serialize, Deserialize)]
pub struct Player {
    #[inspect_with(inspect_mobile_entity)]
    pub col_en: MobileEntity,
    pub vspeed: f32,
    pub hspeed: f32,
    pub jumps_left: u8,
    /// true if the player wants to jump down from a platform
    pub down_intent: bool,
}
fn inspect_mobile_entity(en: &mut MobileEntity, ui: &mut egui::Ui, _id_src: u64) {
    loop {}
}
impl Player {
    pub fn new_at(pos: WorldPos) -> Self {
        loop {}
    }
    #[allow(dead_code)]
    pub fn center_tp(&self) -> TilePos {
        loop {}
    }
    pub fn can_jump(&self) -> bool {
        loop {}
    }
    pub fn feet_y(&self) -> i32 {
        loop {}
    }
    pub(crate) fn save(&self, path: &Path) {
        loop {}
    }
}
