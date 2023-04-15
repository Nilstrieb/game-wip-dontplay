use std::{
    fs::OpenOptions,
    io::{Seek, Write},
    path::Path,
};

use crate::world::{
    format_reg_file_name, loc_byte_idx, loc_idx, reg_chunk_existence::ExistenceBitset,
    REGION_BYTES, TILE_BYTES,
};

use super::{default_chunk_tiles, loc_byte_idx_xy, Chunk, ChunkPos};

pub(super) fn save_chunk(pos: &ChunkPos, chk: &Chunk) {
    let reg_file_name = format_reg_file_name(pos.region());
    let reg_file_exists = Path::new(&reg_file_name).exists();
    if !reg_file_exists {
        log::warn!("Region file doesn't exist. Going to create one.");
    }
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&reg_file_name)
        .unwrap();
    let mut existence_bitset = if reg_file_exists {
        ExistenceBitset::read_from_file(&mut f)
    } else {
        ExistenceBitset::EMPTY
    };
    let mut region_tile_data = if reg_file_exists {
        assert_eq!(f.stream_position().unwrap(), 8);
        zstd::decode_all(&mut f).unwrap()
    } else {
        vec![0; REGION_BYTES]
    };
    // Even the zstd decompressed data should be exactly REGION_BYTES size
    assert_eq!(region_tile_data.len(), REGION_BYTES);
    let (loc_x, loc_y) = pos.local();
    let loc_idx = loc_idx(loc_y, loc_x);
    crate::bitmanip::set_nth_bit(&mut existence_bitset.0, loc_idx as usize, true);
    let byte_idx = loc_byte_idx(loc_idx);
    for (i, tile) in chk.tiles.iter().enumerate() {
        let off = byte_idx + (i * TILE_BYTES);
        region_tile_data[off..off + 2].copy_from_slice(&tile.bg.to_le_bytes());
        region_tile_data[off + 2..off + 4].copy_from_slice(&tile.mid.to_le_bytes());
        region_tile_data[off + 4..off + 6].copy_from_slice(&tile.fg.to_le_bytes());
    }
    f.rewind().unwrap();
    f.write_all(&u64::to_le_bytes(existence_bitset.0)[..])
        .unwrap();
    assert_eq!(f.stream_position().unwrap(), 8);
    assert_eq!(region_tile_data.len(), REGION_BYTES);
    let result = f.write_all(&zstd::encode_all(&region_tile_data[..], COMP_LEVEL).unwrap());
    let cursor = f.stream_position().unwrap();
    f.set_len(cursor).unwrap();
    log::info!("{result:?}");
}

const COMP_LEVEL: i32 = 9;

impl Chunk {
    pub fn load_from_region(data: &[u8], x: u8, y: u8) -> Self {
        let byte_idx = loc_byte_idx_xy(x, y);
        let mut tiles = default_chunk_tiles();
        for (i, t) in tiles.iter_mut().enumerate() {
            let off = byte_idx + (i * TILE_BYTES);
            t.bg = u16::from_le_bytes(data[off..off + 2].try_into().unwrap());
            t.mid = u16::from_le_bytes(data[off + 2..off + 4].try_into().unwrap());
            t.fg = u16::from_le_bytes(data[off + 4..off + 6].try_into().unwrap());
        }
        Self { tiles }
    }
}

#[test]
fn test_chunk_seri() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let result = std::fs::remove_file("0.0.rgn");
    log::info!("File del result: {result:?}");
    let mut chk = Chunk {
        tiles: super::default_chunk_tiles(),
    };
    for t in &mut chk.tiles {
        t.bg = 1;
    }
    save_chunk(&ChunkPos { x: 2, y: 0 }, &chk);
    save_chunk(&ChunkPos { x: 3, y: 0 }, &chk);
    let raw = std::fs::read("0.0.rgn").unwrap();
    zstd::decode_all(&raw[8..]).unwrap();
    std::fs::remove_file("0.0.rgn").unwrap();
}
