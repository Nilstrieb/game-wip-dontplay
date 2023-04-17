use std::fmt::Debug;
use egui_inspect::{derive::Inspect, Inspect};
use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};
#[derive(Debug, Default, Inspect)]
pub(crate) struct TileDbEdit {}
use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};
