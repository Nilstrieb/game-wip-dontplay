use std::fmt::Debug;
use egui_inspect::derive::Inspect;
use num_traits::{Num, Signed};
use serde::{Deserialize, Serialize};
use crate::world::{TPosSc, TilePos};
pub(crate) type WPosSc = u32;
#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct WorldPos {
    pub(crate) x: WPosSc,
    pub(crate) y: WPosSc,
}
/// Tile size in pixels
pub(crate) const TILE_SIZE: u8 = 32;
/// Pixels per meter.
pub(crate) const PX_PER_M: f32 = TILE_SIZE as f32 * 2.;
/// Meters per pixel
pub(crate) const M_PER_PX: f32 = 1. / PX_PER_M;
pub(crate) const FPS_TARGET: u8 = 60;
pub(crate) fn px_per_frame_to_km_h(px_per_frame: f32) -> f32 {
    loop {}
}
/// World extent in tiles. Roughly 50km*50km.
pub(crate) const WORLD_EXTENT: TPosSc = 100_000;
impl WorldPos {
    pub(crate) fn tile_pos(&self) -> TilePos {
        loop {}
    }
    /// Horizontal center of the world
    pub(crate) const CENTER: WPosSc = (WORLD_EXTENT / 2) * TILE_SIZE as WPosSc;
    /// Vertical surface level.
    /// You can build 10 km high.
    pub(crate) const SURFACE: WPosSc = 20_000 * TILE_SIZE as WPosSc;
    pub(crate) const SURFACE_CENTER: Self = Self {
        x: Self::CENTER,
        y: Self::SURFACE,
    };
    pub(crate) fn to_s2dc(self) -> s2dc::Vec2 {
        loop {}
    }
}
pub(crate) fn wp_to_tp(wp: WPosSc) -> TPosSc {
    loop {}
}
pub(crate) fn center_offset<N: From<u8> + Copy + Signed>(xw: N, yw: N) -> N {
    loop {}
}
/// A smooth triangle-wave like transform of the input value, oscillating between 0 and the ceiling.
pub(crate) fn smoothwave<T: Num + From<u8> + PartialOrd + Copy>(input: T, max: T) -> T {
    loop {}
}
#[derive(Serialize, Deserialize, Debug, Inspect, Default, Clone, Copy)]
pub struct IntRect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) w: i32,
    pub(crate) h: i32,
}
