use egui_inspect::derive::Inspect;
use fnv::FnvHashMap;

use crate::worldgen::Worldgen;

pub type ChunkPosScalar = u16;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Inspect)]
pub struct ChunkPos {
    pub x: ChunkPosScalar,
    pub y: ChunkPosScalar,
}

#[derive(Default, Debug, Inspect)]
pub struct World {
    /// The currently loaded chunks
    chunks: FnvHashMap<ChunkPos, Chunk>,
    /// This is the number of ticks since the world has started.
    /// In other words, the age of the world.
    pub ticks: u64,
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
    pub x: TilePosScalar,
    pub y: TilePosScalar,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkLocalTilePos {
    pub x: ChunkLocalTilePosScalar,
    pub y: ChunkLocalTilePosScalar,
}

type ChunkLocalTilePosScalar = u8;

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
}

fn chk_pos(tile: TilePosScalar) -> ChunkPosScalar {
    (tile / CHUNK_EXTENT as TilePosScalar) as ChunkPosScalar
}

#[test]
fn test_chk_pos() {
    assert_eq!(chk_pos(0), 0);
    assert_eq!(chk_pos(1), 0);
    assert_eq!(chk_pos(127), 0);
    assert_eq!(chk_pos(128), 1);
}

fn chunk_local(global: TilePosScalar) -> ChunkLocalTilePosScalar {
    (global % CHUNK_EXTENT as TilePosScalar) as ChunkLocalTilePosScalar
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
pub type TilePosScalar = u32;

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
