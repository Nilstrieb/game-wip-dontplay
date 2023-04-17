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
pub type ChkPosSc = u16;
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Inspect)]
pub struct ChunkPos {
    pub x: ChkPosSc,
    pub y: ChkPosSc,
}
impl ChunkPos {
    /// Returns the region this chunk position belongs to
    pub fn region(&self) -> (u8, u8) {
        loop {}
    }
    /// Returns the local position in the region (0-7)
    pub fn local(&self) -> (u8, u8) {
        loop {}
    }
}
#[derive(Debug, Inspect)]
pub struct World {
    /// The currently loaded chunks
    chunks: FnvHashMap<ChunkPos, Chunk>,
    /// This is the number of ticks since the world has started.
    /// In other words, the age of the world.
    pub ticks: u64,
    pub player: Player,
    pub name: String,
    #[opaque]
    pub path: PathBuf,
}
impl World {
    pub fn new(spawn_point: WorldPos, name: &str, path: PathBuf) -> Self {
        loop {}
    }
    /// Get mutable access to the tile at `pos`.
    ///
    /// Loads or generates the containing chunk if necessary.
    pub fn tile_at_mut(&mut self, pos: TilePos, worldgen: &Worldgen) -> &mut Tile {
        loop {}
    }
    pub fn save(&self) {
        loop {}
    }
    pub fn save_meta(&self) {
        loop {}
    }
    pub fn save_chunks(&self) {
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
#[derive(Serialize, Deserialize)]
struct WorldMetaSave {
    name: String,
    ticks: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct TilePos {
    pub x: TPosSc,
    pub y: TPosSc,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkLocalTilePos {
    pub x: ChkLocalTPosSc,
    pub y: ChkLocalTPosSc,
}
/// Chunk-local tile position scalar. Supports up to 256 tiles per chunk.
type ChkLocalTPosSc = u8;
impl TilePos {
    pub fn to_chunk_and_local(self) -> (ChunkPos, ChunkLocalTilePos) {
        loop {}
    }
    pub(crate) fn to_chunk(self) -> ChunkPos {
        loop {}
    }
}
fn chk_pos(tile: TPosSc) -> ChkPosSc {
    loop {}
}
#[test]
fn test_chk_pos() {
    loop {}
}
fn chunk_local(global: TPosSc) -> ChkLocalTPosSc {
    loop {}
}
#[test]
fn test_chunk_local() {
    loop {}
}
#[test]
fn test_to_chunk_and_local() {
    loop {}
}
pub type TPosSc = u32;
pub const CHUNK_EXTENT: u16 = 128;
const CHUNK_N_TILES: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;
type ChunkTiles = [Tile; CHUNK_N_TILES];
fn default_chunk_tiles() -> ChunkTiles {
    loop {}
}
#[derive(Debug, Inspect)]
pub struct Chunk {
    tiles: ChunkTiles,
}
impl Chunk {
    pub fn gen(pos: ChunkPos, worldgen: &Worldgen) -> Self {
        loop {}
    }
    pub fn load_or_gen(chk: ChunkPos, worldgen: &Worldgen, world_path: &Path) -> Chunk {
        loop {}
    }
    fn at_mut(&mut self, local: ChunkLocalTilePos) -> &mut Tile {
        loop {}
    }
}
fn chunk_exists(reg_path: &Path, pos: ChunkPos) -> bool {
    loop {}
}
#[derive(Clone, Copy, Debug, Inspect)]
pub struct Tile {
    /// Background wall behind entities
    pub bg: BgTileId,
    /// The solid wall on the same level as entities
    pub mid: MidTileId,
    /// A layer on top of the mid wall. Usually ores or decorative pieces.
    pub fg: FgTileId,
}
pub const REGION_CHUNK_EXTENT: u8 = 8;
pub const REGION_N_CHUNKS: u8 = REGION_CHUNK_EXTENT * REGION_CHUNK_EXTENT;
/// This is the uncompressed byte length of a region
pub const REGION_BYTES: usize = REGION_N_CHUNKS as usize * CHUNK_BYTES;
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(
    REGION_N_CHUNKS <= 64,
    "A region file uses an existence bitset that's a 64 bit integer"
);
