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
    let m_per_frame = px_per_frame / PX_PER_M;
    m_per_frame * FPS_TARGET as f32
}

pub fn px_per_frame_to_km_h(px_per_frame: f32) -> f32 {
    px_per_frame_to_m_per_s(px_per_frame) * 3.6
}

/// World extent in tiles. Roughly 50km*50km.
pub const WORLD_EXTENT: TPosSc = 100_000;

impl WorldPos {
    pub fn tile_pos(&self) -> TilePos {
        TilePos {
            x: wp_to_tp(self.x),
            y: wp_to_tp(self.y),
        }
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
}

pub fn wp_to_tp(wp: WPosSc) -> TPosSc {
    (wp / TILE_SIZE as WPosSc) as TPosSc
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

/// A smooth triangle-wave like transform of the input value, oscillating between 0 and the ceiling.
pub fn smoothwave<T: Num + From<u8> + PartialOrd + Copy>(input: T, max: T) -> T {
    let period = max * T::from(2);
    let value = input % period;
    if value < max {
        value
    } else {
        period - value
    }
}

#[derive(Serialize, Deserialize, Debug, Inspect, Default, Clone, Copy)]
pub struct IntRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl IntRect {
    pub(crate) fn to_sf(&self) -> sfml::graphics::Rect<i32> {
        sfml::graphics::Rect::<i32> {
            left: self.x,
            top: self.y,
            width: self.w,
            height: self.h,
        }
    }
}

#[test]
fn test_smooth_wave() {
    assert_eq!(smoothwave(0, 100), 0);
    assert_eq!(smoothwave(50, 100), 50);
    assert_eq!(smoothwave(125, 100), 75);
    assert_eq!(smoothwave(150, 100), 50);
    assert_eq!(smoothwave(175, 100), 25);
    assert_eq!(smoothwave(199, 100), 1);
    assert_eq!(smoothwave(200, 100), 0);
    assert_eq!(smoothwave(201, 100), 1);
}

#[test]
fn test_wp_to_tp() {
    assert_eq!(wp_to_tp(0), 0);
    assert_eq!(wp_to_tp(1), 0);
    assert_eq!(wp_to_tp(33), 1);
}
