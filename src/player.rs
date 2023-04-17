use std::path::Path;
use egui_inspect::{derive::Inspect, inspect};
use s2dc::{vec2, MobileEntity};
use serde::{Deserialize, Serialize};
use crate::{
    math::{WorldPos, TILE_SIZE},
    world::{TPosSc, TilePos},
};
#[derive(Debug, Inspect, Serialize, Deserialize)]
pub(crate) struct Player {}
