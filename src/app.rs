use std::fmt::{self};

use egui_sfml::{egui, SfEgui};
use gamedebug_core::imm_dbg;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key},
};

use crate::{
    game::GameState,
    graphics,
    input::KbInput,
    math::{wp_to_tp, WorldPos},
    res::Res,
};

/// Application level state (includes game and ui state, etc.)
pub struct App {
    pub rw: RenderWindow,
    pub should_quit: bool,
    pub game: GameState,
    pub res: Res,
    pub sf_egui: SfEgui,
    pub kb_input: KbInput,
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
            kb_input: KbInput::default(),
        })
    }

    pub fn do_game_loop(&mut self) {
        while !self.should_quit {
            self.do_event_handling();
            self.do_update();
            self.do_rendering();
            self.kb_input.clear_pressed();
            gamedebug_core::inc_frame();
        }
    }

    fn do_event_handling(&mut self) {
        while let Some(ev) = self.rw.poll_event() {
            self.sf_egui.add_event(&ev);
            self.kb_input.update_from_event(&ev);
            match ev {
                Event::Closed => self.should_quit = true,
                _ => {}
            }
        }
    }

    fn do_update(&mut self) {
        let spd = if self.kb_input.down(Key::LShift) {
            100
        } else if self.kb_input.down(Key::LControl) {
            1000
        } else {
            2
        };
        if self.kb_input.down(Key::Left) {
            self.game.camera_offset.x -= spd;
        }
        if self.kb_input.down(Key::Right) {
            self.game.camera_offset.x += spd;
        }
        if self.kb_input.down(Key::Up) {
            self.game.camera_offset.y -= spd;
        }
        if self.kb_input.down(Key::Down) {
            self.game.camera_offset.y += spd;
        }
        let tp = self.game.camera_offset.tile_pos();
        imm_dbg!(tp);
        imm_dbg!(tp.to_chunk_and_local());
    }

    fn do_rendering(&mut self) {
        self.rw.clear(Color::BLACK);
        self.game.draw_world(&mut self.rw, &self.res);
        self.sf_egui
            .do_frame(|ctx| {
                egui::Window::new("Debug").show(ctx, |ui| {
                    ui.label("Cam x");
                    ui.add(egui::DragValue::new(&mut self.game.camera_offset.x));
                    ui.label("Cam y");
                    ui.add(egui::DragValue::new(&mut self.game.camera_offset.y));
                    let tile_off = self.game.camera_offset.tile_pos();
                    ui.label(format!(
                        "Depth: {}",
                        LengthDisp(tile_off.y as i64 - wp_to_tp(WorldPos::SURFACE) as i64)
                    ));
                    ui.label(format!(
                        "Offset from center: {}",
                        LengthDisp(tile_off.x as i64 - wp_to_tp(WorldPos::CENTER) as i64)
                    ));
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
            })
            .unwrap();
        self.sf_egui.draw(&mut self.rw, None);
        self.rw.display();
    }
}

struct LengthDisp(i64);

impl fmt::Display for LengthDisp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let km = self.0 / 1000;
        let m = self.0 % 1000;
        write!(f, "{km} km, {m} m")
    }
}
