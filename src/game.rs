use sfml::graphics::{Rect, RenderTarget, RenderWindow, Sprite, Transformable};

use crate::{
    graphics::{ScreenPos, ScreenPosScalar, NATIVE_RESOLUTION},
    math::{wp_to_tp, WorldPos, WorldPosScalar},
    res::Res,
    world::{Tile, TilePos, World},
};

pub struct GameState {
    pub camera_offset: WorldPos,
    pub world: World,
}
impl GameState {
    pub(crate) fn draw_world(&mut self, rw: &mut RenderWindow, res: &Res) {
        let mut s = Sprite::with_texture(&res.tile_atlas);
        for_each_tile_on_screen(self.camera_offset, |tp, sp| {
            let tile = self.world.tile_at_mut(tp);
            if tile.id == Tile::AIR {
                return;
            }
            s.set_texture_rect(Rect::new((tile.id - 1) as i32 * 32, 0, 32, 32));
            s.set_position(sp.to_sf_vec());
            rw.draw(&s);
        });
    }
}

fn for_each_tile_on_screen(camera_offset: WorldPos, mut f: impl FnMut(TilePos, ScreenPos)) {
    for y in (-32..NATIVE_RESOLUTION.h + 32).step_by(32) {
        for x in (-32..NATIVE_RESOLUTION.w + 32).step_by(32) {
            f(
                TilePos {
                    x: wp_to_tp(camera_offset.x.saturating_add(x as WorldPosScalar)),
                    y: wp_to_tp(camera_offset.y.saturating_add(y as WorldPosScalar)),
                },
                ScreenPos {
                    x: ((x as i64) - ((camera_offset.x as i64) % 32)) as ScreenPosScalar,
                    y: ((y as i64) - ((camera_offset.y as i64) % 32)) as ScreenPosScalar,
                },
            )
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            camera_offset: WorldPos::SURFACE_CENTER,
            world: Default::default(),
        }
    }
}
