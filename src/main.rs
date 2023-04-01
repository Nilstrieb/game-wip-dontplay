mod app;
mod game;
mod graphics;
mod math;
mod world;

use app::App;

fn main() {
    let mut app = App::new();
    app.do_game_loop();
}
