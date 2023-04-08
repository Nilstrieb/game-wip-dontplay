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
    inspect! {
        ui,
        en.en.pos.x,
        en.en.pos.y,
        en.en.bb.x,
        en.en.bb.y
    }
}

impl Player {
    pub fn new_at(pos: WorldPos) -> Self {
        Self {
            col_en: MobileEntity::from_pos_and_bb(vec2(pos.x as i32, pos.y as i32), vec2(20, 46)),
            vspeed: 0.0,
            hspeed: 0.0,
            jumps_left: 0,
            down_intent: false,
        }
    }
    #[allow(dead_code)]
    pub fn center_tp(&self) -> TilePos {
        TilePos {
            x: (self.col_en.en.pos.x / TILE_SIZE as i32) as TPosSc,
            y: (self.col_en.en.pos.y / TILE_SIZE as i32) as TPosSc,
        }
    }
    pub fn can_jump(&self) -> bool {
        self.jumps_left > 0
    }
    pub fn feet_y(&self) -> i32 {
        self.col_en.en.pos.y + self.col_en.en.bb.y
    }

    pub(crate) fn save(&self) {
        log::info!(
            "{:?}",
            std::fs::write("player.dat", rmp_serde::to_vec(self).unwrap())
        );
    }
}
