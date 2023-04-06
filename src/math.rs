use num_traits::Signed;

use crate::world::{TilePos, TilePosScalar};

pub type WorldPosScalar = u32;

#[derive(Clone, Copy)]
pub struct WorldPos {
    pub x: WorldPosScalar,
    pub y: WorldPosScalar,
}

/// Tile size in pixels
pub const TILE_SIZE: u8 = 32;
/// Pixels per meter.
pub const PX_PER_M: f32 = TILE_SIZE as f32 * 2.;
/// Meters per pixel
pub const M_PER_PX: f32 = 1. / PX_PER_M;
pub const FPS_TARGET: u8 = 60;

pub fn px_per_frame_to_m_per_s(px_per_frame: f32) -> f32 {
    let m_per_frame = px_per_frame / PX_PER_M;
    m_per_frame * FPS_TARGET as f32
}

pub fn px_per_frame_to_km_h(px_per_frame: f32) -> f32 {
    px_per_frame_to_m_per_s(px_per_frame) * 3.6
}

/// World extent in tiles. Roughly 50km*50km.
pub const WORLD_EXTENT: TilePosScalar = 100_000;

impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        TilePos {
            x: wp_to_tp(self.x),
            y: wp_to_tp(self.y),
        }
    }
    /// Horizontal center of the world
    pub const CENTER: WorldPosScalar = (WORLD_EXTENT / 2) * TILE_SIZE as WorldPosScalar;
    /// Vertical surface level.
    /// You can build 10 km high.
    pub const SURFACE: WorldPosScalar = 20_000 * TILE_SIZE as WorldPosScalar;
    pub const SURFACE_CENTER: Self = Self {
        x: Self::CENTER,
        y: Self::SURFACE,
    };
}

pub fn wp_to_tp(wp: WorldPosScalar) -> TilePosScalar {
    (wp / TILE_SIZE as WorldPosScalar) as TilePosScalar
}

// Get the offset required to center an object of `xw` width inside an object of `yw` width.
//
// For example, let's say `xw` (+) is 10 and we want to center it inside `yw` (-), which is 20
//
// ++++++++++           (x uncentered)
// -------------------- (y)
//      ++++++++++      (x centered)
//
// In this case, we needed to add 5 to x to achieve centering.
// This is the offset that this function calculates.
//
// We can calulate it by subtracting `xw` from `yw` (10), and dividing it by 2.
pub fn center_offset<N: From<u8> + Copy + Signed>(xw: N, yw: N) -> N {
    let diff = yw - xw;
    diff / N::from(2)
}

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
}
