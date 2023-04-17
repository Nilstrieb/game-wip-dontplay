use std::fmt::Debug;
use egui_inspect::{derive::Inspect, Inspect};
use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
};
use super::{Bg, Fg, Mid, TileDb, TileDef, TileLayer, DEFAULT_TILE_BB};
