use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct WorldPos {}
/// Tile size in pixels
pub(crate) const TILE_SIZE: u8 = 32;
#[derive(Serialize, Deserialize, Debug, Inspect, Default, Clone, Copy)]
pub(crate) struct IntRect {}
