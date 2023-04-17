use worldgen::{
    constraint, noise::perlin::PerlinNoise,
    noisemap::{NoiseMap, NoiseMapGenerator, Seed, Step},
    world::{
        tile::{Constraint, ConstraintType},
        Size, Tile, World,
    },
};
use crate::{
    tiles::{BgTileId, FgTileId, MidTileId, TileId},
    world::{ChunkPos, Tile as Tl, CHUNK_EXTENT},
};
pub struct Worldgen {
    world: World<crate::world::Tile>,
}
impl Worldgen {
    pub fn from_seed(seed: i64) -> Self {
        loop {}
    }
    pub fn chunk_noise(&self, pos: ChunkPos) -> Vec<Vec<Tl>> {
        loop {}
    }
}
