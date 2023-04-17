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
pub(crate) struct GameState {
    pub(crate) camera_offset: WorldPos,
    pub(crate) world: World,
    pub(crate) tile_db: TileDb,
}
#[derive(PartialEq, Eq, Clone, Copy, Debug, Inspect)]
pub(crate) enum Biome {
    Surface,
    Underground,
}
impl GameState {
    pub(crate) fn update(&mut self, input: &Input) {
        loop {}
    }
    pub(crate) fn draw_world(&mut self, rt: &mut RenderTexture, res: &mut Res) {
        loop {}
    }
    pub(crate) fn draw_entities(&mut self, rt: &mut RenderTexture) {
        loop {}
    }
    pub(crate) fn draw_ui(
        &mut self,
        rt: &mut RenderTexture,
        res: &Res,
        ui_dims: Vector2f,
    ) {
        loop {}
    }
    pub(crate) fn light_pass(&mut self, lightmap: &mut RenderTexture, res: &Res) {
        loop {}
    }
    pub(crate) fn new(world_name: String, path: PathBuf, res: &Res) -> GameState {
        loop {}
    }
}
pub(crate) fn for_each_tile_on_screen(
    camera_offset: WorldPos,
    rt_size: Vector2u,
    mut f: impl FnMut(TilePos, ScreenVec),
) {
    loop {}
}
