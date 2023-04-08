mod player;

use derivative::Derivative;
use egui_inspect::derive::Inspect;
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderTexture, Shape, Sprite, Transformable},
    system::Vector2u,
};

use crate::{
    graphics::{ScreenSc, ScreenVec},
    math::{smoothwave, wp_to_tp, WorldPos},
    res::Res,
    tiles::TileDb,
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
    pub ambient_light: u8,
    pub light_sources: Vec<LightSource>,
    pub tile_db: TileDb,
    /// This is the number of ticks since the world has started.
    /// In other words, the age of the world.
    pub ticks: u64,
}

#[derive(Debug, Inspect)]
pub struct LightSource {
    pub pos: ScreenVec,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Inspect)]
pub enum Biome {
    Surface,
    Underground,
}

impl GameState {
    pub fn update(&mut self) {
        self.ticks += 1;
    }
    pub(crate) fn draw_world(&mut self, rt: &mut RenderTexture, res: &mut Res) {
        self.light_sources.clear();
        let mut s = Sprite::with_texture(&res.tile_atlas);
        for_each_tile_on_screen(self.camera_offset, rt.size(), |tp, sp| {
            let tile = self.world.tile_at_mut(tp, &self.worldgen);
            s.set_position(sp.to_sf_vec());
            if tile.bg != Tile::EMPTY {
                s.set_texture_rect(self.tile_db[tile.bg].atlas_offset.to_sf_rect());
                rt.draw(&s);
            }
            if tile.mid != Tile::EMPTY {
                s.set_texture_rect(self.tile_db[tile.mid].atlas_offset.to_sf_rect());
                if let Some(light) = self.tile_db[tile.mid].light {
                    let pos = ScreenVec {
                        x: sp.x + light.x,
                        y: sp.y + light.y,
                    };
                    self.light_sources.push(LightSource { pos });
                }
                rt.draw(&s);
            }
            if tile.fg != Tile::EMPTY {
                s.set_texture_rect(self.tile_db[tile.fg].atlas_offset.to_sf_rect());
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
        let how_much_below_surface = self.camera_offset.y.saturating_sub(WorldPos::SURFACE);
        let light_dropoff = (how_much_below_surface / 8).min(255) as u8;
        let daylight = 255u8;
        self.ambient_light = daylight.saturating_sub(light_dropoff);
        // Clear light map
        // You can clear to a brighter color to increase ambient light level
        lightmap.clear(Color::rgba(
            self.ambient_light,
            self.ambient_light,
            self.ambient_light,
            255,
        ));
        for ls in &self.light_sources {
            let mut s = Sprite::with_texture(&res.light_texture);
            let flicker = smoothwave(self.ticks, 40) as f32 / 64.0;
            s.set_scale((4. + flicker, 4. + flicker));
            s.set_origin((128., 128.));
            s.set_position((ls.pos.x.into(), ls.pos.y.into()));
            lightmap.draw(&s);
        }
    }
}

pub fn for_each_tile_on_screen(
    camera_offset: WorldPos,
    rt_size: Vector2u,
    mut f: impl FnMut(TilePos, ScreenVec),
) {
    for y in (-32..(rt_size.y as i16) + 32).step_by(32) {
        for x in (-32..(rt_size.x as i16) + 32).step_by(32) {
            f(
                TilePos {
                    x: wp_to_tp(camera_offset.x.saturating_add(x.try_into().unwrap_or(0))),
                    y: wp_to_tp(camera_offset.y.saturating_add(y.try_into().unwrap_or(0))),
                },
                ScreenVec {
                    x: ((x as i64) - ((camera_offset.x as i64) % 32)) as ScreenSc,
                    y: ((y as i64) - ((camera_offset.y as i64) % 32)) as ScreenSc,
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
            gravity: 0.55,
            tile_to_place: 1,
            current_biome: Biome::Surface,
            prev_biome: Biome::Surface,
            worldgen: Worldgen::default(),
            ambient_light: 0,
            light_sources: Vec::new(),
            tile_db: TileDb::load_or_default(),
            ticks: 0,
        }
    }
}
