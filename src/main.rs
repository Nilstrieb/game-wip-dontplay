mod app;
mod bitmanip;
mod cmdline;
mod command;
mod debug;
mod game;
mod graphics;
mod input;
mod inventory;
mod math;
mod player;
mod res;
mod stringfmt;
mod texture_atlas;
mod tiles;
mod world;
mod worldgen;

use app::App;
use clap::Parser;

#[derive(Parser)]
pub struct CliArgs {
    #[arg(default_value = "TestWorld")]
    world_name: String,
}

fn try_main() -> anyhow::Result<()> {
    gamedebug_core::set_enabled(true);
    let cli_args = CliArgs::parse();
    let mut app = App::new(cli_args)?;
    app.do_game_loop();
    Ok(())
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    if let Err(e) = try_main() {
        rfd::MessageDialog::new()
            .set_title("Fatal error")
            .set_description(&e.to_string())
            .show();
    }
}
