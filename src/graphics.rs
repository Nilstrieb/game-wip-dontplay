use egui_inspect::derive::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Inspect, Serialize, Deserialize)]
pub(crate) struct ScreenVec {}
/// Screen position/offset scalar
/// We assume this game won't be played above 32767*32767 resolution
pub(crate) type ScreenSc = i16;
