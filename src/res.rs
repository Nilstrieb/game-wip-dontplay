use sfml::{
    audio::Music,
    graphics::{Shader, ShaderType, Texture},
    SfBox,
};

pub struct Res {
    pub tile_atlas: SfBox<Texture>,
    pub surf_music: Music<'static>,
    pub und_music: Music<'static>,
    pub lighting_shader: Shader<'static>,
}

impl Res {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self {
            tile_atlas: Texture::from_file("res/tiles.png")?,
            surf_music: Music::from_file("res/music.ogg").unwrap(),
            und_music: Music::from_file("res/cave2.ogg").unwrap(),
            lighting_shader: Shader::from_memory(
                include_str!("../shaders/lighting.glsl"),
                ShaderType::Fragment,
            )?,
        })
    }
}
