pub mod tiledb_edit_ui;

use std::ops::Index;

use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};

use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::{IntRect, TILE_SIZE},
    texture_atlas::RectMap,
    world::TileId,
};

#[derive(Serialize, Deserialize, Default, Debug, Inspect)]
pub struct TileDef {
    pub bb: Option<TileBb>,
    /// Whether the tile emits light, and the light source offset
    pub light: Option<ScreenVec>,
    /// Platform behavior: Horizontally passable, vertically passable upwards
    pub platform: bool,
    #[serde(default)]
    pub graphic_name: String,
    pub tex_rect: IntRect,
}

const DEFAULT_TILE_BB: TileBb = TileBb {
    x: 0,
    y: 0,
    w: TILE_SIZE,
    h: TILE_SIZE,
};

#[derive(Serialize, Deserialize, Debug, Inspect, Clone, Copy)]
pub struct TileBb {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

#[derive(Serialize, Deserialize, Debug, Inspect)]
pub struct TileDb {
    db: Vec<TileDef>,
}

impl Default for TileDb {
    fn default() -> Self {
        let unknown = TileDef {
            bb: None,
            light: Some(ScreenVec {
                x: TILE_SIZE as ScreenSc / 2,
                y: TILE_SIZE as ScreenSc / 2,
            }),
            platform: false,
            graphic_name: String::from("tiles/unknown"),
            tex_rect: IntRect::default(),
        };
        Self {
            db: vec![EMPTY, unknown],
        }
    }
}

const EMPTY: TileDef = TileDef {
    bb: None,
    light: None,
    // Rendering empty tile is actually special cased, and no rendering is done.
    // But just in case, put the offset to UNKNOWN
    tex_rect: IntRect {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    },
    platform: false,
    graphic_name: String::new(),
};

impl Index<TileId> for TileDb {
    type Output = TileDef;

    fn index(&self, index: TileId) -> &Self::Output {
        self.db.get(index as usize).unwrap_or_else(|| {
            &self.db[1] // Unknown tile def is stored at index 1
        })
    }
}

const PATH: &str = "tiles.dat";

impl TileDb {
    pub fn load_or_default() -> Self {
        match std::fs::read(PATH) {
            Ok(data) => match rmp_serde::from_slice(&data) {
                Ok(db) => db,
                Err(e) => {
                    log::warn!("Failed to load tile database: {e}\nCreating default.");
                    Default::default()
                }
            },
            Err(e) => {
                log::warn!("Failed to load tile database: {e}\nCreating default.");
                Default::default()
            }
        }
    }
    pub fn try_save(&self) {
        match rmp_serde::to_vec(self) {
            Ok(vec) => match std::fs::write(PATH, vec) {
                Ok(()) => {}
                Err(e) => log::warn!("Failed to save tile db: {e}"),
            },
            Err(e) => log::warn!("Failed to save tile db: {e}"),
        }
    }

    pub(crate) fn update_rects(&mut self, rects: &RectMap) {
        for def in &mut self.db {
            if !def.graphic_name.is_empty() {
                def.tex_rect = rects[&def.graphic_name];
            }
        }
    }
}
