use fnv::FnvHashMap;

type ChunkPosScalar = i16;

#[derive(Hash)]
struct ChunkPos {
    x: ChunkPosScalar,
    y: ChunkPosScalar,
}

pub struct World {
    chunks: FnvHashMap<ChunkPos, Chunk>,
}

const CHUNK_EXTENT: u16 = 256;
const CHUNK_N_BLOCKS: usize = CHUNK_EXTENT as usize * CHUNK_EXTENT as usize;

type ChunkBlocks = [Block; CHUNK_N_BLOCKS];

pub struct Chunk {
    blocks: ChunkBlocks,
}

type BlockId = u16;

pub struct Block {
    id: BlockId,
}
