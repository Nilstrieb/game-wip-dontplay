use sfml::{
    graphics::RenderWindow,
    system::Vector2f,
    window::{ContextSettings, Style, VideoMode},
};
use sfml_xt::graphics::RenderWindowExt;

pub struct ScreenRes {
    pub w: i16,
    pub h: i16,
}

impl ScreenRes {
    fn to_sf(&self) -> VideoMode {
        VideoMode {
            width: self.w as _,
            height: self.h as _,
            bits_per_pixel: 32,
        }
    }
}

// We assume this game won't be played above 32767*32767 resolution
pub struct ScreenPos {
    pub x: i16,
    pub y: i16,
}

impl ScreenPos {
    pub fn to_sf_vec(&self) -> Vector2f {
        Vector2f::new(self.x.into(), self.y.into())
    }
}

pub const NATIVE_RESOLUTION: ScreenRes = ScreenRes { w: 960, h: 540 };

pub fn make_window() -> RenderWindow {
    let mut rw = RenderWindow::new(
        NATIVE_RESOLUTION.to_sf(),
        "Mantle Diver",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    rw.set_framerate_limit(60);
    rw.center();
    rw
}
