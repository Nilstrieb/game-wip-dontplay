mod app;
mod graphics;

use app::App;

fn main() {
    let mut app = App::new();
    app.do_game_loop();
}
