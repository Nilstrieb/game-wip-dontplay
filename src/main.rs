mod app;
mod game;
mod graphics;
mod input;
mod math;
mod res;
mod world;

use app::App;

fn try_main() -> anyhow::Result<()> {
    gamedebug_core::set_enabled(true);
    let mut app = App::new()?;
    app.do_game_loop();
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        rfd::MessageDialog::new()
            .set_title("Fatal error")
            .set_description(&e.to_string())
            .show();
    }
}
