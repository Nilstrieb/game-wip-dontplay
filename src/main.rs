use debug::DebugState;
use egui_sfml::SfEgui;
use game::GameState;

mod debug;
mod game;
mod tiles;

pub(crate) struct App {
    pub(crate) game: GameState,
    pub(crate) sf_egui: SfEgui,
    pub(crate) debug: DebugState,
    /// Integer scale for rendering the game
    pub(crate) scale: u8,
}
impl App {
    pub(crate) fn new() -> anyhow::Result<Self> {
        loop {}
    }
}

fn main() {
    let mut app = App::new().unwrap();
    app.sf_egui
        .do_frame(|ctx| {
            debug::do_debug_ui(
                ctx,
                &mut app.debug,
                &mut app.game,
                &mut app.scale,
            );
        })
        .unwrap();
}
