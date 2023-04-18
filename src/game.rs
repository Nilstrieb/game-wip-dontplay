use crate::tiles::TileDb;
use egui_inspect::derive::Inspect;

#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct WorldPos {}

#[derive(Debug, Inspect)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}
