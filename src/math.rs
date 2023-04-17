use crate::world::TPosSc;
use egui_inspect::derive::Inspect;
use num_traits::Num;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub(crate) type WPosSc = u32;
#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct WorldPos {}
/// Tile size in pixels
pub(crate) const TILE_SIZE: u8 = 32;
pub(crate) const FPS_TARGET: u8 = 60;
pub(crate) fn wp_to_tp(wp: WPosSc) -> TPosSc {
    loop {}
}
/// A smooth triangle-wave like transform of the input value, oscillating between 0 and the ceiling.
pub(crate) fn smoothwave<T: Num + From<u8> + PartialOrd + Copy>(input: T, max: T) -> T {
    loop {}
}
#[derive(Serialize, Deserialize, Debug, Inspect, Default, Clone, Copy)]
pub(crate) struct IntRect {}
