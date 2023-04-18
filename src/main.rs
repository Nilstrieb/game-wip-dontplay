use egui_inspect::Inspect;

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

use egui_inspect::derive::Inspect;

pub(crate) struct WorldPos {}

impl ::core::fmt::Debug for WorldPos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())
    }
}
impl ::egui_inspect::Inspect for WorldPos {
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui) {}
}

#[derive(Debug, Inspect)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}

fn main() {
    let mut app = App::new();
    do_debug_ui(&mut app.game);
}

pub(crate) fn do_debug_ui(game: &mut GameState) {
    show(&|ui| {
        game.inspect_mut(ui);
    });
}

fn show(f: &dyn FnMut(&mut egui::Ui)) {}

// this is actually used
pub struct TileDb {
    unknown_bg: tiles::TileDef,
}
impl Inspect for TileDb {
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui) {
        let _a = &mut self.unknown_bg;
    }
}

impl std::fmt::Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
