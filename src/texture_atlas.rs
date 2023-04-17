use std::{collections::HashMap, path::Path};
use crate::math::IntRect;
use sfml::{graphics::Texture, SfBox};
use texture_packer::{texture::Texture as _, TexturePacker, TexturePackerConfig};
pub(crate) type RectMap = HashMap<String, IntRect>;
#[derive(Debug)]
pub(crate) struct AtlasBundle {}
