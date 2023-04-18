use egui_inspect::Inspect;
use game::GameState;

mod game;
mod tiles;

pub(crate) struct App {
    pub(crate) game: GameState,
}
impl App {
    pub(crate) fn new() -> Self {
        loop {}
    }
}

fn main() {
    let mut app = App::new();
    do_debug_ui(&mut app.game);
}

pub(crate) fn do_debug_ui(game: &mut GameState) {
    show(&|ui| {
        game.inspect_mut(ui, 0);
    });
}

fn show(f: &dyn FnMut(&mut egui::Ui)) {}

// this is actually used
pub struct TileDb {
    unknown_bg: tiles::TileDef,
}
impl Inspect for TileDb {
    fn inspect(&self, ui: &mut ::egui::Ui, id_source: u64) {}
    fn inspect_mut(&mut self, ui: &mut ::egui::Ui, id_source: u64) {
        let _a = &mut self.unknown_bg;
    }
}

impl std::fmt::Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
