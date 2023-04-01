use sfml::graphics::{RenderTarget, RenderWindow, Sprite};

use crate::{math::WorldPos, res::Res, world::World};

pub struct GameState {
    camera_offset: WorldPos,
    world: World,
}
impl GameState {
    pub(crate) fn draw_world(&mut self, rw: &mut RenderWindow, res: &Res) {
        rw.draw(&Sprite::with_texture(&res.tile_atlas));
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            camera_offset: WorldPos { x: 0, y: 0 },
            world: Default::default(),
        }
    }
}
