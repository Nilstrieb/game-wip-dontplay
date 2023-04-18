extern crate alloc;
mod tiles;

pub(crate) struct App {
    pub(crate) game: GameState,
}
impl App {
    pub(crate) fn new() -> Self {
        loop {}
    }
}

use egui_inspect_derive::Inspect;

pub(crate) struct WorldPos {}

#[derive(Inspect)]
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) tile_db: TileDb,
}

fn main() {
    let mut app = App::new();
    do_debug_ui(&mut app.game);
}

pub(crate) fn do_debug_ui(game: &mut GameState) {
    show(&||{
        game.inspect_mut();
    });
}

fn show(f: &dyn FnMut()) {}

// this is actually used
pub struct TileDb {
    unknown_bg: tiles::TileDef,
}


pub trait Inspect {
    fn inspect_mut(&mut self) {
        loop {}
    }
}
impl Inspect for () {}


impl Inspect for TileDb {
    fn inspect_mut(&mut self) {
        let _a = &mut self.unknown_bg;
    }
}

impl std::fmt::Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

pub struct PlatformOutput {
    /// Set the cursor to this icon.
    pub cursor_icon: (),

    /// If set, open this url.
    pub open_url: Option<()>,

    /// If set, put this text in the system clipboard. Ignore if empty.
    ///
    /// This is often a response to [`crate::Event::Copy`] or [`crate::Event::Cut`].
    ///
    /// ```
    /// # egui::__run_test_ui(|ui| {
    /// if ui.button("📋").clicked() {
    ///     ui.output_mut(|o| o.copied_text = "some_text".to_string());
    /// }
    /// # });
    /// ```
    pub copied_text: String,

    /// Events that may be useful to e.g. a screen reader.
    pub events: Vec<()>,

    /// Is there a mutable [`TextEdit`](crate::TextEdit) under the cursor?
    /// Use by `eframe` web to show/hide mobile keyboard and IME agent.
    pub mutable_text_under_cursor: bool,

    /// Screen-space position of text edit cursor (used for IME).
    pub text_cursor_pos: Option<()>,

    #[cfg(feature = "accesskit")]
    pub accesskit_update: Option<accesskit::TreeUpdate>,
}

pub fn output_mut<R>(writer: impl FnOnce(&mut PlatformOutput) -> R) -> R {
    loop {}
}
