use fnv::FnvHashMap;
use rand::{thread_rng, Rng};

type ChunkPosScalar = i16;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct ChunkPos {
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

pub struct TilePos {
    pub x: TilePosScalar,
    pub y: TilePosScalar,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkLocalTilePos {
    pub x: ChunkLocalTilePosScalar,
    pub y: ChunkLocalTilePosScalar,
}

type ChunkLocalTilePosScalar = i16;

impl TilePos {
    fn to_chunk_and_local(&self) -> (ChunkPos, ChunkLocalTilePos) {
        // 0,0 is chunk (0, 0)
        // -1, -1 is chunk (-1, -1)
        // For negative offsets, local offset starts from `CHUNK_EXTENT`
        let mut chk = ChunkPos {
            x: (self.x / CHUNK_EXTENT as i32) as i16,
            y: (self.y / CHUNK_EXTENT as i32) as i16,
        };
        if self.x.is_negative() {
            chk.x -= 1;
        }
        if self.y.is_negative() {
            chk.y -= 1;
        }
        let local = ChunkLocalTilePos {
            x: if chk.x.is_negative() {
                ((CHUNK_EXTENT as i32 + self.x) % CHUNK_EXTENT as i32) as i16
            } else {
                (self.x % CHUNK_EXTENT as i32) as i16
            },
            y: if chk.y.is_negative() {
                ((CHUNK_EXTENT as i32 + self.y) % CHUNK_EXTENT as i32) as i16
            } else {
                (self.y % CHUNK_EXTENT as i32) as i16
            },
        };
        (chk, local)
    }
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
            ChunkLocalTilePos {
                x: CHUNK_EXTENT as i16 - 1,
                y: CHUNK_EXTENT as i16 - 1
            }
        )
    );
}

// Need to support at least 4 million tiles long
type TilePosScalar = i32;

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
