use crate::{math::WorldPos, world::World};

pub struct GameState {
    transient: TransientGameState,
    persistent: PersistentGameState,
}

/// Transient game state, not saved to disk
pub struct TransientGameState {
    camera_offset: WorldPos,
}

/// Persistent game state, saved to disk
pub struct PersistentGameState {
    world: World,
}
