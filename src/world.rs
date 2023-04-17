mod reg_chunk_existence;
mod serialization;
use std::{fmt::Debug, fs::File, io::Seek, path::{Path, PathBuf}};
use egui_inspect::derive::Inspect;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};
use crate::{
    math::WorldPos, player::Player, tiles::{BgTileId, FgTileId, MidTileId, TileId},
    world::reg_chunk_existence::ExistenceBitset, worldgen::Worldgen,
};
use self::serialization::save_chunk;
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Inspect)]
pub(crate) struct ChunkPos {}
#[derive(Debug, Inspect)]
pub(crate) struct World {
    pub(crate) player: Player,
}
impl World {
    pub(crate) fn new(spawn_point: WorldPos, name: &str, path: PathBuf) -> Self {
        loop {}
    }
    /// Get mutable access to the tile at `pos`.
    ///
    /// Loads or generates the containing chunk if necessary.
    pub(crate) fn tile_at_mut(
        &mut self,
        pos: TilePos,
        worldgen: &Worldgen,
    ) -> &mut Tile {
        loop {}
    }
    pub(crate) fn save(&self) {
        loop {}
    }
    pub(crate) fn save_meta(&self) {
        loop {}
    }
    pub(crate) fn save_chunks(&self) {
        loop {}
    }
}
fn loc_byte_idx_xy(x: u8, y: u8) -> usize {
    loop {}
}
fn loc_byte_idx(loc_idx: u8) -> usize {
    loop {}
}
fn loc_idx(loc_y: u8, loc_x: u8) -> u8 {
    loop {}
}
fn format_reg_file_name((x, y): (u8, u8)) -> String {
    loop {}
}
const CHUNK_BYTES: usize = CHUNK_N_TILES * TILE_BYTES;
const TILE_BYTES: usize = 3 * 2;
#[derive(Debug, Clone, Copy)]
pub(crate) struct TilePos {}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct ChunkLocalTilePos {}
/// Chunk-local tile position scalar. Supports up to 256 tiles per chunk.
type ChkLocalTPosSc = u8;
pub(crate) type TPosSc = u32;
pub(crate) const CHUNK_EXTENT: u16 = 128;
const CHUNK_N_TILES: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;
type ChunkTiles = [Tile; CHUNK_N_TILES];
fn default_chunk_tiles() -> ChunkTiles {
    loop {}
}
#[derive(Debug, Inspect)]
pub(crate) struct Chunk {}
#[derive(Clone, Copy, Debug, Inspect)]
pub(crate) struct Tile {}
pub(crate) const REGION_CHUNK_EXTENT: u8 = 8;
pub(crate) const REGION_N_CHUNKS: u8 = REGION_CHUNK_EXTENT * REGION_CHUNK_EXTENT;
/// This is the uncompressed byte length of a region
pub(crate) const REGION_BYTES: usize = REGION_N_CHUNKS as usize * CHUNK_BYTES;
