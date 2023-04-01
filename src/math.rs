use crate::world::TilePos;

pub type WorldPosScalar = i32;

#[derive(Clone, Copy)]
pub struct WorldPos {
    pub x: WorldPosScalar,
    pub y: WorldPosScalar,
}

impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        TilePos {
            x: self.x / TILE_SIZE as i32,
            y: self.y / TILE_SIZE as i32,
        }
    }
}

pub const TILE_SIZE: u8 = 32;
