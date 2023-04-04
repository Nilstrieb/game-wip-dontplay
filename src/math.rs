use crate::world::{TilePos, TilePosScalar};

pub type WorldPosScalar = u32;

#[derive(Clone, Copy)]
pub struct WorldPos {
    pub x: WorldPosScalar,
    pub y: WorldPosScalar,
}

pub const TILE_SIZE: u8 = 32;
/// Pixels per meter. One meter = one tile, so this is the same as `TILE_SIZE`.
pub const PX_PER_M: u8 = TILE_SIZE;
pub const FPS_TARGET: u8 = 60;

pub fn px_per_frame_to_m_per_s(px_per_frame: f32) -> f32 {
    let m_per_frame = px_per_frame / PX_PER_M as f32;
    m_per_frame * FPS_TARGET as f32
}

pub fn px_per_frame_to_km_h(px_per_frame: f32) -> f32 {
    px_per_frame_to_m_per_s(px_per_frame) * 3.6
}

impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        TilePos {
            x: wp_to_tp(self.x),
            y: wp_to_tp(self.y),
        }
    }
    /// Horizontal center of the world
    pub const CENTER: WorldPosScalar =
        (TilePosScalar::MAX / 2) as WorldPosScalar * TILE_SIZE as WorldPosScalar;
    /// Vertical surface level. You can build 5,000 blocks upwards
    pub const SURFACE: WorldPosScalar = 5000 * TILE_SIZE as WorldPosScalar;
    pub const SURFACE_CENTER: Self = Self {
        x: Self::CENTER,
        y: Self::SURFACE,
    };
}

pub fn wp_to_tp(wp: WorldPosScalar) -> TilePosScalar {
    (wp / TILE_SIZE as WorldPosScalar) as TilePosScalar
}

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
}
