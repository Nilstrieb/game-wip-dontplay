pub mod tiledb_edit_ui;

use std::ops::Index;

use egui_inspect::derive::Inspect;
use log::warn;
use serde::{Deserialize, Serialize};
use sfml::graphics::IntRect;

use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::TILE_SIZE,
    world::TileId,
};

#[derive(Serialize, Deserialize, Default, Debug, Inspect)]
pub struct TileDef {
    pub solid: bool,
    /// Whether the tile emits light, and the light source offset
    pub light: Option<ScreenVec>,
    pub atlas_offset: AtlasOffset,
}

#[derive(Serialize, Deserialize, Debug, Inspect)]
pub struct TileDb {
    db: Vec<TileDef>,
}

impl Default for TileDb {
    fn default() -> Self {
        Self {
            // Add empty/air tile
            db: vec![EMPTY],
        }
    }
}

const EMPTY: TileDef = TileDef {
    solid: false,
    light: None,
    // Rendering empty tile is actually special cased, and no rendering is done.
    // But just in case, put the offset to UNKNOWN
    atlas_offset: UNKNOWN_ATLAS_OFF,
};

impl Index<TileId> for TileDb {
    type Output = TileDef;

    fn index(&self, index: TileId) -> &Self::Output {
        self.db.get(index as usize).unwrap_or(&UNKNOWN_TILE)
    }
}

#[derive(Debug, Inspect, Serialize, Deserialize)]
pub struct AtlasOffset {
    pub x: u16,
    pub y: u16,
}
impl AtlasOffset {
    pub(crate) fn to_sf_rect(&self) -> IntRect {
        IntRect {
            left: self.x as i32,
            top: self.y as i32,
            width: TILE_SIZE as i32,
            height: TILE_SIZE as i32,
        }
    }
}

impl Default for AtlasOffset {
    fn default() -> Self {
        UNKNOWN_ATLAS_OFF
    }
}

const UNKNOWN_ATLAS_OFF: AtlasOffset = AtlasOffset { x: 320, y: 0 };

static UNKNOWN_TILE: TileDef = TileDef {
    solid: true,
    light: Some(ScreenVec {
        x: TILE_SIZE as ScreenSc / 2,
        y: TILE_SIZE as ScreenSc / 2,
    }),
    atlas_offset: UNKNOWN_ATLAS_OFF,
};

const PATH: &str = "tiles.dat";

impl TileDb {
    pub fn load_or_default() -> Self {
        match std::fs::read(PATH) {
            Ok(data) => match rmp_serde::from_slice(&data) {
                Ok(db) => db,
                Err(e) => {
                    warn!("Failed to load tile database: {e}\nCreating default.");
                    Default::default()
                }
            },
            Err(e) => {
                warn!("Failed to load tile database: {e}\nCreating default.");
                Default::default()
            }
        }
    }
    pub fn try_save(&self) {
        match rmp_serde::to_vec(self) {
            Ok(vec) => match std::fs::write(PATH, vec) {
                Ok(()) => {}
                Err(e) => warn!("Failed to save tile db: {e}"),
            },
            Err(e) => warn!("Failed to save tile db: {e}"),
        }
    }
}
