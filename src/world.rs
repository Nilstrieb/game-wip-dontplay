use fnv::FnvHashMap;
use rand::{thread_rng, Rng};

type ChunkPosScalar = i16;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct ChunkPos {
    x: ChunkPosScalar,
    y: ChunkPosScalar,
}

pub struct World {
    /// The currently loaded chunks
    chunks: FnvHashMap<ChunkPos, Chunk>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            chunks: Default::default(),
        }
    }
}

impl World {
    /// Get mutable access to the tile at `pos`.
    ///
    /// Loads or generates the containing chunk if necessary.
    pub fn tile_at_mut(&mut self, pos: TilePos) -> &mut Tile {
        let (chk, local) = pos.to_chunk_and_local();
        let chk = self.chunks.entry(chk).or_insert_with(Chunk::new_rand);
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
    if tile.is_negative() {
        ((tile + 1) / CHUNK_EXTENT as i32) as i16 - 1
    } else {
        (tile / CHUNK_EXTENT as i32) as i16
    }
}

#[test]
fn test_chk_pos() {
    assert_eq!(chk_pos(0), 0);
    assert_eq!(chk_pos(1), 0);
    assert_eq!(chk_pos(127), 0);
    assert_eq!(chk_pos(128), 1);
    assert_eq!(chk_pos(-1), -1);
    assert_eq!(chk_pos(-2), -1);
    assert_eq!(chk_pos(-127), -1);
    assert_eq!(chk_pos(-128), -1);
    assert_eq!(chk_pos(-129), -2);
}

/*fn chunk_local(global: TilePosScalar) -> ChunkLocalTilePosScalar {
    if global.is_negative() {
        (CHUNK_EXTENT as i32 + global % CHUNK_EXTENT as i32) as u8
    } else {
        (global % CHUNK_EXTENT as i32) as u8
    }
}*/

fn chunk_local(global: TilePosScalar) -> ChunkLocalTilePosScalar {
    let mut result = global % CHUNK_EXTENT as i32;
    if result.is_negative() {
        result += CHUNK_EXTENT as i32;
    }
    result as u8
}

#[test]
fn test_chunk_local() {
    assert_eq!(chunk_local(0), 0);
    assert_eq!(chunk_local(-1), 127);
    assert_eq!(chunk_local(-128), 0);
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
    assert_eq!(
        TilePos { x: -1, y: -1 }.to_chunk_and_local(),
        (
            ChunkPos { x: -1, y: -1 },
            ChunkLocalTilePos { x: 127, y: 127 }
        )
    );
    assert_eq!(
        TilePos { x: -127, y: -127 }.to_chunk_and_local(),
        (ChunkPos { x: -1, y: -1 }, ChunkLocalTilePos { x: 1, y: 1 })
    );
    assert_eq!(
        TilePos { x: -128, y: -128 }.to_chunk_and_local(),
        (ChunkPos { x: -1, y: -1 }, ChunkLocalTilePos { x: 0, y: 0 })
    );
    assert_eq!(
        TilePos { x: -129, y: -129 }.to_chunk_and_local(),
        (
            ChunkPos { x: -2, y: -2 },
            ChunkLocalTilePos { x: 127, y: 127 }
        )
    );
}

// Need to support at least 4 million tiles long
pub type TilePosScalar = i32;

const CHUNK_EXTENT: u16 = 128;
const CHUNK_N_TILES: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;

type ChunkTiles = [Tile; CHUNK_N_TILES];

pub struct Chunk {
    tiles: ChunkTiles,
}

impl Chunk {
    pub fn new_rand() -> Self {
        let mut rng = thread_rng();
        let mut tiles = [Tile { id: 0 }; CHUNK_N_TILES];
        for b in &mut tiles {
            b.id = rng.gen_range(0..8);
        }
        Self { tiles }
    }

    fn at_mut(&mut self, local: ChunkLocalTilePos) -> &mut Tile {
        &mut self.tiles[CHUNK_EXTENT as usize * local.y as usize + local.x as usize]
    }
}

type TileId = u16;

#[derive(Clone, Copy)]
pub struct Tile {
    pub id: TileId,
}
