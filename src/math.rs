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

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
}
