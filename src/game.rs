mod player;

use derivative::Derivative;
use egui_inspect::{derive::Inspect, inspect};
use sfml::{
    graphics::{
        glsl::{Vec2, Vec4},
        Color, Rect, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape, Sprite,
        Transformable,
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
    lighting_params: LightingData,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Inspect)]
pub enum Biome {
    Surface,
    Underground,
}

impl GameState {
    pub(crate) fn draw_world(&mut self, rw: &mut RenderTexture, res: &mut Res) {
        let mut rs = RenderStates::default();
        res.lighting_shader.set_uniform_bool("has_texture", true);
        rs.set_shader(Some(&res.lighting_shader));
        let mut s = Sprite::with_texture(&res.tile_atlas);
        for_each_tile_on_screen(self.camera_offset, rw.size(), |tp, sp| {
            let tile = self.world.tile_at_mut(tp, &self.worldgen);
            s.set_position(sp.to_sf_vec());
            if tile.bg != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.bg - 1) as i32 * 32, 0, 32, 32));
                rw.draw_with_renderstates(&s, &rs);
            }
            if tile.mid != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.mid - 1) as i32 * 32, 0, 32, 32));
                rw.draw_with_renderstates(&s, &rs);
            }
            if tile.fg != Tile::EMPTY {
                s.set_texture_rect(Rect::new((tile.fg - 1) as i32 * 32, 0, 32, 32));
                rw.draw_with_renderstates(&s, &rs);
            }
        });
    }
    pub fn draw_entities(&mut self, rw: &mut RenderTexture, res: &mut Res) {
        let mut rend_st = RenderStates::default();
        res.lighting_shader.set_uniform_bool("has_texture", false);
        rend_st.set_shader(Some(&res.lighting_shader));
        let (x, y, w, h) = self.player.col_en.en.xywh();
        let mut rect_sh = RectangleShape::new();
        rect_sh.set_position((
            (x - self.camera_offset.x as i32) as f32,
            (y - self.camera_offset.y as i32) as f32,
        ));
        rect_sh.set_size((w as f32, h as f32));
        rw.draw_with_renderstates(&rect_sh, &rend_st);
        rect_sh.set_size((2., 2.));
        rect_sh.set_fill_color(Color::RED);
        rect_sh.set_position((
            (self.player.col_en.en.pos.x - self.camera_offset.x as i32) as f32,
            (self.player.col_en.en.pos.y - self.camera_offset.y as i32) as f32,
        ));
        rw.draw(&rect_sh);
    }
    pub fn render_pre_step(&mut self, res: &mut Res, rt_size: Vector2u) {
        res.lighting_shader.set_uniform_current_texture("texture");
        res.lighting_shader
            .set_uniform_float("time", self.clock.elapsed_time().as_seconds() * 10.0);
        res.lighting_shader.set_uniform_vec2(
            "mouse",
            Vec2::new(rt_size.x as f32 / 2.0, rt_size.y as f32 / 2.0),
        );
        res.lighting_shader
            .set_uniform_vec2("resolution", Vec2::new(rt_size.x as f32, rt_size.y as f32));
        res.lighting_shader
            .set_uniform_vec4("lightData", self.lighting_params.light);
        res.lighting_shader
            .set_uniform_vec4("ambientData", self.lighting_params.ambient);
        res.lighting_shader
            .set_uniform_float("lightSize", self.lighting_params.size);
    }
}

#[derive(Debug, Inspect)]
struct LightingData {
    #[inspect_with(inspect_vec4)]
    light: Vec4,
    #[inspect_with(inspect_vec4)]
    ambient: Vec4,
    size: f32,
}

fn inspect_vec4(val: &mut Vec4, ui: &mut egui::Ui, _id: u64) {
    inspect! {
        ui, val.x, val.y, val.z, val.w
    }
}

impl Default for LightingData {
    fn default() -> Self {
        Self {
            light: Vec4 {
                x: 1.0,
                y: 0.8,
                z: 0.2,
                w: 2.0,
            },
            ambient: Vec4 {
                x: 0.3,
                y: 0.3,
                z: 0.8,
                w: 0.3,
            },
            size: 0.3,
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
            lighting_params: Default::default(),
        }
    }
}
