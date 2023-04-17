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
use clap::Parser;

#[derive(Parser)]
pub(crate) struct CliArgs {}

fn main() {
    gamedebug_core::set_enabled(true);
    let cli_args = CliArgs::parse();
    let mut app = App::new(cli_args).unwrap();
    app.do_game_loop();
}
