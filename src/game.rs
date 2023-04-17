use std::path::PathBuf;
use derivative::Derivative;
use egui_inspect::derive::Inspect;
use sfml::{
    graphics::{
        Color, Rect, RectangleShape, RenderTarget, RenderTexture, Shape, Sprite,
        Transformable,
    },
    system::{Vector2f, Vector2u},
    window::Key,
};
use crate::{
    graphics::{ScreenSc, ScreenVec},
    input::Input, inventory::{Inventory, ItemDb},
    math::{smoothwave, wp_to_tp, WorldPos},
    res::Res, tiles::TileDb, world::{TilePos, World},
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
        loop {}
    }
    pub(crate) fn draw_world(&mut self, rt: &mut RenderTexture, res: &mut Res) {
        loop {}
    }
    pub fn draw_entities(&mut self, rt: &mut RenderTexture) {
        loop {}
    }
    pub fn draw_ui(&mut self, rt: &mut RenderTexture, res: &Res, ui_dims: Vector2f) {
        loop {}
    }
    pub(crate) fn light_pass(&mut self, lightmap: &mut RenderTexture, res: &Res) {
        loop {}
    }
    pub(crate) fn new(world_name: String, path: PathBuf, res: &Res) -> GameState {
        loop {}
    }
}
pub fn for_each_tile_on_screen(
    camera_offset: WorldPos,
    rt_size: Vector2u,
    mut f: impl FnMut(TilePos, ScreenVec),
) {
    loop {}
}
