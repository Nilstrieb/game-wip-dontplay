use sfml::graphics::{Rect, RenderTarget, RenderWindow, Sprite, Transformable};

use crate::{
    graphics::{ScreenPos, NATIVE_RESOLUTION},
    math::WorldPos,
    res::Res,
    world::{TilePos, World},
};

pub struct GameState {
    pub camera_offset: WorldPos,
    pub world: World,
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
    for y in (-32..NATIVE_RESOLUTION.h + 32).step_by(32) {
        for x in (-32..NATIVE_RESOLUTION.w + 32).step_by(32) {
            f(
                TilePos {
                    x: (camera_offset.x + x as i32) / 32,
                    y: (camera_offset.y + y as i32) / 32,
                },
                ScreenPos {
                    x: (x as i32 - camera_offset.x % 32) as i16,
                    y: (y as i32 - (camera_offset.y % 32)) as i16,
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
