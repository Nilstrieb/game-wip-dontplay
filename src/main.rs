mod app;
mod debug;
mod game;
mod graphics;
mod input;
mod math;
mod res;
mod stringfmt;
mod tiles;
mod world;
mod worldgen;

use app::App;

fn try_main() -> anyhow::Result<()> {
    gamedebug_core::set_enabled(true);
    let mut app = App::new()?;
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
