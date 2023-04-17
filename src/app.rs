use crate::{
    command::CmdVec,
    debug::{self, DebugState},
    game::GameState,
    res::Res,
    CliArgs,
};
use egui_sfml::SfEgui;
/// Application level state (includes game and ui state, etc.)
pub(crate) struct App {
    pub(crate) game: GameState,
    pub(crate) res: Res,
    pub(crate) sf_egui: SfEgui,
    pub(crate) debug: DebugState,
    /// Integer scale for rendering the game
    pub(crate) scale: u8,
    pub(crate) cmdvec: CmdVec,
}
impl App {
    pub(crate) fn new(args: CliArgs) -> anyhow::Result<Self> {
        loop {}
    }
    pub(crate) fn do_game_loop(&mut self) {
        self.do_rendering();
    }
    fn do_rendering(&mut self) {
        self.sf_egui
            .do_frame(|ctx| {
                debug::do_debug_ui(
                    ctx,
                    &mut self.debug,
                    &mut self.game,
                    &mut self.res,
                    &mut self.scale,
                    &mut self.cmdvec,
                );
            })
            .unwrap();
    }
}
