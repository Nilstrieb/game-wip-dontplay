use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::{derive::Inspect, Inspect};

#[derive(Inspect)]
pub struct TileDef {
    pub layer: (),
    //ADD pub blend_graphic: String,
}

impl Debug for TileDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

// this is actually used
#[derive(Inspect)]
pub struct TileDb {
    unknown_bg: TileDef,
}

impl Debug for TileDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
