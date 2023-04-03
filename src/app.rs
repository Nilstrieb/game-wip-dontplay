use std::fmt::{self};

use egui_sfml::{egui, SfEgui};
use gamedebug_core::{imm, imm_dbg};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key},
};

use crate::{
    debug::DebugState,
    game::{for_each_tile_on_screen, GameState},
    graphics::{self, NATIVE_RESOLUTION},
    input::Input,
    math::{wp_to_tp, WorldPos, TILE_SIZE},
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
        Ok(Self {
            rw,
            should_quit: false,
            game: GameState::default(),
            res: Res::load()?,
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
                16.0
            } else if self.input.down(Key::LControl) {
                256.0
            } else {
                4.0
            };
            self.game.player.hspeed = 0.;
            if self.input.down(Key::Left) {
                self.game.player.hspeed = -spd;
            }
            if self.input.down(Key::Right) {
                self.game.player.hspeed = spd;
            }
            if self.input.pressed(Key::Up) {
                self.game.player.vspeed = -14.0;
            }
            self.game.player.vspeed = self.game.player.vspeed.clamp(-80., 80.);
            self.game
                .player
                .col_en
                .move_y(self.game.player.vspeed, |player_en, off| {
                    let mut col = false;
                    for_each_tile_on_screen(self.game.camera_offset, |tp, _sp| {
                        let tid = self.game.world.tile_at_mut(tp).id;
                        if tid == Tile::AIR {
                            return;
                        }
                        let tsize = TILE_SIZE as i32;
                        let x = tp.x as i32 * TILE_SIZE as i32;
                        let y = tp.y as i32 * TILE_SIZE as i32;
                        let en = s2dc::Entity::from_rect_corners(x, y, x + tsize, y + tsize);
                        if player_en.would_collide(&en, off) {
                            col = true;
                            self.game.player.vspeed = 0.;
                        }
                    });
                    col
                });
            self.game
                .player
                .col_en
                .move_x(self.game.player.hspeed, |player_en, off| {
                    let mut col = false;
                    for_each_tile_on_screen(self.game.camera_offset, |tp, _sp| {
                        let tid = self.game.world.tile_at_mut(tp).id;
                        if tid == Tile::AIR {
                            return;
                        }
                        let tsize = TILE_SIZE as i32;
                        let x = tp.x as i32 * TILE_SIZE as i32;
                        let y = tp.y as i32 * TILE_SIZE as i32;
                        let en = s2dc::Entity::from_rect_corners(x, y, x + tsize, y + tsize);
                        if player_en.would_collide(&en, off) {
                            col = true;
                            self.game.player.hspeed = 0.;
                        }
                    });
                    col
                });
            self.game.player.vspeed += 1.0;
            let (x, y, _w, _h) = self.game.player.col_en.en.xywh();
            self.game.camera_offset.x = (x - NATIVE_RESOLUTION.w as i32 / 2) as u32;
            self.game.camera_offset.y = (y - NATIVE_RESOLUTION.h as i32 / 2) as u32;
        }
        if self.input.lmb_down {
            let loc = self.input.mouse_down_loc;
            let mut wpos = self.game.camera_offset;
            wpos.x = wpos.x.saturating_add_signed(loc.x.into());
            wpos.y = wpos.y.saturating_add_signed(loc.y.into());
            imm!("Mouse down at {}, {}", wpos.x, wpos.y);
            let tpos = wpos.tile_pos();
            imm_dbg!(tpos);
            self.game.world.tile_at_mut(tpos).id = 0;
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
        if self.input.down(Key::Left) {
            self.game.camera_offset.x = self.game.camera_offset.x.saturating_sub(spd);
        }
        if self.input.down(Key::Right) {
            self.game.camera_offset.x = self.game.camera_offset.x.saturating_add(spd);
        }
        if self.input.down(Key::Up) {
            self.game.camera_offset.y = self.game.camera_offset.y.saturating_sub(spd);
        }
        if self.input.down(Key::Down) {
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
                    debug_panel_ui(&mut self.debug, &mut self.game, ctx);
                }
            })
            .unwrap();
        self.sf_egui.draw(&mut self.rw, None);
        self.rw.display();
    }
}

fn debug_panel_ui(debug: &mut DebugState, game: &mut GameState, ctx: &egui::Context) {
    egui::Window::new("Debug (F12)").show(ctx, |ui| {
        if debug.freecam {
            ui.label("Cam x");
            ui.add(egui::DragValue::new(&mut game.camera_offset.x));
            ui.label("Cam y");
            ui.add(egui::DragValue::new(&mut game.camera_offset.y));
            let tp = game.camera_offset.tile_pos();
            imm_dbg!(tp);
            ui.label(format!(
                "Cam Depth: {}",
                LengthDisp(tp.y as i64 - wp_to_tp(WorldPos::SURFACE) as i64)
            ));
            ui.label(format!(
                "Cam offset from center: {}",
                LengthDisp(tp.x as i64 - wp_to_tp(WorldPos::CENTER) as i64)
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
                LengthDisp(tp.y as i64 - wp_to_tp(WorldPos::SURFACE) as i64)
            ));
            ui.label(format!(
                "Player offset from center: {}",
                LengthDisp(tp.x as i64 - wp_to_tp(WorldPos::CENTER) as i64)
            ));
            ui.label(format!("Vspeed: {}", game.player.vspeed));
        }
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

struct LengthDisp(i64);

impl fmt::Display for LengthDisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let km = self.0 / 1000;
        let m = self.0 % 1000;
        write!(f, "{km} km, {m} m")
    }
}
