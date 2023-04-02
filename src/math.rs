use crate::world::{TilePos, TilePosScalar};

pub type WorldPosScalar = i32;

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
}

pub fn wp_to_tp(wp: WorldPosScalar) -> TilePosScalar {
    if wp.is_negative() {
        (wp + 1) / TILE_SIZE as TilePosScalar - 1
    } else {
        wp / TILE_SIZE as TilePosScalar
    }
}

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
    assert_eq!(wp_to_tp(-1), -1);
    assert_eq!(wp_to_tp(-32), -1);
    assert_eq!(wp_to_tp(-33), -2);
}
