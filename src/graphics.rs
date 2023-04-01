use sfml::{
    graphics::RenderWindow,
    window::{ContextSettings, Style, VideoMode},
};

struct ScreenRes {
    w: u16,
    h: u16,
}

impl ScreenRes {
    fn to_sf(&self) -> VideoMode {
        VideoMode {
            width: self.w.into(),
            height: self.h.into(),
            bits_per_pixel: 32,
        }
    }
}

const NATIVE_RESOLUTION: ScreenRes = ScreenRes { w: 640, h: 360 };

pub fn make_window() -> RenderWindow {
    let mut rw = RenderWindow::new(
        NATIVE_RESOLUTION.to_sf(),
        "Mantle Diver",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_framerate_limit(60);
    rw
}
