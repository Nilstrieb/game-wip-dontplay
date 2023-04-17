use egui_inspect::derive::Inspect;
use crate::{math::IntRect, tiles::{BgTileId, FgTileId, MidTileId}};
/// We won't have more than 65535 different items
pub type ItemId = u16;
/// We won't have more than 65535 item quantity in a single slot
pub type ItemQty = u16;
/// Inventory slot
#[derive(Debug, Inspect)]
pub struct Slot {
    pub id: ItemId,
    pub qty: ItemQty,
}
#[derive(Debug, Inspect)]
pub struct Inventory {
    pub slots: Vec<Slot>,
}
impl Inventory {
    /// A new inventory filled with some debug items
    pub(crate) fn new_debug() -> Inventory {
        loop {}
    }
}
#[derive(Debug, Inspect)]
pub struct ItemDef {
    pub name: String,
    pub graphic_name: String,
    pub tex_rect: IntRect,
    #[opaque]
    pub use_action: UseAction,
    pub consumable: bool,
}
#[derive(Debug, Inspect, PartialEq)]
pub enum TileLayer {
    Bg,
    Mid,
    Fg,
}
#[derive(Debug)]
pub enum UseAction {
    PlaceBgTile { id: BgTileId },
    PlaceMidTile { id: MidTileId },
    PlaceFgTile { id: FgTileId },
    RemoveTile { layer: TileLayer },
}
#[derive(Debug, Inspect)]
pub struct ItemDb {
    pub db: Vec<ItemDef>,
}
impl Default for ItemDb {
    fn default() -> Self {
        loop {}
    }
}
pub mod items {
    use super::ItemId;
    pub const DIRT_BLOCK: ItemId = 0;
    pub const TORCH: ItemId = 1;
    pub const PLATFORM: ItemId = 2;
    pub const WOOD_PICK: ItemId = 3;
    pub const PANZERIUM: ItemId = 4;
    pub const STONE_WALL: ItemId = 5;
}
