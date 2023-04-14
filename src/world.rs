mod reg_chunk_existence;
mod serialization;

use std::{fmt::Debug, fs::File, io::Seek, path::Path};

use egui_inspect::derive::Inspect;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};

use crate::{
    math::WorldPos, player::Player, world::reg_chunk_existence::ExistenceBitset, worldgen::Worldgen,
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

impl World {
    pub fn new(spawn_point: WorldPos, name: &str) -> Self {
        Self {
            chunks: Default::default(),
            ticks: Default::default(),
            player: Player::new_at(spawn_point),
            name: name.to_string(),
        }
    }
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
    pub fn save(&self) {
        let result = std::fs::create_dir(&self.name);
        log::info!("{result:?}");
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
        let result = std::fs::write("world.dat", rmp_serde::to_vec(&meta).unwrap());
        log::info!("{result:?}");
    }
    pub fn save_chunks(&self) {
        for (pos, chk) in self.chunks.iter() {
            save_chunk(pos, chk);
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
        let result = std::env::set_current_dir(world_name);
        log::info!("{result:?}");
        let reg_filename = format_reg_file_name(chk.region());
        let chunk = if chunk_exists(&reg_filename, chk) {
            log::info!("Chunk exists, loading");
            let mut f = File::open(&reg_filename).unwrap();
            let bitset = ExistenceBitset::read_from_file(&mut f);
            log::info!("Existence bitset: {bitset:?}");
            assert_eq!(f.stream_position().unwrap(), 8);
            let decomp_data = zstd::decode_all(f).unwrap();
            assert_eq!(decomp_data.len(), REGION_BYTES);
            let local_pos = chk.local();
            Chunk::load_from_region(&decomp_data, local_pos.0, local_pos.1)
        } else {
            log::warn!("Chunk at {:?} doesn't exist, generating.", chk);
            Chunk::gen(chk, worldgen)
        };
        let result = std::env::set_current_dir(prev_dir);
        log::info!("{result:?}");
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
