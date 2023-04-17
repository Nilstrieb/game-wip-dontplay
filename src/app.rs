use crate::{
    command::{Cmd, CmdVec},
    debug::{self, DebugState},
    game::{for_each_tile_on_screen, Biome, GameState},
    graphics::{self, ScreenSc, ScreenVec},
    input::Input,
    inventory::{ItemId, Slot, TileLayer, UseAction},
    math::{center_offset, TILE_SIZE},
    res::Res,
    tiles::TileId,
    CliArgs,
};
use anyhow::Context;
use directories::ProjectDirs;
use egui_sfml::SfEgui;
use gamedebug_core::{imm, imm_dbg};
use sfml::{
    audio::SoundSource,
    graphics::{
        BlendMode, Color, Rect, RectangleShape, RenderStates, RenderTarget, RenderTexture,
        RenderWindow, Shape, Sprite, Transformable, View,
    },
    system::{Vector2, Vector2u},
    window::{Event, Key},
};
use std::fmt::Write;
/// Application level state (includes game and ui state, etc.)
pub(crate) struct App {
    pub(crate) rw: RenderWindow,
    pub(crate) should_quit: bool,
    pub(crate) game: GameState,
    pub(crate) res: Res,
    pub(crate) sf_egui: SfEgui,
    pub(crate) input: Input,
    pub(crate) debug: DebugState,
    /// Integer scale for rendering the game
    pub(crate) scale: u8,
    /// RenderTexture for rendering the game at its native resolution
    pub(crate) rt: RenderTexture,
    /// Light map overlay, blended together with the non-lighted version of the scene
    pub(crate) light_map: RenderTexture,
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
