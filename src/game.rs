use crate::tiles::TileDb;
use derivative::Derivative;
use egui_inspect::derive::Inspect;

#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct WorldPos {}

#[derive(Derivative, Inspect)]
#[derivative(Debug)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}
