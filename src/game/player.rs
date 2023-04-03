use s2dc::{vec2, MobileEntity};

use crate::{
    math::{WorldPos, TILE_SIZE},
    world::{TilePos, TilePosScalar},
};

pub struct Player {
    pub col_en: MobileEntity,
    pub vspeed: f32,
}

impl Player {
    pub fn new_at(pos: WorldPos) -> Self {
        Self {
            col_en: MobileEntity::from_pos_and_bb(vec2(pos.x as i32, pos.y as i32), vec2(15, 24)),
            vspeed: 0.0,
        }
    }
    pub fn center_tp(&self) -> TilePos {
        TilePos {
            x: (self.col_en.en.pos.x / TILE_SIZE as i32) as TilePosScalar,
            y: (self.col_en.en.pos.y / TILE_SIZE as i32) as TilePosScalar,
        }
    }
}
