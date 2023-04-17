use std::{collections::HashMap, path::Path};
use crate::math::IntRect;
use sfml::{graphics::Texture, SfBox};
use texture_packer::{texture::Texture as _, TexturePacker, TexturePackerConfig};
pub(crate) type RectMap = HashMap<String, IntRect>;
#[derive(Debug)]
pub(crate) struct AtlasBundle {
    pub(crate) tex: SfBox<Texture>,
    pub(crate) rects: RectMap,
}
impl AtlasBundle {
    pub(crate) fn new() -> anyhow::Result<Self> {
        loop {}
    }
}
