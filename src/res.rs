use sfml::{graphics::Texture, SfBox};

pub struct Res {
    pub tile_atlas: SfBox<Texture>,
}

impl Res {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            tile_atlas: Texture::from_file("res/tiles.png")?,
        })
    }
}
