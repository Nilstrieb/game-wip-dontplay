use std::{collections::HashMap, path::Path};
use crate::math::IntRect;
use sfml::{graphics::Texture, SfBox};
use texture_packer::{texture::Texture as _, TexturePacker, TexturePackerConfig};
pub type RectMap = HashMap<String, IntRect>;
#[derive(Debug)]
pub struct AtlasBundle {
    pub tex: SfBox<Texture>,
    pub rects: RectMap,
}
impl AtlasBundle {
    pub fn new() -> anyhow::Result<Self> {
        loop {}
    }
}
fn make_pix_buf(packer: &TexturePacker<image::DynamicImage, String>) -> Vec<u8> {
    loop {}
}
fn path_img_key(path: &Path) -> String {
    loop {}
}
#[test]
fn test_path_img_key() {
    loop {}
}
fn walk_graphics(mut f: impl FnMut(&Path)) {
    loop {}
}
