use sfml::{audio::Music, graphics::Texture, SfBox};

#[derive(Debug)]
pub struct Res {
    pub tile_atlas: SfBox<Texture>,
    pub light_texture: SfBox<Texture>,
    pub surf_music: Music<'static>,
    pub und_music: Music<'static>,
}

impl Res {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            tile_atlas: Texture::from_file("res/graphics/tiles.png")?,
            light_texture: Texture::from_file("res/graphics/light2.png")?,
            surf_music: Music::from_file("res/music/music.ogg").unwrap(),
            und_music: Music::from_file("res/music/cave2.ogg").unwrap(),
        })
    }
}
