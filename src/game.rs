use crate::{math::WorldPos, tiles::TileDb};
use derivative::Derivative;
use egui_inspect::derive::Inspect;

#[derive(Derivative, Inspect)]
#[derivative(Debug)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}
