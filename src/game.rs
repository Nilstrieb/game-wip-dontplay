use std::path::PathBuf;
use derivative::Derivative;
use egui_inspect::derive::Inspect;
use sfml::{
    graphics::{
        Color, Rect, RectangleShape, RenderTarget, RenderTexture, Shape, Sprite,
        Transformable,
    },
    system::{Vector2f, Vector2u},
    window::Key,
};
use crate::{
    graphics::{ScreenSc, ScreenVec},
    input::Input, inventory::{Inventory, ItemDb},
    math::{smoothwave, wp_to_tp, WorldPos},
    res::Res, tiles::TileDb, world::{TilePos, World},
    worldgen::Worldgen,
};
#[derive(Derivative, Inspect)]
#[derivative(Debug)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}
