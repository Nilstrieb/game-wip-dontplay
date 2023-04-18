pub struct TileDef {
    pub layer: (),
    #[cfg(second)]
    pub blend_graphic: String,
}

use egui_inspect_derive::expand;

pub(crate) struct GameState {
    pub(crate) tile_db: TileDb,
}

impl GameState {
    fn inspect_mut(&mut self) {
        expand! {}
        output_mut(|o| o.copied_text = format!("{:?}", self.tile_db));
    }
}

fn new() -> GameState {
    loop {}
}

fn main() {
    let mut app = new();
    app.inspect_mut();
}
// this is actually used
pub struct TileDb {
    unknown_bg: TileDef,
}

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
