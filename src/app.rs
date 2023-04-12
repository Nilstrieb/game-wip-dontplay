use anyhow::Context;
use egui_sfml::SfEgui;
use gamedebug_core::{imm, imm_dbg};
use sfml::{
    audio::SoundSource,
    graphics::{
        BlendMode, Color, Rect, RenderStates, RenderTarget, RenderTexture, RenderWindow, Sprite,
        Transformable, View,
    },
    system::Vector2u,
    window::{Event, Key},
};

use crate::{
    debug::{self, DebugState},
    game::{for_each_tile_on_screen, Biome, GameState},
    graphics::{self, ScreenSc, ScreenVec},
    input::Input,
    math::{center_offset, TILE_SIZE},
    res::Res,
    CliArgs,
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
}

impl App {
    pub fn new(args: CliArgs) -> anyhow::Result<Self> {
        let rw = graphics::make_window();
        let sf_egui = SfEgui::new(&rw);
        let mut res = Res::load()?;
        res.surf_music.set_looping(true);
        res.surf_music.set_volume(10.0);
        res.surf_music.play();
        let rw_size = rw.size();
        let rt =
            RenderTexture::new(rw_size.x, rw_size.y).context("Failed to create render texture")?;
        let light_map = RenderTexture::new(rw_size.x, rw_size.y)
            .context("Failed to create lightmap texture")?;
        Ok(Self {
            rw,
            should_quit: false,
            game: GameState::new(args.world_name),
            res,
            sf_egui,
            input: Input::default(),
            debug: DebugState::default(),
            scale: 1,
            rt,
            light_map,
        })
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
        while let Some(ev) = self.rw.poll_event() {
            self.sf_egui.add_event(&ev);
            self.input.update_from_event(&ev);
            match ev {
                Event::Closed => self.should_quit = true,
                Event::Resized { width, height } => {
                    self.rt =
                        RenderTexture::new(width / self.scale as u32, height / self.scale as u32)
                            .unwrap();
                    self.light_map =
                        RenderTexture::new(width / self.scale as u32, height / self.scale as u32)
                            .unwrap();
                    let view = View::from_rect(Rect::new(0., 0., width as f32, height as f32));
                    self.rw.set_view(&view);
                }
                _ => {}
            }
        }
    }

