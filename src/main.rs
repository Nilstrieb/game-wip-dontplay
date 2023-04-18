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

pub(crate) fn do_debug_ui(mut game: &mut GameState) {
    show(&|ui| {
        ::egui_inspect::UiExt::property(ui, "game", &mut game, &mut 0);
    });
}

fn show(f: &dyn FnMut(&mut egui::Ui)) {}
