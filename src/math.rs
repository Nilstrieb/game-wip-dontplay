use crate::world::{TilePos, TilePosScalar};

pub type WorldPosScalar = u32;

#[derive(Clone, Copy)]
pub struct WorldPos {
    pub x: WorldPosScalar,
    pub y: WorldPosScalar,
}

pub const TILE_SIZE: u8 = 32;

impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        TilePos {
            x: wp_to_tp(self.x),
            y: wp_to_tp(self.y),
        }
    }
    /// Horizontal center of the world
    pub const CENTER: WorldPosScalar = WorldPosScalar::MAX / 2;
    /// Vertical surface level
    pub const SURFACE: WorldPosScalar = WorldPosScalar::MAX / 3;
    pub const SURFACE_CENTER: Self = Self {
        x: Self::CENTER,
        y: Self::SURFACE,
    };
}

pub fn wp_to_tp(wp: WorldPosScalar) -> TilePosScalar {
    wp / TILE_SIZE as TilePosScalar
}

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
}
