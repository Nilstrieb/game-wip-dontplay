use fnv::FnvHashMap;
use rand::{thread_rng, Rng};

type ChunkPosScalar = i16;

#[derive(Hash)]
struct ChunkPos {
    x: ChunkPosScalar,
    y: ChunkPosScalar,
}

pub struct World {
    chunks: FnvHashMap<ChunkPos, Chunk>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            chunks: Default::default(),
        }
    }
}

const CHUNK_EXTENT: u16 = 256;
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
            b.id = rng.gen();
        }
        Self { tiles }
    }
}

type TileId = u16;

#[derive(Clone, Copy)]
pub struct Tile {
    id: TileId,
}
