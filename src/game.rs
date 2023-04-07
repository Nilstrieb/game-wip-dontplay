mod player;

use derivative::Derivative;
use egui_inspect::derive::Inspect;
use sfml::{
    graphics::{
        Color, Rect, RectangleShape, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Clock, Vector2u},
    SfBox,
};

use crate::{
    graphics::{ScreenPos, ScreenPosScalar},
    math::{wp_to_tp, WorldPos},
    res::Res,
    world::{Tile, TileId, TilePos, World},
    worldgen::Worldgen,
};

use self::player::Player;

#[derive(Derivative, Inspect)]
#[derivative(Debug)]
pub struct GameState {
    pub camera_offset: WorldPos,
    pub world: World,
    pub player: Player,
    pub gravity: f32,
    pub tile_to_place: TileId,
    pub current_biome: Biome,
    pub prev_biome: Biome,
    #[derivative(Debug = "ignore")]
    #[opaque]
    pub worldgen: Worldgen,
    pub ambient_light: f32,
    #[opaque]
    pub clock: SfBox<Clock>,
    pub light_sources: Vec<LightSource>,
}

#[derive(Debug, Inspect)]
pub struct LightSource {
    pub pos: WorldPos,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Inspect)]
pub enum Biome {
    Surface,
    Underground,
}

impl GameState {
    pub(crate) fn draw_world(&mut self, rt: &mut RenderTexture, res: &mut Res) {
        let mut s = Sprite::with_texture(&res.tile_atlas);
        for_each_tile_on_screen(self.camera_offset, rt.size(), |tp, sp| {
            let tile = self.world.tile_at_mut(tp, &self.worldgen);
            s.set_position(sp.to_sf_vec());
            if tile.bg != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.bg - 1) as i32 * 32, 0, 32, 32));
                rt.draw(&s);
            }
            if tile.mid != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.mid - 1) as i32 * 32, 0, 32, 32));
                rt.draw(&s);
            }
            if tile.fg != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.fg - 1) as i32 * 32, 0, 32, 32));
                rt.draw(&s);
            }
        });
    }
    pub fn draw_entities(&mut self, rw: &mut RenderTexture) {
        let (x, y, w, h) = self.player.col_en.en.xywh();
        let mut rect_sh = RectangleShape::new();
        rect_sh.set_position((
            (x - self.camera_offset.x as i32) as f32,
            (y - self.camera_offset.y as i32) as f32,
        ));
        rect_sh.set_size((w as f32, h as f32));
        rw.draw(&rect_sh);
        rect_sh.set_size((2., 2.));
        rect_sh.set_fill_color(Color::RED);
        rect_sh.set_position((
            (self.player.col_en.en.pos.x - self.camera_offset.x as i32) as f32,
            (self.player.col_en.en.pos.y - self.camera_offset.y as i32) as f32,
        ));
        rw.draw(&rect_sh);
    }

    pub(crate) fn light_pass(&mut self, lightmap: &mut RenderTexture, res: &Res) {
        // Clear light map
        // You can clear to a brighter color to increase ambient light level
        lightmap.clear(Color::rgba(0, 0, 0, 255));
        for ls in &self.light_sources {
            let (x, y) = (
                ls.pos.x as i32 - self.camera_offset.x as i32,
                ls.pos.y as i32 - self.camera_offset.y as i32,
            );
            let mut s = Sprite::with_texture(&res.light_texture);
            s.set_scale((4., 4.));
            s.set_origin((128., 128.));
            s.set_position((x as f32, y as f32));
            lightmap.draw(&s);
        }
    }
}

pub fn for_each_tile_on_screen(
    camera_offset: WorldPos,
    rt_size: Vector2u,
    mut f: impl FnMut(TilePos, ScreenPos),
) {
    for y in (-32..(rt_size.y as i16) + 32).step_by(32) {
        for x in (-32..(rt_size.x as i16) + 32).step_by(32) {
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
        spawn_point.y -= 1104;
        Self {
            camera_offset: spawn_point,
            world: Default::default(),
            player: Player::new_at(spawn_point),
            gravity: 0.7,
            tile_to_place: 1,
            current_biome: Biome::Surface,
            prev_biome: Biome::Surface,
            worldgen: Worldgen::default(),
            ambient_light: 1.0,
            clock: Clock::start(),
            light_sources: Vec::new(),
        }
    }
}
