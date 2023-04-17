mod app;
mod command;
mod debug;
mod game;
mod graphics;
mod math;
mod res;
mod texture_atlas;
mod tiles;
mod world;
use app::App;

fn main() {
    let mut app = App::new().unwrap();
    app.do_game_loop();
}
