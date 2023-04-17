use std::fmt::Debug;
use egui_inspect::derive::Inspect;
use num_traits::{Num, Signed};
use serde::{Deserialize, Serialize};
use crate::world::{TPosSc, TilePos};
pub type WPosSc = u32;
#[derive(Clone, Copy, Debug, Inspect)]
pub struct WorldPos {
    pub x: WPosSc,
    pub y: WPosSc,
}
/// Tile size in pixels
pub const TILE_SIZE: u8 = 32;
/// Pixels per meter.
pub const PX_PER_M: f32 = TILE_SIZE as f32 * 2.;
/// Meters per pixel
pub const M_PER_PX: f32 = 1. / PX_PER_M;
pub const FPS_TARGET: u8 = 60;
pub fn px_per_frame_to_m_per_s(px_per_frame: f32) -> f32 {
    loop {}
}
pub fn px_per_frame_to_km_h(px_per_frame: f32) -> f32 {
    loop {}
}
/// World extent in tiles. Roughly 50km*50km.
pub const WORLD_EXTENT: TPosSc = 100_000;
impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        loop {}
    }
    /// Horizontal center of the world
    pub const CENTER: WPosSc = (WORLD_EXTENT / 2) * TILE_SIZE as WPosSc;
    /// Vertical surface level.
    /// You can build 10 km high.
    pub const SURFACE: WPosSc = 20_000 * TILE_SIZE as WPosSc;
    pub const SURFACE_CENTER: Self = Self {
        x: Self::CENTER,
        y: Self::SURFACE,
    };
    pub(crate) fn to_s2dc(self) -> s2dc::Vec2 {
        loop {}
    }
}
pub fn wp_to_tp(wp: WPosSc) -> TPosSc {
    loop {}
}
pub fn center_offset<N: From<u8> + Copy + Signed>(xw: N, yw: N) -> N {
    loop {}
}
/// A smooth triangle-wave like transform of the input value, oscillating between 0 and the ceiling.
pub fn smoothwave<T: Num + From<u8> + PartialOrd + Copy>(input: T, max: T) -> T {
    loop {}
}
#[derive(Serialize, Deserialize, Debug, Inspect, Default, Clone, Copy)]
pub struct IntRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl IntRect {
    pub(crate) fn to_sf(self) -> sfml::graphics::Rect<i32> {
        loop {}
    }
}
#[test]
fn test_smooth_wave() {
    loop {}
}
#[test]
fn test_wp_to_tp() {
    loop {}
}
