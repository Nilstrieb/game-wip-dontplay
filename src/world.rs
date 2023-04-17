mod reg_chunk_existence;
mod serialization;
use egui_inspect::derive::Inspect;
use std::fmt::Debug;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Inspect)]
pub(crate) struct ChunkPos {}
#[derive(Debug, Inspect)]
pub(crate) struct World {}
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
