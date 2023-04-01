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
const CHUNK_N_BLOCKS: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;

type ChunkBlocks = [Block; CHUNK_N_BLOCKS];

pub struct Chunk {
    blocks: ChunkBlocks,
}

impl Chunk {
    pub fn new_rand() -> Self {
        let mut rng = thread_rng();
        let mut blocks = [Block { id: 0 }; CHUNK_N_BLOCKS];
        for b in &mut blocks {
            b.id = rng.gen();
        }
        Self { blocks }
    }
}

type BlockId = u16;

#[derive(Clone, Copy)]
pub struct Block {
    id: BlockId,
}
