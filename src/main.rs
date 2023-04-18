use egui_sfml::SfEgui;
use game::GameState;

mod game;
mod tiles;

pub(crate) struct App {
    pub(crate) game: GameState,
    pub(crate) sf_egui: SfEgui,
    /// Integer scale for rendering the game
    pub(crate) scale: u8,
}
impl App {
    pub(crate) fn new() -> Self {
        loop {}
    }
}

fn main() {
    let mut app = App::new();
    app.sf_egui
        .do_frame(|ctx| {
            do_debug_ui(ctx, &mut app.game, &mut app.scale);
        })
        .unwrap();
}

pub(crate) fn do_debug_ui(ctx: &egui::Context, mut game: &mut GameState, mut scale: &mut u8) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            let mut id_source = 0;
            ::egui_inspect::UiExt::property(ui, "scale", &mut scale, &mut id_source);
            ::egui_inspect::UiExt::property(ui, "game", &mut game, &mut id_source);
        });
    });
}
