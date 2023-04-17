use worldgen::{
    constraint,
    noise::perlin::PerlinNoise,
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
        let noise = PerlinNoise::new();

        let nm1 = NoiseMap::new(noise)
            .set(Seed::of(seed))
            .set(Step::of(0.005, 0.005));

        let nm2 = NoiseMap::new(noise)
            .set(Seed::of(seed))
            .set(Step::of(0.05, 0.05));

        let nm = Box::new(nm1 + nm2 * 3);

        let world = World::new()
            .set(Size::of(CHUNK_EXTENT as i64, CHUNK_EXTENT as i64))
            // Dirt coal
            .add(
                Tile::new(Tl {
                    bg: BgTileId::DIRT,
                    mid: MidTileId::DIRT,
                    fg: FgTileId::COAL,
                })
                .when(constraint!(nm.clone(), < -0.8)),
            )
            // Dirt
            .add(
                Tile::new(Tl {
                    bg: BgTileId::DIRT,
                    mid: MidTileId::DIRT,
                    fg: TileId::EMPTY,
                })
                .when(constraint!(nm.clone(), < -0.1)),
            )
            // Stone
            .add(
                Tile::new(Tl {
                    bg: BgTileId::STONE,
                    mid: MidTileId::STONE,
                    fg: TileId::EMPTY,
                })
                .when(constraint!(nm, < 0.45)),
            )
            // Dirt wall
            .add(Tile::new(Tl {
                bg: BgTileId::DIRT,
                mid: TileId::EMPTY,
                fg: TileId::EMPTY,
            }));
        Self { world }
    }
    pub fn chunk_noise(&self, pos: ChunkPos) -> Vec<Vec<Tl>> {
        self.world.generate(pos.x as i64, pos.y as i64).unwrap()
    }
}
