use egui_inspect::derive::Inspect;

use crate::{math::IntRect, world::TileId};

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
        Self {
            slots: vec![
                Slot {
                    id: items::WOOD_PICK,
                    qty: 1,
                },
                Slot {
                    id: items::DIRT_BLOCK,
                    qty: 100,
                },
                Slot {
                    id: items::TORCH,
                    qty: 100,
                },
                Slot {
                    id: items::PLATFORM,
                    qty: 100,
                },
            ],
        }
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
    PlaceTile { layer: TileLayer, id: TileId },
    RemoveTile { layer: TileLayer },
}

#[derive(Debug, Inspect)]
pub struct ItemDb {
    pub db: Vec<ItemDef>,
}

impl Default for ItemDb {
    fn default() -> Self {
        Self {
            db: vec![
                ItemDef {
                    name: String::from("Dirt Block"),
                    graphic_name: String::from("tiles/dirt"),
                    tex_rect: IntRect::default(),
                    use_action: UseAction::PlaceTile {
                        layer: TileLayer::Mid,
                        id: 3,
                    },
                    consumable: true,
                },
                ItemDef {
                    name: String::from("Torch"),
                    graphic_name: String::from("tiles/torch"),
                    tex_rect: IntRect::default(),
                    use_action: UseAction::PlaceTile {
                        layer: TileLayer::Mid,
                        id: 4,
                    },
                    consumable: true,
                },
                ItemDef {
                    name: String::from("Platform"),
                    graphic_name: String::from("tiles/platform"),
                    tex_rect: IntRect::default(),
                    use_action: UseAction::PlaceTile {
                        layer: TileLayer::Mid,
                        id: 5,
                    },
                    consumable: true,
                },
                ItemDef {
                    name: String::from("Wood Pick"),
                    graphic_name: String::from("items/woodpick"),
                    tex_rect: IntRect::default(),
                    use_action: UseAction::RemoveTile {
                        layer: TileLayer::Mid,
                    },
                    consumable: true,
                },
                ItemDef {
                    name: String::from("Panzerium"),
                    graphic_name: String::from("tiles/panzerium"),
                    tex_rect: IntRect::default(),
                    use_action: UseAction::PlaceTile {
                        layer: TileLayer::Mid,
                        id: 6,
                    },
                    consumable: true,
                },
            ],
        }
    }
}

pub mod items {
    use super::ItemId;

    pub const DIRT_BLOCK: ItemId = 0;
    pub const TORCH: ItemId = 1;
    pub const PLATFORM: ItemId = 2;
    pub const WOOD_PICK: ItemId = 3;
}
