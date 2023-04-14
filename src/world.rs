mod reg_chunk_existence;

use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use egui_inspect::derive::Inspect;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};

use crate::{
    math::WorldPos, player::Player, world::reg_chunk_existence::ExistenceBitset, worldgen::Worldgen,
};

pub type ChkPosSc = u16;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Inspect)]
pub struct ChunkPos {
    pub x: ChkPosSc,
    pub y: ChkPosSc,
}

impl ChunkPos {
    /// Returns the region this chunk position belongs to
    pub fn region(&self) -> (u8, u8) {
        (
            (self.x / REGION_CHUNK_EXTENT as ChkPosSc) as u8,
            (self.y / REGION_CHUNK_EXTENT as ChkPosSc) as u8,
        )
    }
    /// Returns the local position in the region (0-7)
    pub fn local(&self) -> (u8, u8) {
        (
            (self.x % REGION_CHUNK_EXTENT as ChkPosSc) as u8,
            (self.y % REGION_CHUNK_EXTENT as ChkPosSc) as u8,
        )
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
}

const COMP_LEVEL: i32 = 9;

impl World {
    pub fn new(spawn_point: WorldPos, name: &str) -> Self {
        Self {
            chunks: Default::default(),
            ticks: Default::default(),
            player: Player::new_at(spawn_point),
            name: name.to_string(),
        }
    }
    pub fn save(&self) {
        log::info!("{:?}", std::fs::create_dir(&self.name));
        std::env::set_current_dir(&self.name).unwrap();
        self.save_meta();
        self.player.save();
        self.save_chunks();
    }
    pub fn save_meta(&self) {
        let meta = WorldMetaSave {
            name: self.name.clone(),
            ticks: self.ticks,
        };
        log::info!(
            "{:?}",
            std::fs::write("world.dat", rmp_serde::to_vec(&meta).unwrap())
        );
    }
    pub fn save_chunks(&self) {
        for (pos, chk) in self.chunks.iter() {
            let reg_file_name = format_reg_file_name(pos.region());
            dbg!(&reg_file_name);
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
            dbg!(existence_bitset);
            let _ = dbg!(f.stream_position());
            let mut region_tile_data = if reg_file_exists {
                zstd::decode_all(&mut f).unwrap()
            } else {
                vec![0; REGION_BYTES]
            };
            // Even the zstd decompressed data should be exactly REGION_BYTES size
            assert_eq!(region_tile_data.len(), REGION_BYTES);
            let (loc_x, loc_y) = pos.local();
            dbg!(loc_x, loc_y);
            let loc_idx = loc_idx(loc_y, loc_x);
            crate::bitmanip::set_nth_bit(&mut existence_bitset.0, loc_idx as usize, true);
            let byte_idx = loc_byte_idx(loc_idx);
            dbg!(byte_idx);
            let end_idx = byte_idx + CHUNK_BYTES;
            dbg!(end_idx);
            for (i, tile) in chk.tiles.iter().enumerate() {
                let off = byte_idx + (i * TILE_BYTES);
                region_tile_data[off..off + 2].copy_from_slice(&tile.bg.to_le_bytes());
                region_tile_data[off + 2..off + 4].copy_from_slice(&tile.mid.to_le_bytes());
                region_tile_data[off + 4..off + 6].copy_from_slice(&tile.fg.to_le_bytes());
            }
            f.seek(SeekFrom::Start(0)).unwrap();
            f.write_all(&u64::to_le_bytes(existence_bitset.0)[..])
                .unwrap();
            assert_eq!(f.stream_position().unwrap(), 8);
            assert_eq!(region_tile_data.len(), REGION_BYTES);
            log::info!(
                "{:?}",
                f.write_all(&zstd::encode_all(&region_tile_data[..], COMP_LEVEL).unwrap())
            );
        }
    }
}

fn loc_byte_idx_xy(x: u8, y: u8) -> usize {
    loc_byte_idx(loc_idx(y, x))
}

fn loc_byte_idx(loc_idx: u8) -> usize {
    loc_idx as usize * CHUNK_BYTES
}

fn loc_idx(loc_y: u8, loc_x: u8) -> u8 {
    (loc_y * REGION_CHUNK_EXTENT) + loc_x
}

fn format_reg_file_name((x, y): (u8, u8)) -> String {
    format!("{x}.{y}.rgn")
}

const CHUNK_BYTES: usize = CHUNK_N_TILES * TILE_BYTES;
const TILE_BYTES: usize = 3 * 2;

#[derive(Serialize, Deserialize)]
struct WorldMetaSave {
    name: String,
    ticks: u64,
}

impl World {
    /// Get mutable access to the tile at `pos`.
    ///
    /// Loads or generates the containing chunk if necessary.
    pub fn tile_at_mut(&mut self, pos: TilePos, worldgen: &Worldgen) -> &mut Tile {
        let (chk, local) = pos.to_chunk_and_local();
        let chk = self
            .chunks
            .entry(chk)
            .or_insert_with(|| Chunk::load_or_gen(chk, worldgen, &self.name));
        chk.at_mut(local)
    }
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
        let chk = ChunkPos {
            x: chk_pos(self.x),
            y: chk_pos(self.y),
        };
        let local = ChunkLocalTilePos {
            x: chunk_local(self.x),
            y: chunk_local(self.y),
        };
        (chk, local)
    }

