use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};
use sfml::{
    graphics::RenderWindow, system::Vector2f, window::{ContextSettings, Style, VideoMode},
};
use sfml_xt::graphics::RenderWindowExt;
use crate::math::FPS_TARGET;
pub struct ScreenRes {
    pub w: u16,
    pub h: u16,
}
impl ScreenRes {
    fn to_sf(&self) -> VideoMode {
        loop {}
    }
}
#[derive(Default, Clone, Copy, Debug, Inspect, Serialize, Deserialize)]
pub struct ScreenVec {
    pub x: ScreenSc,
    pub y: ScreenSc,
}
/// Screen position/offset scalar
/// We assume this game won't be played above 32767*32767 resolution
pub type ScreenSc = i16;
impl ScreenVec {
    pub fn to_sf_vec(self) -> Vector2f {
        loop {}
    }
}
const DEFAULT_RES: ScreenRes = ScreenRes { w: 960, h: 540 };
pub fn make_window() -> RenderWindow {
    loop {}
}
