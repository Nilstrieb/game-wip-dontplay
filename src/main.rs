pub struct TileDef {
    pub layer: (),
    #[cfg(second)]
    pub blend_graphic: String,
}

use egui_inspect_derive::Inspect;

#[derive(Inspect)]
pub(crate) struct GameState {
    pub(crate) tile_db: TileDb,
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
