use std::path::Path;

use egui_inspect::derive::Inspect;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};

use crate::{math::WorldPos, player::Player, worldgen::Worldgen};

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
            let (reg_x, reg_y) = pos.region();
            let reg_file_name = format!("{reg_x}.{reg_y}.rgn");
            dbg!(&reg_file_name);
            if !Path::new(&reg_file_name).exists() {
                log::info!(
                    "{:?}",
                    std::fs::write(&reg_file_name, zstd::encode_all(&[][..], 0).unwrap())
                );
            }
            let mut decomp = zstd::decode_all(&std::fs::read(&reg_file_name).unwrap()[..]).unwrap();
            let (loc_x, loc_y) = pos.local();
            dbg!(loc_x, loc_y);
            let loc_idx = (loc_y * REGION_CHUNK_EXTENT) + loc_x;
            dbg!(loc_idx);
            let byte_idx = loc_idx as usize * CHUNK_BYTES;
            dbg!(byte_idx);
            let end_idx = byte_idx + CHUNK_BYTES;
            dbg!(end_idx);
            if decomp.len() < end_idx + 1 {
                decomp.resize(end_idx + 1, 0);
            }
            for (i, tile) in chk.tiles.iter().enumerate() {
                let off = byte_idx + (i * TILE_BYTES);
                decomp[off..off + 2].copy_from_slice(&tile.bg.to_le_bytes());
                decomp[off + 2..off + 4].copy_from_slice(&tile.mid.to_le_bytes());
                decomp[off + 4..off + 6].copy_from_slice(&tile.fg.to_le_bytes());
            }
            log::info!(
                "{:?}",
                std::fs::write(&reg_file_name, zstd::encode_all(&decomp[..], 0).unwrap())
            );
        }
    }
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
            .or_insert_with(|| Chunk::gen(chk, worldgen));
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

#[derive(Debug, Inspect)]
pub struct Chunk {
    tiles: ChunkTiles,
}

impl Chunk {
    pub fn gen(pos: ChunkPos, worldgen: &Worldgen) -> Self {
        let mut tiles = [Tile {
            bg: 0,
            mid: 0,
            fg: 0,
        }; CHUNK_N_TILES];
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

    fn at_mut(&mut self, local: ChunkLocalTilePos) -> &mut Tile {
        &mut self.tiles[CHUNK_EXTENT as usize * local.y as usize + local.x as usize]
    }
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
