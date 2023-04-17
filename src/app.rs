use std::fmt::Write;
use anyhow::Context;
use directories::ProjectDirs;
use egui_sfml::SfEgui;
use gamedebug_core::{imm, imm_dbg};
use sfml::{
    audio::SoundSource,
    graphics::{
        BlendMode, Color, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderTexture, RenderWindow, Shape, Sprite, Transformable, View,
    },
    system::{Vector2, Vector2u},
    window::{Event, Key},
};
use crate::{
    command::{Cmd, CmdVec},
    debug::{self, DebugState},
    game::{for_each_tile_on_screen, Biome, GameState},
    graphics::{self, ScreenSc, ScreenVec},
    input::Input, inventory::{ItemId, Slot, TileLayer, UseAction},
    math::{center_offset, TILE_SIZE},
    res::Res, tiles::TileId, CliArgs,
};
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
        while !self.should_quit {
            self.do_event_handling();
            self.do_update();
            self.do_rendering();
            self.input.clear_pressed();
            gamedebug_core::inc_frame();
        }
        self.game.tile_db.try_save();
        self.game.world.save();
    }
    fn do_event_handling(&mut self) {
        loop {}
    }
    fn do_update(&mut self) {
        loop {}
    }
    fn do_freecam(&mut self) {
        loop {}
    }
    fn do_rendering(&mut self) {
        self.game.light_pass(&mut self.light_map, &self.res);
        self.rt.clear(Color::rgb(55, 221, 231));
        self.game.draw_world(&mut self.rt, &mut self.res);
        self.game.draw_entities(&mut self.rt);
        self.rt.display();
        let mut spr = Sprite::with_texture(self.rt.texture());
        spr.set_scale((self.scale as f32, self.scale as f32));
        let vco = viewport_center_offset(self.rw.size(), self.rt.size(), self.scale);
        spr.set_position((vco.x as f32, vco.y as f32));
        self.rw.clear(Color::rgb(40, 10, 70));
        self.rw.draw(&spr);
        let mut rst = RenderStates::default();
        rst.blend_mode = BlendMode::MULTIPLY;
        self.light_map.display();


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
    fn execute_commands(&mut self) {
        loop {}
    }
}
fn viewport_center_offset(rw_size: Vector2u, rt_size: Vector2u, scale: u8) -> ScreenVec {
    loop {}
}
