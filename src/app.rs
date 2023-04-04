use std::fmt::{self};

use egui_sfml::{egui, SfEgui};
use gamedebug_core::{imm, imm_dbg};
use sfml::{
    audio::SoundSource,
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key},
};

use crate::{
    debug::DebugState,
    game::{for_each_tile_on_screen, Biome, GameState},
    graphics::{self, NATIVE_RESOLUTION},
    input::Input,
    math::{px_per_frame_to_km_h, wp_to_tp, WorldPos, M_PER_PX, TILE_SIZE},
    res::Res,
    world::Tile,
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
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let rw = graphics::make_window();
        let sf_egui = SfEgui::new(&rw);
        let mut res = Res::load()?;
        res.surf_music.set_looping(true);
        res.surf_music.set_volume(10.0);
        res.surf_music.play();
        Ok(Self {
            rw,
            should_quit: false,
            game: GameState::default(),
            res,
            sf_egui,
            input: Input::default(),
            debug: DebugState::default(),
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
    }

    fn do_event_handling(&mut self) {
        while let Some(ev) = self.rw.poll_event() {
            self.sf_egui.add_event(&ev);
            self.input.update_from_event(&ev);
            match ev {
                Event::Closed => self.should_quit = true,
                _ => {}
            }
        }
    }

    fn do_update(&mut self) {
        self.debug.update(&self.input);
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
            self.game.player.hspeed = 0.;
            if self.input.down(Key::A) {
                self.game.player.hspeed = -spd;
            }
            if self.input.down(Key::D) {
                self.game.player.hspeed = spd;
            }
            if self.input.down(Key::W) && self.game.player.can_jump() {
                self.game.player.vspeed = -10.0;
                self.game.player.jumps_left = 0;
            }
            let terminal_velocity = 60.0;
            self.game.player.vspeed = self
                .game
                .player
                .vspeed
                .clamp(-terminal_velocity, terminal_velocity);
            let mut on_screen_tile_ents = Vec::new();
            for_each_tile_on_screen(self.game.camera_offset, |tp, _sp| {
                let tid = self.game.world.tile_at_mut(tp).mid;
                if tid == Tile::EMPTY {
                    return;
                }
                let tsize = TILE_SIZE as i32;
                let x = tp.x as i32 * TILE_SIZE as i32;
                let y = tp.y as i32 * TILE_SIZE as i32;
                let en = s2dc::Entity::from_rect_corners(x, y, x + tsize, y + tsize);
                on_screen_tile_ents.push(en);
            });
            imm_dbg!(on_screen_tile_ents.len());
            self.game
                .player
                .col_en
                .move_y(self.game.player.vspeed, |player_en, off| {
                    let mut col = false;
                    for en in &on_screen_tile_ents {
                        if player_en.would_collide(en, off) {
                            col = true;
                            if self.game.player.vspeed > 0. {
                                self.game.player.jumps_left = 1;
                            }
                            self.game.player.vspeed = 0.;
                        }
                    }
                    col
                });
            self.game
                .player
                .col_en
                .move_x(self.game.player.hspeed, |player_en, off| {
                    let mut col = false;
                    for en in &on_screen_tile_ents {
                        if player_en.would_collide(en, off) {
                            col = true;
                            self.game.player.hspeed = 0.;
                        }
                    }
                    col
                });
            self.game.player.vspeed += self.game.gravity;
            let (x, y, _w, _h) = self.game.player.col_en.en.xywh();
            self.game.camera_offset.x = (x - NATIVE_RESOLUTION.w as i32 / 2) as u32;
            self.game.camera_offset.y = (y - NATIVE_RESOLUTION.h as i32 / 2) as u32;
        }
        let loc = self.input.mouse_down_loc;
        let mut wpos = self.game.camera_offset;
        wpos.x = wpos.x.saturating_add_signed(loc.x.into());
        wpos.y = wpos.y.saturating_add_signed(loc.y.into());
        let mouse_tpos = wpos.tile_pos();
        imm!("Mouse @ tile {}, {}", mouse_tpos.x, mouse_tpos.y);
        if self.debug.freecam && self.input.pressed(Key::P) {
            self.game.player.col_en.en.pos.x = wpos.x as i32;
            self.game.player.col_en.en.pos.y = wpos.y as i32;
        }
        if self.input.lmb_down {
            let t = self.game.world.tile_at_mut(mouse_tpos);
            t.mid = 0;
            t.fg = 0;
        } else if self.input.rmb_down {
            let t = self.game.world.tile_at_mut(mouse_tpos);
            if self.game.tile_to_place != 7 {
                t.mid = self.game.tile_to_place;
            } else {
                t.bg = self.game.tile_to_place;
            }
        }
        if self.game.camera_offset.y > 163800 {
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
        self.rw.clear(Color::rgb(55, 221, 231));
        self.game.draw_world(&mut self.rw, &self.res);
        self.game.draw_entities(&mut self.rw);
        self.sf_egui
            .do_frame(|ctx| {
                if self.debug.panel {
                    debug_panel_ui(&mut self.debug, &mut self.game, ctx, &mut self.res);
                }
            })
            .unwrap();
        self.sf_egui.draw(&mut self.rw, None);
        self.rw.display();
    }
}

fn debug_panel_ui(
    debug: &mut DebugState,
    game: &mut GameState,
    ctx: &egui::Context,
    res: &mut Res,
) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        if debug.freecam {
            ui.label("Cam x");
            ui.add(egui::DragValue::new(&mut game.camera_offset.x));
            ui.label("Cam y");
            ui.add(egui::DragValue::new(&mut game.camera_offset.y));
            let co = game.camera_offset;
            ui.label(format!(
                "Cam Depth: {}",
                LengthDisp(co.y as f32 - WorldPos::SURFACE as f32)
            ));
            ui.label(format!(
                "Cam offset from center: {}",
                LengthDisp(co.x as f32 - WorldPos::CENTER as f32)
            ));
        } else {
            ui.label("Player x");
            ui.add(egui::DragValue::new(&mut game.player.col_en.en.pos.x));
            ui.label("Player y");
            ui.add(egui::DragValue::new(&mut game.player.col_en.en.pos.y));
            let tp = game.player.center_tp();
            imm_dbg!(tp);
            ui.label(format!(
                "Player Depth: {}",
                LengthDisp(game.player.feet_y() as f32 - WorldPos::SURFACE as f32)
            ));
            ui.label(format!(
                "Player offset from center: {}",
                LengthDisp(game.player.col_en.en.pos.x as f32 - WorldPos::CENTER as f32)
            ));
            ui.label(format!(
                "Hspeed: {} ({} km/h)",
                game.player.hspeed,
                px_per_frame_to_km_h(game.player.hspeed)
            ));
            ui.label(format!(
                "Vspeed: {} ({} km/h)",
                game.player.vspeed,
                px_per_frame_to_km_h(game.player.vspeed)
            ));
            ui.label("Gravity");
            ui.add(egui::DragValue::new(&mut game.gravity));
        }
        ui.label("Tile to place");
        ui.add(egui::DragValue::new(&mut game.tile_to_place));
        ui.label("Music volume");
        let mut vol = res.surf_music.volume();
        ui.add(egui::DragValue::new(&mut vol));
        res.surf_music.set_volume(vol);
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            gamedebug_core::for_each_imm(|info| match info {
                gamedebug_core::Info::Msg(msg) => {
                    ui.label(msg);
                }
                gamedebug_core::Info::Rect(_, _, _, _, _) => todo!(),
            });
        });
        gamedebug_core::clear_immediates();
    });
}

struct LengthDisp(f32);

impl fmt::Display for LengthDisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meters = self.0 * M_PER_PX;
        if meters.abs() > 1000. {
            let km = (meters / 1000.).floor();
            let m = meters % 1000.;
            write!(f, "{km} km, {m} m")
        } else {
            write!(f, "{meters} m")
        }
    }
}
