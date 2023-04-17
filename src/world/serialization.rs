use std::{
    fs::OpenOptions, io::{Seek, Write},
    path::Path,
};
use crate::world::{
    format_reg_file_name, loc_byte_idx, loc_idx, reg_chunk_existence::ExistenceBitset,
    REGION_BYTES, TILE_BYTES,
};
use super::{default_chunk_tiles, loc_byte_idx_xy, Chunk, ChunkPos};
pub(super) fn save_chunk(pos: &ChunkPos, chk: &Chunk, world_dir: &Path) {
    loop {}
}
const COMP_LEVEL: i32 = 9;
impl Chunk {
    pub fn load_from_region(data: &[u8], x: u8, y: u8) -> Self {
        loop {}
    }
}
#[test]
fn test_chunk_seri() {
    loop {}
}
