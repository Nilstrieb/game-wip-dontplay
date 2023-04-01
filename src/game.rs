use sfml::graphics::{Rect, RenderTarget, RenderWindow, Sprite, Transformable};

use crate::{
    graphics::{ScreenPos, NATIVE_RESOLUTION},
    math::{WorldPos, WorldPosScalar, TILE_SIZE},
    res::Res,
    world::{TilePos, World},
};

pub struct GameState {
    camera_offset: WorldPos,
    world: World,
}
impl GameState {
    pub(crate) fn draw_world(&mut self, rw: &mut RenderWindow, res: &Res) {
        let mut s = Sprite::with_texture(&res.tile_atlas);
        for_each_tile(self.camera_offset, |tp, sp| {
            let tile = self.world.tile_at_mut(tp);
            s.set_texture_rect(Rect::new(tile.id as i32 * 32, 0, 32, 32));
            s.set_position(sp.to_sf_vec());
            rw.draw(&s);
        });
    }
}

fn for_each_tile(camera_offset: WorldPos, mut f: impl FnMut(TilePos, ScreenPos)) {
    for y in (camera_offset.y..camera_offset.y + NATIVE_RESOLUTION.h as WorldPosScalar).step_by(32)
    {
        for x in
            (camera_offset.x..camera_offset.x + NATIVE_RESOLUTION.w as WorldPosScalar).step_by(32)
        {
            f(
                TilePos {
                    x: x / 32,
                    y: y / 32,
                },
                ScreenPos {
                    x: (x - camera_offset.x) as i16,
                    y: (y - camera_offset.y) as i16,
                },
            )
        }
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
