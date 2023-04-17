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
pub struct App {
    pub rw: RenderWindow,
    pub should_quit: bool,
    pub game: GameState,
    pub res: Res,
    pub sf_egui: SfEgui,
    pub input: Input,
    pub debug: DebugState,
    /// Integer scale for rendering the game
    pub scale: u8,
    /// RenderTexture for rendering the game at its native resolution
    pub rt: RenderTexture,
    /// Light map overlay, blended together with the non-lighted version of the scene
    pub light_map: RenderTexture,
    pub project_dirs: ProjectDirs,
    pub cmdvec: CmdVec,
}
impl App {
    pub fn new(args: CliArgs) -> anyhow::Result<Self> {
        loop {}
    }
    pub fn do_game_loop(&mut self) {
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
        spr.set_texture(self.light_map.texture(), false);
        self.rw.draw_with_renderstates(&spr, &rst);
        drop(spr);
        self.rt.clear(Color::TRANSPARENT);
        let ui_dims = Vector2 {
            x: (self.rw.size().x / self.scale as u32) as f32,
            y: (self.rw.size().y / self.scale as u32) as f32,
        };
        self.game.draw_ui(&mut self.rt, &self.res, ui_dims);
        self.rt.display();
        let mut spr = Sprite::with_texture(self.rt.texture());
        spr.set_scale((self.scale as f32, self.scale as f32));
        self.rw.draw(&spr);
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
        if self.debug.show_atlas {
            let atlas = &self.res.atlas.tex;
            let size = atlas.size();
            let mut rs = RectangleShape::from_rect(
                Rect::new(0., 0., size.x as f32, size.y as f32),
            );
            rs.set_fill_color(Color::MAGENTA);
            self.rw.draw(&rs);
            self.rw.draw(&Sprite::with_texture(atlas));
        }
        self.sf_egui.draw(&mut self.rw, None);
        self.rw.display();
        drop(spr);
        self.execute_commands();
    }
    fn execute_commands(&mut self) {
        loop {}
    }
}
/// Tile collision entity for doing physics
struct TileColEn {
    col: s2dc::Entity,
    platform: bool,
}
fn viewport_center_offset(rw_size: Vector2u, rt_size: Vector2u, scale: u8) -> ScreenVec {
    loop {}
}
