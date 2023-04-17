use sfml::audio::Music;
use crate::texture_atlas::AtlasBundle;
#[derive(Debug)]
pub struct Res {
    pub atlas: AtlasBundle,
    pub surf_music: Music<'static>,
    pub und_music: Music<'static>,
}
impl Res {
    pub fn load() -> anyhow::Result<Self> {
        loop {}
    }
}
