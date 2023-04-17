use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};
use sfml::{
    graphics::RenderWindow, system::Vector2f, window::{ContextSettings, Style, VideoMode},
};
use sfml_xt::graphics::RenderWindowExt;
use crate::math::FPS_TARGET;
pub(crate) struct ScreenRes {}
#[derive(Default, Clone, Copy, Debug, Inspect, Serialize, Deserialize)]
pub struct ScreenVec {
    pub(crate) x: ScreenSc,
    pub(crate) y: ScreenSc,
}
/// Screen position/offset scalar
/// We assume this game won't be played above 32767*32767 resolution
pub(crate) type ScreenSc = i16;
