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

#[derive(Serialize, Deserialize, Inspect)]
pub struct TileDef
where
{
    /// Whether the tile emits light, and the light source offset
    pub layer: <Bg as TileLayer>::SpecificDef,
    //ADD pub blend_graphic: String,
}

#[derive(Debug, Inspect, Default, Serialize, Deserialize)]
pub struct MidDef {
    /// Platform behavior: Horizontally passable, vertically passable upwards
    pub platform: bool,
    /// Collision bounding box
    pub bb: Option<TileBb>,
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

#[derive(Serialize, Deserialize, Debug, Inspect, Clone, Copy)]
pub struct TileBb {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

// this is actually used
#[derive(Serialize, Deserialize, Debug, Inspect)]
pub struct TileDb {
    unknown_bg: TileDef,
}
