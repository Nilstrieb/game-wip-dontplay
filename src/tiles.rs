pub mod tiledb_edit_ui;

use std::{fmt::Debug, marker::PhantomData, ops::Index};

use egui_inspect::{derive::Inspect, Inspect};
use serde::{Deserialize, Serialize};

use crate::{
    graphics::ScreenVec,
    math::{IntRect, TILE_SIZE},
    texture_atlas::RectMap,
};

#[derive(Inspect, PartialEq, Eq)]
pub struct TileId<Layer>(pub u16, PhantomData<Layer>);

impl<Layer> Copy for TileId<Layer> {}
impl<Layer> Clone for TileId<Layer> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<Layer> Debug for TileId<Layer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TileId").field(&self.0).finish()
    }
}

impl<Layer> TileId<Layer> {
    pub fn empty(&self) -> bool {
        self.0 == 0
    }
    pub const EMPTY: Self = Self(0, PhantomData);
}

#[derive(Debug)]
pub enum Bg {}
#[derive(Debug)]
pub enum Mid {}
#[derive(Debug)]
pub enum Fg {}

impl Bg {
    pub fn unknown_def() -> TileDef<Self> {
        loop {}
    }
}

impl Mid {
    pub fn unknown_def() -> TileDef<Self> {
        loop {}
    }
}

impl Fg {
    pub fn unknown_def() -> TileDef<Self> {
        loop {}
    }
}

pub trait TileLayer {
    /// Definitions specific to this layer
    type SpecificDef;
}

impl TileLayer for Bg {
    type SpecificDef = ();
}

impl TileLayer for Mid {
    type SpecificDef = MidDef;
}

impl TileLayer for Fg {
    type SpecificDef = ();
}

pub type BgTileId = TileId<Bg>;
pub type MidTileId = TileId<Mid>;
pub type FgTileId = TileId<Fg>;

impl BgTileId {
    pub const DIRT: Self = Self(1, PhantomData);
    pub const STONE: Self = Self(2, PhantomData);
}

impl MidTileId {
    pub const DIRT: Self = Self(1, PhantomData);
    pub const STONE: Self = Self(2, PhantomData);
    pub const TORCH: Self = Self(3, PhantomData);
    pub const PLATFORM: Self = Self(4, PhantomData);
    pub const PANZERIUM: Self = Self(5, PhantomData);
    pub const UNBREAKANIUM: Self = Self(6, PhantomData);
}

impl FgTileId {
    pub const COAL: Self = Self(1, PhantomData);
}

#[derive(Serialize, Deserialize, Inspect)]
pub struct TileDef<Layer: TileLayer>
where
    Layer::SpecificDef: Debug + Inspect,
{
    /// Whether the tile emits light, and the light source offset
    pub light: Option<ScreenVec>,
    pub graphic_name: String,
    pub tex_rect: IntRect,
    pub layer: Layer::SpecificDef,
    //ADD pub blend_graphic: String,
}

#[derive(Debug, Inspect, Default, Serialize, Deserialize)]
pub struct MidDef {
    /// Platform behavior: Horizontally passable, vertically passable upwards
    pub platform: bool,
    /// Collision bounding box
    pub bb: Option<TileBb>,
}

impl<Layer: TileLayer> Debug for TileDef<Layer>
where
    Layer::SpecificDef: Debug + Inspect,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}

impl<Layer: TileLayer> Default for TileDef<Layer>
where
    Layer::SpecificDef: Default + Debug + Inspect,
{
    fn default() -> Self {
        loop {}
    }
}

const DEFAULT_TILE_BB: TileBb = TileBb {
    x: 0,
    y: 0,
    w: TILE_SIZE,
    h: TILE_SIZE,
};

#[derive(Serialize, Deserialize, Debug, Inspect, Clone, Copy)]
pub struct TileBb {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

#[derive(Serialize, Deserialize, Debug, Inspect)]
pub struct TileDb {
    unknown_bg: TileDef<Bg>,
    unknown_mid: TileDef<Mid>,
    unknown_fg: TileDef<Fg>,
    bg: Vec<TileDef<Bg>>,
    mid: Vec<TileDef<Mid>>,
    fg: Vec<TileDef<Fg>>,
}

impl Default for TileDb {
    fn default() -> Self {
        loop {}
    }
}

impl Index<BgTileId> for TileDb {
    type Output = TileDef<Bg>;

    fn index(&self, index: BgTileId) -> &Self::Output {
        loop {}
    }
}

impl Index<MidTileId> for TileDb {
    type Output = TileDef<Mid>;

    fn index(&self, index: MidTileId) -> &Self::Output {
        loop {}
    }
}

impl Index<FgTileId> for TileDb {
    type Output = TileDef<Fg>;

    fn index(&self, index: FgTileId) -> &Self::Output {
        loop {}
    }
}

const PATH: &str = "tiles.dat";

impl TileDb {
    pub fn load_or_default() -> Self {
        loop {}
    }
    pub fn try_save(&self) {
        loop {}
    }

    pub(crate) fn update_rects(&mut self, rects: &RectMap) {
        loop {}
    }
}

fn update_rect_db<Layer: TileLayer>(db: &mut Vec<TileDef<Layer>>, rects: &RectMap)
where
    Layer::SpecificDef: Debug + Inspect,
{
    loop {}
}

fn update_rect_def<Layer: TileLayer>(
    def: &mut TileDef<Layer>,
    rects: &std::collections::HashMap<String, IntRect>,
) where
    Layer::SpecificDef: Debug + Inspect,
{
    loop {}
}
