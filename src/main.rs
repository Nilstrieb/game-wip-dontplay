extern crate alloc;
mod tiles;

pub(crate) struct App {
    pub(crate) game: GameState,
}
impl App {
    pub(crate) fn new() -> Self {
        loop {}
    }
}

use egui_inspect_derive::Inspect;

pub(crate) struct WorldPos {}

#[derive(Inspect)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}

fn main() {
    let mut app = App::new();
    app.game.inspect_mut();
}
// this is actually used
pub struct TileDb {
    unknown_bg: tiles::TileDef,
}

pub trait Inspect {
    fn inspect_mut(&mut self) {
        loop {}
    }
}
impl Inspect for () {}

impl std::fmt::Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

pub struct PlatformOutput {
    pub copied_text: String,
}

pub fn output_mut<R>(writer: impl FnOnce(&mut PlatformOutput) -> R) -> R {
    loop {}
}
