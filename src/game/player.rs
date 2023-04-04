use s2dc::{vec2, MobileEntity};

use crate::{
    math::{WorldPos, TILE_SIZE},
    world::{TilePos, TilePosScalar},
};

pub struct Player {
    pub col_en: MobileEntity,
    pub vspeed: f32,
    pub hspeed: f32,
    pub jumps_left: u8,
}

impl Player {
    pub fn new_at(pos: WorldPos) -> Self {
        Self {
            col_en: MobileEntity::from_pos_and_bb(vec2(pos.x as i32, pos.y as i32), vec2(20, 46)),
            vspeed: 0.0,
            hspeed: 0.0,
            jumps_left: 0,
        }
    }
    pub fn center_tp(&self) -> TilePos {
        TilePos {
            x: (self.col_en.en.pos.x / TILE_SIZE as i32) as TilePosScalar,
            y: (self.col_en.en.pos.y / TILE_SIZE as i32) as TilePosScalar,
        }
    }
    pub fn can_jump(&self) -> bool {
        self.jumps_left > 0
    }
    pub fn feet_y(&self) -> i32 {
        self.col_en.en.pos.y + self.col_en.en.bb.y
    }
}
