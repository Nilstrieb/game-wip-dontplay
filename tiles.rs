use std::{fmt::Debug, marker::PhantomData};

use egui_inspect::{derive::Inspect, Inspect};
use serde::{Deserialize, Serialize};

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
        loop {}
    }
}

#[derive(Debug)]
pub enum Bg {}
#[derive(Debug)]
pub enum Mid {}
#[derive(Debug)]
pub enum Fg {}

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
