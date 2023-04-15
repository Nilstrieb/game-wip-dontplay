use worldgen::{
    constraint,
    noise::perlin::PerlinNoise,
    noisemap::{NoiseMap, NoiseMapGenerator, Seed, Step},
    world::{
        tile::{Constraint, ConstraintType},
        Size, Tile, World,
    },
};

use crate::world::{ChunkPos, Tile as Tl, CHUNK_EXTENT};

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
                    bg: 9,
                    mid: 3,
                    fg: 6,
                })
                .when(constraint!(nm.clone(), < -0.8)),
            )
            // Dirt
            .add(
                Tile::new(Tl {
                    bg: 9,
                    mid: 3,
                    fg: 0,
                })
                .when(constraint!(nm.clone(), < -0.1)),
            )
            // Stone
            .add(
                Tile::new(Tl {
                    bg: 7,
                    mid: 2,
                    fg: 0,
                })
                .when(constraint!(nm, < 0.45)),
            )
            // Dirt wall
            .add(Tile::new(Tl {
                bg: 9,
                mid: 0,
                fg: 0,
            }));
        Self { world }
    }
    pub fn chunk_noise(&self, pos: ChunkPos) -> Vec<Vec<Tl>> {
        self.world.generate(pos.x as i64, pos.y as i64).unwrap()
    }
}
