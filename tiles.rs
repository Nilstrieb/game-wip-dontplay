use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::{derive::Inspect, Inspect};
use serde::{Deserialize, Serialize};

#[derive(Inspect)]
pub struct TileDef
where
{
    pub layer: (),
    //ADD pub blend_graphic: String,
}

impl Debug for TileDef
where
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

impl Default for TileDef<>
where
{
    fn default() -> Self {
        loop {}
    }
}


// this is actually used
#[derive(Debug, Inspect)]
pub struct TileDb {
    unknown_bg: TileDef,
}
