mod player;

use sfml::graphics::{Rect, RectangleShape, RenderTarget, RenderWindow, Sprite, Transformable};

use crate::{
    graphics::{ScreenPos, ScreenPosScalar, NATIVE_RESOLUTION},
    math::{wp_to_tp, WorldPos},
    res::Res,
    world::{Tile, TilePos, World},
};

use self::player::Player;

pub struct GameState {
    pub camera_offset: WorldPos,
    pub world: World,
    pub player: Player,
    pub gravity: f32,
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
    pub fn draw_entities(&mut self, rw: &mut RenderWindow) {
        let (x, y, w, h) = self.player.col_en.en.xywh();
        let mut rs = RectangleShape::new();
        rs.set_position((
            (x - self.camera_offset.x as i32) as f32,
            (y - self.camera_offset.y as i32) as f32,
        ));
        rs.set_size((w as f32, h as f32));
        rw.draw(&rs);
    }
}

pub fn for_each_tile_on_screen(camera_offset: WorldPos, mut f: impl FnMut(TilePos, ScreenPos)) {
    for y in (-32..NATIVE_RESOLUTION.h + 32).step_by(32) {
        for x in (-32..NATIVE_RESOLUTION.w + 32).step_by(32) {
            f(
                TilePos {
                    x: wp_to_tp(camera_offset.x.saturating_add(x.try_into().unwrap_or(0))),
                    y: wp_to_tp(camera_offset.y.saturating_add(y.try_into().unwrap_or(0))),
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
        let mut spawn_point = WorldPos::SURFACE_CENTER;
        spawn_point.y -= 300;
        Self {
            camera_offset: spawn_point,
            world: Default::default(),
            player: Player::new_at(spawn_point),
            gravity: 0.7,
        }
    }
}