    fn do_update(&mut self) {
        self.debug.update(&self.input);
        let rt_size = self.rt.size();
        if self.debug.freecam {
            self.do_freecam();
        } else {
            let spd = if self.input.down(Key::LShift) {
                8.0
            } else if self.input.down(Key::LControl) {
                128.0
            } else {
                3.0
            };
            self.game.world.player.hspeed = 0.;
            if self.input.down(Key::A) {
                self.game.world.player.hspeed = -spd;
            }
            if self.input.down(Key::D) {
                self.game.world.player.hspeed = spd;
            }
            if self.input.down(Key::W) && self.game.world.player.can_jump() {
                self.game.world.player.vspeed = -10.0;
                self.game.world.player.jumps_left = 0;
            }
            self.game.world.player.down_intent = self.input.down(Key::S);
            let terminal_velocity = 60.0;
            self.game.world.player.vspeed = self
                .game
                .world
                .player
                .vspeed
                .clamp(-terminal_velocity, terminal_velocity);
            let mut on_screen_tile_ents = Vec::new();
            for_each_tile_on_screen(self.game.camera_offset, self.rt.size(), |tp, _sp| {
                let tid = self.game.world.tile_at_mut(tp, &self.game.worldgen).mid;
                let tdef = &self.game.tile_db[tid];
                let Some(bb) = tdef.bb else {
                    return;
                };
                let x = tp.x as i32 * TILE_SIZE as i32;
                let y = tp.y as i32 * TILE_SIZE as i32;
                let en = s2dc::Entity::from_rect_corners(
                    x + bb.x as i32,
                    y + bb.y as i32,
                    x + bb.w as i32,
                    y + bb.h as i32,
                );
                on_screen_tile_ents.push(TileColEn {
                    col: en,
                    platform: tdef.platform,
                });
            });
            imm_dbg!(on_screen_tile_ents.len());
            self.game.world.player.col_en.move_y(
                self.game.world.player.vspeed,
                |player_en, off| {
                    let mut col = false;
                    for en in &on_screen_tile_ents {
                        if player_en.would_collide(&en.col, off) {
                            if en.platform {
                                if self.game.world.player.vspeed < 0. {
                                    continue;
                                }
                                // If the player's feet are below the top of the platform,
                                // collision shouldn't happen
                                let player_feet = player_en.pos.y + player_en.bb.y;
                                if player_feet > en.col.pos.y || self.game.world.player.down_intent
                                {
                                    continue;
                                }
                            }
                            col = true;
                            if self.game.world.player.vspeed > 0. {
                                self.game.world.player.jumps_left = 1;
                            }
                            self.game.world.player.vspeed = 0.;
                        }
                    }
                    col
                },
            );
            self.game.world.player.col_en.move_x(
                self.game.world.player.hspeed,
                |player_en, off| {
                    let mut col = false;
                    for en in &on_screen_tile_ents {
                        if en.platform {
                            continue;
                        }
                        if player_en.would_collide(&en.col, off) {
                            col = true;
                            self.game.world.player.hspeed = 0.;
                        }
                    }
                    col
                },
            );
            self.game.world.player.vspeed += self.game.gravity;
            let (x, y, _w, _h) = self.game.world.player.col_en.en.xywh();
            self.game.camera_offset.x = (x - rt_size.x as i32 / 2).try_into().unwrap_or(0);
            self.game.camera_offset.y = (y - rt_size.y as i32 / 2).try_into().unwrap_or(0);
        }
        let mut loc = self.input.mouse_down_loc;
        let vco = viewport_center_offset(self.rw.size(), rt_size, self.scale);
        loc.x -= vco.x;
        loc.y -= vco.y;
        loc.x /= self.scale as ScreenSc;
        loc.y /= self.scale as ScreenSc;
        let mut wpos = self.game.camera_offset;
        wpos.x = wpos.x.saturating_add_signed(loc.x.into());
        wpos.y = wpos.y.saturating_add_signed(loc.y.into());
        let mouse_tpos = wpos.tile_pos();
        imm!(
            "Mouse @ tile {}, {} ({:?})",
            mouse_tpos.x,
            mouse_tpos.y,
            self.game.world.tile_at_mut(mouse_tpos, &self.game.worldgen)
        );
        let m_chk = mouse_tpos.to_chunk();
        imm!("@ chunk {}, {}", m_chk.x, m_chk.y);
        let (m_chk_x, m_chk_y) = m_chk.region();
        imm!("@ region {m_chk_x}, {m_chk_y}");
        if self.debug.freecam && self.input.pressed(Key::P) {
            self.game.world.player.col_en.en.pos.x = wpos.x as i32;
            self.game.world.player.col_en.en.pos.y = wpos.y as i32;
        }
        if self.input.lmb_down {
            let t = self.game.world.tile_at_mut(mouse_tpos, &self.game.worldgen);
            t.mid = 0;
            t.fg = 0;
        } else if self.input.rmb_down {
            let t = self.game.world.tile_at_mut(mouse_tpos, &self.game.worldgen);
            if self.game.tile_to_place != 7 {
                t.mid = self.game.tile_to_place;
            } else {
                t.bg = self.game.tile_to_place;
            }
        }
        if self.game.camera_offset.y > 643_000 {
            self.game.current_biome = Biome::Underground;
        } else {
            self.game.current_biome = Biome::Surface;
        }
        if self.game.current_biome != self.game.prev_biome {
            self.game.prev_biome = self.game.current_biome;
            match self.game.current_biome {
                Biome::Surface => {
                    self.res.und_music.stop();
                    self.res.surf_music.play();
                }
                Biome::Underground => {
                    self.res.surf_music.stop();
                    self.res.und_music.set_volume(self.res.surf_music.volume());
                    self.res.und_music.set_looping(true);
                    self.res.und_music.play();
                }
            }
        }
        self.game.update();
    }

    fn do_freecam(&mut self) {
        let spd = if self.input.down(Key::LShift) {
            100
        } else if self.input.down(Key::LControl) {
            1000
        } else {
            2
        };
        if self.input.down(Key::A) {
            self.game.camera_offset.x = self.game.camera_offset.x.saturating_sub(spd);
        }
        if self.input.down(Key::D) {
            self.game.camera_offset.x = self.game.camera_offset.x.saturating_add(spd);
        }
        if self.input.down(Key::W) {
            self.game.camera_offset.y = self.game.camera_offset.y.saturating_sub(spd);
        }
        if self.input.down(Key::S) {
            self.game.camera_offset.y = self.game.camera_offset.y.saturating_add(spd);
        }
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
        // Draw light overlay with multiply blending
        let mut rst = RenderStates::default();
        rst.blend_mode = BlendMode::MULTIPLY;
        self.light_map.display();
        spr.set_texture(self.light_map.texture(), false);
        self.rw.draw_with_renderstates(&spr, &rst);
        self.sf_egui
            .do_frame(|ctx| {
                debug::do_debug_ui(
                    ctx,
                    &mut self.debug,
                    &mut self.game,
                    &mut self.res,
                    &mut self.scale,
                );
            })
            .unwrap();
        self.sf_egui.draw(&mut self.rw, None);
        self.rw.display();
    }
}

/// Tile collision entity for doing physics
struct TileColEn {
    col: s2dc::Entity,
    platform: bool,
}

fn viewport_center_offset(rw_size: Vector2u, rt_size: Vector2u, scale: u8) -> ScreenVec {
    let rw_size = rw_size;
    let rt_size = rt_size * scale as u32;
    let x = center_offset(rt_size.x as i32, rw_size.x as i32);
    let y = center_offset(rt_size.y as i32, rw_size.y as i32);
    ScreenVec {
        x: x as ScreenSc,
        y: y as ScreenSc,
    }
}
