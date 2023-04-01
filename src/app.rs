use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::Event,
};

use crate::graphics;

/// Application level state (includes game and ui state, etc.)
pub struct App {
    rw: RenderWindow,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            rw: graphics::make_window(),
            should_quit: false,
        }
    }

    pub fn do_game_loop(&mut self) {
        while !self.should_quit {
            self.do_event_handling();
            self.do_update();
            self.do_rendering();
        }
    }

    fn do_event_handling(&mut self) {
        while let Some(ev) = self.rw.poll_event() {
            match ev {
                Event::Closed => self.should_quit = true,
                _ => {}
            }
        }
    }

    fn do_update(&mut self) {}

    fn do_rendering(&mut self) {
        self.rw.clear(Color::BLACK);
        self.rw.display();
    }
}
