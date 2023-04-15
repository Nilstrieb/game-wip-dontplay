use std::path::PathBuf;

use derivative::Derivative;
use egui_inspect::derive::Inspect;
use sfml::{
    graphics::{
        Color, Rect, RectangleShape, RenderTarget, RenderTexture, Shape, Sprite, Transformable,
    },
    system::{Vector2f, Vector2u},
    window::Key,
};

use crate::{
    graphics::{ScreenSc, ScreenVec},
    input::Input,
    inventory::{Inventory, ItemDb},
    math::{smoothwave, wp_to_tp, WorldPos},
    res::Res,
    tiles::TileDb,
    world::{Tile, TilePos, World},
    worldgen::Worldgen,
};

#[derive(Derivative, Inspect)]
#[derivative(Debug)]
pub struct GameState {
    pub camera_offset: WorldPos,
    pub world: World,
    pub gravity: f32,
    pub current_biome: Biome,
    pub prev_biome: Biome,
    #[derivative(Debug = "ignore")]
    #[opaque]
    pub worldgen: Worldgen,
    pub ambient_light: u8,
    pub light_sources: Vec<LightSource>,
    pub tile_db: TileDb,
    pub inventory: Inventory,
    pub itemdb: ItemDb,
    pub selected_inv_slot: usize,
    pub spawn_point: WorldPos,
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
    pub fn update(&mut self, input: &Input) {
        if input.pressed(Key::Num1) {
            self.selected_inv_slot = 0;
        }
        if input.pressed(Key::Num2) {
            self.selected_inv_slot = 1;
        }
        if input.pressed(Key::Num3) {
            self.selected_inv_slot = 2;
        }
        if input.pressed(Key::Num4) {
            self.selected_inv_slot = 3;
        }
        if input.pressed(Key::Num5) {
            self.selected_inv_slot = 4;
        }
        self.world.ticks += 1;
    }
    pub(crate) fn draw_world(&mut self, rt: &mut RenderTexture, res: &mut Res) {
        self.light_sources.clear();
        let mut s = Sprite::with_texture(&res.atlas.tex);
        for_each_tile_on_screen(self.camera_offset, rt.size(), |tp, sp| {
            let tile = self.world.tile_at_mut(tp, &self.worldgen);
            s.set_position(sp.to_sf_vec());
            if tile.bg != Tile::EMPTY {
                s.set_texture_rect(self.tile_db[tile.bg].tex_rect.to_sf());
                rt.draw(&s);
            }
            if tile.mid != Tile::EMPTY {
                s.set_texture_rect(self.tile_db[tile.mid].tex_rect.to_sf());
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
                s.set_texture_rect(self.tile_db[tile.fg].tex_rect.to_sf());
                rt.draw(&s);
            }
        });
    }
    pub fn draw_entities(&mut self, rt: &mut RenderTexture) {
        let (x, y, w, h) = self.world.player.col_en.en.xywh();
        let mut rect_sh = RectangleShape::new();
        rect_sh.set_position((
            (x - self.camera_offset.x as i32) as f32,
            (y - self.camera_offset.y as i32) as f32,
        ));
        rect_sh.set_size((w as f32, h as f32));
        rt.draw(&rect_sh);
        rect_sh.set_size((2., 2.));
        rect_sh.set_fill_color(Color::RED);
        rect_sh.set_position((
            (self.world.player.col_en.en.pos.x - self.camera_offset.x as i32) as f32,
            (self.world.player.col_en.en.pos.y - self.camera_offset.y as i32) as f32,
        ));
        rt.draw(&rect_sh);
    }

    pub fn draw_ui(&mut self, rt: &mut RenderTexture, res: &Res, ui_dims: Vector2f) {
        let mut s = Sprite::with_texture(&res.atlas.tex);
        let mut rs = RectangleShape::from_rect(Rect::new(0., 0., 32., 32.));
        rs.set_outline_color(Color::YELLOW);
        rs.set_outline_thickness(1.0);
        rs.set_fill_color(Color::TRANSPARENT);
        for (i, slot) in self.inventory.slots.iter().enumerate() {
            s.set_texture_rect(res.atlas.rects["ui/invframe"].to_sf());
            let pos = ((i * 38) as f32 + 8.0, (ui_dims.y - 38.));
            s.set_position(pos);
            rt.draw(&s);
            let item_def = &self.itemdb.db[slot.id as usize];
            if let Some(rect) = res.atlas.rects.get(&item_def.graphic_name) {
                s.set_texture_rect(rect.to_sf());
                rt.draw(&s);
            } else {
                log::error!("Missing rect for item {}", item_def.name);
            }
            if i == self.selected_inv_slot {
                rs.set_position(pos);
                rt.draw(&rs);
            }
        }
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
        let mut s = Sprite::with_texture(&res.atlas.tex);
        s.set_texture_rect(res.atlas.rects["light/1"].to_sf());
        for ls in &self.light_sources {
            let flicker = smoothwave(self.world.ticks, 40) as f32 / 64.0;
            s.set_scale((4. + flicker, 4. + flicker));
            s.set_origin((128., 128.));
            s.set_position((ls.pos.x.into(), ls.pos.y.into()));
            lightmap.draw(&s);
        }
    }

    pub(crate) fn new(world_name: String, path: PathBuf, res: &Res) -> GameState {
        let mut spawn_point = WorldPos::SURFACE_CENTER;
        spawn_point.y -= 1104;
        let mut tile_db = TileDb::load_or_default();
        tile_db.update_rects(&res.atlas.rects);
        Self {
            camera_offset: spawn_point,
            world: World::new(spawn_point, &world_name, path),
            gravity: 0.55,
            current_biome: Biome::Surface,
            prev_biome: Biome::Surface,
            worldgen: Worldgen::from_seed(0),
            ambient_light: 0,
            light_sources: Vec::new(),
            tile_db,
            inventory: Inventory::new_debug(),
            itemdb: ItemDb::default(),
            selected_inv_slot: 0,
            spawn_point,
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
