use sfml::{audio::Music, graphics::Texture, SfBox};

pub struct Res {
    pub tile_atlas: SfBox<Texture>,
    pub surf_music: Music<'static>,
    pub und_music: Music<'static>,
}

impl Res {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            tile_atlas: Texture::from_file("res/tiles.png")?,
            surf_music: Music::from_file("res/music.ogg").unwrap(),
            und_music: Music::from_file("res/cave.ogg").unwrap(),
        })
    }
}