    pub(crate) fn to_chunk(self) -> ChunkPos {
        self.to_chunk_and_local().0
    }
}

fn chk_pos(tile: TPosSc) -> ChkPosSc {
    (tile / CHUNK_EXTENT as TPosSc) as ChkPosSc
}

#[test]
fn test_chk_pos() {
    assert_eq!(chk_pos(0), 0);
    assert_eq!(chk_pos(1), 0);
    assert_eq!(chk_pos(127), 0);
    assert_eq!(chk_pos(128), 1);
}

fn chunk_local(global: TPosSc) -> ChkLocalTPosSc {
    (global % CHUNK_EXTENT as TPosSc) as ChkLocalTPosSc
}

#[test]
fn test_chunk_local() {
    assert_eq!(chunk_local(0), 0);
}

#[test]
fn test_to_chunk_and_local() {
    assert_eq!(
        TilePos { x: 0, y: 0 }.to_chunk_and_local(),
        (ChunkPos { x: 0, y: 0 }, ChunkLocalTilePos { x: 0, y: 0 })
    );
    assert_eq!(
        TilePos { x: 1, y: 1 }.to_chunk_and_local(),
        (ChunkPos { x: 0, y: 0 }, ChunkLocalTilePos { x: 1, y: 1 })
    );
}

// Need to support at least 4 million tiles long
pub type TPosSc = u32;

pub const CHUNK_EXTENT: u16 = 128;
const CHUNK_N_TILES: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;

type ChunkTiles = [Tile; CHUNK_N_TILES];

fn default_chunk_tiles() -> ChunkTiles {
    [Tile {
        bg: 0,
        mid: 0,
        fg: 0,
    }; CHUNK_N_TILES]
}

#[derive(Debug, Inspect)]
pub struct Chunk {
    tiles: ChunkTiles,
}

impl Chunk {
    pub fn gen(pos: ChunkPos, worldgen: &Worldgen) -> Self {
        let mut tiles = default_chunk_tiles();
        let noise = worldgen.chunk_noise(pos);
        if pos.y >= 156 {
            for (i, t) in tiles.iter_mut().enumerate() {
                let x = i % CHUNK_EXTENT as usize;
                let y = i / CHUNK_EXTENT as usize;
                let noise = noise[x][y];
                *t = noise;
            }
        }
        // Unbreakable layer at bottom
        if pos.y > 780 {
            for b in &mut tiles {
                b.mid = Tile::UNBREAKANIUM;
            }
        }
        Self { tiles }
    }

    pub fn load_or_gen(chk: ChunkPos, worldgen: &Worldgen, world_name: &str) -> Chunk {
        log::info!("Loading chunk {chk:?} (reg: {:?})", chk.region());
        let prev_dir = std::env::current_dir().unwrap();
        log::info!("{:?}", std::env::set_current_dir(world_name));
        let reg_filename = format_reg_file_name(chk.region());
        let chunk = if chunk_exists(&reg_filename, chk) {
            log::info!("Chunk exists, loading");
            let mut f = File::open(&reg_filename).unwrap();
            log::info!(
                "Existence bitset: {:?}",
                ExistenceBitset::read_from_file(&mut f)
            );
            assert_eq!(f.stream_position().unwrap(), 8);
            let decomp_data = zstd::decode_all(f).unwrap();
            assert_eq!(decomp_data.len(), REGION_BYTES);
            let local_pos = chk.local();
            Chunk::load_from_region(&decomp_data, local_pos.0, local_pos.1)
        } else {
            log::warn!("Chunk at {:?} doesn't exist, generating.", chk);
            Chunk::gen(chk, worldgen)
        };
        log::info!("{:?}", std::env::set_current_dir(prev_dir));
        chunk
    }

    fn at_mut(&mut self, local: ChunkLocalTilePos) -> &mut Tile {
        &mut self.tiles[CHUNK_EXTENT as usize * local.y as usize + local.x as usize]
    }

    fn load_from_region(data: &[u8], x: u8, y: u8) -> Self {
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

fn chunk_exists(reg_filename: &str, pos: ChunkPos) -> bool {
    if !Path::new(&reg_filename).exists() {
        return false;
    }
    let bitset = ExistenceBitset::read_from_fs(reg_filename);
    let local = pos.local();
    let idx = loc_idx(local.1, local.0);
    crate::bitmanip::nth_bit_set(bitset.0, idx as usize)
}

pub type TileId = u16;

#[derive(Clone, Copy, Debug, Inspect)]
pub struct Tile {
    /// Background wall behind entities
    pub bg: TileId,
    /// The solid wall on the same level as entities
    pub mid: TileId,
    /// A layer on top of the mid wall. Usually ores or decorative pieces.
    pub fg: TileId,
}

impl Tile {
    pub const EMPTY: TileId = 0;
    pub const UNBREAKANIUM: TileId = 5;
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
