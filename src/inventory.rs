use egui_inspect::derive::Inspect;
use crate::{math::IntRect, tiles::{BgTileId, FgTileId, MidTileId}};
/// We won't have more than 65535 different items
pub(crate) type ItemId = u16;
/// Inventory slot
#[derive(Debug, Inspect)]
pub(crate) struct Slot {}
#[derive(Debug, Inspect)]
pub(crate) struct Inventory {}
#[derive(Debug, Inspect, PartialEq)]
pub(crate) enum TileLayer {
    Bg,
    Mid,
    Fg,
}
#[derive(Debug)]
pub(crate) enum UseAction {
    PlaceBgTile {},
    PlaceMidTile {},
    PlaceFgTile {},
    RemoveTile {},
}
#[derive(Debug, Inspect)]
pub(crate) struct ItemDb {}
